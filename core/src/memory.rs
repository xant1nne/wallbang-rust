// Config block: всё чтение адресов идёт через эти функции; запись — только через MutationGuard.
// Manual-map: пока игра не инициализировалась, указатели из client.dll могут быть
// мусорными, поэтому каждое чтение проверяется через VirtualQuery.
use std::ffi::c_void;

#[link(name = "kernel32")]
extern "system" {
    fn VirtualQuery(
        address: *const c_void,
        buffer: *mut MemoryBasicInformation,
        length: usize,
    ) -> usize;
}

#[repr(C)]
struct MemoryBasicInformation {
    base_address: *mut c_void,
    allocation_base: *mut c_void,
    allocation_protect: u32,
    partition_id: u16,
    _padding: u16,
    region_size: usize,
    state: u32,
    protect: u32,
    region_type: u32,
    _padding2: u32,
}

const MEM_COMMIT: u32 = 0x1000;
const PAGE_GUARD: u32 = 0x100;

fn readable_protect(protect: u32) -> bool {
    if protect & PAGE_GUARD != 0 {
        return false;
    }
    matches!(
        protect & 0xFF,
        0x02 | 0x04 | 0x08 | 0x20 | 0x40 | 0x80
    )
}

fn region(address: usize) -> Option<MemoryBasicInformation> {
    let mut info: MemoryBasicInformation = unsafe { std::mem::zeroed() };
    let len = std::mem::size_of::<MemoryBasicInformation>();
    let written = unsafe { VirtualQuery(address as *const c_void, &mut info, len) };
    if written == len {
        Some(info)
    } else {
        None
    }
}

fn range_readable(address: usize, size: usize) -> bool {
    if address == 0 || size == 0 {
        return false;
    }
    let end = match address.checked_add(size) {
        Some(end) => end,
        None => return false,
    };
    let mut cursor = address;
    while cursor < end {
        let Some(info) = region(cursor) else {
            return false;
        };
        if info.state != MEM_COMMIT || !readable_protect(info.protect) {
            return false;
        }
        let start = info.base_address as usize;
        let region_end = match start.checked_add(info.region_size) {
            Some(value) => value,
            None => return false,
        };
        if cursor < start || cursor >= region_end {
            return false;
        }
        let next = region_end.min(end);
        if next <= cursor {
            return false;
        }
        cursor = next;
    }
    true
}

/// Validates one range up front. Hot paths call this once per entity and then
/// use `read_unchecked` for every field instead of paying a VirtualQuery syscall
/// per field (which cost several milliseconds per world snapshot).
#[inline]
pub unsafe fn validate_range(address: usize, size: usize) -> bool {
    range_readable(address, size)
}

#[inline]
pub unsafe fn read_unchecked<T: Copy>(address: usize) -> Option<T> {
    if address == 0 {
        return None;
    }
    Some(std::ptr::read_unaligned(address as *const T))
}

#[inline]
pub unsafe fn read_cstr_unchecked(address: usize, max: usize) -> Option<String> {
    if address == 0 || max == 0 {
        return None;
    }
    let mut bytes = Vec::with_capacity(max);
    for offset in 0..max {
        let byte = *((address + offset) as *const u8);
        if byte == 0 {
            break;
        }
        bytes.push(byte);
    }
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

#[inline]
pub unsafe fn read<T: Copy>(address: usize) -> Option<T> {
    if !range_readable(address, std::mem::size_of::<T>()) {
        return None;
    }
    Some(std::ptr::read_unaligned(address as *const T))
}

#[inline]
pub unsafe fn read_ptr(address: usize) -> Option<usize> {
    read::<usize>(address)
}

#[inline]
pub unsafe fn read_u8(address: usize) -> Option<u8> {
    read::<u8>(address)
}

#[inline]
pub unsafe fn read_i32(address: usize) -> Option<i32> {
    read::<i32>(address)
}

#[inline]
pub unsafe fn read_f32(address: usize) -> Option<f32> {
    read::<f32>(address)
}

#[inline]
pub unsafe fn read_cstr(address: usize, max: usize) -> Option<String> {
    if max == 0 {
        return None;
    }
    let mut bytes = Vec::with_capacity(max);
    for offset in 0..max {
        let byte = read_u8(address.wrapping_add(offset))?;
        if byte == 0 {
            break;
        }
        bytes.push(byte);
    }
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

#[inline]
pub unsafe fn vfunc<T: Copy>(instance: usize, index: usize) -> Option<T> {
    let vtable = read_ptr(instance)?;
    read::<T>(vtable + index * std::mem::size_of::<usize>())
}

#[inline]
pub unsafe fn write<T: Copy>(address: usize, value: T) -> bool {
    if address == 0 {
        return false;
    }
    std::ptr::write_unaligned(address as *mut T, value);
    true
}

#[inline]
pub unsafe fn call<T>(address: usize) -> Option<T> {
    if address == 0 {
        return None;
    }
    let f: fn() -> T = std::mem::transmute(address);
    Some(f())
}

#[allow(dead_code)]
#[inline]
pub unsafe fn as_ptr(address: usize) -> *mut c_void {
    address as *mut c_void
}
