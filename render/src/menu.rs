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

#[repr(C)]
#[derive(Default)]
struct SystemTime {
    year: u16,
    month: u16,
    day_of_week: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
    milliseconds: u16,
}

#[link(name = "kernel32")]
extern "system" {
    fn GetLocalTime(time: *mut SystemTime);
}

fn local_clock() -> String {
    let mut time = SystemTime::default();
    unsafe { GetLocalTime(&mut time) };
    format!("{:02}:{:02}", time.hour, time.minute)
}

const SIDEBAR_W: f32 = 176.0;
const GAP: f32 = 12.0;
const SUBTAB_BAR_H: f32 = 44.0;
const HEADER_H: f32 = 64.0;
const MOVEMENT_BIND_KEYS: &[u16] = &[0, 0x20, 0x10, 0x11, 0x12, 0x05, 0x06, 0x56, 0x43, 0x46];
const MOVEMENT_BIND_NAMES: &[&str] = &["None", "Space", "Shift", "Ctrl", "Alt", "Mouse4", "Mouse5", "V", "C", "F"];

fn movement_binding(ui: &mut Ui, dl: &mut DrawList, bind: &mut wallbang_config::settings::MovementBind, s: &Style) {
    let mut selected = MOVEMENT_BIND_KEYS.iter().position(|&key| key == bind.key).unwrap_or(0);
    if ui.combo(dl, "key##movement_bind", &mut selected, MOVEMENT_BIND_NAMES, s) {
        bind.key = MOVEMENT_BIND_KEYS[selected.min(MOVEMENT_BIND_KEYS.len() - 1)];
    }
    ui.combo(dl, "mode##movement_bind", &mut bind.mode, &["Always", "Hold", "Toggle"], s);
}
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

#[cfg(test)]
mod panel_tests {
    use super::*;

    #[test]
    fn inventory_browse_apply_and_back_are_separate_actions() {
        let state = GameState::new();
        let mut menu = Menu::new();
        menu.tab=4; menu.subtab=0;
        let mut list=DrawList::new();
        let style=Style::new([120,180,255,255]);
        let original=state.settings().skins.applied.len();
        list.begin(1280.0,720.0);
        menu.ui.test_pointer(300.0,150.0,true,0.0);
        menu.draw_skins_real(&mut list,&state,100.0,100.0,640.0,480.0,&style);
        assert!(menu.ext.skins_browsing.is_some());
        assert_eq!(state.settings().skins.applied.len(),original);
        list.begin(1280.0,720.0);
        menu.ui.test_pointer(680.0,555.0,true,0.0);
        menu.draw_skins_real(&mut list,&state,100.0,100.0,640.0,480.0,&style);
        assert!(state.settings().skins.enabled);
        assert!(state.settings().skins.applied.iter().any(|c|c.def_index==menu.skin_draft.def_index));
        assert!(state.settings().misc.allow_writes);
        list.begin(1280.0,720.0);
        menu.ui.test_pointer(122.0,118.0,true,0.0);
        menu.draw_skins_real(&mut list,&state,100.0,100.0,640.0,480.0,&style);
        assert!(menu.ext.skins_browsing.is_none());
    }

    #[test]
    fn inactive_ui_mirror_cannot_revert_direct_controls() {
        let state = GameState::new();
        let mut menu = Menu::new();
        menu.tab = 5;
        menu.load_ui_settings(&state);
        {
            let mut settings = state.settings_mut();
            settings.misc.movement.jumpbug = true;
            settings.misc.reveal_radar = true;
            settings.visual.ambience.gamma = true;
        }
        for _ in 0..3 {
            menu.commit_ui_settings(&state);
            menu.load_ui_settings(&state);
        }
        let settings = state.settings();
        assert!(settings.misc.movement.jumpbug);
        assert!(settings.misc.reveal_radar);
        assert!(settings.visual.ambience.gamma);
    }

    #[test]
    fn visual_subtabs_and_misc_render_controls() {
        let state = GameState::new();
        let mut menu = Menu::new();
        let style = Style::new([120, 180, 255, 255]);
        let mut list = DrawList::new();
        let mut counts = Vec::new();
        for subtab in 0..3 {
            menu.subtab = subtab;
            list.begin(1280.0, 720.0);
            menu.draw_visuals_real(&mut list, &state, 100.0, 100.0, 640.0, 460.0, &style);
            counts.push(list.vertices().len());
        }
        assert_ne!(counts[0], counts[1]);
        assert_ne!(counts[1], counts[2]);
        assert_ne!(counts[0], counts[2]);
        assert_eq!(SUBTABS[5], &["movement"]);
        menu.subtab = 0;
        list.begin(1280.0, 720.0);
        menu.draw_misc_real(&mut list, &state, 100.0, 100.0, 640.0, 460.0, &style);
        assert!(!list.vertices().is_empty());
    }
}

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
    &["movement"],
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
    skin_dragging: bool,
    skin_scroll: usize,
    skin_sort: usize,
    skin_all_paints: bool,
    inventory_enemy: bool,
    inspect_moving: bool,
    inspect_model_fx: usize,
    inspect_overlay_fx: usize,
    inspect_stattrak_col: [u8; 4],
    skin_draft: wallbang_config::settings::AppliedSkin,
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

