// Config: verified locally in engine2.dll build 14201; no guessed vtable calls.
use crate::{memory, modules};
use std::ffi::CString;
const EXECUTE_SLOT: usize = 51;
const EXECUTE_RVA: usize = 0x738D0;
const LEVEL_SLOT: usize = 65;
const LEVEL_RVA: usize = 0x74020;

fn verified(slot: usize, rva: usize) -> Option<(usize, usize)> {
    if modules::build_supported() != Some(true) { return None; }
    let instance = modules::create_interface("engine2.dll", "Source2EngineToClient001")?;
    let base = modules::module_base("engine2.dll")?;
    let table = unsafe { memory::read_ptr(instance)? };
    let function = unsafe { memory::read_ptr(table + slot * 8)? };
    (function == base + rva).then_some((instance, function))
}

/// Must run on the game thread; supplies both stack arguments used by this build.
pub unsafe fn execute(command: &str) -> bool {
    let Some((instance, function)) = verified(EXECUTE_SLOT, EXECUTE_RVA) else { return false; };
    let Ok(command) = CString::new(command) else { return false; };
    let call: unsafe extern "system" fn(usize, i32, *const i8, bool, f64, u64) = std::mem::transmute(function);
    call(instance, 0, command.as_ptr(), true, f64::NAN, 0);
    true
}

pub fn map_name() -> Option<String> {
    let (instance, function) = verified(LEVEL_SLOT, LEVEL_RVA)?;
    unsafe {
        let call: unsafe extern "system" fn(usize) -> usize = std::mem::transmute(function);
        let text = memory::read_cstr(call(instance), 128)?;
        (!text.is_empty()).then_some(text)
    }
}
