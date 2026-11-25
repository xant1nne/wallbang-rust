// ESP preview: own skinned agent render is the default path. Panorama remains an
// optional engine model; the procedural box mesh is gone.
use crate::draw::{Color, DrawList};
use crate::model as agent_model;
use crate::ui::{Style, Ui};
use parking_lot::Mutex;
use std::time::Instant;
use wallbang_core::state::GameState;

#[derive(Clone, Copy, Debug)]
pub struct ModelRequest {
    pub active: bool,
    pub rect: (f32, f32, f32, f32),
    pub clock: f32,
    pub enemy: bool,
    pub material: agent_model::MaterialOverride,
}

impl ModelRequest {
    const EMPTY: Self = Self {
        active: false,
        rect: (0.0, 0.0, 0.0, 0.0),
        clock: 0.0,
        enemy: true,
        material: agent_model::MaterialOverride::empty(),
    };
}

impl Default for ModelRequest {
    fn default() -> Self {
        Self::EMPTY
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)] // kept as the model-request payload; ESP now draws via esp_draw
struct OverlayConfig {
    enemy: bool,
    visible: bool,
    box_on: bool,
    skeleton_on: bool,
    name_on: bool,
    health_on: bool,
    armor_on: bool,
    distance_on: bool,
    weapon_on: bool,
    ammo_on: bool,
    box_color: Color,
    box_style: usize,
    corner_size: f32,
    skeleton_thickness: f32,
    skeleton_visible: Color,
    skeleton_occluded: Color,
    name_color: Color,
    armor_color: Color,
    distance_color: Color,
}

impl OverlayConfig {
    const DEFAULT: Self = Self {
        enemy: true,
        visible: true,
        box_on: true,
        skeleton_on: true,
        name_on: true,
        health_on: true,
        armor_on: true,
        distance_on: true,
        weapon_on: false,
        ammo_on: false,
        box_color: [255, 122, 110, 240],
        box_style: 0,
        corner_size: 10.0,
        skeleton_thickness: 1.4,
        skeleton_visible: [236, 240, 248, 245],
        skeleton_occluded: [255, 112, 104, 225],
        name_color: [255, 142, 128, 255],
        armor_color: [164, 204, 232, 255],
        distance_color: [190, 194, 202, 255],
    };
}

impl Default for OverlayConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}

static MODEL_REQUEST: Mutex<ModelRequest> = Mutex::new(ModelRequest::EMPTY);
static OVERLAY_CONFIG: Mutex<OverlayConfig> = Mutex::new(OverlayConfig::DEFAULT);

pub fn model_request() -> ModelRequest {
    *MODEL_REQUEST.lock()
}

pub fn clear_model_request() {
    *MODEL_REQUEST.lock() = ModelRequest::default();
}

fn set_model_request(request: ModelRequest, overlay: OverlayConfig) {
    *MODEL_REQUEST.lock() = request;
    *OVERLAY_CONFIG.lock() = overlay;
}

#[derive(Clone, Debug)]
pub struct EspPreviewState {
    enemy: bool,
    visible: bool,
    box_on: bool,
    skeleton_on: bool,
    name_on: bool,
    health_on: bool,
    armor_on: bool,
    distance_on: bool,
    weapon_on: bool,
    ammo_on: bool,
    engine_model: bool,
    debug_overlay: bool,
    material: agent_model::MaterialOverride,
    box_color: Color,
    box_style: usize,
    corner_size: f32,
    skeleton_thickness: f32,
    skeleton_visible: Color,
    skeleton_occluded: Color,
    name_color: Color,
    armor_color: Color,
    distance_color: Color,
    clock: f32,
    last: Option<Instant>,
}

impl Default for EspPreviewState {
    fn default() -> Self {
        Self {
            enemy: true,
            visible: true,
            box_on: true,
            skeleton_on: true,
            name_on: true,
            health_on: true,
            armor_on: true,
            distance_on: true,
            weapon_on: false,
            ammo_on: false,
            engine_model: false,
            debug_overlay: false,
            material: agent_model::MaterialOverride::empty(),
            box_color: [255, 122, 110, 240],
            box_style: 0,
            corner_size: 10.0,
            skeleton_thickness: 1.4,
            skeleton_visible: [236, 240, 248, 245],
            skeleton_occluded: [255, 112, 104, 225],
            name_color: [255, 142, 128, 255],
            armor_color: [164, 204, 232, 255],
            distance_color: [190, 194, 202, 255],
            clock: 0.0,
            last: None,
        }
    }
}

