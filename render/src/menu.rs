// Config block: 1:1 визуальный порт C++ меню (core, ragebot, legitbot, player+ESP preview, world, skins, misc, config).
use crate::draw::{Color, DrawList};
use crate::preview::EspPreviewState;
use crate::ui::{Style, Ui, ITEM_SPACING_X, POPUP_W, ROUNDING, WIDGET_ROUNDING};
use std::sync::atomic::{AtomicBool, Ordering};
use wallbang_config::constants::DEFAULT_PROFILE;
use wallbang_config::storage::Storage;
use wallbang_config::settings::{has_flag, EspPlayerSettings};
use wallbang_config::ui_state::{
    ChamsCfg, ChamsLayer, ExtendedUiState, PlayerOverlayCfg, CATALOG_ITEMS, PAINT_KITS, SkinItem,
};
use wallbang_core::input::CursorController;
use wallbang_core::math::Vec2;
use wallbang_core::state::GameState;
use wallbang_resources::CHAM_MATERIALS;

const SIDEBAR_W: f32 = 176.0;
const GAP: f32 = 12.0;
const SUBTAB_BAR_H: f32 = 44.0;
const HEADER_H: f32 = 64.0;
const ROW_H: f32 = 34.0;
const SECTION_H: f32 = 28.0;
const SECTION_GAP: f32 = 10.0;
const DEFAULT_W: f32 = 848.0;
const DEFAULT_H: f32 = 580.0;

/// Новый сайдбар: только навигация, без выдуманных функций.
const NEW_SIDEBAR_W: f32 = 184.0;
const PREVIEW_TAB: usize = 2;
const CONFIG_TAB: usize = 6;
const NEW_TABS: [&str; 7] = [
    "Ragebot", "Legitbot", "Players", "Visuals", "Skins", "Misc", "Config",
];

static MENU_OPEN: AtomicBool = AtomicBool::new(false);

pub fn is_menu_open() -> bool {
    MENU_OPEN.load(Ordering::Relaxed)
}

const SIDEBAR_LABELS: [&str; 7] = ["Ragebot", "Legitbot", "Players", "World", "Skins", "Misc", "Config"];
const PAGE_TITLES: [&str; 7] = [
    "Ragebot",
    "Legitbot",
    "Players",
    "World",
    "Skins",
    "Miscellaneous",
    "Configuration",
];
const SECTIONS: [&str; 3] = ["Aim", "Common", "Settings"];
const STARTS: [usize; 3] = [0, 2, 6];
const COUNTS: [usize; 3] = [2, 4, 1];

const WEAPON_GROUPS: [&str; 6] = ["Pistols", "SMGs", "Rifles", "Shotguns", "Snipers", "Machine guns"];
const HITBOXES: [&str; 6] = ["head", "chest", "stomach", "arms", "legs", "feet"];
const LEGIT_HITBOXES: [&str; 5] = ["head", "chest", "stomach", "arms", "legs"];
const PITCH_ITEMS: [&str; 3] = ["none", "down", "up"];
const BOX_STYLES: [&str; 2] = ["full", "cornered"];
const BAR_POSITIONS: [&str; 3] = ["left", "top", "bottom"];
const DISPLAY_TYPES: [&str; 3] = ["text", "icon", "text + icon"];
const FLAG_NAMES: [&str; 5] = ["distance", "money", "armor", "scoped", "bomb"];
const ITEM_GROUPS: [&str; 6] = ["pistols", "smgs", "rifles", "snipers", "heavy", "grenades"];
const PROJ_GROUPS: [&str; 6] = ["he grenade", "flashbang", "smoke", "molotov", "decoy", "inferno"];
const SURFACE_PRESETS: [&str; 8] = [
    "Metal",
    "Plastic",
    "Matte",
    "Polished metal",
    "Ceramic",
    "Rubber",
    "Chalk",
    "Brushed metal",
];
const WEATHER_TYPES: [&str; 3] = ["snow", "rain", "stars"];
const SKYBOX_NAMES: [&str; 5] = ["Default", "Night sky", "Tibet", "Vertigo", "Space"];
const MARKER_TYPES: [&str; 3] = ["classic", "damage", "both"];
const MOTION_BLUR_MODES: [&str; 2] = ["screen", "viewmodel"];
const SOUND_TYPES: [&str; 10] = [
    "shop click",
    "home click",
    "bell",
    "killcard",
    "bullet casing",
    "coin pickup",
    "item drop",
    "popcan",
    "key press",
    "custom",
];
const IMPACT_TYPES: [&str; 3] = ["overlay", "sparks", "both"];
const DEATH_EFFECTS: [&str; 4] = [
    "Original particles",
    "Lightning",
    "Shockwave",
    "Lightning + shockwave",
];
const EDGEBUG_MODES: [&str; 5] = [
    "0: loose",
    "1: edge trace (default)",
    "2: no jump held",
    "3: min speed",
    "4: strict vz",
];
const PRIMARY_WEAPONS: [&str; 6] = ["none", "rifle", "scoped rifle", "scout", "awp", "auto sniper"];
const SECONDARY_WEAPONS: [&str; 5] = ["none", "dual elites", "five-seven/tec-9", "deagle", "revolver"];
const GRENADE_NAMES: [&str; 5] = ["molotov", "he grenade", "smoke", "flashbang", "decoy"];
const CLANTAG_STYLES: [&str; 5] = ["Static", "Typewriter", "Scroll", "Bounce", "Blink"];
const WATERMARK_ANCHORS: [&str; 6] = [
    "top left",
    "top center",
    "top right",
    "bottom left",
    "bottom center",
    "bottom right",
];

const RARITY_COLORS: [Color; 8] = [
    [235, 235, 235, 255],
    [138, 173, 233, 255],
    [77, 116, 196, 255],
    [138, 86, 207, 255],
    [211, 44, 230, 255],
    [235, 75, 75, 255],
    [228, 174, 57, 255],
    [255, 215, 0, 255],
];

const SUBTABS: [&[&str]; 7] = [
    &["aimbot"],
    &["aimbot"],
    &["enemies", "allies", "local"],
    &["esp", "scene", "weather"],
    &["guns", "knives", "gloves", "agents"],
    &["main", "removals", "camera", "hud"],
    &["general"],
];


pub struct Menu {
    open: bool,
    tab: usize,
    subtab: usize,
    sidebar_open: [bool; 3],
    ui: Ui,
    cursor: CursorController,
    storage: Storage,
    config_list: Vec<String>,
    selected: Option<usize>,
    need_refresh: bool,
    create_buf: String,
    search_buf: String,
    confirm_delete: bool,
    confirm_reset: bool,
    status: String,
    toggle_held: bool,
    ext: ExtendedUiState,
    preview: EspPreviewState,
    menu_rect: (f32, f32, f32, f32),
    hud_drag_owner: Option<usize>,
    hud_drag_offset: Vec2,
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

fn ui_layer_to_typed(layer: &ChamsLayer) -> wallbang_config::settings::ChamsMaterialLayer {
    wallbang_config::settings::ChamsMaterialLayer {
        enabled: layer.enabled,
        material: layer.material,
        color: layer.color,
    }
}

fn typed_layer_to_ui(layer: &wallbang_config::settings::ChamsMaterialLayer) -> ChamsLayer {
    ChamsLayer {
        enabled: layer.enabled,
        material: layer.material,
        color: layer.color,
    }
}

fn ui_preset_to_typed(cfg: &ChamsCfg) -> wallbang_config::settings::ChamsPreset {
    wallbang_config::settings::ChamsPreset {
        enabled: cfg.enabled,
        primary: ui_layer_to_typed(&cfg.primary),
        secondary: ui_layer_to_typed(&cfg.secondary),
        overlay: ui_layer_to_typed(&cfg.overlay),
    }
}

fn typed_preset_to_ui(cfg: &wallbang_config::settings::ChamsPreset) -> ChamsCfg {
    ChamsCfg {
        enabled: cfg.enabled,
        primary: typed_layer_to_ui(&cfg.primary),
        secondary: typed_layer_to_ui(&cfg.secondary),
        overlay: typed_layer_to_ui(&cfg.overlay),
    }
}

/// Copies the Celerity-style target config into the Players-tab control mirror.
fn esp_target_to_overlay(cfg: &EspPlayerSettings, ov: &mut PlayerOverlayCfg) {
    use wallbang_config::settings::esp_bits;
    ov.enabled = cfg.enabled;
    ov.box_on = cfg.r#box;
    ov.box_style = if cfg.corner_length >= 0.5 { 0 } else { 1 };
    ov.corner_len = cfg.corner_length;
    ov.box_color = cfg.box_color;
    ov.skel_on = cfg.skeleton;
    ov.skel_vis = cfg.skeleton_color;
    ov.skel_occ = cfg.skeleton_color;
    ov.health.enabled = cfg.health_bar;
    ov.health.gradient = has_flag(cfg.health_type, esp_bits::GRADIENT);
    ov.health.show_value = has_flag(cfg.health_type, esp_bits::TEXT);
    ov.health.full_color = cfg.health_high_color;
    ov.health.low_color = cfg.health_low_color;
    ov.name_on = cfg.name;
    ov.name_color = cfg.name_color;
    ov.weapon_on = cfg.weapon;
    ov.weapon_display = if has_flag(cfg.weapon_type, esp_bits::WEAPON_ICON) {
        if has_flag(cfg.weapon_type, esp_bits::WEAPON_TEXT) {
            2
        } else {
            1
        }
    } else {
        0
    };
    ov.weapon_text = cfg.weapon_color;
    ov.ammo.enabled = cfg.ammo;
    ov.ammo.gradient = has_flag(cfg.ammo_type, esp_bits::GRADIENT);
    ov.ammo.show_value = has_flag(cfg.ammo_type, esp_bits::TEXT);
    ov.ammo.full_color = cfg.ammo_high_color;
    ov.ammo.low_color = cfg.ammo_low_color;
    ov.flags_on = cfg.flags;
    let flag_bits = [
        esp_bits::FLAG_DISTANCE,
        esp_bits::FLAG_CASH,
        esp_bits::FLAG_ARMOR,
        esp_bits::FLAG_SCOPED,
        esp_bits::FLAG_BOMB,
    ];
    for (slot, bit) in ov.flags.iter_mut().zip(flag_bits.iter().copied()) {
        *slot = has_flag(cfg.flags_type, bit);
    }
    ov.flags_color = cfg.flags_color;
}

/// Writes the Players-tab controls back into the typed Celerity config.
fn esp_target_from_overlay(ov: &PlayerOverlayCfg, cfg: &mut EspPlayerSettings) {
    use wallbang_config::settings::esp_bits;
    cfg.enabled = ov.enabled;
    cfg.r#box = ov.box_on;
    cfg.corner_length = if ov.box_style == 0 {
        0.5
    } else {
        ov.corner_len.clamp(0.005, 0.5)
    };
    cfg.box_color = ov.box_color;
    cfg.skeleton = ov.skel_on;
    cfg.skeleton_color = ov.skel_vis;
    cfg.health_bar = ov.health.enabled;
    cfg.health_type = esp_bits::BAR
        | if ov.health.gradient { esp_bits::GRADIENT } else { 0 }
        | if ov.health.show_value { esp_bits::TEXT } else { 0 };
    cfg.health_high_color = ov.health.full_color;
    cfg.health_low_color = ov.health.low_color;
    cfg.name = ov.name_on;
    cfg.name_color = ov.name_color;
    cfg.weapon = ov.weapon_on;
    cfg.weapon_type = match ov.weapon_display {
        1 => esp_bits::WEAPON_ICON,
        2 => esp_bits::WEAPON_ICON | esp_bits::WEAPON_TEXT,
        _ => esp_bits::WEAPON_TEXT,
    };
    cfg.weapon_color = ov.weapon_text;
    cfg.ammo = ov.ammo.enabled;
    cfg.ammo_type = esp_bits::BAR
        | if ov.ammo.gradient { esp_bits::GRADIENT } else { 0 }
        | if ov.ammo.show_value { esp_bits::TEXT } else { 0 };
    cfg.ammo_high_color = ov.ammo.full_color;
    cfg.ammo_low_color = ov.ammo.low_color;
    cfg.flags = ov.flags_on;
    let flag_bits = [
        esp_bits::FLAG_DISTANCE,
        esp_bits::FLAG_CASH,
        esp_bits::FLAG_ARMOR,
        esp_bits::FLAG_SCOPED,
        esp_bits::FLAG_BOMB,
    ];
    cfg.flags_type = ov
        .flags
        .iter()
        .zip(flag_bits.iter().copied())
        .filter(|(on, _)| **on)
        .fold(0, |mask, (_, bit)| mask | bit);
    cfg.flags_color = ov.flags_color;
}

// Menu shell only: feature panels are kept in the source for later, but are
// not wired into the frame until the corresponding features actually exist.
#[allow(dead_code)]
impl Menu {
    pub fn new() -> Self {
        Self {
            // Меню открыто при старте, как в оригинальном C++ xdraw-меню.
            // INSERT по-прежнему переключает его в рантайме.
            open: true,
            tab: 0,
            subtab: 0,
            sidebar_open: [true, true, true],
            ui: Ui::new(),
            cursor: CursorController::new(),
            storage: Storage::new(wallbang_config::storage::default_dir()),
            config_list: Vec::new(),
            selected: None,
            need_refresh: true,
            create_buf: String::new(),
            search_buf: String::new(),
            confirm_delete: false,
            confirm_reset: false,
            status: "Ready".to_string(),
            toggle_held: false,
            ext: ExtendedUiState::default(),
            preview: EspPreviewState::default(),
            menu_rect: (0.0, 0.0, 0.0, 0.0),
            hud_drag_owner: None,
            hud_drag_offset: Vec2::default(),
        }
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
        if !self.open {
            self.ui.close_overlays();
        }
    }

