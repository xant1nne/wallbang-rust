// Guarded ConVar backend for the C3 scene controls.
//
// Layout verified offline on build 14201 engine2.dll:
//   ICvar + 0x40 -> ConVar* array, ICvar + 0x58 -> count
//   ConVar + 0x00 -> name string, + 0x28 -> type, + 0x30 -> flags
//   ConVar + 0x58 -> typed values[] (stride comes from the type table)
// Types: 0 bool, 1 int16, 2 uint16, 3 int32, 4 uint32, 5 int64,
//        6 uint64, 7 float32, 8 float64 (9+ are complex and not writable).
//
// Every write goes through MutationGuard and remembers the original 64 bits.
// The guard is kept with the override so a later "allow writes = off" cannot
// prevent restoration.
use parking_lot::Mutex;
use std::sync::OnceLock;
use wallbang_config::collections::HashMap;
use wallbang_core::memory;
use wallbang_core::modules;
use wallbang_core::mutation::{self, MutationGuard};

// Layout verified offline on build 14201 tier0.dll (CCvar::GetNextCvarIterator):
//   VEngineCvar007 is exported by tier0.dll (not engine2.dll).
//   ICvar + 0x4A -> u16 capacity/count (low 15 bits)
//   ICvar + 0x50 -> CUtlLinkedList element array (stride 16 bytes, ConVar* at +0)
//   ConVar + 0x00 -> name string, + 0x28 -> type, + 0x30 -> flags
//   ConVar + 0x58 -> typed values[]
const ICVAR_COUNT: usize = 0x4A;
const ICVAR_ARRAY: usize = 0x50;
const ICVAR_STRIDE: usize = 16;
const CVAR_NAME: usize = 0x00;
const CVAR_TYPE: usize = 0x28;
const CVAR_VALUES: usize = 0x58;
const MAX_CVARS: usize = 32_767;
const MAX_NAME: usize = 128;

const TYPE_BOOL: i16 = 0;
const TYPE_I16: i16 = 1;
const TYPE_U16: i16 = 2;
const TYPE_I32: i16 = 3;
const TYPE_U32: i16 = 4;
const TYPE_I64: i16 = 5;
const TYPE_U64: i16 = 6;
const TYPE_F32: i16 = 7;
const TYPE_F64: i16 = 8;

struct Backend {
    array: usize,
    count: usize,
}

#[derive(Clone, Copy)]
struct Cvar {
    address: usize,
}

struct Override {
    key: &'static str,
    cvar: Cvar,
    previous: u64,
    guard: MutationGuard,
}

static BACKEND: Mutex<Option<Backend>> = Mutex::new(None);
static OVERRIDES: Mutex<Vec<Override>> = Mutex::new(Vec::new());
/// Name lookup is O(number of engine cvars); the address is stable for the
/// lifetime of the backend, so remember it instead of rescanning every frame.
static ADDRESS_CACHE: OnceLock<Mutex<HashMap<String, Option<usize>>>> = OnceLock::new();

fn address_cache() -> &'static Mutex<HashMap<String, Option<usize>>> {
    ADDRESS_CACHE.get_or_init(|| Mutex::new(HashMap::default()))
}

/// Resolves ICvar and validates the cvar array. Returns the number of cvars
/// exposed by the engine, or 0 when the layout does not match this build.
pub fn init() -> usize {
    let Some(icvar) = modules::create_interface("tier0.dll", "VEngineCvar007") else {
        return 0;
    };
    let (Some(base), Some(size)) = (modules::module_base("tier0.dll"), modules::module_size("tier0.dll")) else {
        return 0;
    };
    let Some(vtable) = (unsafe { memory::read_ptr(icvar) }) else {
        return 0;
    };
    if vtable < base || vtable >= base.saturating_add(size) {
        return 0;
    }
    let (Some(array), Some(raw_count)) = (
        unsafe { memory::read_ptr(icvar + ICVAR_ARRAY) },
        unsafe { memory::read::<u16>(icvar + ICVAR_COUNT) },
    ) else {
        return 0;
    };
    let count = (raw_count & 0x7FFF) as usize;
    if count == 0 || count > MAX_CVARS || !unsafe { memory::validate_range(array, count * ICVAR_STRIDE) } {
        return 0;
    }
    let mut backend = BACKEND.lock();
    *backend = Some(Backend { array, count });
    drop(backend);
    address_cache().lock().clear();
    count
}