impl EspPreviewState {
    pub fn sync_settings(&mut self, state: &GameState) {
        let settings = state.settings();
        let esp = &settings.esp;
        self.engine_model = esp.preview_engine;
        self.debug_overlay = esp.preview_debug;
        self.box_on = esp.r#box;
        self.skeleton_on = esp.skeleton;
        self.name_on = esp.name;
        self.health_on = esp.health;
        self.armor_on = esp.armor;
        self.distance_on = esp.distance;
        self.weapon_on = esp.weapon;
        self.ammo_on = esp.ammo;
        self.box_color = esp.box_color;
        self.box_style = esp.box_style;
        self.corner_size = esp.corner_size;
        self.skeleton_thickness = esp.skeleton_thickness;
        self.skeleton_visible = esp.skeleton_visible_color;
        self.skeleton_occluded = esp.skeleton_occluded_color;
        self.name_color = esp.name_color;
        self.armor_color = esp.armor_color;
        self.distance_color = esp.distance_color;
        let chams = if self.enemy {
            &settings.visual.chams.enemy
        } else {
            &settings.visual.chams.team
        };
        self.material = if chams.enabled && chams.primary.enabled {
            agent_model::MaterialOverride {
                enabled: true,
                material: chams.primary.material,
                color: chams.primary.color,
            }
        } else {
            agent_model::MaterialOverride::default()
        };
    }

    pub fn commit_settings(&self, state: &GameState) {
        let enabled = self.box_on
            || self.skeleton_on
            || self.name_on
            || self.health_on
            || self.armor_on
            || self.distance_on
            || self.weapon_on
            || self.ammo_on;
        let mut settings = state.settings_mut();
        let esp = &mut settings.esp;
        if esp.enabled != enabled
            || esp.r#box != self.box_on
            || esp.skeleton != self.skeleton_on
            || esp.name != self.name_on
            || esp.health != self.health_on
            || esp.armor != self.armor_on
            || esp.distance != self.distance_on
            || esp.weapon != self.weapon_on
            || esp.ammo != self.ammo_on
            || esp.preview_engine != self.engine_model
            || esp.preview_debug != self.debug_overlay
        {
            esp.enabled = enabled;
            esp.r#box = self.box_on;
            esp.skeleton = self.skeleton_on;
            esp.name = self.name_on;
            esp.health = self.health_on;
            esp.armor = self.armor_on;
            esp.distance = self.distance_on;
            esp.weapon = self.weapon_on;
            esp.ammo = self.ammo_on;
            esp.preview_engine = self.engine_model;
            esp.preview_debug = self.debug_overlay;
        }
    }

    /// Config for the optional Panorama engine panel.
    pub fn native_config(&self, state: &GameState) -> crate::native_preview::PreviewConfig {
        use wallbang_config::settings::{esp_bits, has_flag};
        let esp = state.settings().esp.clone();
        let cfg = &esp.enemies;
        let mut preview = crate::native_preview::PreviewConfig {
            enemy: self.enemy,
            enabled: false,
            box_on: cfg.r#box,
            corner_fraction: cfg.corner_length,
            box_color: cfg.box_color,
            skeleton_on: cfg.skeleton,
            skeleton_thickness: 1.4,
            skeleton_color: cfg.skeleton_color,
            name_on: cfg.name,
            ammo_on: cfg.ammo,
            weapon_on: cfg.weapon,
            ..Default::default()
        };
        // Show a real player from the snapshot, never invented values: the
        // preview must look exactly like the in-game overlay.
        let snapshot = state.snapshot();
        let local_address = snapshot.local_address;
        let target = snapshot
            .players
            .iter()
            .find(|player| player.address != local_address && player.enemy == self.enemy)
            .or_else(|| {
                snapshot
                    .players
                    .iter()
                    .find(|player| player.address != local_address)
            });
        let Some(target) = target else {
            return preview;
        };
        preview.enabled = true;
        preview.name_text = target
            .name
            .as_deref()
            .unwrap_or("")
            .to_ascii_lowercase();
        preview.name_color = cfg.name_color;
        preview.health_on = cfg.health_bar;
        preview.health_value = target.health;
        preview.health_fraction = target.health.clamp(0, 100) as f32 / 100.0;
        preview.health_text = has_flag(cfg.health_type, esp_bits::TEXT);
        preview.health_low_color = cfg.health_low_color;
        preview.health_high_color = cfg.health_high_color;
        let max_clip = target.weapon.max_clip;
        preview.ammo_on = cfg.ammo && max_clip > 0 && target.weapon.clip >= 0;
        preview.ammo_fraction = if max_clip > 0 {
            target.weapon.clip.max(0) as f32 / max_clip as f32
        } else {
            0.0
        };
        preview.ammo_text = format!("{} / {}", target.weapon.clip.max(0), max_clip.max(0));
        preview.ammo_text_on = has_flag(cfg.ammo_type, esp_bits::TEXT);
        preview.ammo_low_color = cfg.ammo_low_color;
        preview.ammo_high_color = cfg.ammo_high_color;
        preview.weapon_on = cfg.weapon && target.weapon.name.is_some();
        preview.weapon_text = target
            .weapon
            .name
            .as_deref()
            .unwrap_or("")
            .to_ascii_lowercase();
        preview.weapon_color = cfg.weapon_color;
        let mut flags = Vec::new();
        if cfg.flags {
            let distance = target.origin.distance_to(snapshot.local_origin) * 0.0254;
            let mut push = |on: bool, text: String| {
                if on {
                    flags.push(crate::native_preview::PreviewFlag {
                        on: true,
                        text,
                        color: cfg.flags_color,
                    });
                }
            };
            push(
                has_flag(cfg.flags_type, esp_bits::FLAG_DISTANCE),
                format!("{distance:.0}m"),
            );
            push(
                has_flag(cfg.flags_type, esp_bits::FLAG_CASH) && target.money > 0,
                format!("${}", target.money),
            );
            push(
                has_flag(cfg.flags_type, esp_bits::FLAG_ARMOR) && target.armor > 0,
                if target.helmet { "HK" } else { "K" }.to_owned(),
            );
            push(
                has_flag(cfg.flags_type, esp_bits::FLAG_SCOPED) && target.scoped,
                "scoped".to_owned(),
            );
            push(
                has_flag(cfg.flags_type, esp_bits::FLAG_BOMB) && target.has_c4,
                "bomb".to_owned(),
            );
        }
        preview.flags = flags;
        preview
    }