    /// Copies the persisted UI mirror into the hot state used by features.
    fn load_ui_settings(&mut self, state: &GameState) {
        let settings = state.settings();
        self.ext = settings.ui.clone();
        let misc = &settings.misc;
        self.ext.hit_logs = misc.hit_log;
        self.ext.miss_logs = misc.miss_log;
        self.ext.reveal_radar = misc.reveal_radar;
        let tracers = &settings.visual.tracers;
        self.ext.hit_marker = tracers.hit_marker;
        self.ext.hit_marker_type = tracers.hit_marker_type;
        self.ext.hit_effect = tracers.hit_effect;
        self.ext.bullet_tracers = tracers.bullet_tracers;
        let audio = &misc.audio;
        self.ext.hit_sound = audio.hit_sound;
        self.ext.hit_sound_type = audio.hit_sound_type;
        self.ext.hit_sound_vol = audio.hit_sound_volume;
        self.ext.death_sound = audio.death_sound;
        self.ext.death_sound_type = audio.death_sound_type;
        self.ext.death_sound_vol = audio.death_sound_volume;
        let movement = &misc.movement;
        self.ext.jumpbug = movement.jumpbug;
        self.ext.mini_jump = movement.mini_jump;
        self.ext.fastladder = movement.fast_ladder;
        self.ext.edgejump = movement.edgejump;
        self.ext.edgestop = movement.edgestop;
        self.ext.edgebug = movement.edgebug;
        let viewmodel = &settings.visual.viewmodel;
        self.ext.thirdperson = viewmodel.thirdperson;
        self.ext.tp_dist = viewmodel.tp_dist;
        self.ext.tp_hull = viewmodel.tp_hull;
        self.ext.vm_xyzf = viewmodel.offset;
        self.ext.custom_fov = viewmodel.fov_override;
        self.ext.fov_val = viewmodel.fov_value;
        self.ext.scoped_fov_ov = viewmodel.scoped_fov_override;
        self.ext.scoped_fov = viewmodel.scoped_fov;
        self.ext.aspect_on = viewmodel.aspect_enabled;
        self.ext.aspect_val = viewmodel.aspect_value;
        self.ext.vm_adjust = viewmodel.adjust;
        let chams = &settings.visual.chams;
        self.ext.chams[0] = typed_preset_to_ui(&chams.enemy);
        self.ext.chams[1] = typed_preset_to_ui(&chams.team);
        self.ext.chams[2] = typed_preset_to_ui(&chams.local);
        self.ext.chams[3] = typed_preset_to_ui(&chams.weapon);
        self.ext.chams[4] = typed_preset_to_ui(&chams.arms);
        for (slot, preset) in self.ext.ragdoll_chams.iter_mut().zip(chams.ragdoll.iter()) {
            *slot = typed_preset_to_ui(preset);
        }
        // Legacy flat EspSettings still feeds the floating preview panel; the
        // Celerity-style per-target configs below are the source of truth for
        // the world overlay (enemies on subtab 0, teammates on subtab 1).
        let esp = &settings.esp;
        {
            let ov = &mut self.ext.overlays[0];
            ov.enabled = esp.enabled;
            ov.box_on = esp.r#box;
            ov.box_style = esp.box_style;
            ov.corner_len = esp.corner_size;
            ov.box_color = esp.box_color;
            ov.skel_on = esp.skeleton;
            ov.skel_thick = esp.skeleton_thickness;
            ov.skel_vis = esp.skeleton_visible_color;
            ov.skel_occ = esp.skeleton_occluded_color;
            ov.health.enabled = esp.health || esp.health_bar;
            ov.health.position = esp.health_pos;
            ov.health.full_color = esp.health_full_color;
            ov.health.low_color = esp.health_low_color;
            ov.name_on = esp.name;
            ov.name_color = esp.name_color;
            ov.weapon_on = esp.weapon;
            ov.ammo.enabled = esp.ammo;
            ov.ammo.full_color = esp.ammo_color;
            ov.flags[0] = esp.distance;
            ov.flags[2] = esp.armor;
        }
        esp_target_to_overlay(&settings.esp.enemies, &mut self.ext.overlays[0]);
        esp_target_to_overlay(&settings.esp.teammates, &mut self.ext.overlays[1]);
        let ambience = &settings.visual.ambience;
        self.ext.skybox_mat = ambience.skybox;
        self.ext.skybox_idx = ambience.skybox_idx;
        self.ext.skybox_col = ambience.skybox_color;
        self.ext.sky_color = ambience.sky_color;
        self.ext.cloud_color = ambience.cloud_color;
        self.ext.sun_color = ambience.sun_color;
        self.ext.world_mat = ambience.world_material;
        self.ext.world_mat_type = ambience.world_material_type;
        self.ext.world_mat_col = ambience.world_material_color;
        self.ext.lighting = ambience.lighting;
        self.ext.lighting_int = ambience.lighting_intensity;
        self.ext.lighting_col = ambience.lighting_color;
        self.ext.fog_on = ambience.fog;
        self.ext.fog_density = ambience.fog_density;
        self.ext.fog_col = ambience.fog_color;
        self.ext.weather_on = ambience.weather_on;
        self.ext.weather_type = ambience.weather;
        self.ext.weather_col = ambience.weather_color;
        self.ext.proj_traj = settings.visual.grenade.trajectory;
        if settings.visual.items.groups.len() == 6 {
            self.ext.item_esp.copy_from_slice(&settings.visual.items.groups);
        }
        self.ext.item_dist = settings.visual.items.max_distance;
        self.ext.item_display = settings.visual.items.display;
        let autobuy = &misc.autobuy;
        self.ext.autobuy = autobuy.enabled;
        self.ext.ab_nades = autobuy.grenades;
        self.ext.ab_armor = autobuy.armor;
        self.ext.ab_defuser = autobuy.defuser;
        self.ext.ab_taser = autobuy.taser;
    }

    /// Writes legacy controls back into the typed Settings blocks so feature
    /// code can read them without touching the UI mirror.
    fn commit_ui_settings(&self, state: &GameState) {
        let ext = &self.ext;
        let mut settings = state.settings_mut();
        settings.ui = ext.clone();
        let chams = &mut settings.visual.chams;
        chams.enemy = ui_preset_to_typed(&ext.chams[0]);
        chams.team = ui_preset_to_typed(&ext.chams[1]);
        chams.local = ui_preset_to_typed(&ext.chams[2]);
        chams.weapon = ui_preset_to_typed(&ext.chams[3]);
        chams.arms = ui_preset_to_typed(&ext.chams[4]);
        chams.ragdoll = ext.ragdoll_chams.iter().map(ui_preset_to_typed).collect();

        let esp = &mut settings.esp;
        let ov = &ext.overlays[0];
        esp.enabled = ov.enabled;
        esp.r#box = ov.box_on;
        esp.box_style = ov.box_style;
        esp.corner_size = ov.corner_len;
        esp.box_color = ov.box_color;
        esp.skeleton = ov.skel_on;
        esp.skeleton_thickness = ov.skel_thick;
        esp.skeleton_visible_color = ov.skel_vis;
        esp.skeleton_occluded_color = ov.skel_occ;
        esp.health = ov.health.enabled;
        esp.health_bar = ov.health.enabled;
        esp.health_pos = ov.health.position;
        esp.health_full_color = ov.health.full_color;
        esp.health_low_color = ov.health.low_color;
        esp.name = ov.name_on;
        esp.name_color = ov.name_color;
        esp.weapon = ov.weapon_on;
        esp.ammo = ov.ammo.enabled;
        esp.ammo_color = ov.ammo.full_color;
        esp.armor = ov.flags[2];
        esp.distance = ov.flags[0];
        let esp = &mut settings.esp;
        esp_target_from_overlay(&ext.overlays[0], &mut esp.enemies);
        esp_target_from_overlay(&ext.overlays[1], &mut esp.teammates);
        // Master toggle: the world overlay draws when at least one target
        // class is enabled and the global ESP switch is on.
        esp.enabled = esp.enemies.enabled || esp.teammates.enabled;
        settings.misc.hit_log = ext.hit_logs;
        settings.misc.miss_log = ext.miss_logs;
        settings.misc.reveal_radar = ext.reveal_radar;

        let tracers = &mut settings.visual.tracers;
        tracers.hit_marker = ext.hit_marker;
        tracers.hit_marker_type = ext.hit_marker_type;
        tracers.hit_effect = ext.hit_effect;
        tracers.bullet_tracers = ext.bullet_tracers;

        let audio = &mut settings.misc.audio;
        audio.hit_sound = ext.hit_sound;
        audio.hit_sound_type = ext.hit_sound_type;
        audio.hit_sound_volume = ext.hit_sound_vol;
        audio.death_sound = ext.death_sound;
        audio.death_sound_type = ext.death_sound_type;
        audio.death_sound_volume = ext.death_sound_vol;

        let movement = &mut settings.misc.movement;
        movement.jumpbug = ext.jumpbug;
        movement.mini_jump = ext.mini_jump;
        movement.fast_ladder = ext.fastladder;
        movement.edgejump = ext.edgejump;
        movement.edgestop = ext.edgestop;
        movement.edgebug = ext.edgebug;

        let viewmodel = &mut settings.visual.viewmodel;
        viewmodel.thirdperson = ext.thirdperson;
        viewmodel.tp_dist = ext.tp_dist;
        viewmodel.tp_hull = ext.tp_hull;
        viewmodel.offset = ext.vm_xyzf;
        viewmodel.fov_override = ext.custom_fov;
        viewmodel.fov_value = ext.fov_val;
        viewmodel.scoped_fov_override = ext.scoped_fov_ov;
        viewmodel.scoped_fov = ext.scoped_fov;
        viewmodel.aspect_enabled = ext.aspect_on;
        viewmodel.aspect_value = ext.aspect_val;
        viewmodel.adjust = ext.vm_adjust;
        viewmodel.fov = ext.vm_xyzf[3];

        let ambience = &mut settings.visual.ambience;
        ambience.skybox = ext.skybox_mat;
        ambience.skybox_idx = ext.skybox_idx;
        ambience.skybox_color = ext.skybox_col;
        ambience.sky_color = ext.sky_color;
        ambience.cloud_color = ext.cloud_color;
        ambience.sun_color = ext.sun_color;
        ambience.world_material = ext.world_mat;
        ambience.world_material_type = ext.world_mat_type;
        ambience.world_material_color = ext.world_mat_col;
        ambience.lighting = ext.lighting;
        ambience.lighting_intensity = ext.lighting_int;
        ambience.lighting_color = ext.lighting_col;
        ambience.fog = ext.fog_on;
        ambience.fog_density = ext.fog_density;
        ambience.fog_color = ext.fog_col;
        ambience.weather_on = ext.weather_on;
        ambience.weather = ext.weather_type;
        ambience.weather_color = ext.weather_col;

        settings.visual.grenade.trajectory = ext.proj_traj;
        settings.visual.items.groups = ext.item_esp.to_vec();
        settings.visual.items.enabled = ext.item_esp.iter().any(|v| *v);
        settings.visual.items.max_distance = ext.item_dist;
        settings.visual.items.display = ext.item_display;

        let autobuy = &mut settings.misc.autobuy;
        autobuy.enabled = ext.autobuy;
        autobuy.grenades = ext.ab_nades;
        autobuy.armor = ext.ab_armor;
        autobuy.defuser = ext.ab_defuser;
        autobuy.taser = ext.ab_taser;

        let applied: Vec<_> = ext
            .applied_skins
            .iter()
            .map(|(&def_index, &(paint_index, wear, seed, stattrak))| {
                wallbang_config::settings::AppliedSkin {
                    def_index,
                    paint_index,
                    wear,
                    seed,
                    stattrak,
                }
            })
            .collect();
        settings.skins.applied = applied;
    }

    pub fn frame(&mut self, dl: &mut DrawList, state: &GameState, screen_w: f32, screen_h: f32) {
        self.ui.begin_frame();
        let toggle_key = state.settings().menu.toggle_key as u16;
        let key_now = wallbang_core::input::key_down(toggle_key);
        if key_now && !self.toggle_held {
            self.toggle();
        }
        self.toggle_held = key_now;
        MENU_OPEN.store(self.open, Ordering::Relaxed);
        // Menu open: free the OS cursor (game clips it and consumes raw input).
        crate::hooks::update_menu_cursor(self.open);
        if self.open {
            crate::hooks::enforce_menu_cursor();
        }
        self.cursor.sync(self.open);
        self.load_ui_settings(state);
        self.preview.sync_settings(state);
        // HUD widgets are intentionally disabled: this build only has the menu
        // shell, so showing hotkey/indicator/spectator placeholders would be
        // fake data. Re-enable `draw_hud_widgets` once the features exist.
        if !self.open || self.tab != PREVIEW_TAB {
            let cfg = self.preview.native_config(state);
            crate::native_preview::set_request(false, (0.0, 0.0, 0.0, 0.0), (screen_w, screen_h), cfg);
            crate::preview::clear_model_request();
        }
        if !self.open {
            self.commit_ui_settings(state);
            self.preview.commit_settings(state);
            return;
        }
        self.draw_menu_internal(dl, state, screen_w, screen_h);
        self.commit_ui_settings(state);
        self.preview.commit_settings(state);
        dl.flush_top();
    }

    /// Offline preview helper: renders the menu directly at a chosen tab/subtab without OS cursor polling.
    pub fn render_preview_frame(
        &mut self,
        dl: &mut DrawList,
        state: &GameState,
        screen_w: f32,
        screen_h: f32,
        tab: usize,
        subtab: usize,
    ) {
        self.open = true;
        self.tab = tab.min(NEW_TABS.len() - 1);
        self.subtab = subtab.min(2);
        self.ui.set_synthetic_mouse(0.0, 0.0);
        self.draw_menu_internal(dl, state, screen_w, screen_h);
        dl.flush_top();
    }

    fn draw_menu_internal(
        &mut self,
        dl: &mut DrawList,
        state: &GameState,
        screen_w: f32,
        screen_h: f32,
    ) {
        let (mut mx, mut my, mw, mh, accent) = {
            let s = state.settings();
            let mw = DEFAULT_W.max(s.menu.w).min((screen_w - 24.0).max(320.0));
            let mh = DEFAULT_H.max(s.menu.h).min((screen_h - 24.0).max(240.0));
            let mx = if s.menu.x <= 0.0 && s.menu.y <= 0.0 {
                ((screen_w - mw) * 0.5).max(16.0)
            } else {
                s.menu.x
            };
            let my = if s.menu.x <= 0.0 && s.menu.y <= 0.0 {
                ((screen_h - mh) * 0.5).max(16.0)
            } else {
                s.menu.y
            };
            let accent = if s.menu.accent == [120, 180, 255, 255] {
                [164, 204, 232, 255]
            } else {
                s.menu.accent
            };
            (
                mx.clamp(0.0, (screen_w - mw).max(0.0)),
                my.clamp(0.0, (screen_h - mh).max(0.0)),
                mw,
                mh,
                accent,
            )
        };

        // The whole top strip is the drag handle while the shell is empty.
        self.ui
            .handle_window_drag(&mut mx, &mut my, mw, HEADER_H, screen_w, screen_h, mw, mh);
        {
            let mut s = state.settings_mut();
            s.menu.x = mx;
            s.menu.y = my;
            s.menu.w = mw;
            s.menu.h = mh;
        }
        self.menu_rect = (mx, my, mw, mh);

        let style = Style::new(accent);
        self.ui.panel(dl, mx, my, mw, mh, &style);

        // Бренд остаётся в шапке; вся шапка — ручка перетаскивания окна.
        let brand_x = mx + GAP + 4.0;
        let brand_y = my + GAP + 4.0;
        dl.draw_wallbang_logo(brand_x, brand_y, 46.0, 28.0);
        let word_x = brand_x + 46.0 + 6.0;
        let word_h = dl.brand_text_height(1.0);
        dl.text_brand(
            word_x,
            (brand_y + (28.0 - word_h) * 0.5).floor(),
            "WALLBANG",
            style.text,
            1.0,
        );
        dl.line(
            mx + GAP,
            my + HEADER_H - 4.0,
            mx + mw - GAP,
            my + HEADER_H - 4.0,
            1.0,
            style.border,
        );

        self.draw_sidebar_v2(dl, mx, my, mh, &style);
        self.draw_content_v2(dl, state, mx, my, mw, mh, &style);
        // Panels write into `self.ext`; copy them into typed Settings before
        // the floating preview samples chams/ESP state on the same frame.
        self.commit_ui_settings(state);
        self.preview.sync_settings(state);

        // Панель ESP открывается только на вкладке Players.
        if self.tab == PREVIEW_TAB {
            let pw = 320.0;
            let ph = mh;
            let mut px = mx + mw + 12.0;
            if px + pw > screen_w - 8.0 {
                px = mx - pw - 12.0;
            }
            if px < 8.0 {
                px = (screen_w - pw - 8.0).max(8.0);
            }
            let py = my.min((screen_h - ph - 8.0).max(8.0)).max(8.0);
            self.preview.draw_panel(
                state,
                &mut self.ext.overlays[0],
                &mut self.ui,
                dl,
                px,
                py,
                pw,
                ph,
                &style,
            );
        }
    }

