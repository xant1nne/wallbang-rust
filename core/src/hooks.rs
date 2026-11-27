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
const ALLOC_GRANULARITY: usize = 0x1_0000;
const NEAR_STUB_STRIDE: usize = 0x0400_0000;
const NEAR_STUB_ATTEMPTS: usize = 31;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HookError {
    NullAddress,
    NotRelativeCall,
    DetourTooFar,
    NearStubUnavailable,
    ProtectFailed,
}

impl std::fmt::Display for HookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::NullAddress => "null address",
            Self::NotRelativeCall => "not a call rel32",
            Self::DetourTooFar => "detour beyond rel32 range",
            Self::NearStubUnavailable => "no executable page within rel32 range",
            Self::ProtectFailed => "VirtualProtect failed",
        };
        f.write_str(text)
    }
}

/// Displacement of a `call rel32` / `jmp rel32` from the end of the instruction.
fn rel32(from: usize, to: usize) -> Option<i32> {
    i32::try_from((to as i128) - (from as i128)).ok()
}

/// Allocation hints around the caller, on allocation-granularity boundaries.
fn near_stub_candidates(target: usize) -> impl Iterator<Item = usize> {
    let anchor = target & !(ALLOC_GRANULARITY - 1);
    (1..=NEAR_STUB_ATTEMPTS).flat_map(move |step| {
        let delta = step * NEAR_STUB_STRIDE;
        [anchor.wrapping_add(delta), anchor.wrapping_sub(delta)]
    })
}

/// `jmp qword ptr [rip+0]` followed by the absolute detour address.
fn abs_jmp_bytes(detour: usize) -> [u8; ABS_JMP_LEN] {
    let mut jump = [0u8; ABS_JMP_LEN];
    jump[0] = 0xFF;
    jump[1] = 0x25;
    jump[6..ABS_JMP_LEN].copy_from_slice(&(detour as u64).to_le_bytes());
    jump
}

/// Reserves an executable page that a `call rel32` can reach from `target`.
unsafe fn alloc_near(target: usize, size: usize) -> Option<usize> {
    let from = target + REL_CALL_LEN;
    let any = VirtualAlloc(
        std::ptr::null_mut(),
        size,
        MEM_COMMIT | MEM_RESERVE,
        PAGE_EXECUTE_READWRITE,
    ) as usize;
    if any != 0 {
        if rel32(from, any).is_some() {
            return Some(any);
        }
        VirtualFree(any as *mut c_void, 0, MEM_RELEASE);
    }
    for candidate in near_stub_candidates(target) {
        if rel32(from, candidate).is_none() {
            continue;
        }
        let page = VirtualAlloc(
            candidate as *mut c_void,
            size,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        ) as usize;
        if page != 0 {
            if rel32(from, page).is_some() {
                return Some(page);
            }
            VirtualFree(page as *mut c_void, 0, MEM_RELEASE);
        }
    }
    None
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
    /// Optional near `jmp [rip+0]` stub: a manual-mapped payload is often
    /// farther than 2 GiB from client.dll, so the patched call cannot reach
    /// the detour directly. Zero means the detour was in range.
    stub: usize,
    active: bool,
}

impl CallHook {
    pub unsafe fn install(target: usize, detour: usize) -> Option<Self> {
        Self::install_result(target, detour).ok()
    }

    /// Same patch as `install`, with the exact failure reason for the log.
    pub unsafe fn install_result(target: usize, detour: usize) -> Result<Self, HookError> {
        if target == 0 || detour == 0 {
            return Err(HookError::NullAddress);
        }
        let original = std::slice::from_raw_parts(target as *const u8, REL_CALL_LEN).to_vec();
        if original[0] != 0xE8 {
            return Err(HookError::NotRelativeCall);
        }
        let from = target + REL_CALL_LEN;
        let stub = if rel32(from, detour).is_some() {
            0
        } else {
            alloc_near(target, ABS_JMP_LEN).ok_or(HookError::NearStubUnavailable)?
        };
        let destination = if stub == 0 { detour } else { stub };
        let rel = rel32(from, destination).ok_or(HookError::DetourTooFar)?;
        if stub != 0 {
            let jump = abs_jmp_bytes(detour);
            std::ptr::copy_nonoverlapping(jump.as_ptr(), stub as *mut u8, ABS_JMP_LEN);
            FlushInstructionCache(GetCurrentProcess(), stub as *const c_void, ABS_JMP_LEN);
        }
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
            if stub != 0 {
                VirtualFree(stub as *mut c_void, 0, MEM_RELEASE);
            }
            return Err(HookError::ProtectFailed);
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

        Ok(Self {
            target,
            original: original.try_into().expect("five bytes"),
            stub,
            active: true,
        })
    }

    #[inline]
    pub fn uses_stub(&self) -> bool {
        self.stub != 0
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
        if self.stub != 0 {
            VirtualFree(self.stub as *mut c_void, 0, MEM_RELEASE);
            self.stub = 0;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rel32_accepts_near_and_rejects_far_detours() {
        assert_eq!(rel32(0x1000, 0x1100), Some(0x100));
        assert_eq!(rel32(0x1100, 0x1000), Some(-0x100));
        assert_eq!(rel32(0x1000, 0x1000 + i32::MAX as usize + 1), None);
        assert_eq!(rel32(0x1_0000_0000, 0x1000), None);
    }

    #[test]
    fn near_stub_candidates_are_reachable_from_call() {
        let target = 0x7ffb_1234_5678usize;
        let from = target + REL_CALL_LEN;
        let candidates: Vec<usize> = near_stub_candidates(target).collect();
        assert_eq!(candidates.len(), NEAR_STUB_ATTEMPTS * 2);
        for address in candidates {
            assert!(rel32(from, address).is_some(), "candidate {address:X} is out of call range");
        }
    }

    #[test]
    fn abs_jmp_stub_encodes_the_detour_address() {
        let detour = 0x1800_9447_B0usize;
        let bytes = abs_jmp_bytes(detour);
        assert_eq!(&bytes[..6], &[0xFF, 0x25, 0, 0, 0, 0]);
        assert_eq!(&bytes[6..], &(detour as u64).to_le_bytes());
    }

    #[test]
    fn hook_errors_render_for_the_log() {
        assert_eq!(HookError::NearStubUnavailable.to_string(), "no executable page within rel32 range");
        assert_eq!(HookError::NotRelativeCall.to_string(), "not a call rel32");
    }
}
