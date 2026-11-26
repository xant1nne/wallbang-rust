// Config block: vtable-слоты задаются вызывающим (Present=8, Present1=22, ResizeBuffers=13/39).
// PrologueHook ворует ровно `expected.len()` байт пролога и проверяет их перед патчем.
use crate::memory;
use std::ffi::c_void;

const PAGE_EXECUTE_READWRITE: u32 = 0x40;
const MEM_COMMIT: u32 = 0x1000;
const MEM_RESERVE: u32 = 0x2000;
const MEM_RELEASE: u32 = 0x8000;
const ABS_JMP_LEN: usize = 14;
const REL_CALL_LEN: usize = 5;

#[link(name = "kernel32")]
extern "system" {
    fn VirtualProtect(
        address: *mut c_void,
        size: usize,
        protect: u32,
        old: *mut u32,
    ) -> i32;
    fn VirtualAlloc(address: *mut c_void, size: usize, kind: u32, protect: u32) -> *mut c_void;
    fn VirtualFree(address: *mut c_void, size: usize, kind: u32) -> i32;
    fn FlushInstructionCache(process: *mut c_void, address: *const c_void, size: usize) -> i32;
    fn GetCurrentProcess() -> *mut c_void;
}

pub struct VTableHook {
    vtable: usize,
    index: usize,
    original: usize,
    detour: usize,
}

impl VTableHook {
    /// Меняет один слот vtable, возвращая trampoline-адрес original.
    pub unsafe fn install(instance: usize, index: usize, detour: usize) -> Option<Self> {
        if instance == 0 || detour == 0 {
            return None;
        }
        let vtable = memory::read_ptr(instance)?;
        let slot = vtable + index * std::mem::size_of::<usize>();
        let original = memory::read_ptr(slot)?;
        let mut old_protect = 0u32;
        if VirtualProtect(
            slot as *mut c_void,
            std::mem::size_of::<usize>(),
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        ) == 0
        {
            return None;
        }
        if !memory::write(slot, detour) {
            VirtualProtect(
                slot as *mut c_void,
                std::mem::size_of::<usize>(),
                old_protect,
                &mut old_protect,
            );
            return None;
        }
        VirtualProtect(
            slot as *mut c_void,
            std::mem::size_of::<usize>(),
            old_protect,
            &mut old_protect,
        );
        Some(Self {
            vtable,
            index,
            original,
            detour,
        })
    }

    #[inline]
    pub fn original(&self) -> usize {
        self.original
    }

    /// Патчит конкретный слот по его адресу (когда vtable-инстанс не нужен):
    /// поиск указателя на функцию в образе модуля даёт адрес слота напрямую.
    pub unsafe fn install_slot(slot: usize, detour: usize) -> Option<Self> {
        if slot == 0 || detour == 0 {
            return None;
        }
        let original = memory::read_ptr(slot)?;
        let mut old_protect = 0u32;
        if VirtualProtect(
            slot as *mut c_void,
            std::mem::size_of::<usize>(),
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        ) == 0
        {
            return None;
        }
        if !memory::write(slot, detour) {
            VirtualProtect(
                slot as *mut c_void,
                std::mem::size_of::<usize>(),
                old_protect,
                &mut old_protect,
            );
            return None;
        }
        VirtualProtect(
            slot as *mut c_void,
            std::mem::size_of::<usize>(),
            old_protect,
            &mut old_protect,
        );
        // vtable == slot, index 0 => restore() пишет обратно ровно этот слот.
        Some(Self {
            vtable: slot,
            index: 0,
            original,
            detour,
        })
    }

    #[inline]
    pub fn detour(&self) -> usize {
        self.detour
    }

    pub unsafe fn restore(&self) {
        let slot = self.vtable + self.index * std::mem::size_of::<usize>();
        let mut old_protect = 0u32;
        if VirtualProtect(
            slot as *mut c_void,
            std::mem::size_of::<usize>(),
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        ) != 0
        {
            memory::write(slot, self.original);
            VirtualProtect(
                slot as *mut c_void,
                std::mem::size_of::<usize>(),
                old_protect,
                &mut old_protect,
            );
        }
    }
}

/// Пролог-хук: копирует `expected.len()` байт в RWX-трамплин, ставит `FF 25` прыжок.
/// Патчит только когда байты пролога реально совпали с сигнатурой (защита от патча игры).
pub struct PrologueHook {
    target: usize,
    len: usize,
    original: Vec<u8>,
    trampoline: usize,
    active: bool,
}