    /// Новый сайдбар: только навигация по разделам, без выдуманных настроек.
    fn draw_sidebar_v2(&mut self, dl: &mut DrawList, mx: f32, my: f32, mh: f32, s: &Style) {
        let sx = mx + GAP * 0.5;
        let sy = my + HEADER_H;
        let sw = NEW_SIDEBAR_W;
        let sh = (mh - HEADER_H - GAP * 0.5).max(40.0);
        dl.rect_rounded(sx, sy, sw, sh, 4.0, [11, 11, 13, 255]);
        dl.outline_rounded(sx, sy, sw, sh, 4.0, 1.0, s.border);

        let labels: [(usize, &str, f32); 7] = [
            (0, NEW_TABS[0], 24.0),
            (1, NEW_TABS[1], 54.0),
            (2, NEW_TABS[2], 108.0),
            (3, NEW_TABS[3], 138.0),
            (4, NEW_TABS[4], 168.0),
            (5, NEW_TABS[5], 198.0),
            (6, NEW_TABS[6], 252.0),
        ];
        let row_w = sw - 12.0;
        let row_h = ROW_H - 4.0;
        dl.text(sx + 12.0, sy + 12.0, "AIMBOT", s.text_dim, 0.82);
        dl.line(sx + 12.0, sy + 88.0, sx + sw - 12.0, sy + 88.0, 1.0, s.border);
        dl.text(sx + 12.0, sy + 94.0, "COMMON", s.text_dim, 0.82);
        dl.line(sx + 12.0, sy + 232.0, sx + sw - 12.0, sy + 232.0, 1.0, s.border);
        dl.text(sx + 12.0, sy + 238.0, "PRESETS", s.text_dim, 0.82);

        for (idx, label, offset_y) in labels {
            let y = sy + offset_y;
            if y + row_h > sy + sh - 4.0 {
                break;
            }
            let x = sx + 6.0;
            let selected = self.tab == idx;
            let hover = !self.ui.overlay_blocking() && self.ui.inside(x, y, row_w, row_h);
            let bg = if selected {
                s.active
            } else if hover {
                s.hovered
            } else {
                s.elevated
            };
            dl.rect_rounded(x, y, row_w, row_h, 3.0, bg);
            if selected {
                dl.rect(x, y + 4.0, 2.5, row_h - 8.0, s.accent);
            }
            let color = if selected || hover { s.text } else { s.text_dim };
            dl.text(x + 12.0, y + (row_h - dl.text_height(1.0)) * 0.5, label, color, 1.0);
            if hover && self.ui.mouse_clicked() {
                self.tab = idx;
            }
        }
    }

    /// Контент меню пока пуст; ESP-превью рисуется отдельной плавающей панелью.
    fn draw_content_v2(
        &mut self,
        dl: &mut DrawList,
        state: &GameState,
        mx: f32,
        my: f32,
        mw: f32,
        mh: f32,
        s: &Style,
    ) {
        let x = mx + GAP * 0.5 + NEW_SIDEBAR_W + GAP;
        let y = my + HEADER_H;
        let w = mx + mw - GAP * 0.5 - x;
        let h = (mh - HEADER_H - GAP * 0.5).max(40.0);
        if w < 120.0 || h < 120.0 {
            return;
        }
        dl.rect_rounded(x, y, w, h, 5.0, s.card);
        dl.outline_rounded(x, y, w, h, 5.0, 1.0, s.border);
        if self.tab == CONFIG_TAB {
            self.draw_config(dl, state, x, y, w, h, s);
            return;
        }
        if (PREVIEW_TAB..=5).contains(&self.tab) {
            self.draw_top_bar(dl, x, y, w, s);
            let content_y = y + SUBTAB_BAR_H;
            let content_h = (h - SUBTAB_BAR_H).max(80.0);
            match self.tab {
                PREVIEW_TAB => self.draw_players(
                    dl, state, mx, my, mw, x, content_y, w, content_h, s,
                ),
                3 => self.draw_world(dl, x, content_y, w, content_h, s),
                4 => self.draw_skins(dl, x, content_y, w, content_h, s),
                5 => self.draw_misc(dl, state, x, content_y, w, content_h, s),
                _ => {}
            }
            return;
        }
        let title = NEW_TABS[self.tab.min(NEW_TABS.len() - 1)];
        dl.text(x + 18.0, y + 16.0, title, s.text, 1.0);
        dl.line(x + 18.0, y + 38.0, x + w - 18.0, y + 38.0, 1.0, s.border);
        let empty = if self.tab == PREVIEW_TAB {
            "ESP preview opens on the right"
        } else {
            "No features available yet"
        };
        let tw = dl.text_width(empty, 1.0);
        dl.text(
            x + (w - tw) * 0.5,
            y + h * 0.5 - 8.0,
            empty,
            s.text_dim,
            1.0,
        );
    }

    #[inline]
    fn menu_covers(&self, x: f32, y: f32) -> bool {
        let (mx, my, mw, mh) = self.menu_rect;
        x >= mx && x <= mx + mw && y >= my && y <= my + mh
    }

    /// Port of C++ `ui::position`: normalized placement plus header drag with 4px snapping.
    #[allow(dead_code)]
    fn place_hud(
        &mut self,
        id: usize,
        w: f32,
        h: f32,
        screen_w: f32,
        screen_h: f32,
        header_h: f32,
    ) -> (f32, f32) {
        let margin = 14.0f32;
        let mx = margin.min(screen_w * 0.5);
        let my = margin.min(screen_h * 0.5);
        let w = w.clamp(0.0, (screen_w - mx * 2.0).max(0.0));
        let h = h.clamp(0.0, (screen_h - my * 2.0).max(0.0));
        let place = |nx: f32, ny: f32| {
            (
                (mx + nx.clamp(0.0, 1.0) * (screen_w - mx * 2.0 - w).max(0.0)).floor(),
                (my + ny.clamp(0.0, 1.0) * (screen_h - my * 2.0 - h).max(0.0)).floor(),
            )
        };
        let (nx, ny) = (self.ext.hud_pos[id][0], self.ext.hud_pos[id][1]);
        let (mut x, mut y) = place(nx, ny);
        let mouse = self.ui.mouse();
        let down = self.ui.mouse_down();
        if !down || !self.open || !self.ext.edit_hud {
            self.hud_drag_owner = None;
            return (x, y);
        }
        if self.hud_drag_owner.is_none()
            && self.ui.mouse_clicked()
            && !self.ui.overlay_blocking()
            && !self.menu_covers(mouse.x, mouse.y)
            && mouse.x >= x
            && mouse.x < x + w
            && mouse.y >= y
            && mouse.y < y + header_h.min(h)
        {
            self.hud_drag_owner = Some(id);
            self.hud_drag_offset = Vec2::new(mouse.x - x, mouse.y - y);
        }
        if self.hud_drag_owner == Some(id) {
            let free_w = (screen_w - mx * 2.0 - w).max(1.0);
            let free_h = (screen_h - my * 2.0 - h).max(1.0);
            let snap = |v: f32| (v / 4.0).round() * 4.0;
            let new_nx = ((snap(mouse.x - self.hud_drag_offset.x) - mx) / free_w).clamp(0.0, 1.0);
            let new_ny = ((snap(mouse.y - self.hud_drag_offset.y) - my) / free_h).clamp(0.0, 1.0);
            self.ext.hud_pos[id] = [new_nx, new_ny];
            (x, y) = place(new_nx, new_ny);
        }
        (x, y)
    }

    #[allow(dead_code)]
    fn draw_hud_widgets(
        &mut self,
        dl: &mut DrawList,
        state: &GameState,
        screen_w: f32,
        screen_h: f32,
    ) {
        let accent = if state.settings().menu.accent == [120, 180, 255, 255] {
            [164, 204, 232, 255]
        } else {
            state.settings().menu.accent
        };
        let s = Style::new(accent);
        let th = dl.text_height(1.0);

        // 1) Watermark: 28px tall, 14px parallel slants, 18px padding (C++ ui.hpp).
        if state.settings().misc.watermark {
            let segs = ["nexz0r", "0 ms", "0 u/s", "169 fps", "64 tick", "14:04"];
            let brand = "WALLBANG";
            let padding = 18.0;
            let logo_w = 30.0;
            let logo_h = 18.0;
            let logo_gap = 8.0;
            let field_gap = 20.0;
            let divider_h = 10.0;
            let slant = 14.0;
            let wm_h = 28.0;
            let brand_w = dl.brand_text_width(brand, 1.0);
            let mut wm_w = padding * 2.0 + logo_w + logo_gap + brand_w;
            for seg in &segs {
                wm_w += field_gap + dl.text_width(seg, 1.0);
            }
            let (wm_x, wm_y) = self.place_hud(0, wm_w, wm_h, screen_w, screen_h, wm_h);
            dl.parallelogram(
                wm_x,
                wm_y,
                wm_w,
                wm_h,
                slant,
                [8, 8, 10, 238],
                [36, 36, 40, 255],
            );
            dl.push_clip(
                wm_x + 1.0,
                wm_y + 1.0,
                (wm_w - 2.0).max(0.0),
                (wm_h - 2.0).max(0.0),
            );
            let mut px = wm_x + padding;
            dl.draw_wallbang_logo(px, wm_y + (wm_h - logo_h) * 0.5, logo_w, logo_h);
            dl.rect(px + 9.0, wm_y + wm_h - 2.0, 12.0, 1.0, s.accent);
            px += logo_w + logo_gap;
            let brand_h = dl.brand_text_height(1.0);
            dl.text_brand(
                px,
                (wm_y + (wm_h - brand_h) * 0.5).floor(),
                brand,
                s.text,
                1.0,
            );
            px += brand_w;
            for seg in &segs {
                dl.rect(
                    (px + field_gap * 0.5).floor(),
                    (wm_y + (wm_h - divider_h) * 0.5).floor(),
                    1.0,
                    divider_h,
                    s.border,
                );
                px += field_gap;
                let width = dl.text_width(seg, 1.0);
                dl.text(
                    px,
                    (wm_y + (wm_h - th) * 0.5).floor(),
                    seg,
                    s.text_dim,
                    1.0,
                );
                px += width;
            }
            dl.pop_clip();
        }

        // 2) Active hotkeys: draggable chamfered panel on the left.
        if self.ext.hotkeys_hud {
            let rows = [
                ("quick peek", "hold"),
                ("minimum damage", "11 dmg"),
                ("slow walk", "hold"),
            ];
            let w = 248.0;
            let header_h = 28.0;
            let row_h = 24.0;
            let h = header_h + rows.len() as f32 * row_h + 4.0;
            let (x, y) = self.place_hud(1, w, h, screen_w, screen_h, header_h);
            dl.chamfered_panel(x, y, w, h, 5.0, [8, 8, 10, 236], [30, 30, 34, 255]);
            dl.text(
                x + 12.0,
                (y + (header_h - th) * 0.5).floor(),
                "hotkeys",
                s.text,
                1.0,
            );
            let count = rows.len().to_string();
            let cw = dl.text_width(&count, 1.0);
            dl.text(
                x + w - cw - 12.0,
                (y + (header_h - th) * 0.5).floor(),
                &count,
                s.text_dim,
                1.0,
            );
            dl.line(x + 1.0, y + header_h, x + w - 1.0, y + header_h, 1.0, s.border);
            for (i, (name, val)) in rows.iter().enumerate() {
                let ry = y + header_h + i as f32 * row_h;
                dl.rect(x + 4.0, ry + 8.0, 2.0, 8.0, s.accent);
                dl.text(
                    x + 12.0,
                    (ry + (row_h - th) * 0.5).floor(),
                    name,
                    s.text,
                    1.0,
                );
                let vw = dl.text_width(val, 1.0);
                dl.text(
                    x + w - 12.0 - vw,
                    (ry + (row_h - th) * 0.5).floor(),
                    val,
                    s.accent,
                    1.0,
                );
            }
        }

        // 3) Spectators: draggable chamfered panel on the right.
        if self.ext.spectator_list && self.tab != 2 {
            let rows = [("observer_01", "watching"), ("observer_02", "watching")];
            let w = 220.0;
            let header_h = 28.0;
            let row_h = 24.0;
            let h = header_h + rows.len() as f32 * row_h + 4.0;
            let (x, y) = self.place_hud(3, w, h, screen_w, screen_h, header_h);
            dl.chamfered_panel(x, y, w, h, 5.0, [8, 8, 10, 236], [30, 30, 34, 255]);
            dl.text(
                x + 12.0,
                (y + (header_h - th) * 0.5).floor(),
                "spectators",
                s.text,
                1.0,
            );
            let count = rows.len().to_string();
            let cw = dl.text_width(&count, 1.0);
            dl.text(
                x + w - cw - 12.0,
                (y + (header_h - th) * 0.5).floor(),
                &count,
                s.text_dim,
                1.0,
            );
            dl.line(x + 1.0, y + header_h, x + w - 1.0, y + header_h, 1.0, s.border);
            for (i, (name, val)) in rows.iter().enumerate() {
                let ry = y + header_h + i as f32 * row_h;
                dl.text(
                    x + 12.0,
                    (ry + (row_h - th) * 0.5).floor(),
                    name,
                    s.text,
                    1.0,
                );
                let vw = dl.text_width(val, 1.0);
                dl.text(
                    x + w - 12.0 - vw,
                    (ry + (row_h - th) * 0.5).floor(),
                    val,
                    s.text_dim,
                    1.0,
                );
            }
        }

        // 4) Status indicators: draggable panel.
        if self.ext.indicators_hud {
            let rows = [("AA", "back"), ("DMG", "30"), ("HC", "60%"), ("AIR STOP", "on")];
            let w = 150.0;
            let row_h = 24.0;
            let h = rows.len() as f32 * row_h + 4.0;
            let (x, y) = self.place_hud(2, w, h, screen_w, screen_h, row_h);
            dl.chamfered_panel(x, y, w, h, 5.0, [8, 8, 10, 236], [30, 30, 34, 255]);
            for (i, (k, v)) in rows.iter().enumerate() {
                let ry = y + 2.0 + i as f32 * row_h;
                dl.rect(x + 4.0, ry + 8.0, 2.0, 8.0, s.accent);
                dl.text(
                    x + 12.0,
                    (ry + (row_h - th) * 0.5).floor(),
                    k,
                    s.text,
                    1.0,
                );
                let vw = dl.text_width(v, 1.0);
                dl.text(
                    x + w - 12.0 - vw,
                    (ry + (row_h - th) * 0.5).floor(),
                    v,
                    s.accent,
                    1.0,
                );
            }
        }
    }

