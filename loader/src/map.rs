// Config block: PE64 mapping uses fixed Win32 allocation flags (MEM_COMMIT | MEM_RESERVE,
// PAGE_EXECUTE_READWRITE) and the standard PE directory indexes; no external config is read.

use std::{
    ffi::CString,
    fmt,
    mem::size_of,
    ptr,
};

use windows::Win32::{
    Foundation::HANDLE,
    System::{
        Diagnostics::Debug::{
            WriteProcessMemory, IMAGE_DATA_DIRECTORY, IMAGE_DIRECTORY_ENTRY_BASERELOC,
            IMAGE_DIRECTORY_ENTRY_IMPORT, IMAGE_DIRECTORY_ENTRY_TLS, IMAGE_FILE_HEADER,
            IMAGE_NT_HEADERS64, IMAGE_SECTION_HEADER,
        },
        LibraryLoader::{GetModuleHandleA, GetProcAddress, LoadLibraryA},
        Memory::{VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE},
        SystemInformation::IMAGE_FILE_MACHINE_AMD64,
        SystemServices::{
            IMAGE_BASE_RELOCATION, IMAGE_DOS_HEADER, IMAGE_IMPORT_DESCRIPTOR,
            IMAGE_REL_BASED_DIR64, IMAGE_TLS_DIRECTORY64,
        },
        Threading::TlsAlloc,
        WindowsProgramming::IMAGE_THUNK_DATA64,
    },
};

use windows::core::PCSTR;

const DOS_MAGIC: u16 = 0x5A4D; // 'MZ'
const PE_MAGIC: u32 = 0x0000_4550; // 'PE\0\0'
const PE64_MAGIC: u16 = 0x020B;
const IMAGE_BASE_OFFSET_IN_NT_HEADERS64: usize = 4 + 20 + 24; // signature + file header + optional header field

/// A manual-mapped image in the target process.
#[derive(Debug)]
pub struct MappedImage {
    pub base: *mut std::ffi::c_void,
    pub size: usize,
    pub entry: usize,
    pub tls: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapError {
    InvalidPe,
    VirtualAlloc,
    Write,
    Reloc,
    Import,
    Tls,
}

impl fmt::Display for MapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            MapError::InvalidPe => "invalid or unsupported PE64 image",
            MapError::VirtualAlloc => "VirtualAllocEx failed in target process",
            MapError::Write => "WriteProcessMemory failed",
            MapError::Reloc => "base relocation failed",
            MapError::Import => "import resolution failed",
            MapError::Tls => "TLS directory processing failed",
        };
        f.write_str(text)
    }
}

impl std::error::Error for MapError {}

/// Read a `Copy` struct from possibly unaligned PE bytes.
unsafe fn read_struct<T: Copy>(bytes: &[u8], offset: usize) -> Result<T, MapError> {
    let end = offset.checked_add(size_of::<T>()).ok_or(MapError::InvalidPe)?;
    if end > bytes.len() {
        return Err(MapError::InvalidPe);
    }
    Ok(ptr::read_unaligned(bytes.as_ptr().add(offset) as *const T))
}

/// Translate an RVA into a file offset using the section table.
fn rva_to_offset(
    rva: u64,
    sections: &[IMAGE_SECTION_HEADER],
    size_of_headers: usize,
) -> Option<usize> {
    if rva < size_of_headers as u64 {
        return Some(rva as usize);
    }

    for section in sections {
        let virtual_address = u64::from(section.VirtualAddress);
        let virtual_size = u64::from(unsafe { section.Misc.VirtualSize });
        let raw_size = u64::from(section.SizeOfRawData);
        let span = virtual_size.max(raw_size);

        if rva >= virtual_address && rva < virtual_address.saturating_add(span) {
            let delta = rva - virtual_address;
            if delta >= raw_size {
                // Tail of a virtual section: no bytes exist in the file.
                return None;
            }
            return u64::from(section.PointerToRawData)
                .checked_add(delta)
                .and_then(|value| usize::try_from(value).ok());
        }
    }
    None
}

/// Read a NUL-terminated byte string (without the terminator).
fn read_cstr(bytes: &[u8], offset: usize) -> Option<&[u8]> {
    let tail = bytes.get(offset..)?;
    let end = tail.iter().position(|&byte| byte == 0)?;
    Some(&tail[..end])
}