    fn overlay_config(&self) -> OverlayConfig {
        OverlayConfig {
            enemy: self.enemy,
            visible: self.visible,
            box_on: self.box_on,
            skeleton_on: self.skeleton_on,
            name_on: self.name_on,
            health_on: self.health_on,
            armor_on: self.armor_on,
            distance_on: self.distance_on,
            weapon_on: self.weapon_on,
            ammo_on: self.ammo_on,
            box_color: self.box_color,
            box_style: self.box_style,
            corner_size: self.corner_size,
            skeleton_thickness: self.skeleton_thickness,
            skeleton_visible: self.skeleton_visible,
            skeleton_occluded: self.skeleton_occluded,
            name_color: self.name_color,
            armor_color: self.armor_color,
            distance_color: self.distance_color,
        }
    }

    /// Floating panel; opened only on the Players tab.
    pub fn draw_panel(
        &mut self,
        state: &GameState,
        ov_out: &mut wallbang_config::ui_state::PlayerOverlayCfg,
        ui: &mut Ui,
        dl: &mut DrawList,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        s: &Style,
    ) {
        let now = Instant::now();
        let dt = self
            .last
            .map(|last| now.saturating_duration_since(last).as_secs_f32())
            .unwrap_or(0.0)
            .min(0.1);
        self.last = Some(now);
        self.clock += dt;

        dl.begin_top();
        dl.rect_rounded(x, y, w, h, 5.0, s.dark);
        dl.outline_rounded(x, y, w, h, 5.0, 1.0, s.border);
        dl.text(x + 14.0, y + 12.0, "ESP PREVIEW", s.text, 1.0);
        let mode = if self.enemy { "Enemy" } else { "Team" };
        dl.text(x + w - 14.0 - dl.text_width(mode, 0.9), y + 14.0, mode, s.accent, 0.9);
        dl.line(x + 12.0, y + 31.0, x + w - 12.0, y + 31.0, 1.0, s.border);

        // Toggles drive the real per-target config used by the world overlay
        // (Enemy overlay slot) — not preview-only state.
        let mut ov = ov_out.clone();
        let labels = [
            "Box",
            "Skeleton",
            "Name",
            "Health",
            "Armor",
            "Distance",
            "Weapon",
            "Ammo",
            "Flags",
        ];
        let on = [
            ov.box_on,
            ov.skel_on,
            ov.name_on,
            ov.health.enabled,
            ov.flags[2],
            ov.flags[0],
            ov.weapon_on,
            ov.ammo.enabled,
            ov.flags_on,
        ];
        let gap = 6.0;
        let cols = 3usize;
        let btn_w = (w - 28.0 - gap * (cols as f32 - 1.0)) / cols as f32;
        let mut changed = false;
        for (i, label) in labels.iter().enumerate() {
            let col = i % cols;
            let row = i / cols;
            let bx = x + 14.0 + col as f32 * (btn_w + gap);
            let by = y + 38.0 + row as f32 * 25.0;
            if toggle_button(ui, dl, label, bx, by, btn_w, 20.0, on[i], s) {
                changed = true;
                match i {
                    0 => ov.box_on = !ov.box_on,
                    1 => ov.skel_on = !ov.skel_on,
                    2 => ov.name_on = !ov.name_on,
                    3 => ov.health.enabled = !ov.health.enabled,
                    4 => {
                        ov.flags_on = true;
                        ov.flags[2] = !ov.flags[2];
                    }
                    5 => {
                        ov.flags_on = true;
                        ov.flags[0] = !ov.flags[0];
                    }
                    6 => ov.weapon_on = !ov.weapon_on,
                    7 => ov.ammo.enabled = !ov.ammo.enabled,
                    8 => ov.flags_on = !ov.flags_on,
                    _ => {}
                }
            }
        }
        if changed {
            *ov_out = ov;
        }

        let vx = x + 12.0;
        let vy = y + 141.0;
        let vw = w - 24.0;
        let vh = (h - 167.0).max(140.0);
        if vw < 90.0 || vh < 140.0 {
            dl.end_top();
            return;
        }
        let (screen_w, screen_h) = dl.size();
        let overlay = self.overlay_config();

        // The preview must show the REAL ESP. That is only possible on the owned
        // skinned model, whose bones are projected in Rust so the shared
        // `esp_draw` code paints the exact in-game overlay. The Panorama panel
        // cannot hand bones back, so its request is disposed (which also drops
        // the per-frame GPU readback that was costing frames).
        crate::native_preview::set_request(
            false,
            (0.0, 0.0, 0.0, 0.0),
            (screen_w, screen_h),
            self.native_config(state),
        );
        set_model_request(
            ModelRequest {
                active: true,
                rect: (vx, vy, vw, vh),
                clock: self.clock,
                enemy: self.enemy,
                material: self.material,
            },
            overlay,
        );
        if !agent_model::available() {
            dl.rect_rounded(vx, vy, vw, vh, 4.0, [12, 13, 16, 255]);
            let text = "Model assets unavailable";
            dl.text(
                vx + (vw - dl.text_width(text, 0.85)) * 0.5,
                vy + vh * 0.5 - 8.0,
                text,
                [220, 120, 110, 255],
                0.85,
            );
        }

        dl.outline_rounded(vx, vy, vw, vh, 4.0, 1.0, s.border);

        if self.debug_overlay {
            let text = format!(
                "own ready={} bones={}",
                agent_model::available() as usize,
                agent_model::bone_count()
            );
            dl.text(vx + 6.0, vy + vh - 13.0, &text, [150, 170, 190, 230], 0.75);
        }
        dl.end_top();
    }
}

