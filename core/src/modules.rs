// Config block: engine2::dwBuildNumber, inputsystem::dwInputSystem, client.dll dw* офсеты.
use crate::{memory, pattern::Pattern};
use std::ffi::{c_char, c_int, c_void};
use wallbang_config::{
    offsets::{DW_CSGO_INPUT, DW_INPUT_SYSTEM, ENGINE_BUILD_NUMBER},
    signatures::{SIG_INPUT_SYSTEM_DISP, SIG_INPUT_SYSTEM_SVC},
};

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleA(name: *const c_char) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const c_char) -> *mut c_void;
}

type CreateInterfaceFn = unsafe extern "C" fn(*const c_char, *mut c_int) -> *mut c_void;

#[link(name = "user32")]
extern "system" {
    fn GetSystemMetrics(index: c_int) -> c_int;
}

const SM_CXSCREEN: c_int = 0;
const SM_CYSCREEN: c_int = 1;

use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

static CLIENT_BASE: AtomicUsize = AtomicUsize::new(0);
static CLIENT_SIZE: AtomicUsize = AtomicUsize::new(0);
static ENGINE2_BASE: AtomicUsize = AtomicUsize::new(0);
static ENGINE2_SIZE: AtomicUsize = AtomicUsize::new(0);
static CACHED_BUILD: AtomicU32 = AtomicU32::new(0);

#[inline]
pub fn module_base(name: &str) -> Option<usize> {
    let cache = match name {
        "client.dll" => Some(&CLIENT_BASE),
        "engine2.dll" => Some(&ENGINE2_BASE),
        _ => None,
    };
    if let Some(slot) = cache {
        let cached = slot.load(Ordering::Relaxed);
        if cached != 0 {
            return Some(cached);
        }
    }
    let mut buf = [0u8; 64];
    let bytes = name.as_bytes();
    if bytes.len() >= buf.len() {
        return None;
    }
    buf[..bytes.len()].copy_from_slice(bytes);
    let ptr = unsafe { GetModuleHandleA(buf.as_ptr() as *const c_char) };
    if ptr.is_null() {
        None
    } else {
        let value = ptr as usize;
        if let Some(slot) = cache {
            slot.store(value, Ordering::Relaxed);
        }
        Some(value)
    }
}

pub fn export(module: &str, name: &std::ffi::CStr) -> Option<usize> {
    let base = module_base(module)?;
    let address = unsafe { GetProcAddress(base as *mut c_void, name.as_ptr()) } as usize;
    (address != 0).then_some(address)
}

/// PE SizeOfImage загруженного модуля.
#[inline]
pub fn module_size(name: &str) -> Option<usize> {
    let cache = match name {
        "client.dll" => Some(&CLIENT_SIZE),
        "engine2.dll" => Some(&ENGINE2_SIZE),
        _ => None,
    };
    if let Some(slot) = cache {
        let cached = slot.load(Ordering::Relaxed);
        if cached != 0 {
            return Some(cached);
        }
    }
    let base = module_base(name)?;
    unsafe {
        let e_lfanew = memory::read::<u32>(base + 0x3C)? as usize;
        let size = memory::read::<u32>(base + e_lfanew + 0x50)?;
        if size == 0 {
            None
        } else {
            let value = size as usize;
            if let Some(slot) = cache {
                slot.store(value, Ordering::Relaxed);
            }
            Some(value)
        }
    }
}

/// Ищет экспорт CreateInterface в модуле и запрашивает именованный интерфейс.
#[inline]
pub fn create_interface(module: &str, name: &str) -> Option<usize> {
    let base = module_base(module)?;
    let proc = unsafe { GetProcAddress(base as *mut c_void, b"CreateInterface\0".as_ptr() as *const c_char) };
    if proc.is_null() {
        return None;
    }
    let mut name_buf = [0u8; 64];
    let bytes = name.as_bytes();
    if bytes.len() >= name_buf.len() {
        return None;
    }
    name_buf[..bytes.len()].copy_from_slice(bytes);
    let create: CreateInterfaceFn = unsafe { std::mem::transmute(proc) };
    let mut return_code = 0;
    let ptr = unsafe { create(name_buf.as_ptr() as *const c_char, &mut return_code) };
    if ptr.is_null() {
        None
    } else {
        Some(ptr as usize)
    }
}

#[inline]
pub fn build_number() -> Option<u32> {
    let cached = CACHED_BUILD.load(Ordering::Relaxed);
    if cached != 0 {
        return Some(cached);
    }
    let base = module_base("engine2.dll")?;
    let value = unsafe { memory::read::<u32>(base + ENGINE_BUILD_NUMBER) }?;
    if value != 0 {
        CACHED_BUILD.store(value, Ordering::Relaxed);
    }
    Some(value)
}

/// `Some(true)` — билд тот, под который сняты офсеты; `Some(false)` — билд
/// другой (работать с текущими адресами нельзя); `None` — engine2 ещё не
/// загружен или dwBuildNumber не читается, то есть проверить нечем.
#[inline]
pub fn build_supported() -> Option<bool> {
    build_number().map(wallbang_config::offsets::build_matches)
}

#[inline]
fn csgo_input_from_base(client: usize) -> usize {
    client + DW_CSGO_INPUT
}

#[inline]
pub fn csgo_input() -> Option<usize> {
    let client = module_base("client.dll")?;
    // `dwCSGOInput` is the global CCSGOInput object itself, not a pointer to
    // it: its first qword is the vtable. Dereferencing the global returned the
    // read-only vtable in .rdata, and movement then crashed writing
    // m_flForwardMove/m_flLeftMove into that page.
    Some(csgo_input_from_base(client))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csgo_input_points_at_the_object_not_its_vtable() {
        let base = 0x7FFE_6D4A_0000usize;
        assert_eq!(csgo_input_from_base(base), base + 0x2574560);
        assert_ne!(csgo_input_from_base(base), base + 0x1C9D4B0);
    }
}

#[inline]
pub fn input_system() -> Option<usize> {
    let base = module_base("inputsystem.dll")?;
    // STATIC офсет совпадает с pInputSystemSvc из cs2-sdk (RVA 0x46BC0), но на всякий
    // случай падаем back к сигнатуре и RIP-relative разборке.
    if let Some(pointer) = unsafe { memory::read_ptr(base + DW_INPUT_SYSTEM) } {
        if pointer != 0 {
            return Some(pointer);
        }
    }
    let pattern = Pattern::parse(SIG_INPUT_SYSTEM_SVC)?;
    let size = module_size("inputsystem.dll")?;
    let match_address = pattern.scan(base, size)?;
    let disp = unsafe { memory::read::<i32>(match_address + SIG_INPUT_SYSTEM_DISP)? } as isize;
    let global = (match_address + SIG_INPUT_SYSTEM_DISP + 4).wrapping_add_signed(disp);
    unsafe { memory::read_ptr(global) }
}

#[inline]
pub fn screen_size() -> (f32, f32) {
    unsafe {
        (
            GetSystemMetrics(SM_CXSCREEN) as f32,
            GetSystemMetrics(SM_CYSCREEN) as f32,
        )
    }
}