pub fn available() -> bool {
    BACKEND.lock().is_some()
}

/// Number of live guarded overrides, for diagnostics.
pub fn active_overrides() -> usize {
    OVERRIDES.lock().len()
}

pub fn shutdown() {
    restore_all();
    *BACKEND.lock() = None;
    address_cache().lock().clear();
}

/// Finds a cvar by name (case-insensitive, as the engine console does).
pub fn find(name: &str) -> Option<usize> {
    if let Some(hit) = address_cache().lock().get(name).copied() {
        return hit;
    }
    let found = {
        let backend = BACKEND.lock();
        let backend = backend.as_ref()?;
        let mut found = None;
        for index in 0..backend.count {
            let Some(address) = (unsafe { memory::read_unchecked::<usize>(backend.array + index * ICVAR_STRIDE) }) else {
                continue;
            };
            if !memory::is_plausible_ptr(address) || !unsafe { memory::validate_range(address, CVAR_VALUES + 8) } {
                continue;
            }
            let Some(name_ptr) = (unsafe { memory::read_unchecked::<usize>(address + CVAR_NAME) }) else {
                continue;
            };
            if !memory::is_plausible_ptr(name_ptr) || !unsafe { memory::validate_range(name_ptr, MAX_NAME) } {
                continue;
            }
            let Some(candidate) = (unsafe { memory::read_cstr_unchecked(name_ptr, MAX_NAME) }) else {
                continue;
            };
            if candidate.eq_ignore_ascii_case(name) {
                found = Some(address);
                break;
            }
        }
        found
    };
    address_cache().lock().insert(name.to_owned(), found);
    found
}

/// Reads the current numeric value without writing anything.
pub fn read_number(name: &str) -> Option<f64> {
    let address = find(name)?;
    let type_id = unsafe { memory::read_unchecked::<i16>(address + CVAR_TYPE)? };
    let raw = unsafe { memory::read_unchecked::<u64>(address + CVAR_VALUES)? };
    decode(type_id, raw)
}

/// Writes `value` through a MutationGuard and remembers the original value
/// under `key`. Calling again with the same key updates the override without
/// losing that original. Complex cvar types (strings/colors/vectors) are
/// rejected instead of being partially overwritten.
pub fn apply(key: &'static str, name: &str, value: f64) -> bool {
    let Some(address) = find(name) else {
        return false;
    };
    let Some(type_id) = (unsafe { memory::read_unchecked::<i16>(address + CVAR_TYPE) }) else {
        return false;
    };
    if !unsafe { memory::validate_range(address + CVAR_VALUES, 8) } {
        return false;
    }
    let Some(guard) = mutation::acquire() else {
        return false;
    };
    let cvar = Cvar { address };
    let mut overrides = OVERRIDES.lock();
    if let Some(index) = overrides.iter().position(|entry| entry.key == key) {
        let entry = &mut overrides[index];
        if entry.cvar.address != address {
            let old = overrides.remove(index);
            let _ = unsafe {
                guarded_write(&old.guard, old.cvar.address + CVAR_VALUES, old.previous)
            };
        } else {
            let Some(next) = encode(type_id, entry.previous, value) else {
                return false;
            };
            entry.cvar = cvar;
            return unsafe { guarded_write(&entry.guard, address + CVAR_VALUES, next) };
        }
    }
    let Some(previous) = (unsafe { memory::read_unchecked::<u64>(address + CVAR_VALUES) }) else {
        return false;
    };
    let Some(next) = encode(type_id, previous, value) else {
        return false;
    };
    if !unsafe { guarded_write(&guard, address + CVAR_VALUES, next) } {
        return false;
    }
    overrides.push(Override { key, cvar, previous, guard });
    true
}

