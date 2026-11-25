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
    pub fn native_config(&self) -> crate::native_preview::PreviewConfig {
        crate::native_preview::PreviewConfig {
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
        }
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

        let labels = [
            "Box",
            "Skeleton",
            "Name",
            "Health",
            "Armor",
            "Distance",
            "Weapon",
            "Ammo",
            if self.visible { "Visible" } else { "Occluded" },
            if self.enemy { "Enemy model" } else { "Team model" },
            if self.engine_model { "Engine: on" } else { "Engine: off" },
            if self.debug_overlay { "Debug: on" } else { "Debug: off" },
            "Own path",
        ];
        let on = [
            self.box_on,
            self.skeleton_on,
            self.name_on,
            self.health_on,
            self.armor_on,
            self.distance_on,
            self.weapon_on,
            self.ammo_on,
            self.visible,
            self.enemy,
            self.engine_model,
            self.debug_overlay,
            false,
        ];
        let gap = 6.0;
        let btn_w = (w - 28.0 - gap * 3.0) * 0.25;
        for (i, label) in labels.iter().enumerate() {
            let col = i % 4;
            let row = i / 4;
            let bx = x + 14.0 + col as f32 * (btn_w + gap);
            let by = y + 38.0 + row as f32 * 25.0;
            if i == 12 {
                dl.rect(bx, by, btn_w, 20.0, s.elevated);
                dl.outline(bx, by, btn_w, 20.0, 1.0, s.border);
                dl.text(
                    bx + (btn_w - dl.text_width(label, 0.72)) * 0.5,
                    by + 4.0,
                    label,
                    s.text_dim,
                    0.72,
                );
                continue;
            }
            if toggle_button(ui, dl, label, bx, by, btn_w, 20.0, on[i], s) {
                match i {
                    0 => self.box_on = !self.box_on,
                    1 => self.skeleton_on = !self.skeleton_on,
                    2 => self.name_on = !self.name_on,
                    3 => self.health_on = !self.health_on,
                    4 => self.armor_on = !self.armor_on,
                    5 => self.distance_on = !self.distance_on,
                    6 => self.weapon_on = !self.weapon_on,
                    7 => self.ammo_on = !self.ammo_on,
                    8 => self.visible = !self.visible,
                    9 => self.enemy = !self.enemy,
                    10 => self.engine_model = !self.engine_model,
                    11 => self.debug_overlay = !self.debug_overlay,
                    _ => {}
                }
            }
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

        if self.engine_model {
            crate::native_preview::set_request(
                true,
                (vx, vy, vw, vh),
                (screen_w, screen_h),
                self.native_config(),
            );
            clear_model_request();
            if !crate::native_preview::available() {
                dl.rect_rounded(vx, vy, vw, vh, 4.0, [12, 13, 16, 255]);
                let text = "Engine model unavailable";
                dl.text(
                    vx + (vw - dl.text_width(text, 0.85)) * 0.5,
                    vy + vh * 0.5 - 8.0,
                    text,
                    [220, 120, 110, 255],
                    0.85,
                );
            }
        } else {
            crate::native_preview::set_request(
                false,
                (0.0, 0.0, 0.0, 0.0),
                (screen_w, screen_h),
                self.native_config(),
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
        }

        dl.outline_rounded(vx, vy, vw, vh, 4.0, 1.0, s.border);

        if self.debug_overlay {
            let path = if self.engine_model { "engine" } else { "own" };
            let ready = if self.engine_model {
                crate::native_preview::available()
            } else {
                agent_model::available()
            };
            let text = format!("{path} ready={} bones={}", ready as usize, agent_model::bone_count());
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
const HEAD_TOP: usize = agent_model::HEAD_TOP;

const BONES: [(usize, usize); 17] = [
    (HEAD, HEAD_TOP),
    (HEAD, NECK),
    (NECK, CHEST),
    (CHEST, WAIST),
    (SHOULDER_L, SHOULDER_R),
    (SHOULDER_L, ELBOW_L),
    (ELBOW_L, HAND_L),
    (SHOULDER_R, ELBOW_R),
    (ELBOW_R, HAND_R),
    (HIP_L, HIP_R),
    (WAIST, HIP_L),
    (WAIST, HIP_R),
    (HIP_L, KNEE_L),
    (KNEE_L, FOOT_L),
    (HIP_R, KNEE_R),
    (KNEE_R, FOOT_R),
    (CHEST, HIP_L),
];

fn plate(
    dl: &mut DrawList,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    text: &str,
    scale: f32,
    fill: Color,
    border: Color,
    text_color: Color,
) {
    dl.parallelogram(x, y, w, h, 3.0, fill, border);
    let tw = dl.text_width(text, scale);
    let th = dl.text_height(scale);
    dl.text(x + (w - tw) * 0.5 + 1.5, y + (h - th) * 0.5, text, text_color, scale);
}

/// Draws ESP on top of the owned-model texture using real skinned bone projections.
pub fn draw_esp_overlay(dl: &mut DrawList, rect: (f32, f32, f32, f32)) {
    let request = model_request();
    if !request.active || !agent_model::available() {
        return;
    }
    let cfg = *OVERLAY_CONFIG.lock();
    let screen = agent_model::projected_bones();
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for p in screen.iter().flatten() {
        min_x = min_x.min(p.0);
        min_y = min_y.min(p.1);
        max_x = max_x.max(p.0);
        max_y = max_y.max(p.1);
    }
    if !min_x.is_finite() {
        return;
    }
    let accent: Color = cfg.armor_color;
    let bx = min_x - 10.0;
    let by = min_y - 5.0;
    let bw = (max_x - min_x) + 20.0;
    let bh = (max_y - min_y) + 10.0;

    dl.push_clip(rect.0 + 1.0, rect.1 + 1.0, rect.2 - 2.0, rect.3 - 2.0);
    if cfg.skeleton_on {
        let bone: Color = if cfg.visible {
            cfg.skeleton_visible
        } else {
            cfg.skeleton_occluded
        };
        let thickness = cfg.skeleton_thickness.max(0.5);
        for (a, b) in BONES {
            if let (Some(pa), Some(pb)) = (screen[a], screen[b]) {
                dl.line(pa.0, pa.1, pb.0, pb.1, thickness * 2.4, [bone[0], bone[1], bone[2], 38]);
                dl.line(pa.0, pa.1, pb.0, pb.1, thickness, bone);
            }
        }
        for p in screen.iter().flatten() {
            dl.rect(p.0 - thickness, p.1 - thickness, thickness * 2.0, thickness * 2.0, bone);
        }
    }
    if cfg.box_on {
        let color: Color = cfg.box_color;
        let glow: Color = [color[0], color[1], color[2], 42];
        if cfg.box_style == 1 {
            dl.outline(bx, by, bw, bh, 1.5, color);
        } else {
            let len = cfg.corner_size.clamp(5.0, 22.0);
            let slant = 3.0;
            for (x, y, sx, sy) in [
                (bx, by, 1.0, 1.0),
                (bx + bw, by, -1.0, 1.0),
                (bx, by + bh, 1.0, -1.0),
                (bx + bw, by + bh, -1.0, -1.0),
            ] {
                let ax = x + sx * slant;
                dl.line(x, y + sy * len, ax, y, 3.6, glow);
                dl.line(ax, y, ax + sx * len, y, 3.6, glow);
                dl.line(x, y + sy * len, ax, y, 1.5, color);
                dl.line(ax, y, ax + sx * len, y, 1.5, color);
            }
        }
    }

    let plate_fill: Color = [12, 14, 18, 225];
    let plate_border: Color = [accent[0], accent[1], accent[2], 110];
    if cfg.name_on {
        let name = if cfg.enemy { "ENEMY" } else { "TEAM" };
        let pw = (dl.text_width(name, 0.9) + 16.0).max(48.0);
        plate(dl, bx + (bw - pw) * 0.5, by - 25.0, pw, 14.0, name, 0.9, plate_fill, plate_border, cfg.name_color);
    }
    if cfg.health_on {
        let pw = 58.0;
        let py = by - 10.0;
        let px = bx + (bw - pw) * 0.5;
        dl.parallelogram(px, py, pw, 9.0, 3.0, plate_fill, plate_border);
        dl.parallelogram(px, py, pw * 0.84, 9.0, 3.0, [110, 228, 145, 255], [110, 228, 145, 255]);
        dl.text(px + pw + 4.0, py - 1.0, "84", [132, 236, 160, 255], 0.85);
    }
    if cfg.distance_on {
        let pw = 44.0;
        plate(dl, bx + (bw - pw) * 0.5, by + bh + 5.0, pw, 13.0, "12 M", 0.85, plate_fill, plate_border, cfg.distance_color);
    }
    if cfg.weapon_on {
        plate(dl, bx - 10.0, by + bh - 34.0, 52.0, 13.0, "AK-47", 0.8, plate_fill, plate_border, [200, 205, 215, 255]);
    }
    if cfg.ammo_on {
        plate(dl, bx - 10.0, by + bh - 19.0, 52.0, 13.0, "30/90", 0.8, plate_fill, plate_border, [220, 225, 235, 255]);
    }
    if cfg.armor_on {
        plate(dl, bx + bw - 34.0, by + bh - 17.0, 34.0, 12.0, "100", 0.8, plate_fill, plate_border, accent);
    }
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