    fn draw_sidebar(&mut self, dl: &mut DrawList, mx: f32, my: f32, s: &Style) {
        // Brand logo + wordmark (menu.core.cpp:136-143 + branding_layout.hpp).
        let brand_x = mx + 20.0;
        let brand_y = my + 16.0;
        dl.draw_wallbang_logo(brand_x, brand_y, 46.0, 28.0);
        let word_x = brand_x + 46.0 + 6.0;
        let word_h = dl.brand_text_height(1.0);
        dl.text_brand(
            word_x,
            (brand_y + (28.0 - word_h) * 0.5).floor(),
            "WALLBANG",
            s.text,
            1.0,
        );
        dl.line(mx + 20.0, my + HEADER_H - 4.0, mx + SIDEBAR_W, my + HEADER_H - 4.0, 1.0, s.border);

        let x = mx + GAP + 4.0;
        let btn_w = SIDEBAR_W - 12.0;
        let mut y = my + HEADER_H + 10.0;

        for section in 0..3 {
            let hover = !self.ui.overlay_blocking() && self.ui.inside(x, y, btn_w, SECTION_H);
            if hover && self.ui.mouse_clicked() {
                self.sidebar_open[section] = !self.sidebar_open[section];
            }
            if hover {
                dl.rect_rounded(x, y, btn_w, SECTION_H, WIDGET_ROUNDING, s.elevated);
            }
            dl.text(x + 12.0, y + 10.0, SECTIONS[section], s.text, 1.0);
            let arrow_x = x + btn_w - 14.0;
            let arrow_y = y + 14.0;
            if self.sidebar_open[section] {
                dl.line(arrow_x - 3.0, arrow_y - 2.0, arrow_x, arrow_y + 1.0, 1.2, s.text_dim);
                dl.line(arrow_x, arrow_y + 1.0, arrow_x + 3.0, arrow_y - 2.0, 1.2, s.text_dim);
            } else {
                dl.line(arrow_x - 1.0, arrow_y - 3.0, arrow_x + 2.0, arrow_y, 1.2, s.text_dim);
                dl.line(arrow_x + 2.0, arrow_y, arrow_x - 1.0, arrow_y + 3.0, 1.2, s.text_dim);
            }
            y += SECTION_H;

            if self.sidebar_open[section] {
                for offset in 0..COUNTS[section] {
                    let i = STARTS[section] + offset;
                    let row_hover = !self.ui.overlay_blocking() && self.ui.inside(x, y, btn_w, ROW_H);
                    if row_hover && self.ui.mouse_clicked() {
                        if self.tab != i {
                            self.tab = i;
                            self.subtab = 0;
                            self.ui.close_overlays();
                        }
                    }
                    let selected = self.tab == i;
                    if selected || row_hover {
                        dl.rect_rounded(x, y, btn_w, ROW_H, WIDGET_ROUNDING, s.elevated);
                    }
                    if selected {
                        dl.rect(x + 4.0, y + 10.0, 2.0, ROW_H - 20.0, s.accent);
                    }
                    let th = dl.text_height(1.0);
                    let col = if selected {
                        s.accent
                    } else if row_hover {
                        s.text
                    } else {
                        s.text_dim
                    };
                    dl.text(x + 22.0, (y + (ROW_H - th) * 0.5).floor(), SIDEBAR_LABELS[i], col, 1.0);
                    y += ROW_H;
                }
            }
            y += SECTION_GAP;
        }
    }

    fn draw_top_bar(&mut self, dl: &mut DrawList, x: f32, y: f32, w: f32, s: &Style) {
        // C++ uses the regular Inter font for page titles (menu.core.cpp:221-222).
        let title = PAGE_TITLES[self.tab];
        let title_w = dl.text_width(title, 1.0);
        let title_h = dl.text_height(1.0);
        dl.text(
            x,
            (y + (SUBTAB_BAR_H - title_h) * 0.5).floor(),
            title,
            s.text,
            1.0,
        );
        dl.line(x, y + SUBTAB_BAR_H, x + w, y + SUBTAB_BAR_H, 1.0, s.border);

        // Weapon selection lives inside aimbot group (menu.core.cpp:225)
        if self.tab <= 1 {
            return;
        }
        let defs = SUBTABS[self.tab];
        let tabs_x = x + title_w + 24.0;
        let btn_w = (w - title_w - 24.0).max(1.0) / defs.len().max(1) as f32;
        let th = dl.text_height(1.0);

        for (i, &name) in defs.iter().enumerate() {
            let bx = tabs_x + btn_w * i as f32;
            let hover = !self.ui.overlay_blocking() && self.ui.inside(bx, y, btn_w, SUBTAB_BAR_H);
            if hover && self.ui.mouse_clicked() {
                self.subtab = i;
                self.ui.close_overlays();
            }
            let selected = self.subtab == i;
            if hover {
                dl.rect_rounded(bx, y, btn_w, SUBTAB_BAR_H - 1.0, WIDGET_ROUNDING, s.elevated);
            }
            if selected {
                dl.rect(bx + 10.0, y + SUBTAB_BAR_H - 2.0, (btn_w - 20.0).max(1.0), 2.0, s.accent);
            }
            let tw = dl.text_width(name, 1.0);
            let col = if selected {
                s.accent
            } else if hover {
                s.text
            } else {
                s.text_dim
            };
            dl.text(
                (bx + (btn_w - tw) * 0.5).floor(),
                (y + (SUBTAB_BAR_H - th) * 0.5).floor(),
                name,
                col,
                1.0,
            );
        }
    }

    fn draw_ragebot(&mut self, dl: &mut DrawList, state: &GameState, x: f32, y: f32, w: f32, h: f32, s: &Style) {
        let mut settings = state.settings_mut();
        let combat = &mut settings.combat;
        let col_w = (w - GAP) * 0.5;
        let right_x = x + col_w + GAP;
        let top_h = (h - GAP) * 0.54;
        let bottom_h = (h - GAP) * 0.46;

        self.ui.child(dl, x, y, col_w, top_h, true, s, |ui, dl| {
            ui.text(dl, "Aimbot", s.text_dim, 1.0);
            ui.separator(dl, s);
            ui.combo(dl, "weapon group##rage", &mut combat.ragebot.weapon_group, &WEAPON_GROUPS, s);
            let idx = combat.ragebot.weapon_group.min(5);
            combat.ragebot.weapon_group = idx;
            let wg = &mut combat.ragebot.groups[idx];
            ui.checkbox(dl, "enabled", &mut combat.ragebot.enabled, s);
            ui.checkbox(dl, "silent", &mut combat.ragebot.silent, s);
            ui.checkbox(dl, "nospread", &mut combat.ragebot.no_spread, s);
            ui.slider_f32(dl, "fov", &mut wg.max_fov, 1.0, 180.0, 0, " deg", s);
            ui.slider_i32(dl, "mindamage", &mut wg.min_damage, 0, 130, "", s);
            ui.multicombo(dl, "hitboxes", &mut wg.hitboxes, &HITBOXES, s);
            ui.slider_f32(dl, "pointscale", &mut wg.pointscale, 0.0, 100.0, 0, "%", s);
            ui.checkbox(dl, "dynamic point scale", &mut wg.dynamic_pointscale, s);
        });

        self.ui.child(dl, x, y + top_h + GAP, col_w, bottom_h, true, s, |ui, dl| {
            ui.text(dl, "Accuracy & safety", s.text_dim, 1.0);
            ui.separator(dl, s);
            let idx = combat.ragebot.weapon_group.min(5);
            let wg = &mut combat.ragebot.groups[idx];
            ui.slider_i32(dl, "hitchance", &mut wg.hitchance, 0, 100, "%", s);
            ui.checkbox(dl, "hitchance override", &mut wg.hitchance_override, s);
            ui.popup(dl, "##hitchance_popup", POPUP_W, 56.0, s, |ui, dl| {
                ui.slider_i32(dl, "value##hc", &mut wg.hitchance_override_value, 1, 100, "%", s);
            });
            ui.checkbox(dl, "mindamage override", &mut wg.min_damage_override, s);
            ui.popup(dl, "##mindamage_popup", POPUP_W, 56.0, s, |ui, dl| {
                ui.slider_i32(dl, "value##md", &mut wg.min_damage_override_value, 0, 130, "", s);
            });
            ui.checkbox(dl, "lethal aim", &mut wg.lethal_damage, s);
            ui.checkbox(dl, "autostop", &mut wg.autostop, s);
            ui.checkbox(dl, "air autostop", &mut wg.air_autostop, s);
            ui.checkbox(dl, "extrapolation", &mut wg.extrapolation, s);
            if wg.extrapolation {
                ui.slider_i32(dl, "prediction ticks", &mut wg.extrapolation_ticks, 1, 12, "", s);
            }
            if combat.ragebot.weapon_group == 4 {
                ui.checkbox(dl, "autoscope", &mut combat.autos.scope, s);
            }
        });

        self.ui.child(dl, right_x, y, col_w, top_h, true, s, |ui, dl| {
            ui.text(dl, "Anti-aim", s.text_dim, 1.0);
            ui.separator(dl, s);
            ui.checkbox(dl, "anti aim", &mut combat.antiaim.enabled, s);
            let mut pitch = combat.antiaim.pitch.clamp(0, 2) as usize;
            if ui.combo(dl, "pitch", &mut pitch, &PITCH_ITEMS, s) {
                combat.antiaim.pitch = pitch as i32;
            }
            ui.checkbox(dl, "compensate roll", &mut combat.antiaim.auto_yaw_adjust, s);
            ui.checkbox(dl, "force left", &mut combat.antiaim.manual_left, s);
            ui.checkbox(dl, "force right", &mut combat.antiaim.manual_right, s);
            ui.checkbox(dl, "avoid backstab", &mut combat.antiaim.avoid_backstab, s);
            ui.checkbox(dl, "direction indicator", &mut combat.antiaim.direction_indicator, s);
        });

        self.ui.child(dl, right_x, y + top_h + GAP, col_w, bottom_h, true, s, |ui, dl| {
            ui.text(dl, "Assistance", s.text_dim, 1.0);
            ui.separator(dl, s);
            ui.checkbox(dl, "auto revolver", &mut combat.autos.revolver, s);
            ui.checkbox(dl, "zeusbot", &mut combat.zeusbot.enabled, s);
            ui.popup(dl, "##zb_settings", POPUP_W, 74.0, s, |ui, dl| {
                ui.slider_f32(dl, "max fov##zb", &mut combat.zeusbot.max_fov, 1.0, 180.0, 0, " deg", s);
                ui.checkbox(dl, "drop after##zb", &mut combat.zeusbot.drop_after, s);
            });
            ui.checkbox(dl, "knifebot", &mut combat.knifebot.enabled, s);
            ui.popup(dl, "##kb_settings", POPUP_W, 56.0, s, |ui, dl| {
                ui.slider_f32(dl, "max fov##kb", &mut combat.knifebot.max_fov, 1.0, 180.0, 0, " deg", s);
            });
            ui.checkbox(dl, "quick peek assist", &mut combat.quickpeek.enabled, s);
            ui.popup(dl, "##qp_colors", POPUP_W, 68.0, s, |ui, dl| {
                ui.color_picker(dl, "base color##qp", &mut combat.quickpeek.color, s);
                ui.color_picker(dl, "retracting color##qp", &mut combat.quickpeek.retrack_color, s);
            });
            ui.checkbox(dl, "duck peek assist", &mut combat.duckpeek.enabled, s);
        });
    }