const HEAD: usize = agent_model::HEAD;
const NECK: usize = agent_model::NECK;
const CHEST: usize = agent_model::CHEST;
const WAIST: usize = agent_model::WAIST;
const SHOULDER_L: usize = agent_model::SHOULDER_L;
const SHOULDER_R: usize = agent_model::SHOULDER_R;
const ELBOW_L: usize = agent_model::ELBOW_L;
const ELBOW_R: usize = agent_model::ELBOW_R;
const HAND_L: usize = agent_model::HAND_L;
const HAND_R: usize = agent_model::HAND_R;
const HIP_L: usize = agent_model::HIP_L;
const HIP_R: usize = agent_model::HIP_R;
const KNEE_L: usize = agent_model::KNEE_L;
const KNEE_R: usize = agent_model::KNEE_R;
const FOOT_L: usize = agent_model::FOOT_L;
const FOOT_R: usize = agent_model::FOOT_R;

/// Draws the REAL ESP over the owned-model view. Bones come from the owned
/// skinned model's projections; target data comes from the live snapshot (first
/// player) and falls back to a placeholder status out of a match, so the preview
/// always exercises the true overlay code path with real element geometry.
pub fn draw_esp_overlay(dl: &mut DrawList, rect: (f32, f32, f32, f32)) {
    use crate::esp_draw::{draw_target, EspTargetView, BONE_SLOTS};
    use wallbang_core::bone::ids;
    use wallbang_core::math::Vec2;

    let request = model_request();
    if !request.active || !agent_model::available() {
        return;
    }
    let state = wallbang_core::state();
    let cfg = state.settings().esp.enemies.clone();
    if !cfg.enabled {
        return;
    }
    let screen = agent_model::projected_bones();

    // Map the owned model's bone projections onto the CS2 bone ids the shared
    // skeleton pairs use, so the preview draws identical segments.
    let mut bones: [Option<Vec2>; BONE_SLOTS] = [None; BONE_SLOTS];
    let mut map = |target: usize, source: usize| {
        bones[target] = screen
            .get(source)
            .copied()
            .flatten()
            .map(|(x, y)| Vec2::new(x, y));
    };
    map(ids::HEAD, HEAD);
    map(ids::NECK, NECK);
    map(ids::SPINE_3, CHEST);
    map(ids::SPINE_2, CHEST);
    map(ids::SPINE_0, WAIST);
    map(ids::PELVIS, WAIST);
    map(ids::LEFT_SHOULDER, SHOULDER_L);
    map(ids::LEFT_ELBOW, ELBOW_L);
    map(ids::LEFT_HAND, HAND_L);
    map(ids::RIGHT_SHOULDER, SHOULDER_R);
    map(ids::RIGHT_ELBOW, ELBOW_R);
    map(ids::RIGHT_HAND, HAND_R);
    map(ids::LEFT_HIP, HIP_L);
    map(ids::LEFT_KNEE, KNEE_L);
    map(ids::LEFT_FOOT, FOOT_L);
    map(ids::RIGHT_HIP, HIP_R);
    map(ids::RIGHT_KNEE, KNEE_R);
    map(ids::RIGHT_FOOT, FOOT_R);

    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for point in screen.iter().flatten() {
        min_x = min_x.min(point.0);
        min_y = min_y.min(point.1);
        max_x = max_x.max(point.0);
        max_y = max_y.max(point.1);
    }
    if !min_x.is_finite() || max_y <= min_y {
        return;
    }
    // Celerity box: height from feet to head, width = height * 0.5, centred on
    // the projected model, clamped into the viewport.
    let height = max_y - min_y;
    let width = height * 0.5;
    let center_x = (min_x + max_x) * 0.5;
    let box_min = Vec2::new(
        (center_x - width * 0.5).max(rect.0 + 2.0),
        (min_y - 2.0).max(rect.1 + 2.0),
    );
    let box_max = Vec2::new(
        (center_x + width * 0.5).min(rect.0 + rect.2 - 2.0),
        (max_y + 2.0).min(rect.1 + rect.3 - 2.0),
    );

    let snapshot = state.snapshot();
    let live = snapshot
        .players
        .iter()
        .find(|player| player.address != snapshot.local_address);
    let (name, health, armor, money, helmet, scoped, has_c4, weapon, clip, max_clip, distance) =
        match live {
            Some(player) => (
                player.name.as_deref(),
                player.health,
                player.armor,
                player.money,
                player.helmet,
                player.scoped,
                player.has_c4,
                player.weapon.name.as_deref(),
                player.weapon.clip,
                player.weapon.max_clip,
                player.origin.distance_to(snapshot.local_origin) * 0.0254,
            ),
            None => (
                Some("player"),
                100,
                100,
                800,
                true,
                false,
                false,
                Some("AK-47"),
                30,
                90,
                12.0,
            ),
        };

    dl.push_clip(rect.0 + 1.0, rect.1 + 1.0, rect.2 - 2.0, rect.3 - 2.0);
    let target = EspTargetView {
        box_min,
        box_max,
        bones,
        name,
        health,
        animated_health: health as f32,
        armor,
        money,
        helmet,
        scoped,
        has_c4,
        weapon,
        clip,
        max_clip,
        distance,
    };
    draw_target(dl, &cfg, &target);
    dl.pop_clip();
}

fn toggle_button(
    ui: &mut Ui,
    dl: &mut DrawList,
    label: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    on: bool,
    s: &Style,
) -> bool {
    let hover = !ui.overlay_blocking() && ui.inside(x, y, w, h);
    let bg = if hover { s.hovered } else if on { s.active } else { s.elevated };
    dl.rect(x, y, w, h, bg);
    dl.outline(
        x,
        y,
        w,
        h,
        1.0,
        if on { [s.accent[0], s.accent[1], s.accent[2], 130] } else { s.border },
    );
    let tw = dl.text_width(label, 0.72);
    dl.text(
        x + (w - tw) * 0.5,
        y + (h - dl.text_height(0.72)) * 0.5 - 1.0,
        label,
        if on { s.text } else { s.text_dim },
        0.72,
    );
    hover && ui.mouse_clicked()
}
