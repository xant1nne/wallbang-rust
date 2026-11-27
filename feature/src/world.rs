// World overlay + guarded world-color mutation.
//
// Both paths are written from scratch against local CS2 dumps / web signatures.
// Fire area is read-only. World color only writes after MutationGuard is
// acquired, and every cached original value is restored when the toggle is off.
use parking_lot::Mutex;
use wallbang_config::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::OnceLock;

use wallbang_config::offsets::{
    ENTITY_DESIGNER_NAME, INFERNO_FIRE_COUNT, INFERNO_FIRE_POSITIONS, INFERNO_LIFETIME,
    INFERNO_MAX_FIRES,
};
use wallbang_config::signatures::{
    DRAW_SCENE_OBJECT_RVA, DRAW_SCENE_OBJECT_SLOT_RVA, DRAW_SKYBOX_ARRAY_RVA,
    DRAW_SKYBOX_ARRAY_SLOT_RVA, SIG_SETUP_FOG,
};
use wallbang_core::entity::EntityList;
use wallbang_core::hooks::{CallHook, VTableHook};
use wallbang_core::math::{w2s, Vec3};
use wallbang_core::memory;
use wallbang_core::modules::{module_base, module_size};
use wallbang_core::mutation;
use wallbang_core::state::GameState;
use wallbang_render::DrawList;

// Build 14201: DrawSkyboxArray and DrawSceneObject take 8 parameters (4 in
// registers + 4 on stack). Forwarding all 8 preserves the scene context.
type DrawSkyboxFn =
    unsafe extern "system" fn(usize, usize, usize, i32, i32, usize, usize, usize) -> usize;
type DrawSceneObjectFn =
    unsafe extern "system" fn(usize, usize, usize, i32, i32, usize, usize, usize) -> usize;
type SetupFogFn = unsafe extern "system" fn(usize, usize) -> usize;

const PRIMITIVE_STRIDE: usize = 0x70;
const PRIMITIVE_COLOR: usize = 0x50;
const MAX_SCENE_BATCH: usize = 65_536;

#[cfg(test)]
mod tests {
    use super::*;

    unsafe extern "system" fn light_original(_: usize, object: usize, _: usize) -> usize {
        memory::read::<[f32; 3]>(object + LIGHT_RGB).unwrap()[0].to_bits() as usize
    }

    #[test]
    fn light_tint_does_not_compound_and_restores_original() {
        let mut object = [0.0f32; 128];
        let base = object.as_mut_ptr() as usize;
        unsafe { memory::write(base + LIGHT_RGB, [2.0f32, 4.0, 6.0]); }
        let previous = ORIGINAL_LIGHT.swap(light_original as *const () as usize, Ordering::SeqCst);
        LIGHT_COLOR.store(u32::MAX, Ordering::Relaxed);
        LIGHT_SCALE.store(0.5f32.to_bits(), Ordering::Relaxed);
        LIGHT_ENABLED.store(true, Ordering::Release);
        let first = unsafe { hk_light(0, base, 0) };
        let second = unsafe { hk_light(0, base, 0) };
        LIGHT_ENABLED.store(false, Ordering::Release);
        restore_lights();
        ORIGINAL_LIGHT.store(previous, Ordering::SeqCst);
        assert_eq!(first, 1.0f32.to_bits() as usize);
        assert_eq!(second, first);
        assert_eq!(unsafe { memory::read::<[f32; 3]>(base + LIGHT_RGB) }, Some([2.0, 4.0, 6.0]));
    }

    unsafe extern "system" fn skybox_original(
        a1: usize, a2: usize, a3: usize, a4: i32, a5: i32, a6: usize, a7: usize, a8: usize,
    ) -> usize {
        usize::from(a1 == 11 && a2 == 22 && a3 == 33 && a4 == 44
            && a5 == 55 && a6 == 0x1234_5678 && a7 == 0x7FFF_9999_1234 && a8 == 0xDEAD_BEEF)
    }