    fn draw_legitbot(&mut self, dl: &mut DrawList, state: &GameState, x: f32, y: f32, w: f32, h: f32, s: &Style) {
        let mut settings = state.settings_mut();
        let combat = &mut settings.combat;
        let col_w = (w - GAP) * 0.5;
        let right_x = x + col_w + GAP;
        let master_w = if combat.legit.enabled { col_w } else { w };
        let master_h = 114.0;

        self.ui.child(dl, x, y, master_w, master_h, true, s, |ui, dl| {
            ui.text(dl, "Aimbot", s.text_dim, 1.0);
            ui.separator(dl, s);
            ui.combo(dl, "weapon group##legit", &mut combat.legit.weapons, &WEAPON_GROUPS, s);
            ui.checkbox(dl, "enabled", &mut combat.legit.enabled, s);
        });
        if !combat.legit.enabled {
            return;
        }
        let idx = combat.legit.weapons.min(5);
        combat.legit.weapons = idx;

        let rem_h = (h - master_h - GAP * 2.0).max(160.0);
        let aim_h = rem_h * 0.58;
        let rcs_h = rem_h * 0.42;

        self.ui.child(dl, x, y + master_h + GAP, col_w, aim_h, true, s, |ui, dl| {
            ui.text(dl, "Aim assistance", s.text_dim, 1.0);
            ui.separator(dl, s);
            let wg = &mut combat.legit.groups[idx];
            ui.checkbox(dl, "aimbot", &mut wg.aimbot, s);
            ui.checkbox(dl, "autostop", &mut wg.autostop, s);
            if combat.legit.weapons == 4 {
                ui.checkbox(dl, "autoscope", &mut wg.autoscope, s);
            }
            ui.slider_f32(dl, "fov", &mut wg.fov, 0.5, 30.0, 1, " deg", s);
            ui.slider_i32(dl, "smooth", &mut wg.smooth, 0, 100, "", s);
            ui.multicombo(dl, "hitboxes", &mut wg.hitboxes, &LEGIT_HITBOXES, s);
            ui.checkbox(dl, "draw fov", &mut wg.visualize_fov, s);
            ui.popup(dl, "##fov_color_popup", POPUP_W, 44.0, s, |ui, dl| {
                ui.color_picker(dl, "color##fov", &mut wg.fov_color, s);
            });
        });

        self.ui
            .child(dl, x, y + master_h + GAP + aim_h + GAP, col_w, rcs_h, true, s, |ui, dl| {
                ui.text(dl, "Recoil control", s.text_dim, 1.0);
                ui.separator(dl, s);
                let wg = &mut combat.legit.groups[idx];
                ui.checkbox(dl, "rcs", &mut wg.rcs, s);
                ui.popup(dl, "##rcs_popup", POPUP_W, 88.0, s, |ui, dl| {
                    ui.slider_i32(dl, "min##rcs", &mut wg.rcs_min, 50, 150, "%", s);
                    ui.slider_i32(dl, "max##rcs", &mut wg.rcs_max, 50, 150, "%", s);
                });
                ui.checkbox(dl, "standalone rcs", &mut wg.standalone_rcs, s);
                ui.popup(dl, "##srcs_popup", POPUP_W, 120.0, s, |ui, dl| {
                    ui.slider_i32(dl, "strength##srcs", &mut wg.standalone_rcs_strength, 0, 100, "%", s);
                    ui.slider_i32(dl, "min##srcs", &mut wg.standalone_rcs_min, 50, 150, "%", s);
                    ui.slider_i32(dl, "max##srcs", &mut wg.standalone_rcs_max, 50, 150, "%", s);
                });
            });

        let half_h = (h - GAP) * 0.5;
        self.ui.child(dl, right_x, y, col_w, half_h, true, s, |ui, dl| {
            ui.text(dl, "Triggerbot", s.text_dim, 1.0);
            ui.separator(dl, s);
            let wg = &mut combat.legit.groups[idx];
            ui.checkbox(dl, "triggerbot", &mut wg.triggerbot, s);
            ui.slider_i32(dl, "delay", &mut wg.trigger_delay, 0, 250, " ms", s);
            ui.slider_i32(dl, "hitchance", &mut wg.trigger_hitchance, 0, 100, "%", s);
            ui.checkbox(dl, "seed prediction", &mut wg.give_me_your_seed, s);
        });

        self.ui.child(dl, right_x, y + half_h + GAP, col_w, half_h, true, s, |ui, dl| {
            ui.text(dl, "Other", s.text_dim, 1.0);
            ui.separator(dl, s);
            let wg = &mut combat.legit.groups[idx];
            ui.checkbox(dl, "autowall", &mut wg.autowall, s);
            ui.popup(dl, "##aw_popup", POPUP_W, 56.0, s, |ui, dl| {
                ui.slider_i32(dl, "min damage##aw", &mut wg.min_damage, 1, 125, "", s);
            });
        });
    }

    fn draw_chams_cfg(ui: &mut Ui, dl: &mut DrawList, title: &str, suffix: &str, cfg: &mut ChamsCfg, show_overlay: bool, s: &Style) {
        ui.checkbox(dl, &format!("{}##{}", title, suffix), &mut cfg.enabled, s);
        ui.checkbox(dl, &format!("primary layer##{}", suffix), &mut cfg.primary.enabled, s);
        ui.popup(dl, &format!("##primary_{}", suffix), POPUP_W, 86.0, s, |ui, dl| {
            ui.combo(dl, "material", &mut cfg.primary.material, &CHAM_MATERIALS, s);
            ui.color_picker(dl, "color", &mut cfg.primary.color, s);
        });
        ui.checkbox(dl, &format!("secondary layer##{}", suffix), &mut cfg.secondary.enabled, s);
        ui.popup(dl, &format!("##secondary_{}", suffix), POPUP_W, 86.0, s, |ui, dl| {
            ui.combo(dl, "material", &mut cfg.secondary.material, &CHAM_MATERIALS, s);
            ui.color_picker(dl, "color", &mut cfg.secondary.color, s);
        });
        if show_overlay {
            ui.checkbox(dl, &format!("overlay layer##{}", suffix), &mut cfg.overlay.enabled, s);
            ui.popup(dl, &format!("##overlay_{}", suffix), POPUP_W, 86.0, s, |ui, dl| {
                ui.combo(dl, "material", &mut cfg.overlay.material, &CHAM_MATERIALS, s);
                ui.color_picker(dl, "color", &mut cfg.overlay.color, s);
            });
            if ui.button(dl, &format!("Wireframe overlay##{}", suffix), 160.0, 24.0, s) {
                cfg.enabled = true;
                cfg.overlay.enabled = true;
                cfg.overlay.material = 19;
            }
        }
    }

    fn draw_players(
        &mut self,
        dl: &mut DrawList,
        _state: &GameState,
        _mx: f32,
        _my: f32,
        _mw: f32,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        s: &Style,
    ) {
        let col_w = (w - GAP) * 0.5;
        let right_x = x + col_w + GAP;
        let subtab = self.subtab.min(2);

        if subtab <= 1 {
            let ov = &mut self.ext.overlays[subtab];
            self.ui.child(dl, x, y, col_w, h - 128.0, true, s, |ui, dl| {
                ui.text(dl, "Player overlay", s.text_dim, 1.0);
                ui.separator(dl, s);
                ui.checkbox(dl, "enable", &mut ov.enabled, s);
                ui.checkbox(dl, "box", &mut ov.box_on, s);
                ui.popup(dl, "##box_popup", POPUP_W, 124.0, s, |ui, dl| {
                    ui.combo(dl, "style##box", &mut ov.box_style, &BOX_STYLES, s);
                    ui.slider_f32(dl, "corner length", &mut ov.corner_len, 0.05, 0.5, 2, "", s);
                    ui.color_picker(dl, "color##box", &mut ov.box_color, s);
                });
                ui.checkbox(dl, "skeleton", &mut ov.skel_on, s);
                ui.popup(dl, "##skeleton_popup", POPUP_W, 114.0, s, |ui, dl| {
                    ui.slider_f32(dl, "thickness##skel", &mut ov.skel_thick, 0.5, 4.0, 1, "", s);
                    ui.color_picker(dl, "visible color##skel", &mut ov.skel_vis, s);
                    ui.color_picker(dl, "occluded color##skel", &mut ov.skel_occ, s);
                });
                ui.checkbox(dl, "health bar", &mut ov.health.enabled, s);
                ui.popup(dl, "##health_popup", POPUP_W, 168.0, s, |ui, dl| {
                    ui.combo(dl, "position##hp", &mut ov.health.position, &BAR_POSITIONS, s);
                    ui.checkbox(dl, "gradient##hp", &mut ov.health.gradient, s);
                    ui.checkbox(dl, "show value##hp", &mut ov.health.show_value, s);
                    ui.checkbox(dl, "glow##hp", &mut ov.health.glow, s);
                    ui.color_picker(dl, "full color##hp", &mut ov.health.full_color, s);
                    ui.color_picker(dl, "low color##hp", &mut ov.health.low_color, s);
                });
                ui.checkbox(dl, "ammo bar", &mut ov.ammo.enabled, s);
                ui.popup(dl, "##ammo_popup", POPUP_W, 168.0, s, |ui, dl| {
                    ui.combo(dl, "position##ammo", &mut ov.ammo.position, &BAR_POSITIONS, s);
                    ui.checkbox(dl, "gradient##ammo", &mut ov.ammo.gradient, s);
                    ui.checkbox(dl, "show value##ammo", &mut ov.ammo.show_value, s);
                    ui.checkbox(dl, "glow##ammo", &mut ov.ammo.glow, s);
                    ui.color_picker(dl, "full color##ammo", &mut ov.ammo.full_color, s);
                    ui.color_picker(dl, "low color##ammo", &mut ov.ammo.low_color, s);
                });
                ui.checkbox(dl, "name", &mut ov.name_on, s);
                ui.popup(dl, "##name_popup", POPUP_W, 44.0, s, |ui, dl| {
                    ui.color_picker(dl, "color##name", &mut ov.name_color, s);
                });
                ui.checkbox(dl, "weapon", &mut ov.weapon_on, s);
                ui.popup(dl, "##weapon_popup", POPUP_W, 114.0, s, |ui, dl| {
                    ui.combo(dl, "display##wep", &mut ov.weapon_display, &DISPLAY_TYPES, s);
                    ui.color_picker(dl, "text color##wep", &mut ov.weapon_text, s);
                    ui.color_picker(dl, "icon color##wep", &mut ov.weapon_icon, s);
                });
                ui.checkbox(dl, "flags", &mut ov.flags_on, s);
                ui.popup(dl, "##flags_popup", POPUP_W, 110.0, s, |ui, dl| {
                    ui.multicombo(dl, "flags##mc", &mut ov.flags, &FLAG_NAMES, s);
                    ui.color_picker(dl, "color##flags", &mut ov.flags_color, s);
                });
                ui.checkbox(dl, "oof arrows", &mut ov.oof_on, s);
                ui.popup(dl, "##oof_popup", POPUP_W, 146.0, s, |ui, dl| {
                    ui.checkbox(dl, "glow##oof", &mut ov.oof_glow, s);
                    ui.slider_f32(dl, "width##oof", &mut ov.oof_w, 4.0, 40.0, 0, "", s);
                    ui.slider_f32(dl, "height##oof", &mut ov.oof_h, 4.0, 40.0, 0, "", s);
                    ui.color_picker(dl, "visible color##oof", &mut ov.oof_vis, s);
                    ui.color_picker(dl, "occluded color##oof", &mut ov.oof_occ, s);
                });
            });

            let glow = &mut self.ext.glow[subtab];
            let rag_glow = &mut self.ext.ragdoll_glow[subtab];
            self.ui.child(dl, x, y + h - 116.0, col_w, 116.0, true, s, |ui, dl| {
                ui.text(dl, "Glow", s.text_dim, 1.0);
                ui.separator(dl, s);
                ui.checkbox(dl, "glow", &mut glow.0, s);
                ui.popup(dl, "##glow_popup", POPUP_W, 44.0, s, |ui, dl| {
                    ui.color_picker(dl, "color##glow", &mut glow.1, s);
                });
                ui.checkbox(dl, "ragdoll glow", &mut rag_glow.0, s);
                ui.popup(dl, "##glow_rag_popup", POPUP_W, 44.0, s, |ui, dl| {
                    ui.color_picker(dl, "color##glow_rag", &mut rag_glow.1, s);
                });
            });

            self.ui.child(dl, right_x, y, col_w, h, true, s, |ui, dl| {
                ui.text(dl, "Models", s.text_dim, 1.0);
                ui.separator(dl, s);
                Self::draw_chams_cfg(ui, dl, "chams", "main", &mut self.ext.chams[subtab], true, s);
                ui.separator(dl, s);
                Self::draw_chams_cfg(
                    ui,
                    dl,
                    "ragdoll chams",
                    "ragdoll",
                    &mut self.ext.ragdoll_chams[subtab],
                    false,
                    s,
                );
            });
        } else {
            // Local player + Viewmodel
            self.ui.child(dl, x, y, col_w, h, true, s, |ui, dl| {
                ui.text(dl, "Local player", s.text_dim, 1.0);
                ui.separator(dl, s);
                Self::draw_chams_cfg(ui, dl, "chams", "local_main", &mut self.ext.chams[2], true, s);
                ui.separator(dl, s);
                ui.checkbox(dl, "lower opacity", &mut self.ext.local_alpha, s);
                ui.popup(dl, "##local_alpha_popup", POPUP_W, 74.0, s, |ui, dl| {
                    ui.slider_f32(dl, "opacity", &mut self.ext.local_alpha_val, 0.0, 1.0, 2, "", s);
                    ui.checkbox(dl, "only when scoped", &mut self.ext.local_alpha_scoped, s);
                });
                ui.separator(dl, s);
                Self::draw_chams_cfg(
                    ui,
                    dl,
                    "ragdoll chams",
                    "local_ragdoll",
                    &mut self.ext.ragdoll_chams[2],
                    false,
                    s,
                );
                ui.separator(dl, s);
                ui.checkbox(dl, "glow##local", &mut self.ext.glow[2].0, s);
                ui.checkbox(dl, "ragdoll glow##local", &mut self.ext.ragdoll_glow[2].0, s);
            });

            self.ui.child(dl, right_x, y, col_w, h, true, s, |ui, dl| {
                ui.text(dl, "Viewmodel", s.text_dim, 1.0);
                ui.separator(dl, s);
                Self::draw_chams_cfg(ui, dl, "weapon chams", "vm_weapon", &mut self.ext.chams[3], true, s);
                ui.separator(dl, s);
                Self::draw_chams_cfg(ui, dl, "arms chams", "vm_arms", &mut self.ext.chams[4], true, s);
            });
        }

        // The old floating preview is dead code: the active side panel is drawn
        // by draw_menu_internal through preview::EspPreviewState.
    }