/// Restores one override and drops its guard.
pub fn restore(key: &'static str) -> bool {
    let mut overrides = OVERRIDES.lock();
    let Some(index) = overrides.iter().position(|entry| entry.key == key) else {
        return false;
    };
    let entry = overrides.remove(index);
    let _ = unsafe {
        guarded_write(&entry.guard, entry.cvar.address + CVAR_VALUES, entry.previous)
    };
    true
}

/// Restores every override. Used on shutdown and whenever writes are disabled.
pub fn restore_all() {
    let entries = std::mem::take(&mut *OVERRIDES.lock());
    for entry in entries {
        let _ = unsafe {
            guarded_write(&entry.guard, entry.cvar.address + CVAR_VALUES, entry.previous)
        };
    }
}

#[inline]
unsafe fn guarded_write(guard: &MutationGuard, address: usize, value: u64) -> bool {
    guard.write(address, value)
}

fn mask(value: u64, bits: u32) -> u64 {
    if bits >= 64 {
        value
    } else {
        value & ((1u64 << bits) - 1)
    }
}

fn encode(type_id: i16, previous: u64, value: f64) -> Option<u64> {
    let bits = match type_id {
        TYPE_BOOL => return Some((previous & !0xff) | (u64::from(value != 0.0))),
        TYPE_I16 => 16,
        TYPE_U16 => 16,
        TYPE_I32 | TYPE_F32 => 32,
        TYPE_U32 => 32,
        TYPE_I64 | TYPE_U64 | TYPE_F64 => 64,
        _ => return None,
    };
    let raw = match type_id {
        TYPE_I16 => (value as i16) as u16 as u64,
        TYPE_U16 => (value as u16) as u64,
        TYPE_I32 => (value as i32) as u32 as u64,
        TYPE_U32 => (value as u32) as u64,
        TYPE_I64 => (value as i64) as u64,
        TYPE_U64 => value as u64,
        TYPE_F32 => (value as f32).to_bits() as u64,
        TYPE_F64 => value.to_bits(),
        _ => return None,
    };
    let keep = if bits >= 64 { 0 } else { previous & !((1u64 << bits) - 1) };
    Some(keep | mask(raw, bits))
}

fn decode(type_id: i16, raw: u64) -> Option<f64> {
    Some(match type_id {
        TYPE_BOOL => f64::from((raw & 0xff) != 0),
        TYPE_I16 => f64::from(mask(raw, 16) as u16 as i16),
        TYPE_U16 => f64::from(mask(raw, 16) as u16),
        TYPE_I32 => f64::from(mask(raw, 32) as u32 as i32),
        TYPE_U32 => f64::from(mask(raw, 32) as u32),
        TYPE_I64 => mask(raw, 64) as i64 as f64,
        TYPE_U64 => mask(raw, 64) as f64,
        TYPE_F32 => f64::from(f32::from_bits(mask(raw, 32) as u32)),
        TYPE_F64 => f64::from_bits(mask(raw, 64)),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_bool_without_touching_upper_bits() {
        let encoded = encode(TYPE_BOOL, 0xAABB_CCDD_EEFF_0000, 1.0).unwrap();
        assert_eq!(encoded, 0xAABB_CCDD_EEFF_0001);
        assert_eq!(decode(TYPE_BOOL, encoded), Some(1.0));
    }

    #[test]
    fn encodes_float32_and_int32_in_low_dword() {
        let previous = 0x1122_3344_FFFF_FFFF;
        let encoded = encode(TYPE_F32, previous, 0.5).unwrap();
        assert_eq!(encoded, 0x1122_3344_0000_0000 | f32::to_bits(0.5) as u64);
        assert_eq!(decode(TYPE_F32, encoded), Some(0.5));
        let encoded = encode(TYPE_I32, previous, -2.0).unwrap();
        assert_eq!(encoded, 0x1122_3344_FFFF_FFFE);
        assert_eq!(decode(TYPE_I32, encoded), Some(-2.0));
    }

    #[test]
    fn rejects_complex_types() {
        assert!(encode(9, 0, 1.0).is_none());
        assert!(decode(9, 0).is_none());
    }
}
