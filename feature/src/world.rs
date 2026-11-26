// World overlay + guarded world-color mutation.
//
// Both paths are written from scratch against local CS2 dumps / web signatures.
// Fire area is read-only. World color only writes after MutationGuard is
// acquired, and every cached original value is restored when the toggle is off.
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;

use wallbang_config::offsets::{
    ENTITY_DESIGNER_NAME, INFERNO_FIRE_COUNT, INFERNO_FIRE_POSITIONS, INFERNO_LIFETIME,
    INFERNO_MAX_FIRES,
};
use wallbang_config::signatures::{
    DRAW_SCENE_OBJECT_ARRAY_RVA, DRAW_SCENE_OBJECT_ARRAY_SLOT_RVA, DRAW_SKYBOX_ARRAY_RVA,
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

type DrawSceneFn = unsafe extern "system" fn(
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
) -> usize;
type DrawSkyboxFn = unsafe extern "system" fn(usize, usize, usize, usize) -> usize;
type SetupFogFn = unsafe extern "system" fn(usize, usize) -> usize;

static HOOK: Mutex<Option<VTableHook>> = Mutex::new(None);
static ORIGINAL_DRAW_SCENE: AtomicUsize = AtomicUsize::new(0);
static SKYBOX_HOOK: Mutex<Option<VTableHook>> = Mutex::new(None);
static ORIGINAL_DRAW_SKYBOX: AtomicUsize = AtomicUsize::new(0);
static FOG_HOOKS: Mutex<Vec<CallHook>> = Mutex::new(Vec::new());
static ORIGINAL_SETUP_FOG: AtomicUsize = AtomicUsize::new(0);
static ORIGINAL_COLORS: OnceLock<Mutex<HashMap<usize, [f32; 4]>>> = OnceLock::new();

fn original_colors() -> &'static Mutex<HashMap<usize, [f32; 4]>> {
    ORIGINAL_COLORS.get_or_init(|| Mutex::new(HashMap::new()))
}

const SCENE_NODE_COUNT: usize = 0xCA0;
const SCENE_NODE_LIST: usize = 0xCA8;
const MAX_SCENE_NODES: usize = 4096;
const NODE_SCENE_OBJECTS: usize = 0x0;
const SCENE_OBJECT_STRIDE: usize = 0x90;
const SCENE_OBJECT_TINT: usize = 0x3C;
const MAX_SCENE_OBJECTS: usize = 65_536;

/// Install the scene-draw hook through its vtable slot. The old prologue hook
/// copied relative code into a trampoline and crashed in scenesystem.dll; a
/// slot swap keeps the original function untouched and is branch-safe.
pub fn install() {
    install_world_color();
    install_skybox_removal();
    install_fog_removal();
}

fn install_world_color() {
    let Some(base) = module_base("scenesystem.dll") else {
        return;
    };
    let slot = base + DRAW_SCENE_OBJECT_ARRAY_SLOT_RVA;
    let expected = base + DRAW_SCENE_OBJECT_ARRAY_RVA;
    let Some(original) = (unsafe { memory::read_ptr(slot) }) else {
        return;
    };
    if original != expected {
        return;
    }
    let Some(hook) = (unsafe {
        VTableHook::install_slot(slot, hk_draw_scene_object_array as *const () as usize)
    }) else {
        return;
    };
    ORIGINAL_DRAW_SCENE.store(original, Ordering::SeqCst);
    *HOOK.lock() = Some(hook);
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
    ORIGINAL_DRAW_SCENE.load(Ordering::Relaxed) != 0
}

pub fn skybox_installed() -> bool {
    ORIGINAL_DRAW_SKYBOX.load(Ordering::Relaxed) != 0
}

pub fn fog_installed() -> bool {
    !FOG_HOOKS.lock().is_empty()
}

pub fn shutdown() {
    restore_colors();
    ORIGINAL_DRAW_SCENE.store(0, Ordering::SeqCst);
    ORIGINAL_DRAW_SKYBOX.store(0, Ordering::SeqCst);
    ORIGINAL_SETUP_FOG.store(0, Ordering::SeqCst);
    for mut hook in FOG_HOOKS.lock().drain(..) {
        unsafe { hook.remove() };
    }
    if let Some(hook) = SKYBOX_HOOK.lock().take() {
        unsafe { hook.restore() };
    }
    if let Some(hook) = HOOK.lock().take() {
        unsafe { hook.restore() };
    }
}

unsafe extern "system" fn hk_draw_skybox_array(
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
) -> usize {
    let original = ORIGINAL_DRAW_SKYBOX.load(Ordering::Relaxed);
    if original == 0 {
        return 0;
    }
    let skip = wallbang_core::state().settings().visual.ambience.remove_3d_skybox;
    if skip {
        return 0;
    }
    let original: DrawSkyboxFn = std::mem::transmute(original);
    original(a1, a2, a3, a4)
}

unsafe extern "system" fn hk_setup_fog(a1: usize, a2: usize) -> usize {
    let original = ORIGINAL_SETUP_FOG.load(Ordering::Relaxed);
    if original == 0 {
        return 0;
    }
    let skip = wallbang_core::state().settings().visual.ambience.remove_skybox_fog;
    if skip {
        return 0;
    }
    let original: SetupFogFn = std::mem::transmute(original);
    original(a1, a2)
}

unsafe extern "system" fn hk_draw_scene_object_array(
    a1: usize,
    scene: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
    a8: usize,
    a9: usize,
    a10: usize,
    a11: usize,
    a12: usize,
) -> usize {
    let original = ORIGINAL_DRAW_SCENE.load(Ordering::Relaxed);
    if original == 0 {
        return 0;
    }
    apply_world_color(scene);
    let original: DrawSceneFn = std::mem::transmute(original);
    original(a1, scene, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12)
}