    fn draw_esp_preview(&mut self, dl: &mut DrawList, mx: f32, my: f32, mw: f32, s: &Style) {
        let (sw, sh) = dl.size();
        let width = 232.0;
        let height = 416.0;
        let x = (mx + mw + 12.0).min(sw - width - 8.0).max(8.0);
        let y = my.min(sh - height - 8.0).max(8.0);

        dl.begin_top();
        dl.rect_rounded(x, y, width, height, ROUNDING, s.dark);
        dl.outline_rounded(x, y, width, height, ROUNDING, 1.0, s.border);
        dl.text(x + 12.0, y + 12.0, "ESP PREVIEW", s.text, 1.0);

        let gbtn = (x + 12.0, y + 34.0, 100.0, 24.0);
        let vbtn = (x + 120.0, y + 34.0, 100.0, 24.0);
        if self.ui.mouse_clicked() && !self.ui.overlay_blocking() {
            if self.ui.inside(gbtn.0, gbtn.1, gbtn.2, gbtn.3) {
                self.ext.preview_group = 1 - self.ext.preview_group;
            }
            if self.ui.inside(vbtn.0, vbtn.1, vbtn.2, vbtn.3) {
                self.ext.preview_visible = !self.ext.preview_visible;
            }
        }
        for b in [gbtn, vbtn] {
            dl.rect_rounded(b.0, b.1, b.2, b.3, WIDGET_ROUNDING, s.elevated);
            dl.outline_rounded(b.0, b.1, b.2, b.3, WIDGET_ROUNDING, 1.0, s.border);
        }
        dl.text(
            gbtn.0 + 10.0,
            gbtn.1 + 8.0,
            if self.ext.preview_group == 1 { "Team" } else { "Enemy" },
            s.text,
            1.0,
        );
        dl.text(
            vbtn.0 + 10.0,
            vbtn.1 + 8.0,
            if self.ext.preview_visible { "Visible" } else { "Occluded" },
            s.text,
            1.0,
        );

        // Agent preview viewport
        let vx = x + 20.0;
        let vy = y + 66.0;
        let vw = width - 40.0;
        let vh = 290.0;
        dl.rect_rounded(vx, vy, vw, vh, 3.0, [15, 16, 19, 255]);
        dl.outline_rounded(vx, vy, vw, vh, 3.0, 1.0, s.border);

        // Live stylized CS2 agent silhouette + ESP overlays
        let cx = vx + vw * 0.5;
        let bx = cx - 44.0;
        let by = vy + 36.0;
        let bw = 88.0;
        let bh = 216.0;

        let sil_col: Color = if self.ext.preview_group == 1 {
            [42, 52, 66, 255]
        } else {
            [52, 46, 42, 255]
        };
        // Head, torso, arms, legs silhouette
        dl.circle_filled(cx, by + 18.0, 12.0, sil_col);
        dl.rect_rounded(cx - 20.0, by + 33.0, 40.0, 74.0, 6.0, sil_col);
        dl.rect_rounded(cx - 32.0, by + 37.0, 11.0, 64.0, 4.0, sil_col);
        dl.rect_rounded(cx + 21.0, by + 37.0, 11.0, 64.0, 4.0, sil_col);
        dl.rect_rounded(cx - 18.0, by + 108.0, 15.0, 96.0, 4.0, sil_col);
        dl.rect_rounded(cx + 3.0, by + 108.0, 15.0, 96.0, 4.0, sil_col);

        let ov = &self.ext.overlays[self.ext.preview_group];
        if ov.enabled {
            if ov.skel_on {
                let scol = if self.ext.preview_visible { ov.skel_vis } else { ov.skel_occ };
                let t = ov.skel_thick.clamp(0.8, 3.0);
                // Spine & head
                dl.line(cx, by + 18.0, cx, by + 108.0, t, scol);
                // Shoulders & arms
                dl.line(cx - 26.0, by + 44.0, cx + 26.0, by + 44.0, t, scol);
                dl.line(cx - 26.0, by + 44.0, cx - 28.0, by + 95.0, t, scol);
                dl.line(cx + 26.0, by + 44.0, cx + 28.0, by + 95.0, t, scol);
                // Hips & legs
                dl.line(cx - 12.0, by + 108.0, cx + 12.0, by + 108.0, t, scol);
                dl.line(cx - 12.0, by + 108.0, cx - 14.0, by + 202.0, t, scol);
                dl.line(cx + 12.0, by + 108.0, cx + 14.0, by + 202.0, t, scol);
            }
            if ov.box_on {
                if ov.box_style == 0 {
                    dl.outline(bx, by, bw, bh, 1.0, ov.box_color);
                } else {
                    let c = ov.corner_len.clamp(6.0, 24.0);
                    // Top-left
                    dl.rect(bx, by, c, 1.2, ov.box_color);
                    dl.rect(bx, by, 1.2, c, ov.box_color);
                    // Top-right
                    dl.rect(bx + bw - c, by, c, 1.2, ov.box_color);
                    dl.rect(bx + bw - 1.2, by, 1.2, c, ov.box_color);
                    // Bottom-left
                    dl.rect(bx, by + bh - 1.2, c, 1.2, ov.box_color);
                    dl.rect(bx, by + bh - c, 1.2, c, ov.box_color);
                    // Bottom-right
                    dl.rect(bx + bw - c, by + bh - 1.2, c, 1.2, ov.box_color);
                    dl.rect(bx + bw - 1.2, by + bh - c, 1.2, c, ov.box_color);
                }
            }
            if ov.health.enabled {
                let hx = bx - 6.0;
                let fill_h = bh * 0.84;
                dl.rect(hx, by, 3.0, bh, [10, 10, 10, 220]);
                dl.rect(hx, by + (bh - fill_h), 3.0, fill_h, ov.health.full_color);
                if ov.health.show_value {
                    dl.text(hx - 16.0, by + (bh - fill_h), "84", ov.health.text_color, 1.0);
                }
            }
            if ov.ammo.enabled {
                let ay = by + bh + 4.0;
                dl.rect(bx, ay, bw, 3.0, [10, 10, 10, 220]);
                dl.rect(bx, ay, bw * 0.72, 3.0, ov.ammo.full_color);
            }
            if ov.name_on {
                let n = if self.ext.preview_group == 1 { "Teammate" } else { "Enemy" };
                let nw = dl.text_width(n, 0.9);
                dl.text(cx - nw * 0.5, by - 16.0, n, ov.name_color, 0.9);
            }
            if ov.weapon_on {
                let wtxt = "AK-47";
                let ww = dl.text_width(wtxt, 0.85);
                dl.text(cx - ww * 0.5, by + bh + 11.0, wtxt, ov.weapon_text, 0.85);
            }
            if ov.flags_on {
                const FTXT: [&str; 8] = ["$16000", "HK", "KIT", "ZOOM", "DEF", "FLSH", "12ms", "10m"];
                let mut fy = by;
                for (i, &lbl) in FTXT.iter().enumerate() {
                    if ov.flags[i] {
                        let col = if i == 0 {
                            [130, 220, 140, 255]
                        } else if i == 1 || i == 2 {
                            s.accent
                        } else {
                            s.text_dim
                        };
                        dl.text(bx + bw + 5.0, fy, lbl, col, 0.82);
                        fy += 14.0;
                    }
                }
            }
        }

        dl.text(x + 12.0, y + height - 20.0, "Native agent / ESP + chams", s.text_dim, 1.0);
        dl.end_top();
    }

    fn draw_world(&mut self, dl: &mut DrawList, x: f32, y: f32, w: f32, h: f32, s: &Style) {
        let col_w = (w - GAP) * 0.5;
        let right_x = x + col_w + GAP;
        match self.subtab {
            0 => {
                self.ui.child(dl, x, y, col_w, h, true, s, |ui, dl| {
                    ui.text(dl, "Items", s.text_dim, 1.0);
                    ui.separator(dl, s);
                    ui.combo(dl, "group##item_sel", &mut self.ext.item_group, &ITEM_GROUPS, s);
                    let ig = self.ext.item_group.min(5);
                    ui.separator(dl, s);
                    ui.checkbox(dl, "item esp", &mut self.ext.item_esp[ig], s);
                    ui.popup(dl, "##ie_grp_cfg", POPUP_W, 96.0, s, |ui, dl| {
                        ui.combo(dl, "display##ie", &mut self.ext.item_display, &DISPLAY_TYPES, s);
                        ui.slider_f32(dl, "max dist##ie", &mut self.ext.item_dist, 1.0, 200.0, 0, "m", s);
                    });
                    ui.checkbox(dl, "item chams", &mut self.ext.item_chams[ig], s);
                    ui.checkbox(dl, "item glow", &mut self.ext.item_glow[ig], s);
                });

                self.ui.child(dl, right_x, y, col_w, h - 128.0, true, s, |ui, dl| {
                    ui.text(dl, "Projectiles", s.text_dim, 1.0);
                    ui.separator(dl, s);
                    ui.combo(dl, "group##proj_sel", &mut self.ext.proj_group, &PROJ_GROUPS, s);
                    let pg = self.ext.proj_group.min(5);
                    ui.separator(dl, s);
                    let is_inf = pg == 5;
                    ui.checkbox(
                        dl,
                        if is_inf { "inferno esp" } else { "projectile esp" },
                        &mut self.ext.proj_esp[pg],
                        s,
                    );
                    let ind = match pg {
                        0 => Some(0),
                        3 => Some(1),
                        5 => Some(2),
                        _ => None,
                    };
                    if let Some(ii) = ind {
                        ui.checkbox(
                            dl,
                            if is_inf { "indicator" } else { "landing indicator" },
                            &mut self.ext.proj_ind[ii],
                            s,
                        );
                    }
                });

                self.ui.child(dl, right_x, y + h - 116.0, col_w, 116.0, true, s, |ui, dl| {
                    ui.text(dl, "Other", s.text_dim, 1.0);
                    ui.separator(dl, s);
                    ui.checkbox(dl, "bomb timer", &mut self.ext.bomb_timer, s);
                    ui.checkbox(dl, "spectator list", &mut self.ext.spectator_list, s);
                });
            }
            1 => {
                self.ui.child(dl, x, y, col_w, h, true, s, |ui, dl| {
                    ui.text(dl, "Scene", s.text_dim, 1.0);
                    ui.separator(dl, s);
                    ui.checkbox(dl, "skybox material", &mut self.ext.skybox_mat, s);
                    ui.popup(dl, "##skybox_popup", POPUP_W, 88.0, s, |ui, dl| {
                        ui.combo(dl, "skybox", &mut self.ext.skybox_idx, &SKYBOX_NAMES, s);
                        ui.button(dl, "refresh skyboxes", 180.0, 24.0, s);
                    });
                    ui.checkbox(dl, "skybox color", &mut self.ext.skybox_col, s);
                    ui.popup(dl, "##skycolor_popup", POPUP_W, 88.0, s, |ui, dl| {
                        ui.color_picker(dl, "sky color", &mut self.ext.sky_color, s);
                        ui.color_picker(dl, "cloud color", &mut self.ext.cloud_color, s);
                        ui.color_picker(dl, "sun color", &mut self.ext.sun_color, s);
                    });
                    ui.checkbox(dl, "world material", &mut self.ext.world_mat, s);
                    ui.popup(dl, "##world_material_popup", POPUP_W, 88.0, s, |ui, dl| {
                        ui.combo(dl, "surface", &mut self.ext.world_mat_type, &SURFACE_PRESETS, s);
                        ui.color_picker(dl, "tint##world_material", &mut self.ext.world_mat_col, s);
                    });
                    ui.checkbox(dl, "lighting", &mut self.ext.lighting, s);
                    ui.popup(dl, "##lighting_popup", POPUP_W, 84.0, s, |ui, dl| {
                        ui.slider_f32(dl, "intensity##light", &mut self.ext.lighting_int, 0.0, 2.0, 2, "", s);
                        ui.color_picker(dl, "color##light", &mut self.ext.lighting_col, s);
                    });
                    ui.checkbox(dl, "bloom", &mut self.ext.bloom, s);
                    ui.popup(dl, "##bloom_popup", POPUP_W, 56.0, s, |ui, dl| {
                        ui.slider_f32(dl, "value##bloom", &mut self.ext.bloom_val, 0.0, 2.0, 2, "", s);
                    });
                    ui.checkbox(dl, "gamma", &mut self.ext.gamma, s);
                    ui.popup(dl, "##gamma_popup", POPUP_W, 56.0, s, |ui, dl| {
                        ui.slider_f32(dl, "value##gamma", &mut self.ext.gamma_val, 0.5, 5.0, 1, "", s);
                    });
                });

                self.ui.child(dl, right_x, y, col_w, h, true, s, |ui, dl| {
                    ui.text(dl, "Lighting & effects", s.text_dim, 1.0);
                    ui.separator(dl, s);
                    ui.checkbox(dl, "depth of field", &mut self.ext.dof, s);
                    ui.popup(dl, "##dof_popup", POPUP_W, 148.0, s, |ui, dl| {
                        ui.slider_f32(dl, "near blurry", &mut self.ext.dof_vals[0], 0.0, 50.0, 0, "", s);
                        ui.slider_f32(dl, "near crisp", &mut self.ext.dof_vals[1], 0.0, 100.0, 0, "", s);
                        ui.slider_f32(dl, "far crisp", &mut self.ext.dof_vals[2], 100.0, 2000.0, 0, "", s);
                        ui.slider_f32(dl, "far blurry", &mut self.ext.dof_vals[3], 200.0, 5000.0, 0, "", s);
                    });
                });
            }
            _ => {
                self.ui.child(dl, x, y, col_w, h, true, s, |ui, dl| {
                    ui.text(dl, "Weather", s.text_dim, 1.0);
                    ui.separator(dl, s);
                    ui.checkbox(dl, "weather", &mut self.ext.weather_on, s);
                    ui.popup(dl, "##weather_popup", POPUP_W, 88.0, s, |ui, dl| {
                        ui.combo(dl, "type", &mut self.ext.weather_type, &WEATHER_TYPES, s);
                        ui.color_picker(dl, "color##weather", &mut self.ext.weather_col, s);
                    });
                    ui.checkbox(dl, "fog", &mut self.ext.fog_on, s);
                    ui.popup(dl, "##fog_popup", POPUP_W, 148.0, s, |ui, dl| {
                        ui.slider_f32(dl, "density##fog", &mut self.ext.fog_density, 0.0, 1.0, 2, "", s);
                        ui.slider_f32(dl, "anisotropy", &mut self.ext.fog_aniso, 0.0, 1.0, 2, "", s);
                        ui.slider_f32(dl, "draw distance", &mut self.ext.fog_dist, 500.0, 20000.0, 0, "", s);
                        ui.color_picker(dl, "color##fog", &mut self.ext.fog_col, s);
                    });
                    ui.checkbox(dl, "wetness", &mut self.ext.wetness, s);
                    ui.popup(dl, "##wetness_popup", POPUP_W, 88.0, s, |ui, dl| {
                        ui.slider_f32(dl, "density##wet", &mut self.ext.wetness_density, 0.0, 5.0, 1, "", s);
                        ui.slider_f32(dl, "speed##wet", &mut self.ext.wetness_speed, 0.0, 3.0, 1, "", s);
                    });
                    ui.checkbox(dl, "wind", &mut self.ext.wind, s);
                    ui.popup(dl, "##wind_popup", POPUP_W, 120.0, s, |ui, dl| {
                        ui.slider_f32(dl, "strength##wind", &mut self.ext.wind_str, 0.0, 5.0, 1, "", s);
                        ui.slider_f32(dl, "direction##wind", &mut self.ext.wind_dir, 0.0, 360.0, 0, "", s);
                        ui.slider_f32(dl, "turbulence##wind", &mut self.ext.wind_turb, 0.0, 5.0, 1, "", s);
                    });
                });
            }
        }
    }