    #[test]
    fn skybox_hook_preserves_register_and_stack_arguments() {
        let previous = ORIGINAL_DRAW_SKYBOX.swap(skybox_original as *const () as usize, Ordering::SeqCst);
        struct Restore(usize);
        impl Drop for Restore {
            fn drop(&mut self) { ORIGINAL_DRAW_SKYBOX.store(self.0, Ordering::SeqCst); }
        }
        let _restore = Restore(previous);
        assert!(!wallbang_core::state().settings().visual.ambience.remove_3d_skybox);
        let result = unsafe { hk_draw_skybox_array(11, 22, 33, 44, 55, 0x1234_5678, 0x7FFF_9999_1234, 0xDEAD_BEEF) };
        assert_eq!(result, 1, "disabled skybox hook must transparently forward all eight parameters");
    }

    unsafe extern "system" fn scene_object_original(
        _: usize, _: usize, batch: usize, count: i32, _: i32, _: usize, _: usize, _: usize,
    ) -> usize {
        if count < 2 { return 0; }
        let c0 = memory::read::<u32>(batch + PRIMITIVE_COLOR).unwrap_or(0);
        let c1 = memory::read::<u32>(batch + PRIMITIVE_STRIDE + PRIMITIVE_COLOR).unwrap_or(0);
        usize::from(c0 == u32::from_le_bytes([0x10, 0x20, 0x30, 0xAA])
            && c1 == u32::from_le_bytes([0x10, 0x20, 0x30, 0xBB]))
    }

    #[test]
    fn scene_object_tint_applies_and_restores_primitives() {
        let mut batch = vec![0u8; PRIMITIVE_STRIDE * 2];
        let base = batch.as_mut_ptr() as usize;
        let orig0 = u32::from_le_bytes([0xFF, 0xEE, 0xDD, 0xAA]);
        let orig1 = u32::from_le_bytes([0x11, 0x22, 0x33, 0xBB]);
        unsafe {
            memory::write(base + PRIMITIVE_COLOR, orig0);
            memory::write(base + PRIMITIVE_STRIDE + PRIMITIVE_COLOR, orig1);
        }
        let previous = ORIGINAL_DRAW_SCENE_OBJECT.swap(scene_object_original as *const () as usize, Ordering::SeqCst);
        WORLD_TINT_COLOR.store(u32::from_le_bytes([0x10, 0x20, 0x30, 0xFF]), Ordering::Relaxed);
        WORLD_TINT_ENABLED.store(true, Ordering::Release);
        let result = unsafe { hk_draw_scene_object(0, 0, base, 2, 0, 0, 0, 0) };
        WORLD_TINT_ENABLED.store(false, Ordering::Release);
        ORIGINAL_DRAW_SCENE_OBJECT.store(previous, Ordering::SeqCst);
        assert_eq!(result, 1, "original must observe tinted RGB with preserved alpha");
        assert_eq!(unsafe { memory::read::<u32>(base + PRIMITIVE_COLOR) }, Some(orig0));
        assert_eq!(unsafe { memory::read::<u32>(base + PRIMITIVE_STRIDE + PRIMITIVE_COLOR) }, Some(orig1));
    }
}

static SCENE_OBJECT_HOOK: Mutex<Option<VTableHook>> = Mutex::new(None);
static ORIGINAL_DRAW_SCENE_OBJECT: AtomicUsize = AtomicUsize::new(0);
static WORLD_TINT_ENABLED: AtomicBool = AtomicBool::new(false);
static WORLD_TINT_COLOR: AtomicU32 = AtomicU32::new(u32::MAX);
static SKYBOX_HOOK: Mutex<Option<VTableHook>> = Mutex::new(None);
static ORIGINAL_DRAW_SKYBOX: AtomicUsize = AtomicUsize::new(0);
static FOG_HOOKS: Mutex<Vec<CallHook>> = Mutex::new(Vec::new());
static ORIGINAL_SETUP_FOG: AtomicUsize = AtomicUsize::new(0);
static SCENE_HITS: AtomicUsize = AtomicUsize::new(0);
static SKYBOX_HITS: AtomicUsize = AtomicUsize::new(0);
static FOG_HITS: AtomicUsize = AtomicUsize::new(0);
static LAST_NODE_COUNT: AtomicUsize = AtomicUsize::new(0);
static LAST_TINTED_OBJECTS: AtomicUsize = AtomicUsize::new(0);
/// Install the scene-draw hook through its vtable slot. The old prologue hook
/// copied relative code into a trampoline and crashed in scenesystem.dll; a
/// slot swap keeps the original function untouched and is branch-safe.
pub fn install() {
    if wallbang_core::modules::build_supported() != Some(true) { return; }
    // World tint uses the verified DrawSceneObject vtable slot (primitives stride
    // 0x70, color +0x50) alongside the scene-light call hook.
    install_scene_object();
    install_lighting();
    install_skybox_removal();
    install_fog_removal();
}

