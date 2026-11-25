// Serialized mirror of the C++ menu controls that are not mapped to dedicated Settings blocks.
// Kept as one object so profile save/load never drops a visual or inventory field.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type UiColor = [u8; 4];
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ChamsLayer {
    pub enabled: bool,
    pub material: usize,
    pub color: UiColor,
}

impl Default for ChamsLayer {
    fn default() -> Self {
        Self {
            enabled: false,
            material: 0,
            color: [164, 204, 232, 255],
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ChamsCfg {
    pub enabled: bool,
    pub primary: ChamsLayer,
    pub secondary: ChamsLayer,
    pub overlay: ChamsLayer,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct BarCfg {
    pub enabled: bool,
    pub position: usize,
    pub gradient: bool,
    pub show_value: bool,
    pub glow: bool,
    pub full_color: UiColor,
    pub low_color: UiColor,
    pub text_color: UiColor,
    pub glow_color: UiColor,
    pub glow_strength: f32,
}

impl Default for BarCfg {
    fn default() -> Self {
        Self {
            enabled: true,
            position: 0,
            gradient: true,
            show_value: true,
            glow: false,
            full_color: [110, 228, 145, 255],
            low_color: [235, 85, 85, 255],
            text_color: [235, 235, 235, 255],
            glow_color: [110, 228, 145, 160],
            glow_strength: 0.5,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayerOverlayCfg {
    pub enabled: bool,
    pub box_on: bool,
    pub box_style: usize,
    pub corner_len: f32,
    pub box_color: UiColor,
    pub skel_on: bool,
    pub skel_thick: f32,
    pub skel_vis: UiColor,
    pub skel_occ: UiColor,
    pub health: BarCfg,
    pub ammo: BarCfg,
    pub name_on: bool,
    pub name_color: UiColor,
    pub weapon_on: bool,
    pub weapon_display: usize,
    pub weapon_text: UiColor,
    pub weapon_icon: UiColor,
    pub flags_on: bool,
    pub flags: [bool; 8],
    pub oof_on: bool,
    pub oof_glow: bool,
    pub oof_w: f32,
    pub oof_h: f32,
    pub oof_rx: f32,
    pub oof_ry: f32,
    pub oof_vis: UiColor,
    pub oof_occ: UiColor,
}

impl Default for PlayerOverlayCfg {
    fn default() -> Self {
        let mut ammo = BarCfg::default();
        ammo.position = 2;
        ammo.full_color = [130, 190, 240, 255];
        ammo.low_color = [90, 130, 190, 255];
        Self {
            enabled: true,
            box_on: true,
            box_style: 1,
            corner_len: 10.0,
            box_color: [230, 230, 230, 255],
            skel_on: true,
            skel_thick: 1.4,
            skel_vis: [164, 204, 232, 230],
            skel_occ: [235, 110, 110, 210],
            health: BarCfg::default(),
            ammo,
            name_on: true,
            name_color: [235, 235, 235, 255],
            weapon_on: true,
            weapon_display: 0,
            weapon_text: [200, 205, 215, 255],
            weapon_icon: [220, 225, 235, 255],
            flags_on: true,
            flags: [true, true, true, false, false, false, true, true],
            oof_on: false,
            oof_glow: false,
            oof_w: 14.0,
            oof_h: 18.0,
            oof_rx: 220.0,
            oof_ry: 160.0,
            oof_vis: [164, 204, 232, 255],
            oof_occ: [235, 95, 95, 255],
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct SkinItem {
    pub def_index: i16,
    pub name: &'static str,
    pub category: usize, // 0 gun, 1 knife, 2 glove, 3 agent
    pub rarity: usize,
    pub default_skin: Option<(&'static str, usize)>,
}

pub const CATALOG_ITEMS: &[SkinItem] = &[
    SkinItem { def_index: 7, name: "AK-47", category: 0, rarity: 5, default_skin: Some(("Redline", 4)) },
    SkinItem { def_index: 16, name: "M4A4", category: 0, rarity: 5, default_skin: Some(("Howl", 6)) },
    SkinItem { def_index: 60, name: "M4A1-S", category: 0, rarity: 5, default_skin: Some(("Printstream", 5)) },
    SkinItem { def_index: 9, name: "AWP", category: 0, rarity: 5, default_skin: Some(("Dragon Lore", 5)) },
    SkinItem { def_index: 1, name: "Desert Eagle", category: 0, rarity: 4, default_skin: Some(("Blaze", 4)) },
    SkinItem { def_index: 61, name: "USP-S", category: 0, rarity: 4, default_skin: Some(("Kill Confirmed", 5)) },
    SkinItem { def_index: 4, name: "Glock-18", category: 0, rarity: 3, default_skin: Some(("Fade", 4)) },
    SkinItem { def_index: 40, name: "SSG 08", category: 0, rarity: 3, default_skin: Some(("Blood in Water", 5)) },
    SkinItem { def_index: 13, name: "Galil AR", category: 0, rarity: 2, default_skin: None },
    SkinItem { def_index: 10, name: "FAMAS", category: 0, rarity: 2, default_skin: None },
    SkinItem { def_index: 34, name: "MP9", category: 0, rarity: 2, default_skin: None },
    SkinItem { def_index: 17, name: "MAC-10", category: 0, rarity: 2, default_skin: None },
    SkinItem { def_index: 30, name: "Tec-9", category: 0, rarity: 2, default_skin: None },
    SkinItem { def_index: 3, name: "Five-SeveN", category: 0, rarity: 2, default_skin: None },
    SkinItem { def_index: 64, name: "R8 Revolver", category: 0, rarity: 2, default_skin: None },
    // Knives
    SkinItem { def_index: 507, name: "Karambit", category: 1, rarity: 6, default_skin: Some(("Doppler", 6)) },
    SkinItem { def_index: 515, name: "Butterfly Knife", category: 1, rarity: 6, default_skin: Some(("Fade", 6)) },
    SkinItem { def_index: 508, name: "M9 Bayonet", category: 1, rarity: 6, default_skin: None },
    SkinItem { def_index: 500, name: "Bayonet", category: 1, rarity: 6, default_skin: None },
    SkinItem { def_index: 523, name: "Talon Knife", category: 1, rarity: 6, default_skin: None },
    SkinItem { def_index: 525, name: "Skeleton Knife", category: 1, rarity: 6, default_skin: None },
    // Gloves
    SkinItem { def_index: 5030, name: "Sport Gloves", category: 2, rarity: 6, default_skin: Some(("Vice", 6)) },
    SkinItem { def_index: 5034, name: "Specialist Gloves", category: 2, rarity: 6, default_skin: None },
    SkinItem { def_index: 5033, name: "Moto Gloves", category: 2, rarity: 6, default_skin: None },
    SkinItem { def_index: 5031, name: "Driver Gloves", category: 2, rarity: 6, default_skin: None },
    // Agents
    SkinItem { def_index: 5601, name: "Cmdr. Davida Fernandez", category: 3, rarity: 5, default_skin: None },
    SkinItem { def_index: 5105, name: "Sir Bloody Darryl", category: 3, rarity: 5, default_skin: None },
];

pub const PAINT_KITS: &[(&str, usize)] = &[
    ("Default", 0),
    ("Redline", 4),
    ("Asiimov", 5),
    ("Printstream", 5),
    ("Dragon Lore", 5),
    ("Fade", 4),
    ("Doppler", 6),
    ("Hyper Beast", 5),
    ("Vulcan", 5),
    ("Neo-Noir", 5),
    ("Case Hardened", 4),
    ("Crimson Web", 5),
    ("Gungnir", 6),
    ("Vice", 6),
    ("Howl", 6),
];

/// Serialized mirror of the C++ menu controls.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ExtendedUiState {
    pub overlays: [PlayerOverlayCfg; 2],
    pub chams: [ChamsCfg; 5], // enemy, team, local, vm_weapon, vm_arms
    pub ragdoll_chams: [ChamsCfg; 3],
    pub glow: [(bool, UiColor); 3],
    pub ragdoll_glow: [(bool, UiColor); 3],
    pub local_alpha: bool,
    pub local_alpha_val: f32,
    pub local_alpha_scoped: bool,
    pub preview_group: usize,
    pub preview_visible: bool,

    // World
    pub item_group: usize,
    pub item_esp: [bool; 6],
    pub item_chams: [bool; 6],
    pub item_glow: [bool; 6],
    pub item_display: usize,
    pub item_dist: f32,
    pub proj_group: usize,
    pub proj_esp: [bool; 6],
    pub proj_ind: [bool; 3],
    pub bomb_timer: bool,
    pub spectator_list: bool,
    pub skybox_mat: bool,
    pub skybox_idx: usize,
    pub skybox_col: bool,
    pub sky_color: UiColor,
    pub cloud_color: UiColor,
    pub sun_color: UiColor,
    pub world_mat: bool,
    pub world_mat_type: usize,
    pub world_mat_col: UiColor,
    pub lighting: bool,
    pub lighting_int: f32,
    pub lighting_col: UiColor,
    pub bloom: bool,
    pub bloom_val: f32,
    pub gamma: bool,
    pub gamma_val: f32,
    pub dof: bool,
    pub dof_vals: [f32; 4],
    pub weather_on: bool,
    pub weather_type: usize,
    pub weather_col: UiColor,
    pub fog_on: bool,
    pub fog_density: f32,
    pub fog_aniso: f32,
    pub fog_dist: f32,
    pub fog_col: UiColor,
    pub wetness: bool,
    pub wetness_density: f32,
    pub wetness_speed: f32,
    pub wind: bool,
    pub wind_str: f32,
    pub wind_dir: f32,
    pub wind_turb: f32,

    // Skins
    pub skins_browsing: Option<i16>,
    pub skins_search: String,
    pub applied_skins: HashMap<i16, (usize, f32, i32, bool)>, // (paint_idx, wear, seed, stattrak)
    pub inspect_editor: bool,
    pub sticker_editor: bool,
    pub sticker_slot: usize,
    pub sticker_id: i32,

    // Misc
    pub hit_logs: bool,
    pub miss_logs: bool,
    pub hit_marker: bool,
    pub hit_marker_type: usize,
    pub hit_sound: bool,
    pub hit_sound_type: usize,
    pub hit_sound_vol: f32,
    pub death_sound: bool,
    pub death_sound_type: usize,
    pub death_sound_vol: f32,
    pub hit_effect: bool,
    pub death_effect: bool,
    pub death_effect_style: usize,
    pub bullet_impacts: bool,
    pub bullet_impact_type: usize,
    pub bullet_tracers: bool,
    pub proj_traj: bool,
    pub dlight: bool,
    pub jumpbug: bool,
    pub fastladder: bool,
    pub edgejump: bool,
    pub edgestop: bool,
    pub edgebug: bool,
    pub edgebug_mode: usize,
    pub edgebug_passes: i32,
    pub slowwalk: bool,
    pub slowwalk_speed: f32,
    pub autobuy: bool,
    pub ab_primary: usize,
    pub ab_secondary: usize,
    pub ab_armor: bool,
    pub ab_defuser: bool,
    pub ab_taser: bool,
    pub ab_nades: [bool; 5],
    pub clantag: bool,
    pub clantag_phrase: String,
    pub clantag_style: usize,
    pub clantag_ms: i32,
    pub override_name: bool,
    pub custom_name: String,
    pub wm_anchor: usize,
    pub wm_flags: [bool; 8],
    pub scoreboard_weapons: bool,
    pub reveal_radar: bool,
    pub preserve_killfeed: bool,
    pub disable_game_logs: bool,
    pub removals: [bool; 9],
    pub flash_alpha: f32,
    pub custom_fov: bool,
    pub fov_val: f32,
    pub scoped_fov_ov: bool,
    pub scoped_fov: f32,
    pub thirdperson: bool,
    pub tp_dist: f32,
    pub tp_hull: f32,
    pub aspect_on: bool,
    pub aspect_val: f32,
    pub vm_adjust: bool,
    pub vm_xyzf: [f32; 4],
    pub hotkeys_hud: bool,
    pub indicators_hud: bool,
    pub edit_hud: bool,
    /// Normalized HUD positions: [watermark, hotkeys, indicators, spectators].
    /// Defaults match the C++ interface layout.
    pub hud_pos: [[f32; 2]; 4],
    pub xhair_ov: bool,
    pub scope_ov: bool,
    pub vel_counter: bool,
    pub vel_chart: bool,
}

impl Default for ExtendedUiState {
    fn default() -> Self {
        let mut applied_skins = HashMap::new();
        applied_skins.insert(7, (1, 0.12, 421, true));
        applied_skins.insert(60, (3, 0.04, 108, true));
        applied_skins.insert(9, (4, 0.02, 661, false));
        applied_skins.insert(507, (6, 0.01, 387, true));
        applied_skins.insert(5030, (13, 0.08, 12, false));
        Self {
            overlays: [PlayerOverlayCfg::default(), PlayerOverlayCfg::default()],
            chams: std::array::from_fn(|_| ChamsCfg::default()),
            ragdoll_chams: std::array::from_fn(|_| ChamsCfg::default()),
            glow: [(false, [164, 204, 232, 200]); 3],
            ragdoll_glow: [(false, [164, 204, 232, 160]); 3],
            local_alpha: false,
            local_alpha_val: 0.5,
            local_alpha_scoped: true,
            preview_group: 0,
            preview_visible: true,
            item_group: 0,
            item_esp: [false; 6],
            item_chams: [false; 6],
            item_glow: [false; 6],
            item_display: 0,
            item_dist: 60.0,
            proj_group: 0,
            proj_esp: [false; 6],
            proj_ind: [false; 3],
            bomb_timer: true,
            spectator_list: false,
            skybox_mat: false,
            skybox_idx: 0,
            skybox_col: false,
            sky_color: [120, 165, 220, 255],
            cloud_color: [220, 225, 235, 255],
            sun_color: [255, 235, 190, 255],
            world_mat: false,
            world_mat_type: 0,
            world_mat_col: [180, 190, 200, 255],
            lighting: false,
            lighting_int: 1.0,
            lighting_col: [255, 255, 255, 255],
            bloom: false,
            bloom_val: 0.5,
            gamma: false,
            gamma_val: 2.2,
            dof: false,
            dof_vals: [10.0, 40.0, 600.0, 1800.0],
            weather_on: false,
            weather_type: 0,
            weather_col: [220, 230, 245, 255],
            fog_on: false,
            fog_density: 0.25,
            fog_aniso: 0.5,
            fog_dist: 4500.0,
            fog_col: [180, 195, 215, 255],
            wetness: false,
            wetness_density: 1.5,
            wetness_speed: 1.0,
            wind: false,
            wind_str: 1.0,
            wind_dir: 90.0,
            wind_turb: 1.0,
            skins_browsing: None,
            skins_search: String::new(),
            applied_skins,
            inspect_editor: false,
            sticker_editor: false,
            sticker_slot: 0,
            sticker_id: 0,
            hit_logs: true,
            miss_logs: false,
            hit_marker: true,
            hit_marker_type: 0,
            hit_sound: true,
            hit_sound_type: 2,
            hit_sound_vol: 75.0,
            death_sound: false,
            death_sound_type: 3,
            death_sound_vol: 70.0,
            hit_effect: false,
            death_effect: false,
            death_effect_style: 1,
            bullet_impacts: false,
            bullet_impact_type: 0,
            bullet_tracers: true,
            proj_traj: true,
            dlight: false,
            jumpbug: false,
            fastladder: false,
            edgejump: false,
            edgestop: false,
            edgebug: false,
            edgebug_mode: 1,
            edgebug_passes: 2,
            slowwalk: false,
            slowwalk_speed: 35.0,
            autobuy: false,
            ab_primary: 1,
            ab_secondary: 3,
            ab_armor: true,
            ab_defuser: true,
            ab_taser: false,
            ab_nades: [true, true, true, true, false],
            clantag: false,
            clantag_phrase: "WALLBANG".to_string(),
            clantag_style: 2,
            clantag_ms: 450,
            override_name: false,
            custom_name: String::new(),
            wm_anchor: 2,
            wm_flags: [true, true, true, true, true, false, false, false],
            scoreboard_weapons: false,
            reveal_radar: false,
            preserve_killfeed: true,
            disable_game_logs: false,
            removals: [false; 9],
            flash_alpha: 100.0,
            custom_fov: false,
            fov_val: 90.0,
            scoped_fov_ov: false,
            scoped_fov: 45.0,
            thirdperson: false,
            tp_dist: 120.0,
            tp_hull: 6.0,
            aspect_on: false,
            aspect_val: 1.333,
            vm_adjust: false,
            vm_xyzf: [0.0, 0.0, 0.0, 68.0],
            // HUD overlays stay off until the features are actually implemented.
            hotkeys_hud: false,
            indicators_hud: false,
            edit_hud: false,
            hud_pos: [
                [1.0, 0.0], // watermark: top right
                [0.0, 0.45], // hotkeys: left middle
                [0.5, 0.58], // indicators: center lower
                [1.0, 0.45], // spectators: right middle
            ],
            xhair_ov: false,
            scope_ov: false,
            vel_counter: false,
            vel_chart: false,
        }
    }
}