/// Overwrite a little-endian u64 in the local image copy.
fn patch_u64(bytes: &mut [u8], offset: usize, value: u64) -> Result<(), MapError> {
    let end = offset.checked_add(8).ok_or(MapError::InvalidPe)?;
    let slot = bytes.get_mut(offset..end).ok_or(MapError::InvalidPe)?;
    slot.copy_from_slice(&value.to_le_bytes());
    Ok(())
}

unsafe fn write_remote(handle: HANDLE, address: usize, data: &[u8]) -> Result<(), MapError> {
    if data.is_empty() {
        return Ok(());
    }
    WriteProcessMemory(
        handle,
        address as *const std::ffi::c_void,
        data.as_ptr() as *const std::ffi::c_void,
        data.len(),
        None,
    )
    .map_err(|_| MapError::Write)
}

/// Accept both a VA left at the original image base and one already rewritten by relocations.
fn va_to_rva(va: u64, original_base: u64, current_base: u64, image_size: usize) -> Option<u64> {
    if va >= current_base {
        let rva = va - current_base;
        if rva < image_size as u64 {
            return Some(rva);
        }
    }
    va.checked_sub(original_base)
        .filter(|rva| *rva < image_size as u64)
}

/// Find an exported function RVA by name (used for `wb_entry` instead of the PE entry point).
pub unsafe fn find_export_rva(dll_bytes: &[u8], wanted: &str) -> Option<u64> {
    let dos: IMAGE_DOS_HEADER = read_struct(dll_bytes, 0).ok()?;
    if dos.e_magic != DOS_MAGIC {
        return None;
    }
    let nt_offset = usize::try_from(i64::from(dos.e_lfanew)).ok()?;
    let nt: IMAGE_NT_HEADERS64 = read_struct(dll_bytes, nt_offset).ok()?;
    let sections_offset = nt_offset
        .checked_add(4)?
        .checked_add(size_of::<IMAGE_FILE_HEADER>())?
        .checked_add(usize::from(nt.FileHeader.SizeOfOptionalHeader))?;
    let count = usize::from(nt.FileHeader.NumberOfSections);
    let mut sections = Vec::with_capacity(count);
    for index in 0..count {
        let offset = sections_offset.checked_add(index.checked_mul(40)?)?;
        sections.push(read_struct::<IMAGE_SECTION_HEADER>(dll_bytes, offset).ok()?);
    }
    let headers_size = (nt.OptionalHeader.SizeOfHeaders as usize)
        .max(sections_offset + count * 40)
        .min(dll_bytes.len());

    let export = &nt.OptionalHeader.DataDirectory[0];
    if export.VirtualAddress == 0 || export.Size == 0 {
        return None;
    }
    let dir_offset = rva_to_offset(u64::from(export.VirtualAddress), &sections, headers_size)?;
    let read_u32 = |field_offset: usize| -> Option<u32> {
        read_struct::<u32>(dll_bytes, dir_offset.checked_add(field_offset)?).ok()
    };
    let function_count = read_u32(20)?;
    let name_count = read_u32(24)?;
    let functions = read_u32(28)?;
    let names = read_u32(32)?;
    let ordinals = read_u32(36)?;

    for index in 0..name_count {
        let name_entry_offset = rva_to_offset(u64::from(names) + u64::from(index) * 4, &sections, headers_size)?;
        let name_rva = read_struct::<u32>(dll_bytes, name_entry_offset).ok()? as u64;
        let name_offset = rva_to_offset(name_rva, &sections, headers_size)?;
        if read_cstr(dll_bytes, name_offset)? != wanted.as_bytes() {
            continue;
        }
        let ordinal_offset = rva_to_offset(u64::from(ordinals) + u64::from(index) * 2, &sections, headers_size)?;
        let ordinal = read_struct::<u16>(dll_bytes, ordinal_offset).ok()? as u32;
        if ordinal >= function_count {
            return None;
        }
        let function_offset = rva_to_offset(u64::from(functions) + u64::from(ordinal) * 4, &sections, headers_size)?;
        return read_struct::<u32>(dll_bytes, function_offset).ok().map(u64::from);
    }
    None
}