    fn draw_weapon_silhouette(dl: &mut DrawList, cx: f32, cy: f32, w: f32, h: f32, cat: usize, col: Color) {
        match cat {
            1 => {
                // Knife silhouette
                dl.convex_poly(
                    &[
                        (cx - w * 0.34, cy + h * 0.14),
                        (cx + w * 0.06, cy - h * 0.18),
                        (cx + w * 0.36, cy - h * 0.12),
                        (cx + w * 0.12, cy + h * 0.08),
                    ],
                    col,
                );
                dl.rect_rounded(cx - w * 0.38, cy + h * 0.06, w * 0.22, h * 0.16, 2.0, [48, 50, 56, 255]);
            }
            2 => {
                // Gloves pair silhouette
                dl.rect_rounded(cx - w * 0.30, cy - h * 0.22, w * 0.24, h * 0.44, 5.0, col);
                dl.rect_rounded(cx + w * 0.06, cy - h * 0.22, w * 0.24, h * 0.44, 5.0, col);
            }
            3 => {
                // Agent head + shoulders
                dl.circle_filled(cx, cy - h * 0.16, h * 0.16, col);
                dl.rect_rounded(cx - w * 0.22, cy + h * 0.04, w * 0.44, h * 0.28, 6.0, col);
            }
            _ => {
                // Rifle/Pistol silhouette
                dl.rect_rounded(cx - w * 0.36, cy - h * 0.12, w * 0.62, h * 0.18, 2.0, col);
                dl.rect(cx + w * 0.24, cy - h * 0.08, w * 0.16, h * 0.08, col);
                dl.rect_rounded(cx - w * 0.04, cy + h * 0.04, w * 0.11, h * 0.22, 2.0, col);
                dl.rect_rounded(cx - w * 0.26, cy + h * 0.04, w * 0.10, h * 0.18, 2.0, col);
            }
        }
    }

    fn draw_skins(&mut self, dl: &mut DrawList, x: f32, y: f32, w: f32, h: f32, s: &Style) {
        let subtab = self.subtab.min(3);
        let cols = 5usize;
        let card_gap = 8.0;

        if let Some(browsing_def) = self.ext.skins_browsing {
            self.ui.child(dl, x, y, w, h, true, s, |ui, dl| {
                let inner_w = ui.avail_w();
                let start = ui.cursor();
                if ui.button(dl, "back", 60.0, 24.0, s) {
                    self.ext.skins_browsing = None;
                    self.ext.skins_search.clear();
                    return;
                }
                ui.same_line();
                if ui.button(
                    dl,
                    if self.ext.inspect_editor { "Close 3D editor" } else { "3D weapon editor" },
                    148.0,
                    24.0,
                    s,
                ) {
                    self.ext.inspect_editor = !self.ext.inspect_editor;
                }
                ui.same_line();
                if ui.button(dl, "Inspect stickers", 136.0, 24.0, s) {
                    self.ext.sticker_editor = !self.ext.sticker_editor;
                }
                ui.text_input(dl, "##skin_search", &mut self.ext.skins_search, 64, "search...", s);

                if self.ext.sticker_editor {
                    let slots = ["1", "2", "3", "4", "5"];
                    ui.combo(dl, "slot##inspect", &mut self.ext.sticker_slot, &slots, s);
                    ui.slider_i32(dl, "sticker ID", &mut self.ext.sticker_id, 0, 20000, "", s);
                }

                let grid_y = ui.cursor().y + 4.0;
                let total_gap = (cols - 1) as f32 * card_gap;
                let card_w = ((inner_w - total_gap) / cols as f32).floor();
                let card_h = (card_w * (100.0 / 118.0)).floor();
                let query = self.ext.skins_search.to_lowercase();
                let current_kit = self.ext.applied_skins.get(&browsing_def).map(|v| v.0);
                let mut shown_idx = 0usize;

                for (kit_idx, &(kit_name, rarity)) in PAINT_KITS.iter().enumerate() {
                    if !query.is_empty() && !kit_name.to_lowercase().contains(&query) {
                        continue;
                    }
                    let col = shown_idx % cols;
                    let row = shown_idx / cols;
                    shown_idx += 1;
                    let cx = start.x + col as f32 * (card_w + card_gap);
                    let cy = grid_y + row as f32 * (card_h + card_gap);
                    if cy + card_h > y + h - 8.0 {
                        break;
                    }
                    let hovered = !ui.overlay_blocking() && ui.inside(cx, cy, card_w, card_h);
                    let equipped = current_kit == Some(kit_idx);
                    let rcol = RARITY_COLORS[rarity.min(7)];
                    dl.rect_rounded(cx, cy, card_w, card_h, WIDGET_ROUNDING, if hovered { s.hovered } else { s.card });
                    if equipped {
                        dl.outline_rounded(cx, cy, card_w, card_h, WIDGET_ROUNDING, 1.5, s.text_dim);
                    }
                    let img_h = (card_h * 0.70).floor();
                    Self::draw_weapon_silhouette(dl, cx + card_w * 0.5, cy + img_h * 0.5, card_w - 16.0, img_h - 14.0, subtab, rcol);
                    dl.rect(cx + 6.0, cy + img_h, card_w - 12.0, 2.0, rcol);
                    let trunc = dl.truncate(kit_name, card_w - 12.0, 1.0);
                    let nw = dl.text_width(&trunc, 1.0);
                    dl.text((cx + (card_w - nw) * 0.5).floor(), cy + img_h + 7.0, &trunc, if hovered { s.text } else { s.text_dim }, 1.0);
                    if equipped {
                        let b = 14.0;
                        dl.circle_filled(cx + card_w - b * 0.5 - 4.0, cy + b * 0.5 + 4.0, b * 0.5, s.text_dim);
                    }
                    if hovered && ui.mouse_clicked() {
                        self.ext.applied_skins.insert(browsing_def, (kit_idx, 0.01, 0, false));
                        self.ext.skins_browsing = None;
                    }
                }
            });
            return;
        }

        // Main 5-column skins grid (menu.skins.cpp:960-1012)
        self.ui.child(dl, x, y, w, h, true, s, |ui, dl| {
            let inner_w = ui.avail_w();
            let base = ui.cursor();
            let total_gap = (cols - 1) as f32 * card_gap;
            let card_w = ((inner_w - total_gap) / cols as f32).floor();
            let card_h = (card_w * (100.0 / 118.0)).floor();
            let items: Vec<&SkinItem> = CATALOG_ITEMS.iter().filter(|it| it.category == subtab).collect();

            for (i, item) in items.iter().enumerate() {
                let col = i % cols;
                let row = i / cols;
                let cx = (base.x + col as f32 * (card_w + card_gap)).floor();
                let cy = (base.y + row as f32 * (card_h + card_gap)).floor();
                if cy + card_h > y + h - 8.0 {
                    break;
                }
                let hovered = !ui.overlay_blocking() && ui.inside(cx, cy, card_w, card_h);
                let applied = self.ext.applied_skins.get(&item.def_index).copied();
                let skin_info = applied.and_then(|(k, _, _, _)| PAINT_KITS.get(k).copied());
                let rcol = skin_info.map(|(_, r)| RARITY_COLORS[r.min(7)]);

                dl.rect_rounded(cx, cy, card_w, card_h, WIDGET_ROUNDING, if hovered { s.hovered } else { s.card });
                if let Some(rc) = rcol {
                    dl.outline_rounded(cx, cy, card_w, card_h, WIDGET_ROUNDING, 1.0, [rc[0], rc[1], rc[2], 140]);
                } else {
                    dl.outline_rounded(cx, cy, card_w, card_h, WIDGET_ROUNDING, 1.0, s.border);
                }

                let img_h = (card_h * 0.70).floor();
                let sil_col = rcol.unwrap_or([90, 96, 108, 255]);
                Self::draw_weapon_silhouette(
                    dl,
                    cx + card_w * 0.5,
                    cy + img_h * 0.5,
                    card_w - 16.0,
                    img_h - 14.0,
                    item.category,
                    sil_col,
                );

                if let Some(rc) = rcol {
                    dl.rect(cx + 6.0, cy + img_h, card_w - 12.0, 2.0, rc);
                }

                let label = skin_info.map(|(n, _)| n).unwrap_or(item.name);
                let trunc = dl.truncate(label, card_w - 12.0, 1.0);
                let nw = dl.text_width(&trunc, 1.0);
                let lcol = rcol.unwrap_or(if hovered { s.text } else { s.text_dim });
                dl.text((cx + (card_w - nw) * 0.5).floor(), cy + img_h + 7.0, &trunc, lcol, 1.0);

                if hovered && ui.mouse_clicked() {
                    self.ext.skins_browsing = Some(item.def_index);
                }
            }
        });
    }