fn install_scene_object() {
    let Some(base) = module_base("scenesystem.dll") else { return; };
    let slot = base + DRAW_SCENE_OBJECT_SLOT_RVA;
    let expected = base + DRAW_SCENE_OBJECT_RVA;
    let Some(original) = (unsafe { memory::read_ptr(slot) }) else { return; };
    if original != expected { return; }
    let Some(hook) = (unsafe {
        VTableHook::install_slot(slot, hk_draw_scene_object as *const () as usize)
    }) else { return; };
    ORIGINAL_DRAW_SCENE_OBJECT.store(original, Ordering::SeqCst);
    *SCENE_OBJECT_HOOK.lock() = Some(hook);
}

unsafe extern "system" fn hk_draw_scene_object(
    a1: usize,
    a2: usize,
    batch: usize,
    batch_count: i32,
    a5: i32,
    a6: usize,
    a7: usize,
    a8: usize,
) -> usize {
    SCENE_HITS.fetch_add(1, Ordering::Relaxed);
    let original = ORIGINAL_DRAW_SCENE_OBJECT.load(Ordering::Acquire);
    if original == 0 { return 0; }
    let call: DrawSceneObjectFn = std::mem::transmute(original);
    if !WORLD_TINT_ENABLED.load(Ordering::Acquire) || batch == 0 || batch_count <= 0 {
        return call(a1, a2, batch, batch_count, a5, a6, a7, a8);
    }
    let count = (batch_count as usize).min(MAX_SCENE_BATCH);
    LAST_NODE_COUNT.store(count, Ordering::Relaxed);
    let tint_bytes = WORLD_TINT_COLOR.load(Ordering::Relaxed).to_le_bytes();
    let mut backups: Vec<(usize, u32)> = Vec::with_capacity(count);
    for i in 0..count {
        let addr = batch + i * PRIMITIVE_STRIDE + PRIMITIVE_COLOR;
        let Some(orig) = memory::read::<u32>(addr) else { continue; };
        let orig_bytes = orig.to_le_bytes();
        let tinted = u32::from_le_bytes([tint_bytes[0], tint_bytes[1], tint_bytes[2], orig_bytes[3]]);
        if memory::write(addr, tinted) {
            backups.push((addr, orig));
        }
    }
    if !backups.is_empty() {
        LAST_TINTED_OBJECTS.fetch_add(backups.len(), Ordering::Relaxed);
    }
    let result = call(a1, a2, batch, batch_count, a5, a6, a7, a8);
    for (addr, orig) in backups.into_iter().rev() {
        memory::write(addr, orig);
    }
    result
}