/// Manually map a PE64 DLL into `handle`'s address space and resolve its imports.
///
/// The DLL is never written into our own address space: all parsing happens on `dll_bytes`,
/// all patching happens on a local copy plus direct writes into the remote process.
pub unsafe fn map_image(handle: HANDLE, dll_bytes: &[u8]) -> Result<MappedImage, MapError> {
    let dos: IMAGE_DOS_HEADER = read_struct(dll_bytes, 0)?;
    if dos.e_magic != DOS_MAGIC {
        return Err(MapError::InvalidPe);
    }

    let nt_offset = usize::try_from(i64::from(dos.e_lfanew)).map_err(|_| MapError::InvalidPe)?;
    let nt: IMAGE_NT_HEADERS64 = read_struct(dll_bytes, nt_offset)?;
    if nt.Signature != PE_MAGIC || nt.OptionalHeader.Magic.0 != PE64_MAGIC {
        return Err(MapError::InvalidPe);
    }
    if nt.FileHeader.Machine != IMAGE_FILE_MACHINE_AMD64 {
        return Err(MapError::InvalidPe);
    }

    let original_base = nt.OptionalHeader.ImageBase;
    let image_size = nt.OptionalHeader.SizeOfImage as usize;
    let entry_rva = nt.OptionalHeader.AddressOfEntryPoint as usize;
    if image_size == 0 || entry_rva >= image_size {
        return Err(MapError::InvalidPe);
    }

    // Parse the section table before allocating so all header data can be written in one pass.
    let sections_offset = nt_offset
        .checked_add(4)
        .and_then(|value| value.checked_add(size_of::<IMAGE_FILE_HEADER>()))
        .and_then(|value| {
            value.checked_add(usize::from(nt.FileHeader.SizeOfOptionalHeader))
        })
        .ok_or(MapError::InvalidPe)?;
    let section_count = usize::from(nt.FileHeader.NumberOfSections);
    let mut sections = Vec::with_capacity(section_count);
    for index in 0..section_count {
        let offset = sections_offset
            .checked_add(index.checked_mul(size_of::<IMAGE_SECTION_HEADER>()).ok_or(MapError::InvalidPe)?)
            .ok_or(MapError::InvalidPe)?;
        sections.push(read_struct::<IMAGE_SECTION_HEADER>(dll_bytes, offset)?);
    }
    let section_table_end = sections_offset
        .checked_add(section_count.checked_mul(size_of::<IMAGE_SECTION_HEADER>()).ok_or(MapError::InvalidPe)?)
        .ok_or(MapError::InvalidPe)?;

    let remote_base = VirtualAllocEx(
        handle,
        None,
        image_size,
        MEM_COMMIT | MEM_RESERVE,
        PAGE_EXECUTE_READWRITE,
    );
    if remote_base.is_null() {
        return Err(MapError::VirtualAlloc);
    }
    let base = remote_base as usize;
    let delta = (base as u64).wrapping_sub(original_base);

    // Work on a private copy: relocations rewrite absolute pointers before imports/TLS are read.
    let mut image_bytes = dll_bytes.to_vec();
    let headers_size = (nt.OptionalHeader.SizeOfHeaders as usize)
        .max(section_table_end)
        .min(image_bytes.len());
    if headers_size < section_table_end {
        return Err(MapError::InvalidPe);
    }
    patch_u64(
        &mut image_bytes,
        nt_offset + IMAGE_BASE_OFFSET_IN_NT_HEADERS64,
        base as u64,
    )?;
    write_remote(handle, base, &image_bytes[..headers_size])?;

    for section in &sections {
        let raw_size = section.SizeOfRawData as usize;
        if raw_size == 0 {
            continue; // VirtualAllocEx memory is already zero-filled.
        }
        let raw_offset = section.PointerToRawData as usize;
        let raw_end = raw_offset.checked_add(raw_size).ok_or(MapError::InvalidPe)?;
        let data = image_bytes.get(raw_offset..raw_end).ok_or(MapError::InvalidPe)?;
        write_remote(handle, base + section.VirtualAddress as usize, data)?;
    }

    let directories: &[IMAGE_DATA_DIRECTORY; 16] = &nt.OptionalHeader.DataDirectory;

    // Base relocations: only DIR64 entries occur in x64 images, ABSOLUTE entries are padding.
    let reloc = directories[IMAGE_DIRECTORY_ENTRY_BASERELOC.0 as usize];
    if reloc.VirtualAddress != 0 && reloc.Size >= size_of::<IMAGE_BASE_RELOCATION>() as u32 {
        let reloc_end = u64::from(reloc.VirtualAddress) + u64::from(reloc.Size);
        let mut block_rva = u64::from(reloc.VirtualAddress);
        while block_rva + size_of::<IMAGE_BASE_RELOCATION>() as u64 <= reloc_end {
            let block_offset = rva_to_offset(block_rva, &sections, headers_size).ok_or(MapError::Reloc)?;
            let block: IMAGE_BASE_RELOCATION =
                read_struct(&image_bytes, block_offset).map_err(|_| MapError::Reloc)?;
            if block.SizeOfBlock < 8 || block_rva + u64::from(block.SizeOfBlock) > reloc_end {
                return Err(MapError::Reloc);
            }

            let entry_count = (block.SizeOfBlock as usize - 8) / 2;
            let entries_offset = block_offset + 8;
            for index in 0..entry_count {
                let entry: u16 = read_struct(&image_bytes, entries_offset + index * 2)
                    .map_err(|_| MapError::Reloc)?;
                let relocation_type = entry >> 12;
                let offset_in_page = u64::from(entry & 0x0FFF);

                if relocation_type != IMAGE_REL_BASED_DIR64 as u16 {
                    continue; // IMAGE_REL_BASED_ABSOLUTE and anything else need no fixup here.
                }

                let target_rva = u64::from(block.VirtualAddress) + offset_in_page;
                let target_offset = rva_to_offset(target_rva, &sections, headers_size)
                    .ok_or(MapError::Reloc)?;
                let value: u64 = read_struct(&image_bytes, target_offset)
                    .map_err(|_| MapError::Reloc)?;
                let patched = value.wrapping_add(delta);
                patch_u64(&mut image_bytes, target_offset, patched).map_err(|_| MapError::Reloc)?;
                write_remote(handle, base + target_rva as usize, &patched.to_le_bytes())
                    .map_err(|_| MapError::Reloc)?;
            }
            block_rva += u64::from(block.SizeOfBlock);
        }
    }

    // Imports: resolve by name (or ordinal) with GetProcAddress and fill the remote IAT.
    let imports = directories[IMAGE_DIRECTORY_ENTRY_IMPORT.0 as usize];
    if imports.VirtualAddress != 0 && imports.Size >= size_of::<IMAGE_IMPORT_DESCRIPTOR>() as u32 {
        let descriptor_count = imports.Size as usize / size_of::<IMAGE_IMPORT_DESCRIPTOR>();
        for index in 0..descriptor_count {
            let descriptor_rva = u64::from(imports.VirtualAddress)
                + (index * size_of::<IMAGE_IMPORT_DESCRIPTOR>()) as u64;
            let descriptor_offset =
                rva_to_offset(descriptor_rva, &sections, headers_size).ok_or(MapError::Import)?;
            let descriptor: IMAGE_IMPORT_DESCRIPTOR =
                read_struct(&image_bytes, descriptor_offset).map_err(|_| MapError::Import)?;

            if descriptor.Name == 0 && descriptor.FirstThunk == 0 {
                break; // null terminator of the descriptor array
            }

            let name_offset =
                rva_to_offset(u64::from(descriptor.Name), &sections, headers_size).ok_or(MapError::Import)?;
            let dll_name = read_cstr(&image_bytes, name_offset).ok_or(MapError::Import)?;
            let dll_name = CString::new(dll_name).map_err(|_| MapError::Import)?;
            let name_ptr = PCSTR::from_raw(dll_name.as_ptr() as *const u8);

            // System DLLs are already mapped in the loader; load others into our process
            // so that GetProcAddress can find them (stable addresses across processes).
            let module = GetModuleHandleA(name_ptr)
                .unwrap_or_else(|_| LoadLibraryA(name_ptr).unwrap_or_default());
            if module.0.is_null() {
                return Err(MapError::Import);
            }

            let original_first_thunk = unsafe { descriptor.Anonymous.OriginalFirstThunk };
            let lookup_rva = if original_first_thunk != 0 {
                u64::from(original_first_thunk)
            } else {
                u64::from(descriptor.FirstThunk)
            };

            let mut thunk_index: u64 = 0;
            loop {
                let lookup_address = lookup_rva + thunk_index * 8;
                let lookup_offset = rva_to_offset(lookup_address, &sections, headers_size)
                    .ok_or(MapError::Import)?;
                let thunk: IMAGE_THUNK_DATA64 =
                    read_struct(&image_bytes, lookup_offset).map_err(|_| MapError::Import)?;
                let value = unsafe { thunk.u1.AddressOfData };
                if value == 0 {
                    break;
                }

                let procedure = if value & (1u64 << 63) != 0 {
                    let ordinal = (value & 0xFFFF) as usize;
                    GetProcAddress(module, PCSTR::from_raw(ordinal as *const u8))
                } else {
                    let name_offset = rva_to_offset(value, &sections, headers_size)
                        .ok_or(MapError::Import)?;
                    // IMAGE_IMPORT_BY_NAME: u16 hint followed by a NUL-terminated name.
                    let function_name = read_cstr(&image_bytes, name_offset + 2)
                        .ok_or(MapError::Import)?;
                    let function_name =
                        CString::new(function_name).map_err(|_| MapError::Import)?;
                    GetProcAddress(
                        module,
                        PCSTR::from_raw(function_name.as_ptr() as *const u8),
                    )
                };
                let procedure = procedure.ok_or(MapError::Import)?;

                let iat_rva = u64::from(descriptor.FirstThunk) + thunk_index * 8;
                if iat_rva + 8 > image_size as u64 {
                    return Err(MapError::Import);
                }
                let address = (procedure as usize as u64).to_le_bytes();
                write_remote(handle, base + iat_rva as usize, &address)
                    .map_err(|_| MapError::Import)?;

                thunk_index += 1;
                if thunk_index > 1 << 20 {
                    return Err(MapError::Import);
                }
            }
        }
    }

    // TLS: initialise the index slot and collect callbacks for the caller to invoke.
    let mut tls = Vec::new();
    let tls_directory = directories[IMAGE_DIRECTORY_ENTRY_TLS.0 as usize];
    if tls_directory.VirtualAddress != 0
        && tls_directory.Size >= size_of::<IMAGE_TLS_DIRECTORY64>() as u32
    {
        let directory_offset =
            rva_to_offset(u64::from(tls_directory.VirtualAddress), &sections, headers_size)
                .ok_or(MapError::Tls)?;
        let tls_dir: IMAGE_TLS_DIRECTORY64 =
            read_struct(&image_bytes, directory_offset).map_err(|_| MapError::Tls)?;

        if tls_dir.AddressOfIndex != 0 {
            let index_rva = va_to_rva(
                tls_dir.AddressOfIndex,
                original_base,
                base as u64,
                image_size,
            )
            .ok_or(MapError::Tls)?;
            let index = TlsAlloc();
            if index != u32::MAX {
                write_remote(handle, base + index_rva as usize, &index.to_le_bytes())
                    .map_err(|_| MapError::Tls)?;
            }
        }

        if tls_dir.AddressOfCallBacks != 0 {
            let callbacks_rva = va_to_rva(
                tls_dir.AddressOfCallBacks,
                original_base,
                base as u64,
                image_size,
            )
            .ok_or(MapError::Tls)?;
            let callbacks_offset =
                rva_to_offset(callbacks_rva, &sections, headers_size).ok_or(MapError::Tls)?;

            for index in 0..4096usize {
                let callback: u64 = read_struct(&image_bytes, callbacks_offset + index * 8)
                    .map_err(|_| MapError::Tls)?;
                if callback == 0 {
                    break;
                }
                let callback_rva = va_to_rva(callback, original_base, base as u64, image_size)
                    .ok_or(MapError::Tls)?;
                tls.push(base + callback_rva as usize);
            }
        }
    }

    Ok(MappedImage {
        base: remote_base,
        size: image_size,
        entry: base + entry_rva,
        tls,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn finds_wb_entry_export() {
        // The fixture is the release cdylib. A clean tree has no release build
        // yet, so skip instead of failing the whole suite; run
        // `cargo build --release --workspace` to exercise this check.
        let Ok(bytes) = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../target/release/wallbang.dll"
        )) else {
            eprintln!("skip: target/release/wallbang.dll is not built");
            return;
        };
        assert!(unsafe { super::find_export_rva(&bytes, "wb_entry") }.is_some());
    }
}