impl PrologueHook {
    pub unsafe fn install(target: usize, detour: usize, expected: &[Option<u8>]) -> Option<Self> {
        let len = expected.len();
        if target == 0 || detour == 0 || len < ABS_JMP_LEN || len > 64 {
            return None;
        }
        for (i, byte) in expected.iter().enumerate() {
            if let Some(byte) = byte {
                if *((target + i) as *const u8) != *byte {
                    return None;
                }
            }
        }
        let trampoline = VirtualAlloc(
            std::ptr::null_mut(),
            4096,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        ) as usize;
        if trampoline == 0 {
            return None;
        }
        let original = std::slice::from_raw_parts(target as *const u8, len).to_vec();
        std::ptr::copy_nonoverlapping(original.as_ptr(), trampoline as *mut u8, len);

        let mut back = [0u8; ABS_JMP_LEN];
        back[0] = 0xFF;
        back[1] = 0x25;
        back[6..14].copy_from_slice(&((target + len) as u64).to_le_bytes());
        std::ptr::copy_nonoverlapping(back.as_ptr(), (trampoline + len) as *mut u8, ABS_JMP_LEN);

        let mut patch = [0x90u8; 64];
        patch[0] = 0xFF;
        patch[1] = 0x25;
        patch[6..14].copy_from_slice(&(detour as u64).to_le_bytes());

        let mut old_protect = 0u32;
        if VirtualProtect(
            target as *mut c_void,
            len,
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        ) == 0
        {
            VirtualFree(trampoline as *mut c_void, 0, MEM_RELEASE);
            return None;
        }
        std::ptr::copy_nonoverlapping(patch.as_ptr(), target as *mut u8, len);
        let mut restored = 0u32;
        VirtualProtect(target as *mut c_void, len, old_protect, &mut restored);
        FlushInstructionCache(GetCurrentProcess(), target as *const c_void, len);
        FlushInstructionCache(
            GetCurrentProcess(),
            trampoline as *const c_void,
            len + ABS_JMP_LEN,
        );
        Some(Self {
            target,
            len,
            original,
            trampoline,
            active: true,
        })
    }

    #[inline]
    pub fn trampoline(&self) -> usize {
        self.trampoline
    }

    #[inline]
    pub fn target(&self) -> usize {
        self.target
    }

    pub unsafe fn remove(&mut self) {
        if !self.active {
            return;
        }
        let mut old_protect = 0u32;
        if VirtualProtect(
            self.target as *mut c_void,
            self.len,
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        ) != 0
        {
            std::ptr::copy_nonoverlapping(
                self.original.as_ptr(),
                self.target as *mut u8,
                self.len,
            );
            let mut restored = 0u32;
            VirtualProtect(
                self.target as *mut c_void,
                self.len,
                old_protect,
                &mut restored,
            );
            FlushInstructionCache(GetCurrentProcess(), self.target as *const c_void, self.len);
        }
        VirtualFree(self.trampoline as *mut c_void, 0, MEM_RELEASE);
        self.trampoline = 0;
        self.active = false;
    }
}

impl Drop for PrologueHook {
    fn drop(&mut self) {
        unsafe { self.remove() };
    }
}

/// Replaces a single `call rel32` instruction with a call to a detour.
///
/// Unlike `PrologueHook`, this does not copy the callee's prologue, so it is
/// safe for functions whose stolen bytes contain relative branches. The
/// detour calls the original callee by address and returns to the instruction
/// after the patched call, preserving the caller's stack.
pub struct CallHook {
    target: usize,
    original: [u8; REL_CALL_LEN],
    active: bool,
}

impl CallHook {
    pub unsafe fn install(target: usize, detour: usize) -> Option<Self> {
        if target == 0 || detour == 0 {
            return None;
        }
        let original = std::slice::from_raw_parts(target as *const u8, REL_CALL_LEN).to_vec();
        if original[0] != 0xE8 {
            return None;
        }
        let rel = detour as isize - (target + REL_CALL_LEN) as isize;
        let Ok(rel) = i32::try_from(rel) else {
            return None;
        };
        let mut bytes = [0xE8u8, 0, 0, 0, 0];
        bytes[1..].copy_from_slice(&rel.to_le_bytes());

        let mut old_protect = 0u32;
        if VirtualProtect(
            target as *mut c_void,
            REL_CALL_LEN,
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        ) == 0
        {
            return None;
        }
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), target as *mut u8, REL_CALL_LEN);
        let mut restored = 0u32;
        VirtualProtect(
            target as *mut c_void,
            REL_CALL_LEN,
            old_protect,
            &mut restored,
        );
        FlushInstructionCache(GetCurrentProcess(), target as *const c_void, REL_CALL_LEN);

        Some(Self {
            target,
            original: original.try_into().ok()?,
            active: true,
        })
    }

    /// Address the patched instruction originally called.
    pub fn original_target(&self) -> usize {
        let rel = i32::from_le_bytes(self.original[1..].try_into().unwrap()) as isize;
        (self.target + REL_CALL_LEN)
            .wrapping_add(rel as usize)
    }

    #[inline]
    pub fn target(&self) -> usize {
        self.target
    }

    pub unsafe fn remove(&mut self) {
        if !self.active {
            return;
        }
        let mut old_protect = 0u32;
        if VirtualProtect(
            self.target as *mut c_void,
            REL_CALL_LEN,
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        ) != 0
        {
            std::ptr::copy_nonoverlapping(
                self.original.as_ptr(),
                self.target as *mut u8,
                REL_CALL_LEN,
            );
            let mut restored = 0u32;
            VirtualProtect(
                self.target as *mut c_void,
                REL_CALL_LEN,
                old_protect,
                &mut restored,
            );
            FlushInstructionCache(GetCurrentProcess(), self.target as *const c_void, REL_CALL_LEN);
        }
        self.active = false;
    }
}

impl Drop for CallHook {
    fn drop(&mut self) {
        unsafe { self.remove() };
    }
}

/// Парсит сигнатуру в список байт с wildcard'ами (для PrologueHook::install).
pub fn parse_prefix(pattern: &str) -> Option<Vec<Option<u8>>> {
    let mut out = Vec::new();
    for part in pattern.split_whitespace() {
        if part == "?" || part == "??" {
            out.push(None);
        } else {
            out.push(Some(u8::from_str_radix(part, 16).ok()?));
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}