/// Restores every tint this module has touched, then drops the cache.
pub fn restore_colors() {
    let Some(guard) = mutation::acquire() else {
        return;
    };
    let mut cache = original_colors().lock();
    for (&address, &original) in cache.iter() {
        unsafe { guard.write(address, original) };
    }
    cache.clear();
}

/// `CRenderScene` keeps up to `SCENE_NODE_COUNT` 16-byte world-node handles at
/// `+SCENE_NODE_LIST`. Each `WorldNode_t::m_sceneObjects` CUtlVector holds
/// `SceneObject_t` entries whose tint is a float4 at +0x3C (local dump).
unsafe fn apply_world_color(scene: usize) {
    let settings = wallbang_core::state().settings().clone();
    let ambience = settings.visual.ambience;
    if !ambience.world_color {
        if !original_colors().lock().is_empty() {
            restore_colors();
        }
        return;
    }
    let Some(guard) = mutation::acquire() else {
        return;
    };
    let count = memory::read_unchecked::<i32>(scene + SCENE_NODE_COUNT).unwrap_or(0);
    if count <= 0 || count as usize > MAX_SCENE_NODES {
        return;
    }
    let list = scene + SCENE_NODE_LIST;
    if !memory::validate_range(list, count as usize * 16) {
        return;
    }
    let tint = ambience.world_color_tint;
    let color = [
        tint[0] as f32 / 255.0,
        tint[1] as f32 / 255.0,
        tint[2] as f32 / 255.0,
        tint[3] as f32 / 255.0,
    ];
    let mut cache = original_colors().lock();
    for index in 0..count as usize {
        let Some(node) = memory::read_unchecked::<usize>(list + index * 16) else {
            continue;
        };
        if node == 0 || !memory::validate_range(node, 0x20) {
            continue;
        }
        let objects = node + NODE_SCENE_OBJECTS;
        let object_count =
            memory::read_unchecked::<i32>(objects).unwrap_or(0).max(0) as usize;
        if object_count == 0 || object_count > MAX_SCENE_OBJECTS {
            continue;
        }
        let data = memory::read_unchecked::<usize>(objects + 8).unwrap_or(0);
        if data == 0 || !memory::validate_range(data, object_count * SCENE_OBJECT_STRIDE) {
            continue;
        }
        for object in 0..object_count {
            let address = data + object * SCENE_OBJECT_STRIDE + SCENE_OBJECT_TINT;
            let original = memory::read_unchecked::<[f32; 4]>(address);
            let Some(original) = original else {
                continue;
            };
            if !cache.contains_key(&address) {
                cache.insert(address, original);
            }
            guard.write(address, color);
        }
    }
}

/// Read-only inferno/fire hull overlay.
pub fn draw_world_overlay(dl: &mut DrawList, state: &GameState) {
    if !state.settings().visual.fire_area {
        return;
    }
    let snapshot = state.snapshot();
    if !snapshot.valid {
        return;
    }
    let color = state.settings().visual.fire_area_color;
    let matrix = &snapshot.view_matrix;
    let (sw, sh) = (snapshot.screen_w, snapshot.screen_h);
    if sw <= 0.0 || sh <= 0.0 {
        return;
    }

    unsafe {
        for (_, identity) in EntityList::iterate() {
            let name = memory::read_ptr(identity + ENTITY_DESIGNER_NAME)
                .and_then(|ptr| memory::read_cstr_unchecked(ptr, 64))
                .unwrap_or_default();
            if name != "C_Inferno" {
                continue;
            }
            let entity = memory::read_ptr(identity + 0x10).unwrap_or(0);
            if entity == 0 || !memory::validate_range(entity, INFERNO_LIFETIME + 4) {
                continue;
            }
            let count = memory::read::<i32>(entity + INFERNO_FIRE_COUNT).unwrap_or(0);
            if count <= 0 {
                continue;
            }
            let lifetime = memory::read::<f32>(entity + INFERNO_LIFETIME).unwrap_or(0.0);
            for index in 0..count.min(INFERNO_MAX_FIRES as i32) as usize {
                let Some(point) = memory::read::<Vec3>(
                    entity + INFERNO_FIRE_POSITIONS + index * std::mem::size_of::<Vec3>(),
                ) else {
                    continue;
                };
                let Some(screen) = w2s(matrix, point, sw, sh) else {
                    continue;
                };
                if screen.x < 0.0 || screen.y < 0.0 || screen.x > sw || screen.y > sh {
                    continue;
                }
                dl.circle_filled(screen.x, screen.y, 5.0, color);
                dl.line(
                    screen.x - 7.0,
                    screen.y,
                    screen.x + 7.0,
                    screen.y,
                    1.0,
                    color,
                );
                dl.line(
                    screen.x,
                    screen.y - 7.0,
                    screen.x,
                    screen.y + 7.0,
                    1.0,
                    color,
                );
            }
            let Some(origin) = memory::read::<Vec3>(entity + 0x330 + 0xC8) else {
                continue;
            };
            if let Some(screen) = w2s(matrix, origin, sw, sh) {
                if screen.x >= 0.0 && screen.y >= 0.0 && screen.x <= sw && screen.y <= sh {
                    dl.text(
                        screen.x + 8.0,
                        screen.y - 8.0,
                        &format!("FIRE {lifetime:.1}s"),
                        color,
                        1.0,
                    );
                }
            }
        }
    }
}