const LIGHT_CALL_RVA: usize = 0x1A30A6;
const LIGHT_TARGET_RVA: usize = 0x19F300;
const LIGHT_RGB: usize = 0xE4;
const SKY_RGB: usize = 0xE8;
const SHADER_FLOAT_RVA: usize = 0x16A960;
const SHADER_INT_RVA: usize = 0x16A530;
static LIGHT_HOOK: Mutex<Option<CallHook>> = Mutex::new(None);
static ORIGINAL_LIGHT: AtomicUsize = AtomicUsize::new(0);
static LIGHT_SCALE: AtomicU32 = AtomicU32::new(1.0f32.to_bits());
static LIGHT_COLOR: AtomicU32 = AtomicU32::new(u32::MAX);
static LIGHT_ENABLED: AtomicBool = AtomicBool::new(false);
#[derive(Clone, Copy)]
struct LightTint { original: [f32; 3], applied: [f32; 3] }
static LIGHT_ORIGINALS: OnceLock<Mutex<HashMap<usize, LightTint>>> = OnceLock::new();
fn light_originals() -> &'static Mutex<HashMap<usize, LightTint>> {
    LIGHT_ORIGINALS.get_or_init(|| Mutex::new(HashMap::default()))
}
fn restore_lights() {
    let mut originals = light_originals().lock();
    for (&address, tint) in originals.iter() {
        unsafe {
            if memory::read::<[f32; 3]>(address) == Some(tint.applied) { memory::write(address, tint.original); }
        }
    }
    originals.clear();
}
fn install_lighting() {
    if wallbang_core::modules::build_supported() != Some(true) { return; }
    let Some(base) = module_base("scenesystem.dll") else { return; };
    let site = base + LIGHT_CALL_RVA;
    let Some(relative) = (unsafe { memory::read::<i32>(site + 1) }) else { return; };
    if unsafe { memory::read::<u8>(site) } != Some(0xE8)
        || (site + 5).wrapping_add_signed(relative as isize) != base + LIGHT_TARGET_RVA { return; }
    ORIGINAL_LIGHT.store(base + LIGHT_TARGET_RVA, Ordering::Release);
    *LIGHT_HOOK.lock() = unsafe { CallHook::install(site, hk_light as *const () as usize) };
}
unsafe extern "system" fn hk_light(a1: usize, object: usize, a3: usize) -> usize {
    SCENE_HITS.fetch_add(1, Ordering::Relaxed);
    let original = ORIGINAL_LIGHT.load(Ordering::Acquire);
    if original == 0 { return 0; }
    let call: unsafe extern "system" fn(usize, usize, usize) -> usize = std::mem::transmute(original);
    if !LIGHT_ENABLED.load(Ordering::Acquire) { return call(a1, object, a3); }
    let address = object + LIGHT_RGB;
    let Some(saved) = memory::read::<[f32; 3]>(address) else { return call(a1, object, a3); };
    if !saved.iter().all(|v| v.is_finite()) { return call(a1, object, a3); }
    let color = LIGHT_COLOR.load(Ordering::Relaxed).to_le_bytes();
    let scale = f32::from_bits(LIGHT_SCALE.load(Ordering::Relaxed));
    {
        let mut originals = light_originals().lock();
        if originals.len() < 4096 || originals.contains_key(&address) {
            let tint = originals.entry(address).or_insert(LightTint { original: saved, applied: saved });
            if saved != tint.applied { tint.original = saved; }
            let applied = std::array::from_fn(|i| tint.original[i] * scale * color[i] as f32 / 255.0);
            if memory::write(address, applied) {
                tint.applied = applied;
                LAST_TINTED_OBJECTS.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
    // This routine queues the object pointer; consumers read RGB later in the frame.
    call(a1, object, a3)
}

/// Same branch-safe approach as world color: swap the verified skybox vtable
/// slot instead of copying a relative prologue.
fn install_skybox_removal() {
    let Some(base) = module_base("scenesystem.dll") else {
        return;
    };
    let slot = base + DRAW_SKYBOX_ARRAY_SLOT_RVA;
    let expected = base + DRAW_SKYBOX_ARRAY_RVA;
    let Some(original) = (unsafe { memory::read_ptr(slot) }) else {
        return;
    };
    if original != expected {
        return;
    }
    let Some(hook) = (unsafe {
        VTableHook::install_slot(slot, hk_draw_skybox_array as *const () as usize)
    }) else {
        return;
    };
    ORIGINAL_DRAW_SKYBOX.store(original, Ordering::SeqCst);
    *SKYBOX_HOOK.lock() = Some(hook);
}

/// `SetupFog` is reached through three direct calls in client.dll. Replacing
/// the call sites (instead of the callee prologue) keeps relative branches in
/// the original function intact and lets us skip the pass on demand.
fn install_fog_removal() {
    let Some(target) = wallbang_core::pattern::scan_module("client.dll", SIG_SETUP_FOG) else {
        return;
    };
    ORIGINAL_SETUP_FOG.store(target, Ordering::SeqCst);
    let (Some(base), Some(size)) = (
        module_base("client.dll"),
        module_size("client.dll"),
    ) else {
        return;
    };
    if size < 5 {
        return;
    }
    let mut hooks = Vec::new();
    let end = base + size - 5;
    let mut address = base;
    while address <= end {
        if unsafe { memory::read_unchecked::<u8>(address) } == Some(0xE8) {
            let rel = unsafe { memory::read_unchecked::<i32>(address + 1) }.unwrap_or(0);
            let call = (address + 5).wrapping_add(rel as usize);
            if call == target {
                if let Some(hook) = unsafe {
                    CallHook::install(address, hk_setup_fog as *const () as usize)
                } {
                    hooks.push(hook);
                }
            }
        }
        address += 1;
    }
    *FOG_HOOKS.lock() = hooks;
}

pub fn installed() -> bool {
    SCENE_OBJECT_HOOK.lock().is_some() || LIGHT_HOOK.lock().is_some()
}

pub fn skybox_installed() -> bool {
    ORIGINAL_DRAW_SKYBOX.load(Ordering::Relaxed) != 0
}

pub fn fog_installed() -> bool {
    !FOG_HOOKS.lock().is_empty()
}

pub fn diag_summary() -> String {
    format!(
        "world diag: scene_hits={} skybox_hits={} fog_hits={} last_nodes={} last_tinted={}",
        SCENE_HITS.load(Ordering::Relaxed),
        SKYBOX_HITS.load(Ordering::Relaxed),
        FOG_HITS.load(Ordering::Relaxed),
        LAST_NODE_COUNT.load(Ordering::Relaxed),
        LAST_TINTED_OBJECTS.load(Ordering::Relaxed),
    )
}

pub fn sync_settings(state: &GameState) {
    let settings = state.settings();
    let ambience = &settings.visual.ambience;
    let tint = ambience.world_color_tint;
    let world_tint_enabled = settings.misc.allow_writes && ambience.world_color;
    let light_enabled = settings.misc.allow_writes && (ambience.lighting || ambience.world_color);
    let scale = if ambience.lighting && ambience.lighting_intensity.is_finite() { ambience.lighting_intensity.clamp(0.0, 4.0) } else { 1.0 };
    let light_color = if ambience.world_color { tint } else { ambience.lighting_color };
    drop(settings);
    WORLD_TINT_COLOR.store(u32::from_le_bytes(tint), Ordering::Relaxed);
    WORLD_TINT_ENABLED.store(world_tint_enabled, Ordering::Release);
    LIGHT_COLOR.store(u32::from_le_bytes(light_color), Ordering::Relaxed);
    LIGHT_SCALE.store(scale.to_bits(), Ordering::Relaxed);
    LIGHT_ENABLED.store(light_enabled, Ordering::Release);
    if !light_enabled { restore_lights(); }
}

pub fn shutdown() {
    WORLD_TINT_ENABLED.store(false, Ordering::Release);
    LIGHT_ENABLED.store(false, Ordering::Release);
    restore_lights();
    ORIGINAL_DRAW_SCENE_OBJECT.store(0, Ordering::SeqCst);
    if let Some(hook) = SCENE_OBJECT_HOOK.lock().take() {
        unsafe { hook.restore() };
    }
    if let Some(mut hook) = LIGHT_HOOK.lock().take() { unsafe { hook.remove(); } }
    ORIGINAL_DRAW_SKYBOX.store(0, Ordering::SeqCst);
    ORIGINAL_SETUP_FOG.store(0, Ordering::SeqCst);
    for mut hook in FOG_HOOKS.lock().drain(..) {
        unsafe { hook.remove() };
    }
    if let Some(hook) = SKYBOX_HOOK.lock().take() {
        unsafe { hook.restore() };
    }
}

unsafe extern "system" fn hk_draw_skybox_array(
    a1: usize,
    a2: usize,
    a3: usize,
    a4: i32,
    a5: i32,
    a6: usize,
    a7: usize,
    a8: usize,
) -> usize {
    let original = ORIGINAL_DRAW_SKYBOX.load(Ordering::Relaxed);
    if original == 0 {
        return 0;
    }
    SKYBOX_HITS.fetch_add(1, Ordering::Relaxed);
    let skip = mutation::is_allowed() && wallbang_core::state().settings().visual.ambience.remove_3d_skybox;
    if skip {
        return 0;
    }
    let original: DrawSkyboxFn = std::mem::transmute(original);
    let cfg = wallbang_core::state().settings().visual.ambience.clone();
    let tint = mutation::is_allowed() && cfg.skybox_color && a4 > 0 && a4 <= 4096;
    let address = if tint {
        memory::read_ptr(a3 + (a4 as usize) * PRIMITIVE_STRIDE - 0x58).map(|descriptor| descriptor + SKY_RGB)
    } else { None };
    let saved = address.and_then(|address| memory::read::<[f32; 3]>(address));
    if let (Some(address), Some(_)) = (address, saved) {
        memory::write(address, [cfg.sky_color[0] as f32 / 255.0, cfg.sky_color[1] as f32 / 255.0, cfg.sky_color[2] as f32 / 255.0]);
    }
    let result = original(a1, a2, a3, a4, a5, a6, a7, a8);
    if let (Some(address), Some(saved)) = (address, saved) { memory::write(address, saved); }
    result
}

unsafe extern "system" fn hk_setup_fog(a1: usize, a2: usize) -> usize {
    let original = ORIGINAL_SETUP_FOG.load(Ordering::Relaxed);
    if original == 0 {
        return 0;
    }
    FOG_HITS.fetch_add(1, Ordering::Relaxed);
    let original: SetupFogFn = std::mem::transmute(original);
    let cfg = wallbang_core::state().settings().visual.ambience.clone();
    if mutation::is_allowed() && (cfg.fog || cfg.remove_skybox_fog) && a1 != 0 && a2 != 0 {
        if let Some(base) = module_base("client.dll") {
            let set: unsafe extern "system" fn(usize, u32, *const FogVector) -> usize = std::mem::transmute(base + SHADER_FLOAT_RVA);
            let set_int: unsafe extern "system" fn(usize, u32, i32) -> usize = std::mem::transmute(base + SHADER_INT_RVA);
            let end = if cfg.fog_end.is_finite() { cfg.fog_end.clamp(100.0, 20_000.0) } else { 4000.0 };
            let density = if cfg.remove_skybox_fog { 0.0 } else if cfg.fog_density.is_finite() { cfg.fog_density.clamp(0.0, 1.0) } else { 0.25 };
            set(a1, 0x4B01FF63, &FogVector([0.0, end, 0.0, 0.0]));
            set(a1, 0x0AA49C2A, &FogVector([density, 1.0, 0.0, 0.0]));
            set(a1, 0xFBF6448D, &FogVector([cfg.fog_color[0] as f32 / 255.0, cfg.fog_color[1] as f32 / 255.0, cfg.fog_color[2] as f32 / 255.0, 0.0]));
            set_int(a1 + 17 * 16, 0x6E0FAD7E, i32::from(!cfg.remove_skybox_fog));
            memory::write(a2, 0i32);
            return 0;
        }
    }
    original(a1, a2)
}

#[repr(C, align(16))]
struct FogVector([f32; 4]);

const FIRE_DISCOVERY_INTERVAL: std::time::Duration = std::time::Duration::from_millis(500);
const FIRE_REFRESH_INTERVAL: std::time::Duration = std::time::Duration::from_millis(100);
const FIRE_DISCOVERY_BATCH: usize = 16;
const ENTITY_IDENTITY_OFFSET: usize = 0x10;
#[derive(Default)]
struct FireCache {
    discovered_at: Option<std::time::Instant>, refreshed_at: Option<std::time::Instant>,
    candidates: Vec<usize>, cursor: usize, entities: Vec<(usize, usize)>, points: Vec<Vec3>,
}
static FIRE_CACHE: OnceLock<Mutex<FireCache>> = OnceLock::new();
/// Entity discovery is bounded per frame; drawing only projects cached points.
pub fn draw_world_overlay(dl: &mut DrawList, state: &GameState) {
    let (enabled, color) = { let settings = state.settings();
        (settings.visual.fire_area, settings.visual.fire_area_color) };
    let snapshot = state.snapshot();
    if !enabled || !snapshot.valid {
        if let Some(cache) = FIRE_CACHE.get() { *cache.lock() = FireCache::default(); }
        return;
    }
    let mut cache = FIRE_CACHE.get_or_init(|| Mutex::new(FireCache::default())).lock();
    let now = std::time::Instant::now();
    if cache.discovered_at.is_none_or(|last| now.duration_since(last) >= FIRE_DISCOVERY_INTERVAL)
        && cache.cursor >= cache.candidates.len() {
        cache.candidates = unsafe { EntityList::iterate() }.into_iter().map(|(_, entity)| entity).collect();
        cache.cursor = 0; cache.discovered_at = Some(now);
    }
    let end = (cache.cursor + FIRE_DISCOVERY_BATCH).min(cache.candidates.len());
    for index in cache.cursor..end {
        let entity = cache.candidates[index];
        unsafe {
            let Some(identity) = memory::read_ptr(entity + ENTITY_IDENTITY_OFFSET) else { continue; };
            let Some(name) = memory::read_ptr(identity + ENTITY_DESIGNER_NAME) else { continue; };
            if !memory::validate_range(name, 32) { continue; }
            let name = memory::read_cstr_unchecked(name, 32).unwrap_or_default();
            if (name == "inferno" || name == "C_Inferno") && !cache.entities.contains(&(entity, identity)) {
                cache.entities.push((entity, identity));
            }
        }
    }
    cache.cursor = end;
    if cache.refreshed_at.is_none_or(|last| now.duration_since(last) >= FIRE_REFRESH_INTERVAL) {
        let mut points = Vec::new();
        cache.entities.retain(|&(entity, identity)| unsafe {
            if memory::read_ptr(entity + ENTITY_IDENTITY_OFFSET) != Some(identity)
                || !memory::validate_range(entity, INFERNO_LIFETIME + 4) { return false; }
            let count = memory::read_unchecked::<i32>(entity + INFERNO_FIRE_COUNT).unwrap_or(0);
            if count <= 0 || count as usize > INFERNO_MAX_FIRES { return false; }
            let start = entity + INFERNO_FIRE_POSITIONS;
            if !memory::validate_range(start, count as usize * std::mem::size_of::<Vec3>()) { return false; }
            for index in 0..count as usize {
                if let Some(point) = memory::read_unchecked::<Vec3>(start + index * std::mem::size_of::<Vec3>()) {
                    if point.x.is_finite() && point.y.is_finite() && point.z.is_finite() { points.push(point); }
                }
            }
            true
        });
        cache.points = points; cache.refreshed_at = Some(now);
    }
    for &point in &cache.points {
        let Some(screen) = w2s(&snapshot.view_matrix, point, snapshot.screen_w, snapshot.screen_h) else { continue; };
        if screen.x < 0.0 || screen.y < 0.0 || screen.x > snapshot.screen_w || screen.y > snapshot.screen_h { continue; }
        dl.circle_filled(screen.x, screen.y, 5.0, color);
        dl.line(screen.x - 7.0, screen.y, screen.x + 7.0, screen.y, 1.0, color);
        dl.line(screen.x, screen.y - 7.0, screen.x, screen.y + 7.0, 1.0, color);
    }
}