    fn draw_misc(&mut self, dl: &mut DrawList, state: &GameState, x: f32, y: f32, w: f32, h: f32, s: &Style) {
        let col_w = (w - GAP) * 0.5;
        let right_x = x + col_w + GAP;
        match self.subtab {
            0 => {
                let half_h = (h - GAP) * 0.5;
                self.ui.child(dl, x, y, col_w, half_h, true, s, |ui, dl| {
                    ui.text(dl, "Hit feedback", s.text_dim, 1.0);
                    ui.separator(dl, s);
                    ui.checkbox(dl, "hit logs", &mut self.ext.hit_logs, s);
                    ui.checkbox(dl, "miss logs", &mut self.ext.miss_logs, s);
                    ui.checkbox(dl, "hit marker", &mut self.ext.hit_marker, s);
                    ui.popup(dl, "##hitmarker_popup", POPUP_W, 56.0, s, |ui, dl| {
                        ui.combo(dl, "type##hm", &mut self.ext.hit_marker_type, &MARKER_TYPES, s);
                    });
                    ui.checkbox(dl, "hit sound", &mut self.ext.hit_sound, s);
                    ui.popup(dl, "##hitsound_popup", POPUP_W, 88.0, s, |ui, dl| {
                        ui.combo(dl, "type##hs", &mut self.ext.hit_sound_type, &SOUND_TYPES, s);
                        ui.slider_f32(dl, "volume##hs", &mut self.ext.hit_sound_vol, 1.0, 100.0, 0, "%", s);
                    });
                    ui.checkbox(dl, "death sound", &mut self.ext.death_sound, s);
                    ui.popup(dl, "##deathsound_popup", POPUP_W, 88.0, s, |ui, dl| {
                        ui.combo(dl, "type##ds", &mut self.ext.death_sound_type, &SOUND_TYPES, s);
                        ui.slider_f32(dl, "volume##ds", &mut self.ext.death_sound_vol, 1.0, 100.0, 0, "%", s);
                    });
                    ui.checkbox(dl, "hit effect", &mut self.ext.hit_effect, s);
                    ui.checkbox(dl, "death effect", &mut self.ext.death_effect, s);
                    ui.popup(dl, "##deathfx_popup", POPUP_W, 56.0, s, |ui, dl| {
                        ui.combo(dl, "effect##deathfx", &mut self.ext.death_effect_style, &DEATH_EFFECTS, s);
                    });
                    ui.checkbox(dl, "bullet impacts", &mut self.ext.bullet_impacts, s);
                    ui.popup(dl, "##bulletfx_popup", POPUP_W, 56.0, s, |ui, dl| {
                        ui.combo(dl, "type##bulletfx", &mut self.ext.bullet_impact_type, &IMPACT_TYPES, s);
                    });
                    ui.checkbox(dl, "bullet tracers", &mut self.ext.bullet_tracers, s);
                });

                self.ui.child(dl, x, y + half_h + GAP, col_w, half_h, true, s, |ui, dl| {
                    ui.text(dl, "Aim assistance", s.text_dim, 1.0);
                    ui.separator(dl, s);
                    let mut settings = state.settings_mut();
                    let pen = &mut settings.combat.penetration_crosshair;
                    ui.checkbox(dl, "penetration crosshair", &mut pen.enabled, s);
                    ui.popup(dl, "##pen_popup", POPUP_W, 134.0, s, |ui, dl| {
                        ui.checkbox(dl, "glow##pen", &mut pen.glow, s);
                        ui.slider_f32(dl, "glow strength##pen", &mut pen.glow_strength, 0.1, 1.0, 2, "", s);
                        ui.color_picker(dl, "can penetrate##pen", &mut pen.can_penetrate_fill, s);
                        ui.color_picker(dl, "blocked##pen", &mut pen.blocked_fill, s);
                    });
                    ui.checkbox(dl, "projectile trajectory", &mut self.ext.proj_traj, s);
                    ui.checkbox(dl, "dynamic light", &mut self.ext.dlight, s);
                });

                self.ui.child(dl, right_x, y, col_w, half_h, true, s, |ui, dl| {
                    ui.text(dl, "Movement", s.text_dim, 1.0);
                    ui.separator(dl, s);
                    let mut settings = state.settings_mut();
                    ui.checkbox(dl, "bhop", &mut settings.misc.bhop, s);
                    ui.checkbox(dl, "autostrafe", &mut settings.misc.autostrafe, s);
                    ui.checkbox(dl, "jumpbug", &mut self.ext.jumpbug, s);
                    ui.checkbox(dl, "mini jump", &mut self.ext.mini_jump, s);
                    ui.checkbox(dl, "fastladder", &mut self.ext.fastladder, s);
                    ui.checkbox(dl, "edgejump", &mut self.ext.edgejump, s);
                    ui.checkbox(dl, "edgestop", &mut self.ext.edgestop, s);
                    ui.checkbox(dl, "edgebug", &mut self.ext.edgebug, s);
                    ui.popup(dl, "##edgebug_popup", 240.0, 92.0, s, |ui, dl| {
                        ui.combo(dl, "mode##eb", &mut self.ext.edgebug_mode, &EDGEBUG_MODES, s);
                        ui.slider_i32(dl, "passes##eb", &mut self.ext.edgebug_passes, 1, 5, "", s);
                    });
                    ui.checkbox(dl, "slowwalk", &mut self.ext.slowwalk, s);
                    ui.popup(dl, "##slowwalk_popup", POPUP_W, 56.0, s, |ui, dl| {
                        ui.slider_f32(dl, "speed", &mut self.ext.slowwalk_speed, 1.0, 100.0, 1, "%", s);
                    });
                });

                self.ui.child(dl, right_x, y + half_h + GAP, col_w, half_h, true, s, |ui, dl| {
                    ui.text(dl, "Utility", s.text_dim, 1.0);
                    ui.separator(dl, s);
                    ui.checkbox(dl, "auto buy", &mut self.ext.autobuy, s);
                    ui.popup(dl, "##autobuy_popup", POPUP_W, 196.0, s, |ui, dl| {
                        ui.combo(dl, "primary##ab", &mut self.ext.ab_primary, &PRIMARY_WEAPONS, s);
                        ui.combo(dl, "secondary##ab", &mut self.ext.ab_secondary, &SECONDARY_WEAPONS, s);
                        ui.checkbox(dl, "armor##ab", &mut self.ext.ab_armor, s);
                        ui.checkbox(dl, "defuser##ab", &mut self.ext.ab_defuser, s);
                        ui.checkbox(dl, "taser##ab", &mut self.ext.ab_taser, s);
                        ui.multicombo(dl, "grenades##ab", &mut self.ext.ab_nades, &GRENADE_NAMES, s);
                    });
                    ui.checkbox(dl, "clantag", &mut self.ext.clantag, s);
                    ui.popup(dl, "##clantag_options", 240.0, 146.0, s, |ui, dl| {
                        ui.text_input(dl, "phrase##clantag", &mut self.ext.clantag_phrase, 32, "WALLBANG", s);
                        ui.combo(dl, "animation##clantag", &mut self.ext.clantag_style, &CLANTAG_STYLES, s);
                        ui.slider_i32(dl, "step ms##clantag", &mut self.ext.clantag_ms, 250, 2000, " ms", s);
                    });
                    ui.checkbox(dl, "override name", &mut self.ext.override_name, s);
                    ui.popup(dl, "##override_name_popup", POPUP_W, 64.0, s, |ui, dl| {
                        ui.text_input(dl, "name##nc", &mut self.ext.custom_name, 32, "player name...", s);
                    });
                    let mut wm = state.settings().misc.watermark;
                    if ui.checkbox(dl, "watermark", &mut wm, s) {
                        state.settings_mut().misc.watermark = wm;
                    }
                    ui.popup(dl, "##watermark_popup", 200.0, 178.0, s, |ui, dl| {
                        ui.combo(dl, "position##wm", &mut self.ext.wm_anchor, &WATERMARK_ANCHORS, s);
                        const LABELS: [&str; 8] = [
                            "separators##wm", "fps##wm", "ping##wm", "time##wm",
                            "user##wm", "map##wm", "tick rate##wm", "velocity##wm",
                        ];
                        for (i, &l) in LABELS.iter().enumerate() {
                            ui.checkbox(dl, l, &mut self.ext.wm_flags[i], s);
                        }
                    });
                    ui.checkbox(dl, "scoreboard weapons", &mut self.ext.scoreboard_weapons, s);
                    ui.checkbox(dl, "reveal radar", &mut self.ext.reveal_radar, s);
                    ui.checkbox(dl, "preserve killfeed", &mut self.ext.preserve_killfeed, s);
                    ui.checkbox(dl, "disable game logs", &mut self.ext.disable_game_logs, s);
                });
            }
            1 => {
                self.ui.child(dl, x, y, col_w, h, true, s, |ui, dl| {
                    ui.text(dl, "Removals", s.text_dim, 1.0);
                    ui.separator(dl, s);
                    const REM: [&str; 9] = [
                        "remove crosshair", "remove scope", "remove overhead",
                        "remove legs", "remove recoil", "remove skybox fog",
                        "remove 3d skybox", "remove decals", "remove smoke",
                    ];
                    for (i, &lbl) in REM.iter().enumerate() {
                        ui.checkbox(dl, lbl, &mut self.ext.removals[i], s);
                    }
                    ui.slider_f32(dl, "flash alpha##flash", &mut self.ext.flash_alpha, 0.0, 100.0, 0, "%", s);
                });
            }
            2 => {
                self.ui.child(dl, x, y, col_w, h, true, s, |ui, dl| {
                    ui.text(dl, "Camera", s.text_dim, 1.0);
                    ui.separator(dl, s);
                    ui.checkbox(dl, "custom fov", &mut self.ext.custom_fov, s);
                    ui.popup(dl, "##fov_popup", POPUP_W, 120.0, s, |ui, dl| {
                        ui.slider_f32(dl, "fov", &mut self.ext.fov_val, 60.0, 150.0, 0, "", s);
                        ui.checkbox(dl, "scoped fov override", &mut self.ext.scoped_fov_ov, s);
                        ui.slider_f32(dl, "scoped fov", &mut self.ext.scoped_fov, 10.0, 90.0, 0, "", s);
                    });
                    ui.checkbox(dl, "thirdperson", &mut self.ext.thirdperson, s);
                    ui.popup(dl, "##tp_popup", POPUP_W, 88.0, s, |ui, dl| {
                        ui.slider_f32(dl, "distance", &mut self.ext.tp_dist, 35.0, 200.0, 0, "", s);
                        ui.slider_f32(dl, "hull size", &mut self.ext.tp_hull, 0.0, 20.0, 0, "", s);
                    });
                    ui.checkbox(dl, "aspect ratio", &mut self.ext.aspect_on, s);
                    ui.popup(dl, "##ar_popup", POPUP_W, 56.0, s, |ui, dl| {
                        ui.slider_f32(dl, "ratio##ar", &mut self.ext.aspect_val, 1.0, 1.78, 3, "", s);
                    });
                    let mut blur = state.settings().misc.motion_blur.clone();
                    ui.checkbox(dl, "motion blur", &mut blur.enabled, s);
                    ui.popup(dl, "##motion_blur_popup", POPUP_W, 110.0, s, |ui, dl| {
                        ui.slider_f32(dl, "strength##mb", &mut blur.strength, 0.0, 1.0, 2, "", s);
                        ui.combo(dl, "mode##mb", &mut blur.mode, &MOTION_BLUR_MODES, s);
                        ui.checkbox(dl, "while moving##mb", &mut blur.on_velocity, s);
                    });
                    state.settings_mut().misc.motion_blur = blur;
                });

                self.ui.child(dl, right_x, y, col_w, h, true, s, |ui, dl| {
                    ui.text(dl, "Viewmodel", s.text_dim, 1.0);
                    ui.separator(dl, s);
                    ui.checkbox(dl, "viewmodel adjust", &mut self.ext.vm_adjust, s);
                    ui.popup(dl, "##vm_popup", POPUP_W, 148.0, s, |ui, dl| {
                        ui.slider_f32(dl, "offset x", &mut self.ext.vm_xyzf[0], -10.0, 10.0, 1, "", s);
                        ui.slider_f32(dl, "offset y", &mut self.ext.vm_xyzf[1], -10.0, 10.0, 1, "", s);
                        ui.slider_f32(dl, "offset z", &mut self.ext.vm_xyzf[2], -10.0, 10.0, 1, "", s);
                        ui.slider_f32(dl, "fov", &mut self.ext.vm_xyzf[3], 54.0, 90.0, 0, "", s);
                    });
                });
            }
            _ => {
                self.ui.child(dl, x, y, col_w, h, true, s, |ui, dl| {
                    ui.text(dl, "Interface & HUD", s.text_dim, 1.0);
                    ui.separator(dl, s);
                    let mut accent = state.settings().menu.accent;
                    if ui.color_picker(dl, "menu accent", &mut accent, s) {
                        state.settings_mut().menu.accent = accent;
                    }
                    ui.checkbox(dl, "active hotkeys", &mut self.ext.hotkeys_hud, s);
                    ui.checkbox(dl, "status indicators", &mut self.ext.indicators_hud, s);
                    ui.checkbox(dl, "edit hud positions", &mut self.ext.edit_hud, s);
                    if ui.button(dl, "reset hud positions", 180.0, 26.0, s) {
                        self.ext.hud_pos = [
                            [1.0, 0.0],
                            [0.0, 0.45],
                            [0.5, 0.58],
                            [1.0, 0.45],
                        ];
                    }
                    ui.separator(dl, s);
                    let mut allow = state.settings().misc.allow_writes;
                    if ui.checkbox(dl, "allow writes", &mut allow, s) {
                        state.settings_mut().misc.allow_writes = allow;
                    }
                });

                self.ui.child(dl, right_x, y, col_w, h, true, s, |ui, dl| {
                    ui.text(dl, "Overlays", s.text_dim, 1.0);
                    ui.separator(dl, s);
                    ui.checkbox(dl, "crosshair overlay", &mut self.ext.xhair_ov, s);
                    ui.checkbox(dl, "scope overlay", &mut self.ext.scope_ov, s);
                    ui.checkbox(dl, "velocity counter", &mut self.ext.vel_counter, s);
                    ui.checkbox(dl, "velocity chart", &mut self.ext.vel_chart, s);
                });
            }
        }
    }

    fn draw_config(&mut self, dl: &mut DrawList, state: &GameState, x: f32, y: f32, w: f32, h: f32, s: &Style) {
        if self.need_refresh {
            let _ = self.storage.ensure_dir();
            self.config_list = self.storage.list().unwrap_or_default();
            self.need_refresh = false;
            if let Some(sel) = self.selected {
                if sel >= self.config_list.len() {
                    self.selected = None;
                }
            }
        }

        self.ui.child(dl, x, y, w, h, false, s, |ui, dl| {
            let avail_w = ui.avail_w();
            let top = ui.cursor();
            dl.text(top.x + 4.0, top.y + 4.0, "CONFIG LIBRARY", s.text, 1.0);
            dl.text(top.x + 4.0, top.y + 20.0, "Select a profile, then Load to apply", s.text_dim, 1.0);
            ui.advance(36.0);

            ui.text_input(dl, "new profile", &mut self.create_buf, 64, "profile name...", s);
            if ui.button(dl, "Create profile", 126.0, 26.0, s) && !self.create_buf.trim().is_empty() {
                let name = self.create_buf.trim().to_string();
                let mut snapshot = state.settings().clone();
                snapshot.active_profile = name.clone();
                if self.storage.save(&name, &snapshot).is_ok() {
                    *state.settings_mut() = snapshot;
                    self.status = format!("Created {name}.wb");
                    self.create_buf.clear();
                    self.need_refresh = true;
                } else {
                    self.status = "Could not create profile".to_string();
                }
            }
            ui.text_input(dl, "##cfg_search", &mut self.search_buf, 64, "search configs...", s);

            let btn_h = 28.0;
            let list_y = ui.cursor().y;
            let list_h = ((y + h) - list_y - btn_h - 42.0).max(80.0);
            let search = self.search_buf.to_lowercase();

            ui.child(dl, top.x, list_y, avail_w, list_h, true, s, |ui, dl| {
                let rw = ui.avail_w();
                let row_h = 40.0;
                let mut visible = 0usize;
                for (i, name) in self.config_list.iter().enumerate() {
                    if !search.is_empty() && !name.to_lowercase().contains(&search) {
                        continue;
                    }
                    let pos = ui.cursor();
                    if pos.y + row_h > list_y + list_h {
                        break;
                    }
                    let is_sel = self.selected == Some(i);
                    let is_hov = !ui.overlay_blocking() && ui.inside(pos.x, pos.y, rw, row_h);
                    if is_hov && ui.mouse_clicked() {
                        self.selected = Some(i);
                        self.confirm_delete = false;
                        self.confirm_reset = false;
                    }
                    if is_sel {
                        dl.rect_rounded(pos.x + 2.0, pos.y + 2.0, rw - 4.0, row_h - 4.0, WIDGET_ROUNDING, [18, 18, 18, 255]);
                        dl.rect(pos.x + 3.0, pos.y + 7.0, 2.0, row_h - 14.0, s.accent);
                    }
                    let tcol = if is_sel || is_hov { s.text } else { s.text_dim };
                    dl.text(pos.x + 14.0, pos.y + 7.0, &dl.truncate(name, rw - 28.0, 1.0), tcol, 1.0);
                    dl.text(
                        pos.x + 14.0,
                        pos.y + 23.0,
                        if is_sel { "selected profile" } else { "local profile" },
                        s.text_dim,
                        1.0,
                    );
                    dl.line(pos.x + 12.0, pos.y + row_h - 1.0, pos.x + rw - 12.0, pos.y + row_h - 1.0, 1.0, s.border);
                    ui.advance(row_h);
                    visible += 1;
                }
                if visible == 0 {
                    let pos = ui.cursor();
                    dl.text(pos.x + 10.0, pos.y + 8.0, "no configs found", s.text_dim, 1.0);
                }
            });

            ui.set_cursor(top.x, list_y + list_h + 8.0);
            let sel_name = self
                .selected
                .and_then(|i| self.config_list.get(i))
                .map(|s| s.as_str())
                .unwrap_or("No profile selected");
            let summary = format!("{}  |  {}", self.status, sel_name);
            ui.text(dl, &dl.truncate(&summary, avail_w - 12.0, 1.0), s.text_dim, 1.0);

            let btn_w = (avail_w - ITEM_SPACING_X * 3.0) / 4.0;
            if ui.button(dl, "Load", btn_w, btn_h, s) {
                if let Some(i) = self.selected {
                    if let Some(name) = self.config_list.get(i) {
                        match self.storage.load(name) {
                            Ok(loaded) => {
                                *state.settings_mut() = loaded;
                                wallbang_config::hotkeys::finalize_binds(&mut state.settings_mut());
                                self.status = format!("Loaded {name}.wb");
                            }
                            Err(_) => self.status = "Could not load profile".to_string(),
                        }
                    }
                }
            }
            ui.same_line();
            if ui.button(dl, "save", btn_w, btn_h, s) {
                if let Some(i) = self.selected {
                    if let Some(name) = self.config_list.get(i).cloned() {
                        let mut snapshot = state.settings().clone();
                        snapshot.active_profile = name.clone();
                        self.status = if self.storage.save(&name, &snapshot).is_ok() {
                            *state.settings_mut() = snapshot;
                            format!("Saved {name}.wb")
                        } else {
                            "Could not save profile".to_string()
                        };
                    }
                }
            }
            ui.same_line();
            if self.confirm_reset {
                if ui.button(dl, "confirm##reset", btn_w, btn_h, s) {
                    let mut defaults = wallbang_config::Settings::default();
                    defaults.active_profile = DEFAULT_PROFILE.to_owned();
                    *state.settings_mut() = defaults;
                    self.status = format!("Defaults restored ({DEFAULT_PROFILE}.wb)");
                    self.confirm_reset = false;
                }
            } else if ui.button(dl, "reset", btn_w, btn_h, s) {
                self.confirm_reset = true;
                self.confirm_delete = false;
            }
            ui.same_line();
            if self.confirm_delete {
                if ui.button(dl, "confirm##del", btn_w, btn_h, s) {
                    if let Some(i) = self.selected {
                        if let Some(name) = self.config_list.get(i).cloned() {
                            if self.storage.remove(&name).is_ok() {
                                if state.settings().active_profile == name {
                                    state.settings_mut().active_profile = DEFAULT_PROFILE.to_owned();
                                }
                                self.selected = None;
                                self.need_refresh = true;
                                self.status = format!("Deleted {name}.wb");
                            } else {
                                self.status = "Could not delete profile".to_string();
                            }
                        }
                    }
                    self.confirm_delete = false;
                }
            } else if ui.button(dl, "delete", btn_w, btn_h, s) && self.selected.is_some() {
                self.confirm_delete = true;
                self.confirm_reset = false;
            }
        });
        // Create/load/reset replace the whole Settings object; refresh the
        // cached UI state so the next commit cannot restore stale controls.
        self.load_ui_settings(state);
    }
}