fn typed_layer_to_ui(layer: &wallbang_config::settings::ChamsMaterialLayer) -> ChamsLayer {
    ChamsLayer {
        enabled: layer.enabled,
        material: layer.material,
        color: layer.color,
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
            skin_dragging: false,
            skin_scroll: 0,
            skin_sort: 0,
            skin_all_paints: false,
            inventory_enemy: false,
            inspect_moving: false,
            inspect_model_fx: 0,
            inspect_overlay_fx: 0,
            inspect_stattrak_col: [235, 168, 52, 255],
            skin_draft: Default::default(),
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
        self.ext.world_color = ambience.world_color;
        self.ext.world_col = ambience.world_color_tint;
        self.ext.remove_3d_skybox = ambience.remove_3d_skybox;
        self.ext.remove_skybox_fog = ambience.remove_skybox_fog;
        self.ext.lighting = ambience.lighting;
        self.ext.lighting_int = ambience.lighting_intensity;
        self.ext.lighting_col = ambience.lighting_color;
        self.ext.fog_on = ambience.fog;
        self.ext.fog_density = ambience.fog_density;
        self.ext.fog_col = ambience.fog_color;
        self.ext.weather_on = ambience.weather_on;
        self.ext.weather_type = ambience.weather;
        self.ext.weather_col = ambience.weather_color;
        self.ext.fire_area = settings.visual.fire_area;
        self.ext.fire_area_col = settings.visual.fire_area_color;
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
        // Only controls owned by the active page may update typed state.
        // Other pages edit typed Settings directly and must never be mirrored back.
        if self.tab == PREVIEW_TAB {
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

        }
        if self.tab == 3 {
            settings.visual.fire_area = ext.fire_area;
            settings.visual.fire_area_color = ext.fire_area_col;
            settings.visual.items.groups = ext.item_esp.to_vec();
            let settings_ref = &mut *settings;
        let ambience = &mut settings_ref.visual.ambience;
            ambience.world_color = ext.world_color;
            ambience.world_color_tint = ext.world_col;
            ambience.remove_3d_skybox = ext.remove_3d_skybox;
            ambience.remove_skybox_fog = ext.remove_skybox_fog;
            ambience.weather_on = ext.weather_on;
            ambience.weather = ext.weather_type;
            ambience.weather_color = ext.weather_col;
        }
    }

    pub fn frame(&mut self, dl: &mut DrawList, state: &GameState, screen_w: f32, screen_h: f32) {
        self.ui.begin_frame_with_cursor(crate::hooks::menu_pointer(screen_w, screen_h));
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
        if !self.open || self.tab != 4 {
            crate::skin_inspect::park();
            self.skin_dragging = false;
        }
        self.load_ui_settings(state);
        self.draw_hud_widgets(dl, state, screen_w, screen_h);
        if !self.open || self.tab != PREVIEW_TAB {
            crate::native_preview::clear_request();
            crate::preview::clear_model_request();
        }
        if !self.open {
            dl.flush_top();
            return;
        }
        self.preview.sync_settings(state);
        self.draw_menu_internal(dl, state, screen_w, screen_h);
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
        self.subtab = subtab.min(SUBTABS[self.tab].len().saturating_sub(1));
        self.load_ui_settings(state);
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
        if self.tab == PREVIEW_TAB && screen_w >= mw + 356.0 {
            let fits_right = mx + mw + 332.0 <= screen_w - 8.0;
            let fits_left = mx >= 340.0;
            if !fits_right && !fits_left { mx = ((screen_w - mw - 332.0) * 0.5).max(8.0); }
        }
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
        self.preview.set_enemy(self.subtab == 0);
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
                &mut self.ext.overlays[self.subtab.min(1)],
                &mut self.ui,
                dl,
                px,
                py,
                pw,
                ph,
                &style,
            );
        }
        self.commit_ui_settings(state);
    }

    /// Новый сайдбар: только навигация по разделам, без выдуманных настроек.
    fn draw_sidebar_v2(&mut self, dl: &mut DrawList, mx: f32, my: f32, mh: f32, s: &Style) {
        let sx = mx + 1.0;
        let sy = my + HEADER_H - 3.0;
        let sw = NEW_SIDEBAR_W + GAP * 0.5;
        let sh = (mh - HEADER_H + 2.0).max(40.0);
        dl.rect(sx, sy, sw, sh, [11, 11, 13, 255]);
        dl.line(sx + sw, sy, sx + sw, sy + sh, 1.0, s.border);

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
            let hover = self.ui.inside(x, y, row_w, row_h);
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
                self.subtab = 0;
                self.ui.close_overlays();
                self.skin_dragging = false;
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
        if self.tab <= 5 {
            self.draw_top_bar(dl, x, y, w, s);
            let content_y = y + SUBTAB_BAR_H;
            let content_h = (h - SUBTAB_BAR_H).max(80.0);
            match self.tab {
                0 => self.draw_ragebot(dl, state, x, content_y, w, content_h, s),
                1 => self.draw_legitbot(dl, state, x, content_y, w, content_h, s),
                PREVIEW_TAB => {
                    self.draw_esp_real(dl, x, content_y, w, content_h, s);
                }
                3 => self.draw_visuals_real(dl, state, x, content_y, w, content_h, s),
                4 => self.draw_skins_real(dl, state, x, content_y, w, content_h, s),
                5 => self.draw_misc_real(dl, state, x, content_y, w, content_h, s),
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

    /// Only ESP switches that are actually read by `esp_draw.rs`. Everything
    /// else from the ported menu (oof arrows, glow, extra chams layers) was
    /// UI-only state and is intentionally not shown.
    fn draw_esp_real(&mut self, dl: &mut DrawList, x: f32, y: f32, w: f32, h: f32, s: &Style) {
        let subtab = self.subtab.min(1);
        let ov = &mut self.ext.overlays[subtab];
        let col_w = ((w - GAP) * 0.5).max(160.0);
        let right_x = x + col_w + GAP;
        self.ui.child(dl, x, y, col_w, h, true, s, |ui, dl| {
            ui.text(dl, if subtab == 0 { "Enemies" } else { "Allies" }, s.text_dim, 1.0);
            ui.separator(dl, s);
            ui.checkbox(dl, "enable", &mut ov.enabled, s);
            ui.checkbox(dl, "box", &mut ov.box_on, s);
            ui.popup(dl, "##box_popup_real", POPUP_W, 124.0, s, |ui, dl| {
                ui.combo(dl, "style##box_real", &mut ov.box_style, &BOX_STYLES, s);
                ui.slider_f32(dl, "corner length##real", &mut ov.corner_len, 0.05, 0.5, 2, "", s);
                ui.color_picker(dl, "color##box_real", &mut ov.box_color, s);
            });
            ui.checkbox(dl, "skeleton", &mut ov.skel_on, s);
            ui.popup(dl, "##skel_popup_real", POPUP_W, 56.0, s, |ui, dl| {
                ui.color_picker(dl, "visible color##skel_real", &mut ov.skel_vis, s);
            });
            ui.checkbox(dl, "health bar", &mut ov.health.enabled, s);
            ui.popup(dl, "##hp_popup_real", POPUP_W, 124.0, s, |ui, dl| {
                ui.checkbox(dl, "gradient##hp_real", &mut ov.health.gradient, s);
                ui.checkbox(dl, "show value##hp_real", &mut ov.health.show_value, s);
                ui.color_picker(dl, "full color##hp_real", &mut ov.health.full_color, s);
                ui.color_picker(dl, "low color##hp_real", &mut ov.health.low_color, s);
            });
            ui.checkbox(dl, "name", &mut ov.name_on, s);
            ui.popup(dl, "##name_popup_real", POPUP_W, 56.0, s, |ui, dl| {
                ui.color_picker(dl, "color##name_real", &mut ov.name_color, s);
            });
        });
        self.ui.child(dl, right_x, y, col_w, h, true, s, |ui, dl| {
            ui.text(dl, "Weapon & Extras", s.text_dim, 1.0);
            ui.separator(dl, s);
            ui.checkbox(dl, "weapon", &mut ov.weapon_on, s);
            ui.popup(dl, "##wep_popup_real", POPUP_W, 56.0, s, |ui, dl| {
                ui.color_picker(dl, "text color##wep_real", &mut ov.weapon_text, s);
            });
            ui.checkbox(dl, "ammo bar", &mut ov.ammo.enabled, s);
            ui.popup(dl, "##ammo_popup_right_real", POPUP_W, 124.0, s, |ui, dl| {
                ui.checkbox(dl, "gradient##ammo_right_real", &mut ov.ammo.gradient, s);
                ui.checkbox(dl, "show value##ammo_right_real", &mut ov.ammo.show_value, s);
                ui.color_picker(dl, "full color##ammo_right_real", &mut ov.ammo.full_color, s);
                ui.color_picker(dl, "low color##ammo_right_real", &mut ov.ammo.low_color, s);
            });
            ui.checkbox(dl, "flags", &mut ov.flags_on, s);
            ui.popup(dl, "##flags_popup_real", POPUP_W, 110.0, s, |ui, dl| {
                ui.multicombo(dl, "flags##mc_real", &mut ov.flags, &FLAG_NAMES, s);
                ui.color_picker(dl, "color##flags_real", &mut ov.flags_color, s);
            });
        });
    }

    /// Only controls with a real backend: read-only fire hull, guarded world
    /// tint, branch-safe removal hooks, and confirmed engine cvars.
    fn draw_visuals_real(
        &mut self,
        dl: &mut DrawList,
        state: &GameState,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        s: &Style,
    ) {
        let subtab = self.subtab.min(2);
        let mut settings = state.settings_mut();
        let settings_ref = &mut *settings;
        let visual = &mut settings_ref.visual;
        let ambience = &mut visual.ambience;
        let items = &mut visual.items;
        let grenade = &mut visual.grenade;
        let pp = &mut settings_ref.postprocess;
        if subtab == 0 {
            let col_w = ((w - GAP) * 0.5).max(160.0);
            let right_x = x + col_w + GAP;
            self.ui.child(dl, x, y, col_w, h.min(460.0), true, s, |ui, dl| {
                ui.text(dl, "World ESP", s.text_dim, 1.0);
                ui.separator(dl, s);
                ui.checkbox(dl, "dropped weapons", &mut items.enabled, s);
                if items.enabled {
                    ui.multicombo(dl, "groups##world_items", &mut self.ext.item_esp, &ITEM_GROUPS, s);
                    ui.combo(dl, "display##world_items", &mut items.display, &DISPLAY_TYPES, s);
                    ui.slider_f32(dl, "distance##world_items", &mut items.max_distance, 1.0, 100.0, 0, " m", s);
                    ui.color_picker(dl, "color##world_items", &mut items.color, s);
                }
                ui.checkbox(dl, "grenade trajectory", &mut grenade.trajectory, s);
                ui.checkbox(dl, "grenade warnings", &mut grenade.warnings, s);
                ui.checkbox(dl, "C4 timer", &mut self.ext.bomb_timer, s);
                ui.checkbox(dl, "fire area", &mut self.ext.fire_area, s);
                ui.popup(dl, "##fire_area_real", POPUP_W, 56.0, s, |ui, dl| {
                    ui.color_picker(dl, "color##fire_real", &mut self.ext.fire_area_col, s);
                });
            });
            self.ui.child(dl, right_x, y, col_w, h.min(460.0), true, s, |ui, dl| {
                ui.text(dl, "Removals & Local", s.text_dim, 1.0);
                ui.separator(dl, s);
                ui.checkbox(dl, "reduce smoke effects", &mut ambience.smoke_removal, s);
                ui.checkbox(dl, "remove 3d skybox", &mut self.ext.remove_3d_skybox, s);
                ui.checkbox(dl, "remove skybox fog", &mut self.ext.remove_skybox_fog, s);
                ui.checkbox(dl, "custom crosshair", &mut self.ext.xhair_ov, s);
                ui.checkbox(dl, "scope overlay", &mut self.ext.scope_ov, s);
                ui.checkbox(dl, "velocity counter", &mut self.ext.vel_counter, s);
                ui.separator(dl, s);
                ui.text(dl, "Flash and recoil removals stay hidden until a backend is wired.", s.text_dim, 0.85);
            });
        } else if subtab == 1 {
            let col_gap = 10.0;
            let col_w = ((w - col_gap) * 0.5).max(160.0);
            let card_h = h.min(480.0);
            const PP_PRESETS: [&str; 9] = [
                "Custom",
                "Competitive",
                "Vibrant",
                "Cinematic",
                "High Contrast",
                "Cold",
                "Warm",
                "Night Vision",
                "Desaturated",
            ];
            // Left card: D3D11 Color Grading & World Tint
            self.ui.child(dl, x, y, col_w, card_h, true, s, |ui, dl| {
                ui.text(dl, "Color Grading (D3D11)", s.text_dim, 1.0);
                ui.separator(dl, s);
                let blur = &mut settings_ref.misc.motion_blur;
                ui.checkbox(dl, "motion blur##active_scene", &mut blur.enabled, s);
                if blur.enabled {
                    ui.slider_f32(dl, "blur strength##active_scene", &mut blur.strength, 0.0, 1.0, 2, "", s);
                    ui.checkbox(dl, "only while moving##active_scene", &mut blur.on_velocity, s);
                }
                ui.checkbox(dl, "enable post-process##pp", &mut pp.enabled, s);
                if pp.enabled {
                    let mut preset_idx = pp.preset.min(PP_PRESETS.len() - 1);
                    if ui.combo(dl, "look preset##pp", &mut preset_idx, &PP_PRESETS, s) {
                        pp.apply_preset(preset_idx);
                    }
                    ui.slider_f32(dl, "nightmode##pp", &mut pp.nightmode, 0.0, 1.0, 2, "", s);
                    ui.color_picker(dl, "world tint##pp", &mut pp.world_tint, s);
                    ui.slider_f32(dl, "tint strength##pp", &mut pp.world_tint_strength, 0.0, 1.0, 2, "", s);
                    ui.slider_f32(dl, "exposure##pp", &mut pp.exposure, -1.5, 1.5, 2, "", s);
                    ui.slider_f32(dl, "contrast##pp", &mut pp.contrast, 0.5, 2.0, 2, "", s);
                    ui.slider_f32(dl, "saturation##pp", &mut pp.saturation, 0.0, 2.0, 2, "", s);
                    ui.slider_f32(dl, "vibrance##pp", &mut pp.vibrance, -1.0, 1.0, 2, "", s);
                    ui.slider_f32(dl, "temperature##pp", &mut pp.temperature, -1.0, 1.0, 2, "", s);
                    ui.slider_f32(dl, "tint##pp", &mut pp.tint, -1.0, 1.0, 2, "", s);
                    ui.slider_f32(dl, "gamma##pp", &mut pp.gamma, 0.5, 2.0, 2, "", s);
                    ui.slider_f32(dl, "vignette##pp", &mut pp.vignette, 0.0, 1.0, 2, "", s);
                    ui.slider_f32(dl, "sharpen##pp", &mut pp.sharpen, 0.0, 1.5, 2, "", s);
                }
            });
            // Right card: Scene, Lighting, Fog, Engine Post
            self.ui.child(dl, x + col_w + col_gap, y, col_w, card_h, true, s, |ui, dl| {
                ui.text(dl, "Scene & World", s.text_dim, 1.0);
                ui.separator(dl, s);
                ui.checkbox(dl, "world color", &mut self.ext.world_color, s);
                ui.popup(dl, "##world_color_real", POPUP_W, 56.0, s, |ui, dl| {
                    ui.color_picker(dl, "color##world_real", &mut self.ext.world_col, s);
                });
                ui.checkbox(dl, "remove 3d skybox", &mut self.ext.remove_3d_skybox, s);
                ui.checkbox(dl, "remove skybox fog", &mut self.ext.remove_skybox_fog, s);
                ui.separator(dl, s);
                ui.checkbox(dl, "skybox tint", &mut ambience.skybox_color, s);
                if ambience.skybox_color {
                    ui.color_picker(dl, "sky color##scene", &mut ambience.sky_color, s);
                }
                ui.text(dl, "Lighting & Fog", s.text_dim, 1.0);
                ui.checkbox(dl, "lighting", &mut ambience.lighting, s);
                if ambience.lighting {
                    ui.slider_f32(dl, "intensity##light", &mut ambience.lighting_intensity, 0.0, 4.0, 2, "", s);
                    ui.color_picker(dl, "light color##scene", &mut ambience.lighting_color, s);
                }
                ui.checkbox(dl, "fog override", &mut ambience.fog, s);
                if ambience.fog {
                    ui.slider_f32(dl, "density##fog", &mut ambience.fog_density, 0.0, 1.0, 2, "", s);
                    ui.color_picker(dl, "fog color##scene", &mut ambience.fog_color, s);
                    ui.slider_f32(dl, "end##fog", &mut ambience.fog_end, 0.0, 20_000.0, 0, "", s);
                }
                ui.separator(dl, s);
                ui.text(dl, "Engine Post", s.text_dim, 1.0);
                ui.checkbox(dl, "bloom", &mut ambience.bloom, s);
                if ambience.bloom {
                    ui.slider_f32(dl, "strength##bloom", &mut ambience.bloom_strength, 0.0, 4.0, 2, "", s);
                }
                ui.checkbox(dl, "gamma", &mut ambience.gamma, s);
                if ambience.gamma {
                    ui.slider_f32(dl, "value##gamma", &mut ambience.gamma_value, 0.5, 4.0, 2, "", s);
                }
                ui.checkbox(dl, "depth of field", &mut ambience.dof, s);
                if ambience.dof {
                    ui.slider_f32(dl, "near blur##dof", &mut ambience.dof_near_blurry, 0.0, 2_000.0, 0, "", s);
                    ui.slider_f32(dl, "far blur##dof", &mut ambience.dof_far_blurry, 0.0, 2_000.0, 0, "", s);
                }
            });
        } else {
        self.ui.child(dl, x, y, w, h.min(460.0), true, s, |ui, dl| {
            if subtab == 1 {
            ui.text(dl, "Atmosphere presets", s.text_dim, 1.0);
            if ui.button(dl, "Moonlit", 90.0, 24.0, s) {
                self.ext.world_color = true;
                self.ext.world_col = [130, 158, 204, 255];
                ambience.lighting = true; ambience.lighting_intensity = 0.75;
                ambience.fog = true; ambience.fog_density = 0.08; ambience.fog_end = 6500.0;
                self.ext.weather_on = true; self.ext.weather_type = 0;
                self.ext.weather_col = [195, 221, 255, 210];
                ambience.weather_density = 2200.0; ambience.weather_wind = 0.35;
                ambience.ambient_particles = false;
            }
            ui.same_line();
            if ui.button(dl, "Ember", 90.0, 24.0, s) {
                self.ext.world_color = true;
                self.ext.world_col = [230, 173, 132, 255];
                ambience.lighting = true; ambience.lighting_intensity = 0.85;
                ambience.fog = false;
                self.ext.weather_on = false;
                ambience.ambient_particles = true; ambience.particle_style = 1;
                ambience.particle_color = [255, 174, 77, 210]; ambience.particle_density = 130.0;
            }
            ui.same_line();
            if ui.button(dl, "Aurora", 90.0, 24.0, s) {
                self.ext.world_color = true;
                self.ext.world_col = [137, 207, 194, 255];
                ambience.lighting = true; ambience.lighting_intensity = 0.8;
                ambience.fog = false;
                self.ext.weather_on = true; self.ext.weather_type = 2;
                self.ext.weather_col = [159, 238, 222, 220]; ambience.weather_density = 1800.0;
                ambience.ambient_particles = true; ambience.particle_style = 0;
                ambience.particle_color = [184, 255, 205, 180]; ambience.particle_density = 60.0;
            }
            ui.separator(dl, s);
            }
            if subtab == 0 {
            ui.text(dl, "World overlay", s.text_dim, 1.0);
            ui.separator(dl, s);
            ui.checkbox(dl, "fire area", &mut self.ext.fire_area, s);
            ui.popup(dl, "##fire_area_real", POPUP_W, 56.0, s, |ui, dl| {
                ui.color_picker(dl, "color##fire_real", &mut self.ext.fire_area_col, s);
            });
            }
            if subtab == 1 {
            const PP_PRESETS: [&str; 9] = [
                "Custom",
                "Competitive",
                "Vibrant",
                "Cinematic",
                "High Contrast",
                "Cold",
                "Warm",
                "Night Vision",
                "Desaturated",
            ];
            ui.text(dl, "Color Grading (D3D11)", s.text_dim, 1.0);
            ui.separator(dl, s);
                ui.checkbox(dl, "enable post-process##pp", &mut pp.enabled, s);
                let blur = &mut settings_ref.misc.motion_blur;
                ui.checkbox(dl, "motion blur##scene", &mut blur.enabled, s);
                if blur.enabled {
                    ui.slider_f32(dl, "blur strength##scene", &mut blur.strength, 0.0, 1.0, 2, "", s);
                    ui.checkbox(dl, "only while moving##scene", &mut blur.on_velocity, s);
                }
            if pp.enabled {
                let mut preset_idx = pp.preset.min(PP_PRESETS.len() - 1);
                if ui.combo(dl, "look preset##pp", &mut preset_idx, &PP_PRESETS, s) {
                    pp.apply_preset(preset_idx);
                }
                ui.slider_f32(dl, "nightmode##pp", &mut pp.nightmode, 0.0, 1.0, 2, "", s);
                ui.slider_f32(dl, "exposure##pp", &mut pp.exposure, -1.5, 1.5, 2, "", s);
                ui.slider_f32(dl, "contrast##pp", &mut pp.contrast, 0.5, 2.0, 2, "", s);
                ui.slider_f32(dl, "saturation##pp", &mut pp.saturation, 0.0, 2.0, 2, "", s);
                ui.slider_f32(dl, "vibrance##pp", &mut pp.vibrance, -1.0, 1.0, 2, "", s);
                ui.slider_f32(dl, "temperature##pp", &mut pp.temperature, -1.0, 1.0, 2, "", s);
                ui.slider_f32(dl, "tint##pp", &mut pp.tint, -1.0, 1.0, 2, "", s);
                ui.slider_f32(dl, "gamma##pp", &mut pp.gamma, 0.5, 2.0, 2, "", s);
                ui.slider_f32(dl, "vignette##pp", &mut pp.vignette, 0.0, 1.0, 2, "", s);
                ui.slider_f32(dl, "sharpen##pp", &mut pp.sharpen, 0.0, 1.5, 2, "", s);
            }
            ui.separator(dl, s);
            ui.checkbox(dl, "world color", &mut self.ext.world_color, s);
            ui.popup(dl, "##world_color_real", POPUP_W, 56.0, s, |ui, dl| {
                ui.color_picker(dl, "color##world_real", &mut self.ext.world_col, s);
            });
            ui.separator(dl, s);
            ui.text(dl, "Scene", s.text_dim, 1.0);
            ui.checkbox(dl, "remove 3d skybox", &mut self.ext.remove_3d_skybox, s);
            ui.checkbox(dl, "remove skybox fog", &mut self.ext.remove_skybox_fog, s);
            ui.separator(dl, s);
            ui.text(dl, "Lighting", s.text_dim, 1.0);
            ui.checkbox(dl, "lighting", &mut ambience.lighting, s);
            if ambience.lighting {
                ui.slider_f32(dl, "intensity##light", &mut ambience.lighting_intensity, 0.0, 4.0, 2, "", s);
            }
            ui.separator(dl, s);
            ui.text(dl, "Fog", s.text_dim, 1.0);
            ui.checkbox(dl, "fog override", &mut ambience.fog, s);
            if ambience.fog {
                ui.slider_f32(dl, "density##fog", &mut ambience.fog_density, 0.0, 1.0, 2, "", s);
                ui.slider_f32(dl, "end##fog", &mut ambience.fog_end, 0.0, 20_000.0, 0, "", s);
            }
            ui.separator(dl, s);
            ui.text(dl, "Post", s.text_dim, 1.0);
            ui.checkbox(dl, "bloom", &mut ambience.bloom, s);
            if ambience.bloom {
                ui.slider_f32(dl, "strength##bloom", &mut ambience.bloom_strength, 0.0, 4.0, 2, "", s);
            }
            ui.checkbox(dl, "gamma", &mut ambience.gamma, s);
            if ambience.gamma {
                ui.slider_f32(dl, "value##gamma", &mut ambience.gamma_value, 0.5, 4.0, 2, "", s);
            }
            ui.checkbox(dl, "depth of field", &mut ambience.dof, s);
            if ambience.dof {
                ui.slider_f32(dl, "near blur##dof", &mut ambience.dof_near_blurry, 0.0, 2_000.0, 0, "", s);
                ui.slider_f32(dl, "far blur##dof", &mut ambience.dof_far_blurry, 0.0, 2_000.0, 0, "", s);
            }
            ui.separator(dl, s);
            }
            if subtab == 2 {
            ui.text(dl, "Weather", s.text_dim, 1.0);
            ui.checkbox(dl, "weather", &mut self.ext.weather_on, s);
            if self.ext.weather_on {
                ui.combo(dl, "type##weather_real", &mut self.ext.weather_type, &WEATHER_TYPES, s);
                ui.color_picker(dl, "color##weather_real", &mut self.ext.weather_col, s);
                ui.slider_f32(dl, "density##rain", &mut ambience.weather_density, 0.0, 10_000.0, 0, "", s);
                ui.slider_f32(dl, "speed##weather", &mut ambience.weather_speed, 0.1, 4.0, 2, "", s);
                ui.slider_f32(dl, "wind##weather", &mut ambience.weather_wind, -2.0, 2.0, 2, "", s);
            }
            ui.checkbox(dl, "ambient particles", &mut ambience.ambient_particles, s);
            if ambience.ambient_particles {
                ui.combo(dl, "style##particles", &mut ambience.particle_style, &["Fireflies", "Embers"], s);
                ui.color_picker(dl, "color##particles", &mut ambience.particle_color, s);
                ui.slider_f32(dl, "density##particles", &mut ambience.particle_density, 0.0, 192.0, 0, "", s);
            }
            ui.separator(dl, s);
            }
            if subtab == 1 {
            ui.text(dl, "Smoke", s.text_dim, 1.0);
            ui.checkbox(dl, "reduce smoke effects", &mut ambience.smoke_removal, s);
            }
        });
        }
        self.ext.lighting = ambience.lighting;
        self.ext.lighting_int = ambience.lighting_intensity;
        self.ext.fog_on = ambience.fog;
        self.ext.fog_density = ambience.fog_density;
    }

    fn draw_skins_real(&mut self, dl: &mut DrawList, state: &GameState,
        x: f32, y: f32, w: f32, h: f32, s: &Style) {
        use wallbang_config::settings::AppliedSkin;
        use wallbang_config::skin_catalog::{PAINT_CATALOG, paint_kit};
        use crate::skin_inspect::Request;
        let mut settings = state.settings_mut();
        let allow_writes = settings.misc.allow_writes;
        let skins = &mut settings.skins;
        if self.subtab == 3 {
            crate::skin_inspect::park();
            let left_w = (w * 0.46).max(220.0);
            let gap = 10.0;
            let right_x = x + left_w + gap;
            let right_w = (w - left_w - gap).max(160.0);

            // Left: Loadout Preview (T / CT Loadout + centered 3D agent + 4 slot tiles)
            dl.rect_rounded(x, y, left_w, h, 5.0, s.surface);
            dl.outline_rounded(x, y, left_w, h, 5.0, 1.0, s.border);
            let hdr_y = y + 10.0;
            let team_w = 96.0;
            let team_h = 24.0;
            let team_x0 = x + (left_w - team_w * 2.0 - 6.0) * 0.5;
            for (idx, label) in [(true, "T Loadout"), (false, "CT Loadout")] {
                let bx = team_x0 + if idx { 0.0 } else { team_w + 6.0 };
                let active = self.inventory_enemy == idx;
                let hover = !self.ui.overlay_blocking() && self.ui.inside(bx, hdr_y, team_w, team_h);
                if hover && self.ui.mouse_clicked() {
                    self.inventory_enemy = idx;
                }
                let bg = if active {
                    [s.accent[0], s.accent[1], s.accent[2], 38]
                } else if hover {
                    s.hovered
                } else {
                    s.elevated
                };
                dl.rect_rounded(bx, hdr_y, team_w, team_h, 3.0, bg);
                dl.outline_rounded(bx, hdr_y, team_w, team_h, 3.0, 1.0, if active { s.accent } else { s.border });
                let tw = dl.text_width(label, 0.9);
                dl.text(
                    (bx + (team_w - tw) * 0.5).floor(),
                    (hdr_y + (team_h - dl.text_height(0.9)) * 0.5).floor(),
                    label,
                    if active || hover { s.text } else { s.text_dim },
                    0.9,
                );
            }

            let tiles_h = 56.0;
            let vp_x = x + 12.0;
            let vp_y = hdr_y + team_h + 10.0;
            let vp_w = (left_w - 24.0).max(80.0);
            let vp_h = (h - (vp_y - y) - tiles_h - 22.0).max(120.0);
            dl.studio_stage((vp_x,vp_y,vp_w,vp_h),s.accent);
            // Soft ground shadow under centered agent
            let sh_w = (vp_w * 0.42).min(110.0);
            dl.rect_rounded(vp_x + (vp_w - sh_w) * 0.5, vp_y + vp_h - 18.0, sh_w, 8.0, 4.0, [0, 0, 0, 76]);
            dl.outline_rounded(vp_x, vp_y, vp_w, vp_h, 4.0, 1.0, s.border);
            crate::preview::inventory_model((vp_x, vp_y, vp_w, vp_h), self.inventory_enemy);

            // 4 slot tiles below agent: Weapon, Gloves, Knife, Agent
            let tile_y = vp_y + vp_h + 10.0;
            let tile_gap = 6.0;
            let tile_w = (vp_w - tile_gap * 3.0) / 4.0;
            let slot_defs: [(&str, &str, usize); 4] = [
                ("Weapon", if skins.applied.is_empty() { "Default" } else { "Equipped" }, 0),
                ("Gloves", if skins.gloves.def_index != 0 { "Equipped" } else { "Default" }, 2),
                ("Knife", if skins.knife.def_index != 0 { "Equipped" } else { "Default" }, 1),
                ("Agent", if skins.agent_enabled { "Custom" } else { "Default" }, 3),
            ];
            for (i, (title, sub, target_subtab)) in slot_defs.iter().enumerate() {
                let tx = vp_x + (tile_w + tile_gap) * i as f32;
                let active = *target_subtab == self.subtab;
                let hover = !self.ui.overlay_blocking() && self.ui.inside(tx, tile_y, tile_w, tiles_h);
                if hover && self.ui.mouse_clicked() {
                    self.subtab = *target_subtab;
                }
                let bg = if active {
                    [s.accent[0], s.accent[1], s.accent[2], 32]
                } else if hover {
                    s.hovered
                } else {
                    s.elevated
                };
                dl.rect_rounded(tx, tile_y, tile_w, tiles_h, 4.0, bg);
                dl.outline_rounded(tx, tile_y, tile_w, tiles_h, 4.0, 1.0, if active { s.accent } else { s.border });
                let tw = dl.text_width(title, 0.88);
                dl.text((tx + (tile_w - tw) * 0.5).floor(), tile_y + 12.0, title, s.text, 0.88);
                let sw = dl.text_width(sub, 0.78);
                dl.text((tx + (tile_w - sw) * 0.5).floor(), tile_y + 32.0, sub, s.text_dim, 0.78);
            }

            // Right: Agent equipment settings
            self.ui.child(dl, right_x, y, right_w, h, true, s, |ui, dl| {
                ui.text(dl, "Agent equipment", s.text_dim, 1.0);
                ui.separator(dl, s);
                ui.checkbox(dl, "enable skin changer", &mut skins.enabled, s);
                ui.checkbox(dl, "agent model", &mut skins.agent_enabled, s);
                let agents: Vec<_> = wallbang_config::skin_catalog::ITEM_MODELS.iter()
                    .filter(|(_, model)| model.starts_with("agents/") && !model.contains("/shared/")).collect();
                if !agents.is_empty() {
                    let labels: Vec<_> = agents.iter().map(|(_, model)| model.rsplit('/').next().unwrap_or(model).trim_end_matches(".vmdl")).collect();
                    if skins.agent_enabled && skins.agent_def_index == 0 { skins.agent_def_index = agents[0].0; }
                    let mut index = agents.iter().position(|(definition, _)| *definition == skins.agent_def_index).unwrap_or(0);
                    if ui.combo(dl, "agent##econ", &mut index, &labels, s) { skins.agent_def_index = agents[index.min(agents.len() - 1)].0; }
                }
                let _ = allow_writes;
            });
            return;
        }

        let category = self.subtab.min(2);
        let items: Vec<_> = CATALOG_ITEMS.iter().filter(|i| i.category == category).collect();
        if items.is_empty() { crate::skin_inspect::park(); return; }
        let browse = self.ext.skins_browsing.and_then(|id| items.iter().copied().find(|i| i.def_index == id));
        let footer_y = y + h - 38.0;
        let enabled_label = if skins.enabled { "Changer: on" } else { "Changer: off" };
        if self.ui.button_at(dl, enabled_label, x+12.0, footer_y+4.0, 112.0, 26.0, s) { skins.enabled = !skins.enabled; }
        let backend_status = wallbang_core::skin_status::message();
        dl.text(x+134.0, footer_y+11.0, &backend_status, s.text_dim, 0.7);
        if let Some(item) = browse {
            if self.skin_draft.def_index != item.def_index {
                self.skin_draft = match item.category {
                    1 => (skins.knife.def_index==item.def_index).then_some(&skins.knife),
                    2 => (skins.gloves.def_index==item.def_index).then_some(&skins.gloves),
                    _ => skins.applied.iter().find(|c|c.def_index==item.def_index),
                }.cloned().unwrap_or(AppliedSkin { def_index:item.def_index,..Default::default() });
            }
            if self.ui.button_at(dl, "< Back", x+12.0, y+8.0, 68.0, 26.0, s) {
                self.ext.skins_browsing = None; self.skin_scroll = 0; self.skin_dragging = false;
                crate::skin_inspect::park(); return;
            }
            let label = paint_kit(self.skin_draft.paint_kit_id).map(|p| p.name).unwrap_or("Default");
            let rarity_col = s.rarity_color(item.rarity as u8);
            // Collection badge icon + Weapon | Finish title + collection subtitle
            let badge_x = x + 90.0;
            let badge_y = y + 6.0;
            dl.rect_rounded(badge_x, badge_y, 26.0, 26.0, 4.0, s.elevated);
            dl.outline_rounded(badge_x, badge_y, 26.0, 26.0, 4.0, 1.0, rarity_col);
            dl.circle_filled(badge_x + 13.0, badge_y + 13.0, 5.5, rarity_col);
            dl.text(badge_x + 34.0, y + 5.0, &format!("{} | {}", item.name, label), s.text, 0.96);
            dl.text(badge_x + 34.0, y + 20.0, "CS2 Collection", s.text_dim, 0.78);
            dl.rect_rounded(x + 12.0, y + 36.0, w - 24.0, 2.5, 1.0, rarity_col);

            let rect = (x + 12.0, y + 44.0, w - 24.0, (footer_y - y - 94.0).max(160.0));
            let model_h = rect.3 * 0.57;
            let model_rect = (rect.0, rect.1, rect.2, model_h);
            let bar_y = rect.1 + rect.3 + 4.0;

            // Search + Options + Sort + Apply bar
            let btn_w = 72.0;
            let search_w = (w - 24.0 - btn_w * 3.0 - 18.0).max(110.0);
            self.ui.child(dl, x + 12.0, bar_y, search_w, 36.0, false, s, |ui, dl| {
                ui.text_input(dl, "##skin_catalog_search", &mut self.ext.skins_search, 64, "Search finishes...", s);
            });
            let opt_x = x + 12.0 + search_w + 6.0;
            let sort_x = opt_x + btn_w + 6.0;
            let apply_x = sort_x + btn_w + 6.0;
            const FX_MODES: [&str; 8] = [
                "Disabled",
                "Shaded",
                "Solid",
                "Glow",
                "Glow Outline",
                "Iridescent",
                "Water Flow",
                "Glossy",
            ];
            const STATTRAK_MODES: [&str; 2] = ["None", "On"];
            self.ui.popup_button(dl, "Options", "skin_finish_options", (opt_x, bar_y + 2.0, btn_w, 26.0), 268.0, 320.0, s, |ui, dl| {
                ui.text(dl, "Options", s.text, 0.96);
                ui.separator(dl, s);
                ui.text_input(dl, "Name##skin_opt_name", &mut self.skin_draft.custom_name, 160, "Custom name...", s);
                ui.slider_i32(dl, "Seed##skin_opt_seed", &mut self.skin_draft.seed, 0, 1000, "", s);
                ui.wear_slider(dl, "Wear##skin_opt_wear", &mut self.skin_draft.wear, s);
                let mut st_idx = usize::from(self.skin_draft.stattrak);
                if ui.combo(dl, "StatTrak##skin_opt_st", &mut st_idx, &STATTRAK_MODES, s) {
                    self.skin_draft.stattrak = st_idx == 1;
                }
                ui.combo(dl, "Model##skin_opt_model", &mut self.inspect_model_fx, &FX_MODES, s);
                ui.combo(dl, "Overlay##skin_opt_overlay", &mut self.inspect_overlay_fx, &FX_MODES, s);
                ui.color_picker(dl, "Colorize StatTrak##skin_opt_stcol", &mut self.inspect_stattrak_col, s);
            });
            let sort_lbl = if self.skin_sort == 0 { "Sort: A-Z" } else { "Sort: ID" };
            if self.ui.button_at(dl, sort_lbl, sort_x, bar_y + 2.0, btn_w, 26.0, s) {
                self.skin_sort = 1 - self.skin_sort;
            }
            if self.ui.button_at(dl, "Apply", apply_x, bar_y + 2.0, btn_w, 26.0, s) {
                let draft = self.skin_draft.clone();
                match item.category {
                    1 => skins.knife = draft,
                    2 => skins.gloves = draft,
                    _ => {
                        if let Some(slot) = skins.applied.iter_mut().find(|c| c.def_index == draft.def_index) {
                            *slot = draft;
                        } else {
                            skins.applied.push(draft);
                        }
                    }
                }
            }

            // Right-side Stickers (5 slots) + Moving rotation toggle inside model viewport
            let st_slot = 26.0;
            let st_gap = 5.0;
            let st_total_w = st_slot * 5.0 + st_gap * 4.0;
            let st_x = model_rect.0 + model_rect.2 - st_total_w - 8.0;
            let st_y = model_rect.1 + model_rect.3 - st_slot - 8.0;
            // Moving toggle above stickers
            let mov_w = 74.0;
            let mov_h = 20.0;
            let mov_x = model_rect.0 + model_rect.2 - mov_w - 8.0;
            let mov_y = st_y - mov_h - 6.0;
            let mov_hover = !self.ui.overlay_blocking() && self.ui.inside(mov_x, mov_y, mov_w, mov_h);
            if mov_hover && self.ui.mouse_clicked() {
                self.inspect_moving = !self.inspect_moving;
            }
            dl.rect_rounded(
                mov_x,
                mov_y,
                mov_w,
                mov_h,
                3.0,
                if self.inspect_moving {
                    [s.accent[0], s.accent[1], s.accent[2], 44]
                } else if mov_hover {
                    s.hovered
                } else {
                    s.elevated
                },
            );
            dl.outline_rounded(mov_x, mov_y, mov_w, mov_h, 3.0, 1.0, if self.inspect_moving { s.accent } else { s.border });
            let mov_tw = dl.text_width("Moving", 0.8);
            dl.text(
                (mov_x + (mov_w - mov_tw) * 0.5).floor(),
                (mov_y + (mov_h - dl.text_height(0.8)) * 0.5).floor(),
                "Moving",
                if self.inspect_moving || mov_hover { s.text } else { s.text_dim },
                0.8,
            );
            dl.text(st_x, mov_y + 3.0, "Stickers", s.text_dim, 0.78);
            for slot_i in 0..5 {
                let sx = st_x + (st_slot + st_gap) * slot_i as f32;
                dl.rect_rounded(sx, st_y, st_slot, st_slot, st_slot * 0.5, s.elevated);
                dl.outline_rounded(sx, st_y, st_slot, st_slot, st_slot * 0.5, 1.0, s.border);
            }

            if self.inspect_moving && !self.skin_dragging {
                skins.inspect_yaw = (skins.inspect_yaw + 0.35).rem_euclid(360.0);
            }
            if self.ui.mouse_clicked() && self.ui.inside(model_rect.0, model_rect.1, model_rect.2, model_rect.3) && !mov_hover && !self.ui.overlay_blocking() { self.skin_dragging = true; }
            if !self.ui.mouse_down() { self.skin_dragging = false; }
            if self.skin_dragging {
                let delta = self.ui.mouse_delta();
                skins.inspect_yaw = (skins.inspect_yaw + delta.x * 0.6).rem_euclid(360.0);
                skins.inspect_pitch = (skins.inspect_pitch - delta.y * 0.35).clamp(-60.0,60.0);
            }
            skins.inspect_zoom = (skins.inspect_zoom + self.ui.take_wheel_at(model_rect)*0.08).clamp(0.65,1.8);
            let query = self.ext.skins_search.to_ascii_lowercase();
            let allow_all = self.skin_all_paints && item.category != 2;
            let mut kits: Vec<_> = PAINT_CATALOG.iter().filter(|kit| (allow_all || wallbang_config::skin_catalog::compatible(item.def_index,kit.id))
                && (query.is_empty() || kit.name.to_ascii_lowercase().contains(&query) || kit.id.to_string() == query)).collect();
            if self.skin_sort == 0 { kits.sort_by_key(|kit| kit.name); } else { kits.sort_by_key(|kit| kit.id); }
            let grid_rect = (rect.0, rect.1+model_h+12.0, rect.2, rect.3-model_h-12.0);
            let columns = 5usize; let rows = 2usize;
            let total_rows = kits.len().div_ceil(columns);
            let wheel = self.ui.take_wheel_at(grid_rect);
            self.skin_scroll = ((self.skin_scroll as f32 - wheel).max(0.0) as usize).min(total_rows.saturating_sub(rows));
            let cw = (rect.2 - 8.0*(columns-1) as f32)/columns as f32;
            let ch = (grid_rect.3-8.0)/rows as f32;
            for (slot,kit) in kits.iter().skip(self.skin_scroll*columns).take(columns*rows).enumerate() {
                let card_rect = (grid_rect.0+(cw+8.0)*(slot%columns) as f32, grid_rect.1+(ch+8.0)*(slot/columns) as f32, cw,ch);
                let selected = kit.id == self.skin_draft.paint_kit_id;
                dl.rect_rounded(card_rect.0,card_rect.1,cw,ch,4.0,if selected { s.active } else { s.elevated });
                dl.text(card_rect.0+5.0,card_rect.1+ch-17.0,kit.name,s.text_dim,0.65);
                if self.ui.mouse_clicked() && !self.ui.overlay_blocking() && self.ui.inside(card_rect.0,card_rect.1,cw,ch) { self.skin_draft.paint_kit_id = kit.id; }
                dl.inventory_icon(item.def_index,kit.id,(card_rect.0+4.0,card_rect.1+2.0,cw-8.0,(ch-21.0).max(1.0)));
                dl.rect(card_rect.0,card_rect.1+ch-3.0,cw,3.0,RARITY_COLORS[item.rarity.min(7)]);
            }
            if total_rows > rows {
                let thumb_h = grid_rect.3*rows as f32/total_rows as f32;
                dl.rect(grid_rect.0+grid_rect.2-3.0,grid_rect.1+(grid_rect.3-thumb_h)*self.skin_scroll as f32/(total_rows-rows) as f32,3.0,thumb_h,s.accent);
            }
            crate::skin_inspect::set_request(Some(Request { rect:[rect.0,rect.1,rect.2,model_h],
                definition:item.def_index,paint:self.skin_draft.paint_kit_id,yaw:skins.inspect_yaw,pitch:skins.inspect_pitch,zoom:skins.inspect_zoom,
                show_model:true }));
            if !crate::skin_inspect::capture_ready() {
                let message=crate::skin_inspect::error().unwrap_or_else(||"Preparing weapon mesh...".into());
                dl.text(rect.0+12.0,rect.1+14.0,&message,s.text_dim,0.8);
            } else if self.skin_draft.paint_kit_id!=0 && wallbang_resources::weapon_finishes::texture(self.skin_draft.paint_kit_id).is_none() {
                dl.text(rect.0+12.0,rect.1+14.0,"Preview: base finish",s.text_dim,0.8);
            }
            if self.ui.button_at(dl,"Apply",x+w-76.0,footer_y+4.0,64.0,26.0,s) {
                self.skin_draft.def_index = item.def_index;
                match item.category {
                    1 => skins.knife = self.skin_draft.clone(),
                    2 => skins.gloves = self.skin_draft.clone(),
                    _ => { if let Some(config) = skins.applied.iter_mut().find(|c| c.def_index==item.def_index) { *config=self.skin_draft.clone(); }
                        else { skins.applied.push(self.skin_draft.clone()); } }
                }
                skins.enabled = true;
                skins.apply_revision=skins.apply_revision.wrapping_add(1);
            }
            if self.ui.button_at(dl,"Sort",x+w-132.0,footer_y+4.0,48.0,26.0,s) { self.skin_sort = 1-self.skin_sort.min(1); self.skin_scroll=0; }
            if item.category != 2 {
                let all_label = if self.skin_all_paints { "All" } else { "Native" };
                if self.ui.button_at(dl,all_label,x+w-196.0,footer_y+4.0,56.0,26.0,s) { self.skin_all_paints = !self.skin_all_paints; self.skin_scroll=0; }
            }
            return;
        }
        self.ext.skins_browsing = None;
        let agent_w = (w*0.29).clamp(130.0,180.0);
        dl.text(x+12.0,y+14.0,if self.inventory_enemy { "T equipment" } else { "CT equipment" },s.text,0.9);
        if self.ui.button_at(dl,"CT / T",x+agent_w-66.0,y+7.0,62.0,26.0,s) { self.inventory_enemy = !self.inventory_enemy; }
        let agent_rect = (x+8.0,y+43.0,agent_w-16.0,(footer_y-y-59.0).max(100.0));
        dl.studio_stage(agent_rect,s.accent);
        crate::preview::inventory_model(agent_rect,self.inventory_enemy);
        let rect = (x+agent_w+8.0,y+43.0,(w-agent_w-20.0).max(120.0),(footer_y-y-51.0).max(100.0));
        let rows = ((rect.3+8.0)/83.0).floor().max(1.0) as usize;
        let columns = 3usize;
        let column = |def:i16| -> usize { if [1,2,3,4,30,32,36,61,63,64].contains(&def) {0} else if [7,8,9,10,11,13,16,38,39,40,60].contains(&def) {2} else {1} };
        let mut groups: [Vec<&SkinItem>;3] = Default::default();
        for item in &items { let col = if category==0 { column(item.def_index) } else { groups.iter().enumerate().min_by_key(|(_,v)|v.len()).map(|(i,_)|i).unwrap_or(0) }; groups[col].push(*item); }
        let max_rows = groups.iter().map(|v|v.len()).max().unwrap_or(0);
        self.skin_scroll = ((self.skin_scroll as f32-self.ui.take_wheel_at(rect)).max(0.0) as usize).min(max_rows.saturating_sub(rows));
        let cw = (rect.2-16.0)/columns as f32;
        for (col,group) in groups.iter().enumerate() {
            dl.text(rect.0+(cw+8.0)*col as f32,y+16.0,if category==0 { ["Pistols","Mid-tier","Rifles"][col] } else { ["Collection","Models","Equipment"][col] },s.text_dim,0.8);
            for (row,item) in group.iter().skip(self.skin_scroll).take(rows).enumerate() {
                let card_rect = (rect.0+(cw+8.0)*col as f32,rect.1+83.0*row as f32,cw,75.0);
                let config = match item.category { 1 => (skins.knife.def_index==item.def_index).then_some(&skins.knife),2 => (skins.gloves.def_index==item.def_index).then_some(&skins.gloves),_ => skins.applied.iter().find(|c|c.def_index==item.def_index) };
                let paint = config.map(|c|c.paint_kit_id).unwrap_or(0);
                let selected = config.is_some();
                dl.rect_rounded(card_rect.0,card_rect.1,cw,75.0,4.0,if selected {s.active} else {s.elevated});
                dl.text(card_rect.0+6.0,card_rect.1+56.0,item.name,s.text_dim,0.75);
                dl.rect(card_rect.0,card_rect.1+72.0,cw,3.0,RARITY_COLORS[item.rarity.min(7)]);
                if self.ui.mouse_clicked() && !self.ui.overlay_blocking() && self.ui.inside(card_rect.0,card_rect.1,cw,75.0) {
                    self.ext.skins_browsing=Some(item.def_index);self.skin_draft=config.cloned().unwrap_or(AppliedSkin { def_index:item.def_index,..Default::default() });self.skin_scroll=0;self.ext.skins_search.clear();
                }
                dl.inventory_icon(item.def_index,paint,(card_rect.0+5.0,card_rect.1+3.0,cw-10.0,49.0));
            }
        }
        crate::skin_inspect::park();
    }

    fn draw_unavailable(&mut self, dl: &mut DrawList, x: f32, y: f32, w: f32, h: f32, s: &Style) {
        self.ui.child(dl, x, y, w, h.min(160.0), true, s, |ui, dl| {
            ui.text(dl, "No implemented features in this section.", s.text_dim, 1.0);
        });
    }

    /// Movement is the Celerity 1:1 port; radar reveal and the write gate have
    /// real backends. Everything else in the old Misc panel was UI-only.
    fn draw_misc_real(
        &mut self,
        dl: &mut DrawList,
        state: &GameState,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        s: &Style,
    ) {
        let _subtab = self.subtab.min(1);
        let mut settings = state.settings_mut();
        let col_gap = 10.0;
        let col_w = ((w - col_gap) * 0.5).max(160.0);
        let card_h = h.min(340.0);
        self.ui.child(dl, x, y, col_w, card_h, true, s, |ui, dl| {
            ui.text(dl, "Movement", s.text_dim, 1.0);
            ui.separator(dl, s);
            ui.checkbox(dl, "bhop", &mut settings.misc.bhop, s);
            ui.popup(dl, "##bhop_bind", POPUP_W, 100.0, s, |ui, dl| {
                movement_binding(ui, dl, &mut settings.misc.movement.bhop_bind, s);
            });
            ui.checkbox(dl, "autostrafe", &mut settings.misc.autostrafe, s);
            ui.checkbox(dl, "jumpbug", &mut settings.misc.movement.jumpbug, s);
            ui.popup(dl, "##jumpbug_bind", POPUP_W, 100.0, s, |ui, dl| {
                movement_binding(ui, dl, &mut settings.misc.movement.jumpbug_bind, s);
            });
            ui.checkbox(dl, "mini jump", &mut settings.misc.movement.mini_jump, s);
            ui.checkbox(dl, "edgejump", &mut settings.misc.movement.edgejump, s);
            ui.popup(dl, "##edgejump_bind", POPUP_W, 100.0, s, |ui, dl| {
                movement_binding(ui, dl, &mut settings.misc.movement.edgejump_bind, s);
            });
        });
        self.ui.child(dl, x + col_w + col_gap, y, col_w, h.min(420.0), true, s, |ui, dl| {
            ui.text(dl, "HUD Widgets & Utility", s.text_dim, 1.0);
            ui.separator(dl, s);
            ui.checkbox(dl, "watermark", &mut settings.misc.watermark, s);
            ui.checkbox(dl, "hotkeys", &mut self.ext.hotkeys_hud, s);
            ui.checkbox(dl, "spectators", &mut self.ext.spectator_list, s);
            ui.checkbox(dl, "indicators", &mut self.ext.indicators_hud, s);
            ui.checkbox(dl, "Spotify player", &mut self.ext.spotify_hud, s);
            ui.checkbox(dl, "hit log", &mut settings.misc.hit_log, s);
            ui.checkbox(dl, "miss log", &mut settings.misc.miss_log, s);
            ui.checkbox(dl, "vote revealer", &mut settings.misc.vote_revealer, s);
            ui.separator(dl, s);
            ui.text(dl, "Clantag", s.text_dim, 1.0);
            ui.checkbox(dl, "enable clantag##ct", &mut settings.misc.clantag.enabled, s);
            if settings.misc.clantag.enabled {
                const CT_STYLES: [&str; 5] = ["Fixed", "Typewriter", "Scroll", "Bounce", "Blink"];
                ui.combo(dl, "animation##ct", &mut settings.misc.clantag.style, &CT_STYLES, s);
                ui.slider_i32(dl, "speed (ms)##ct", &mut settings.misc.clantag.step_ms, 250, 2000, "ms", s);
                ui.text_input(dl, "phrase##ct", &mut settings.misc.clantag.phrase, 32, "WALLBANG", s);
            }
            ui.separator(dl, s);
            ui.checkbox(dl, "reveal radar", &mut settings.misc.reveal_radar, s);
        });
        self.ext.jumpbug = settings.misc.movement.jumpbug;
        self.ext.mini_jump = settings.misc.movement.mini_jump;
        self.ext.edgejump = settings.misc.movement.edgejump;
        self.ext.reveal_radar = settings.misc.reveal_radar;
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

        crate::music::set_art_rect(None);
        if self.ext.spotify_hud {
            if let Some(track) = crate::music::snapshot() {
                let w = 380.0;
                let x = ((screen_w - w) * 0.5).max(8.0);
                let y = (screen_h - 146.0).max(8.0);
                dl.chamfered_panel(x, y, w, 126.0, 5.0, [8, 8, 10, 240], [36, 36, 40, 255]);
                dl.rect(x + 14.0, y, 42.0, 2.0, s.accent);
                dl.rect_rounded(x + 12.0, y + 14.0, 76.0, 76.0, 3.0, s.elevated);
                if !self.open || !self.menu_covers(x + 50.0, y + 52.0) {
                    crate::music::set_art_rect(Some((x + 14.0, y + 16.0, 72.0, 72.0)));
                }
                dl.text(x + 100.0, y + 12.0, "SPOTIFY", s.accent, 0.75);
                let status = if track.playing { "PLAYING" } else { "PAUSED" };
                dl.text(x+w-14.0-dl.text_width(status,0.65), y+13.0, status, s.text_dim, 0.65);
                let title = if track.title.trim().is_empty() { "Unknown title" } else { track.title.as_str() };
                let title = dl.truncate(title, w - 114.0, 1.0);
                let artist = dl.truncate(&track.artist, w - 114.0, 0.85);
                dl.text(x + 100.0, y + 30.0, &title, s.text, 1.0);
                dl.text(x + 100.0, y + 49.0, &artist, s.text_dim, 0.85);
                for (i,id) in ["##music_previous","##music_play","##music_next"].iter().enumerate() {
                    let bx = x + 100.0 + i as f32 * 38.0;
                    if self.open && self.ui.button_at(dl, id, bx, y + 69.0, 30.0, 22.0, &s) {
                        crate::music::command(i as u8 + 1);
                    }
                    let cx = bx + 15.0;
                    let cy = y + 80.0;
                    let color = if i == 1 { s.accent } else { s.text_dim };
                    match i {
                        0 => {
                            dl.rect(cx-5.0,cy-4.0,1.5,8.0,color);
                            dl.convex_poly(&[(cx+4.0,cy-4.0),(cx-3.0,cy),(cx+4.0,cy+4.0)],color);
                        }
                        1 if track.playing => {
                            dl.rect(cx-4.0,cy-4.0,2.5,8.0,color);
                            dl.rect(cx+1.5,cy-4.0,2.5,8.0,color);
                        }
                        1 => dl.convex_poly(&[(cx-3.0,cy-5.0),(cx+5.0,cy),(cx-3.0,cy+5.0)],color),
                        _ => {
                            dl.rect(cx+4.0,cy-4.0,1.5,8.0,color);
                            dl.convex_poly(&[(cx-4.0,cy-4.0),(cx+3.0,cy),(cx-4.0,cy+4.0)],color);
                        }
                    }
                }
                let elapsed = if track.playing { track.sampled.elapsed().as_secs_f64() } else { 0.0 };
                let position = (track.position + elapsed).clamp(0.0, track.duration.max(0.0));
                let fraction = if track.duration > 0.0 { (position / track.duration) as f32 } else { 0.0 };
                dl.rect_rounded(x + 12.0, y + 99.0, w - 24.0, 3.0, 1.5, s.border);
                dl.rect_rounded(x + 12.0, y + 99.0, (w - 24.0) * fraction, 3.0, 1.5, s.accent);
                let timestamp = |seconds: f64| { let t = seconds.max(0.0) as u64; format!("{}:{:02}", t / 60, t % 60) };
                dl.text(x + 12.0, y + 108.0, &timestamp(position), s.text_dim, 0.7);
                let end = timestamp(track.duration);
                dl.text(x + w - 12.0 - dl.text_width(&end, 0.7), y + 108.0, &end, s.text_dim, 0.7);
            }
        }

        // 1) Watermark: 28px tall, 14px parallel slants, 18px padding (C++ ui.hpp).
        if state.settings().misc.watermark {
            let snapshot = state.snapshot();
            let nickname = snapshot.local_name.as_deref().unwrap_or("player").to_owned();
            let ping = snapshot.local_ping;
            let velocity = snapshot.local_velocity;
            let speed = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt().round() as i32;
            drop(snapshot);
            let fps = crate::hooks::frame_fps().round() as i32;
            let map = wallbang_core::engine::map_name().unwrap_or_else(|| "menu".to_owned());
            let segs = [
                nickname,
                map,
                format!("{ping} ms"),
                format!("{speed} u/s"),
                format!("{fps} fps"),
                local_clock(),
            ];
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
            let settings = state.settings();
            let (bhop, edgejump, jumpbug) = wallbang_core::state::movement_activity();
            let mut rows: Vec<(&str, &str)> = Vec::new();
            if bhop { rows.push(("bhop", "active")); }
            if edgejump { rows.push(("edgejump", "active")); }
            if jumpbug { rows.push(("jumpbug", "active")); }
            if settings.misc.autostrafe { rows.push(("autostrafe", "active")); }
            if settings.combat.ragebot.enabled { rows.push(("ragebot", "active")); }
            if settings.combat.legit.enabled { rows.push(("legitbot", "active")); }
            drop(settings);
            if rows.is_empty() && !self.open {
                // An empty runtime widget is intentionally hidden outside menu edit mode.
            } else {
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
        }

        // 3) Spectators: draggable chamfered panel on the right.
        if self.ext.spectator_list && self.tab != 2 {
            let rows: Vec<String> = state.snapshot().spectators.iter().map(|name| name.to_string()).collect();
            if rows.is_empty() && !self.open {
                // Hidden outside menu edit mode when nobody observes the local pawn.
            } else {
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
            for (i, name) in rows.iter().enumerate() {
                let ry = y + header_h + i as f32 * row_h;
                dl.text(
                    x + 12.0,
                    (ry + (row_h - th) * 0.5).floor(),
                    name,
                    s.text,
                    1.0,
                );
                let val = "watching";
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
        }

        // 4) Status indicators: draggable panel.
        if self.ext.indicators_hud {
            let settings = state.settings();
            let (bhop, edgejump, jumpbug) = wallbang_core::state::movement_activity();
            let mut rows: Vec<(&str, &str)> = Vec::new();
            if bhop { rows.push(("BHOP", "on")); }
            if edgejump { rows.push(("EDGEJUMP", "on")); }
            if jumpbug { rows.push(("JUMPBUG", "on")); }
            if settings.misc.autostrafe { rows.push(("AUTOSTRAFE", "on")); }
            if settings.postprocess.enabled { rows.push(("PP", "on")); }
            if settings.misc.reveal_radar { rows.push(("RADAR", "on")); }
            drop(settings);
            if !rows.is_empty() || self.open {
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

        // 5) Top-left Hit / Miss / Vote event logs (live ring buffer from wallbang_core::event_log).
        let (hit_on, miss_on, vote_on) = {
            let m = &state.settings().misc;
            (m.hit_log, m.miss_log, m.vote_revealer)
        };
        if hit_on || miss_on || vote_on {
            let entries = wallbang_core::event_log::snapshot(hit_on, miss_on, vote_on);
            let mut ly = 12.0f32;
            let lx = 12.0f32;
            let pill_h = 24.0f32;
            for (entry, alpha) in entries {
                let pw = dl.text_width(&entry.prefix, 1.0);
                let hw = dl.text_width(&entry.highlight, 1.0);
                let sw = dl.text_width(&entry.suffix, 1.0);
                let total_w = (22.0 + pw + hw + sw).max(120.0);
                let a_bg = (232.0 * alpha) as u8;
                let a_fg = (255.0 * alpha) as u8;
                dl.rect_rounded(lx, ly, total_w, pill_h, 3.0, [8, 8, 10, a_bg]);
                dl.outline_rounded(lx, ly, total_w, pill_h, 3.0, 1.0, [30, 30, 34, a_bg]);
                dl.rect(lx + 4.0, ly + 5.0, 2.0, pill_h - 10.0, [s.accent[0], s.accent[1], s.accent[2], a_fg]);
                let ty = (ly + (pill_h - th) * 0.5).floor();
                let mut tx = lx + 12.0;
                if !entry.prefix.is_empty() {
                    dl.text(tx, ty, &entry.prefix, [s.text[0], s.text[1], s.text[2], a_fg], 1.0);
                    tx += pw;
                }
                if !entry.highlight.is_empty() {
                    dl.text(tx, ty, &entry.highlight, [s.accent[0], s.accent[1], s.accent[2], a_fg], 1.0);
                    tx += hw;
                }
                if !entry.suffix.is_empty() {
                    dl.text(tx, ty, &entry.suffix, [s.text_dim[0], s.text_dim[1], s.text_dim[2], a_fg], 1.0);
                }
                ly += pill_h + 4.0;
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
        let mut bx = x + title_w + 22.0;
        let th = dl.text_height(1.0);
        let seg_h = 28.0;
        let seg_y = (y + (SUBTAB_BAR_H - seg_h) * 0.5).floor();
        let pad_x = 12.0;
        let gap_x = 4.0;

        for (i, &name) in defs.iter().enumerate() {
            let tw = dl.text_width(name, 1.0);
            let btn_w = (tw + pad_x * 2.0).max(62.0);
            if bx + btn_w > x + w - 8.0 {
                break;
            }
            let hover = !self.ui.overlay_blocking() && self.ui.inside(bx, seg_y, btn_w, seg_h);
            if hover && self.ui.mouse_clicked() {
                if self.subtab != i {
                    self.ext.skins_browsing = None;
                    self.skin_scroll = 0;
                }
                self.subtab = i;
                self.ui.close_overlays();
            }
            let selected = self.subtab == i;
            if selected {
                dl.rect_rounded(bx, seg_y, btn_w, seg_h, WIDGET_ROUNDING, s.elevated);
                dl.rect_rounded(
                    bx + 6.0,
                    seg_y + seg_h - 2.0,
                    (btn_w - 12.0).max(1.0),
                    2.0,
                    1.0,
                    s.accent,
                );
            } else if hover {
                dl.rect_rounded(bx, seg_y, btn_w, seg_h, WIDGET_ROUNDING, s.hovered);
            }
            let col = if selected {
                s.text
            } else if hover {
                s.text
            } else {
                s.text_dim
            };
            dl.text(
                (bx + (btn_w - tw) * 0.5).floor(),
                (seg_y + (seg_h - th) * 0.5).floor(),
                name,
                col,
                1.0,
            );
            bx += btn_w + gap_x;
        }
    }

    /// Ragebot panel with only settings that `combat::run` actually reads.
    /// Anti-aim, zeusbot, knifebot and quick-peek are not implemented, so they
    /// are deliberately absent instead of being advertised as working.
    fn draw_ragebot(&mut self, dl: &mut DrawList, state: &GameState, x: f32, y: f32, w: f32, h: f32, s: &Style) {
        let mut settings = state.settings_mut();
        let combat = &mut settings.combat;
        let col_w = (w - GAP) * 0.5;
        let right_x = x + col_w + GAP;
        let top_h = (h - GAP) * 0.56;
        let bottom_h = (h - GAP) * 0.44;

        self.ui.child(dl, x, y, col_w, top_h, true, s, |ui, dl| {
            ui.text(dl, "Aimbot", s.text_dim, 1.0);
            ui.separator(dl, s);
            ui.combo(dl, "weapon group##rage", &mut combat.ragebot.weapon_group, &WEAPON_GROUPS, s);
            let idx = combat.ragebot.weapon_group.min(5);
            combat.ragebot.weapon_group = idx;
            let wg = &mut combat.ragebot.groups[idx];
            ui.checkbox(dl, "enabled", &mut combat.ragebot.enabled, s);
            ui.slider_f32(dl, "fov", &mut wg.max_fov, 1.0, 180.0, 0, " deg", s);
            ui.slider_i32(dl, "min damage", &mut wg.min_damage, 0, 130, "", s);
        });

        self.ui.child(dl, x, y + top_h + GAP, col_w, bottom_h, true, s, |ui, dl| {
            ui.text(dl, "Target selection", s.text_dim, 1.0);
            ui.separator(dl, s);
            let idx = combat.ragebot.weapon_group.min(5);
            let wg = &mut combat.ragebot.groups[idx];
            ui.multicombo(dl, "hitboxes", &mut wg.hitboxes, &HITBOXES, s);
            ui.slider_f32(dl, "pointscale", &mut wg.pointscale, 0.0, 100.0, 0, "%", s);
            ui.checkbox(dl, "dynamic pointscale", &mut wg.dynamic_pointscale, s);
            ui.slider_i32(dl, "hitchance", &mut wg.hitchance, 0, 100, "%", s);
        });

        self.ui.child(dl, right_x, y, col_w, top_h, true, s, |ui, dl| {
            ui.text(dl, "Overrides & timing", s.text_dim, 1.0);
            ui.separator(dl, s);
            let idx = combat.ragebot.weapon_group.min(5);
            let wg = &mut combat.ragebot.groups[idx];
            ui.checkbox(dl, "hitchance override", &mut wg.hitchance_override, s);
            ui.popup(dl, "##hitchance_popup", POPUP_W, 56.0, s, |ui, dl| {
                ui.slider_i32(dl, "value##hc", &mut wg.hitchance_override_value, 1, 100, "%", s);
            });
            ui.checkbox(dl, "min damage override", &mut wg.min_damage_override, s);
            ui.popup(dl, "##mindamage_popup", POPUP_W, 56.0, s, |ui, dl| {
                ui.slider_i32(dl, "value##md", &mut wg.min_damage_override_value, 0, 130, "", s);
            });
            ui.checkbox(dl, "extrapolation", &mut wg.extrapolation, s);
            if wg.extrapolation {
                ui.slider_i32(dl, "prediction ticks", &mut wg.extrapolation_ticks, 1, 12, "", s);
            }
            ui.checkbox(dl, "backtrack", &mut wg.backtrack, s);
            if wg.backtrack {
                ui.slider_i32(dl, "backtrack ticks", &mut wg.backtrack_ticks, 1, 12, "", s);
            }
        });

        self.ui.child(dl, right_x, y + top_h + GAP, col_w, bottom_h, true, s, |ui, dl| {
            ui.text(dl, "Runtime", s.text_dim, 1.0);
            ui.separator(dl, s);
            ui.text(dl, "Game writes enabled.", s.text_dim, 0.9);
            ui.text(dl, "Movement controls are in Misc.", s.text_dim, 0.9);
        });
    }

    /// Legitbot panel limited to the aim-assist path implemented in combat.
    fn draw_legitbot(&mut self, dl: &mut DrawList, state: &GameState, x: f32, y: f32, w: f32, h: f32, s: &Style) {
        let mut settings = state.settings_mut();
        let combat = &mut settings.combat;
        let col_w = ((w - GAP) * 0.5).max(160.0);
        let right_x = x + col_w + GAP;
        let mut selected = combat.legit.weapons.min(5);
        let idx = selected;
        let wg = &mut combat.legit.groups[idx];

        self.ui.child(dl, x, y, col_w, h, true, s, |ui, dl| {
            ui.text(dl, "Aimbot", s.text_dim, 1.0);
            ui.separator(dl, s);
            ui.combo(dl, "weapon group##legit", &mut selected, &WEAPON_GROUPS, s);
            ui.checkbox(dl, "enabled", &mut combat.legit.enabled, s);
            ui.checkbox(dl, "aimbot", &mut wg.aimbot, s);
            ui.slider_f32(dl, "fov", &mut wg.fov, 0.5, 30.0, 1, " deg", s);
            ui.slider_i32(dl, "smooth", &mut wg.smooth, 0, 100, "", s);
            ui.checkbox(dl, "recoil control", &mut wg.rcs, s);
            if wg.rcs {
                ui.slider_i32(dl, "rcs minimum", &mut wg.rcs_min, 0, 200, "%", s);
                ui.slider_i32(dl, "rcs maximum", &mut wg.rcs_max, 0, 200, "%", s);
            }
            ui.checkbox(dl, "visualize fov", &mut wg.visualize_fov, s);
            if wg.visualize_fov {
                ui.color_picker(dl, "fov color##legit", &mut wg.fov_color, s);
            }
        });

        self.ui.child(dl, right_x, y, col_w, h, true, s, |ui, dl| {
            ui.text(dl, "Hitboxes & Trigger", s.text_dim, 1.0);
            ui.separator(dl, s);
            ui.multicombo(dl, "hitboxes##legit", &mut wg.hitboxes, &LEGIT_HITBOXES, s);
            ui.checkbox(dl, "triggerbot", &mut wg.triggerbot, s);
            if wg.triggerbot {
                ui.slider_i32(dl, "delay", &mut wg.trigger_delay, 0, 250, " ms", s);
                ui.slider_i32(dl, "hitchance", &mut wg.trigger_hitchance, 0, 100, "%", s);
            }
            ui.checkbox(dl, "autowall", &mut wg.autowall, s);
            if wg.autowall {
                ui.slider_i32(dl, "minimum damage", &mut wg.min_damage, 1, 100, "", s);
            }
            ui.checkbox(dl, "autostop", &mut wg.autostop, s);
            ui.checkbox(dl, "autoscope", &mut wg.autoscope, s);
        });
        combat.legit.weapons = selected.min(5);
    }

    #[allow(dead_code)]
    fn draw_ragebot_legacy(&mut self, dl: &mut DrawList, state: &GameState, x: f32, y: f32, w: f32, h: f32, s: &Style) {
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

    #[allow(dead_code)]
    fn draw_legitbot_legacy(&mut self, dl: &mut DrawList, state: &GameState, x: f32, y: f32, w: f32, h: f32, s: &Style) {
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

    #[allow(dead_code)] // Legacy C++ port; draw_esp_real is the active panel.
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

    #[allow(dead_code)] // Legacy C++ port; draw_visuals_real is the active panel.
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

                self.ui.child(dl, right_x, y, col_w, h - 172.0, true, s, |ui, dl| {
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

                self.ui.child(dl, right_x, y + h - 160.0, col_w, 160.0, true, s, |ui, dl| {
                    ui.text(dl, "Other", s.text_dim, 1.0);
                    ui.separator(dl, s);
                    ui.checkbox(dl, "bomb timer", &mut self.ext.bomb_timer, s);
                    ui.checkbox(dl, "spectator list", &mut self.ext.spectator_list, s);
                    ui.checkbox(dl, "fire area", &mut self.ext.fire_area, s);
                    ui.popup(dl, "##fire_area_popup", POPUP_W, 56.0, s, |ui, dl| {
                        ui.color_picker(dl, "color##fire", &mut self.ext.fire_area_col, s);
                    });
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
                    ui.checkbox(dl, "world color", &mut self.ext.world_color, s);
                    ui.popup(dl, "##world_color_popup", POPUP_W, 56.0, s, |ui, dl| {
                        ui.color_picker(dl, "color##world", &mut self.ext.world_col, s);
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

    #[allow(dead_code)] // No skin backend yet; the tab shows an empty pane.
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

    #[allow(dead_code)] // Legacy C++ port; draw_misc_real is the active panel.
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
