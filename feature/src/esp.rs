// Config block: world ESP — построение экранной цели из snapshot и вызов общего
// рендера `wallbang_render::esp_draw` (один и тот же код рисует мир и превью).
// Только чтение Snapshot + DrawList; запись в игру отсутствует.
use parking_lot::Mutex;
use wallbang_config::settings::EspPlayerSettings;
use wallbang_core::bone::BoneArray;
use wallbang_core::math::{w2s, Vec2, Vec3};
use wallbang_core::state::{GameState, PlayerSnapshot};
use wallbang_render::esp_draw::{draw_target, EspTargetView, BONE_SLOTS};
use wallbang_render::DrawList;

/// Source units -> metres. Celerity multiplies the distance by 0.0254.
const UNIT_TO_METRE: f32 = 0.0254;
const PROJECTION_MARGIN: f32 = 100.0;
/// Collision height fallback when the collision property is not streamed yet.
const FALLBACK_HEIGHT: f32 = 72.0;

static HEALTH_ANIM: Mutex<[f32; 65]> = Mutex::new([0.0; 65]);

/// Projects the 24 CS2 bone slots for one pawn into screen space.
fn project_bones(
    player: &PlayerSnapshot,
    matrix: &[f32; 16],
    screen_w: f32,
    screen_h: f32,
) -> [Option<Vec2>; BONE_SLOTS] {
    let mut bones = [None; BONE_SLOTS];
    let Some(array) = (unsafe { BoneArray::from_pawn(player.address) }) else {
        return bones;
    };
    for (index, slot) in bones.iter_mut().enumerate() {
        if let Some(world) = unsafe { array.position(index) } {
            *slot = w2s(matrix, world, screen_w, screen_h);
        }
    }
    bones
}

/// `draw()` body from Celerity, applied to one target class.
fn draw_targets(dl: &mut DrawList, state: &GameState, cfg: &EspPlayerSettings, teammates: bool) {
    if !cfg.enabled {
        return;
    }
    let snapshot = state.snapshot();
    if !snapshot.valid {
        return;
    }
    let (screen_w, screen_h) = (snapshot.screen_w, snapshot.screen_h);
    if screen_w <= 1.0 || screen_h <= 1.0 {
        return;
    }
    let matrix = snapshot.view_matrix;
    let local_origin = snapshot.local_origin;
    let local_team = snapshot.local_team;
    let local_address = snapshot.local_address;
    let mut health_anim = HEALTH_ANIM.lock();

    for player in snapshot.players.iter() {
        if player.address == local_address || !player.alive {
            continue;
        }
        let is_teammate = player.team == local_team;
        if is_teammate != teammates {
            continue;
        }
        let scene_origin = player.origin;
        let height = (player.bounds_max.z - player.bounds_min.z).max(1.0);
        let height = if height <= 1.0 { FALLBACK_HEIGHT } else { height };
        let top_world = Vec3::new(scene_origin.x, scene_origin.y, scene_origin.z + height);
        let (Some(screen_top), Some(screen_bottom)) = (
            w2s(&matrix, top_world, screen_w, screen_h),
            w2s(&matrix, scene_origin, screen_w, screen_h),
        ) else {
            continue;
        };
        let inside = |p: Vec2| {
            p.x >= -PROJECTION_MARGIN
                && p.x <= screen_w + PROJECTION_MARGIN
                && p.y >= -PROJECTION_MARGIN
                && p.y <= screen_h + PROJECTION_MARGIN
        };
        if !inside(screen_top) || !inside(screen_bottom) {
            continue;
        }
        let box_height = screen_bottom.y - screen_top.y;
        if box_height.abs() < 1.0 {
            continue;
        }
        let box_width = box_height * 0.5;
        let box_min = Vec2::new((screen_top.x - box_width * 0.5).floor(), screen_top.y.floor());
        let box_max = Vec2::new((box_min.x + box_width).floor(), (box_min.y + box_height).floor());

        // Smoothed health drain, same animation as Celerity.
        let slot = player.index.min(health_anim.len() - 1);
        let health_target = player.health as f32;
        if health_anim[slot] <= 0.0 || health_target > health_anim[slot] {
            health_anim[slot] = health_target;
        } else {
            health_anim[slot] += (health_target - health_anim[slot]) * 0.1;
        }

        let target = EspTargetView {
            box_min,
            box_max,
            bones: project_bones(player, &matrix, screen_w, screen_h),
            name: player.name.as_deref(),
            health: player.health,
            animated_health: health_anim[slot],
            armor: player.armor,
            money: player.money,
            helmet: player.helmet,
            scoped: player.scoped,
            has_c4: player.has_c4,
            weapon: player.weapon.name.as_deref(),
            clip: player.weapon.clip,
            max_clip: player.weapon.max_clip,
            distance: player.origin.distance_to(local_origin) * UNIT_TO_METRE,
        };
        draw_target(dl, cfg, &target);
    }
}

pub fn draw_esp(dl: &mut DrawList, state: &GameState) {
    let esp = state.settings().esp.clone();
    if esp.enabled {
        draw_targets(dl, state, &esp.enemies, false);
        draw_targets(dl, state, &esp.teammates, true);
    }
    crate::world::draw_world_overlay(dl, state);
}
