// Config: client.dll build 14201, offline verified vtable entry and event accessors.
use parking_lot::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use wallbang_core::{entity::EntityList, hooks::VTableHook, memory, modules, pawn::Pawn};
const FIRE_SLOT_RVA: usize = 0x1C32038;
const FIRE_RVA: usize = 0x9E22C0;
const GET_NAME_RVA: usize = 0x9E7650;
const GET_INT_RVA: usize = 0x9E6920;
const NAME_SLOT: usize = 1;
const INT_SLOT: usize = 7;
static HOOK: Mutex<Option<VTableHook>> = Mutex::new(None);
static ORIGINAL: AtomicUsize = AtomicUsize::new(0);
#[repr(C)]
struct Token { hash: u32, padding: u32, text: *const u8 }
fn token(text: &'static [u8]) -> Token {
    let bytes = &text[..text.len()-1];
    let mut h = 0x31415926u32 ^ bytes.len() as u32;
    let mut chunks = bytes.chunks_exact(4);
    for block in &mut chunks {
        let mut k = u32::from_le_bytes(block.try_into().unwrap()).wrapping_mul(0x5BD1E995);
        k ^= k >> 24; k = k.wrapping_mul(0x5BD1E995);
        h = h.wrapping_mul(0x5BD1E995) ^ k;
    }
    let tail = chunks.remainder();
    for (i,b) in tail.iter().enumerate() { h ^= (*b as u32) << (i*8); }
    if !tail.is_empty() { h = h.wrapping_mul(0x5BD1E995); }
    h ^= h >> 13; h = h.wrapping_mul(0x5BD1E995); h ^= h >> 15;
    Token { hash: h, padding: u32::MAX, text: text.as_ptr() }
}
pub fn install() -> bool {
    if modules::build_supported() != Some(true) { return false; }
    let Some(base) = modules::module_base("client.dll") else { return false; };
    if unsafe { memory::read_ptr(base + FIRE_SLOT_RVA) } != Some(base + FIRE_RVA) { return false; }
    ORIGINAL.store(base + FIRE_RVA, Ordering::Release);
    let hook = unsafe { VTableHook::install_slot(base + FIRE_SLOT_RVA, fire as *const () as usize) };
    let ready = hook.is_some();
    *HOOK.lock() = hook;
    ready
}
pub fn shutdown() {
    if let Some(hook) = HOOK.lock().take() { unsafe { hook.restore(); } }
}
unsafe extern "system" fn fire(manager: usize, event: usize) -> bool {
    if event != 0 && wallbang_core::state().settings().misc.vote_revealer { reveal(event); }
    let original = ORIGINAL.load(Ordering::Acquire);
    if original == 0 { return false; }
    let call: unsafe extern "system" fn(usize, usize) -> bool = std::mem::transmute(original);
    call(manager, event)
}
unsafe fn reveal(event: usize) {
    let Some(base) = modules::module_base("client.dll") else { return; };
    let Some(table) = memory::read_ptr(event) else { return; };
    if memory::read_ptr(table + NAME_SLOT * 8) != Some(base + GET_NAME_RVA)
        || memory::read_ptr(table + INT_SLOT * 8) != Some(base + GET_INT_RVA) { return; }
    let name: unsafe extern "system" fn(usize) -> usize = std::mem::transmute(base + GET_NAME_RVA);
    let Some(name) = memory::read_cstr(name(event), 64) else { return; };
    let get: unsafe extern "system" fn(usize, *const Token, i32) -> i32 = std::mem::transmute(base + GET_INT_RVA);
    match name.as_str() {
        "vote_cast" => {
            let choice = get(event, &token(b"vote_option\0"), -1);
            if !(0..=1).contains(&choice) { return; }
            let id = get(event, &token(b"entityid\0"), -1);
            if !(1..=64).contains(&id) { return; }
            let voter = EntityList::slot(id as usize).and_then(Pawn::player_name)
                .unwrap_or_else(|| format!("player #{id}"));
            wallbang_core::event_log::push_vote(&voter, if choice == 0 { "YES" } else { "NO" });
        }
        "vote_started" => wallbang_core::event_log::push_vote_started(),
        _ => {}
    }
}
