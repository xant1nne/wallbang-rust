// Config block: DEFAULT_MENU_W/DEFAULT_MENU_H; все новые поля имеют serde(default),
// чтобы старые JSON-профили продолжали грузиться.

use serde::{Deserialize, Serialize};

use crate::constants::{DEFAULT_MENU_H, DEFAULT_MENU_W, DEFAULT_PROFILE};
use crate::ui_state::ExtendedUiState;

/// Weapon groups в ragebot: rifle, smg, pistol, shotgun, sniper, lmg.
pub const WEAPON_GROUP_COUNT: usize = 6;

/// RGBA colour used by every visual setting.
pub type Rgba = [u8; 4];

/// One chams layer. `material` indexes `wallbang_resources::CHAM_MATERIALS`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ChamsMaterialLayer {
    pub enabled: bool,
    pub material: usize,
    pub color: Rgba,
}

impl Default for ChamsMaterialLayer {
    fn default() -> Self {
        Self {
            enabled: false,
            material: 3,
            color: [173, 192, 255, 150],
        }
    }
}

/// Up to three layers per chams target: primary, ignore-z, overlay.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ChamsPreset {
    pub enabled: bool,
    pub primary: ChamsMaterialLayer,
    pub secondary: ChamsMaterialLayer,
    pub overlay: ChamsMaterialLayer,
}

/// Player and viewmodel chams presets. Everything is off by default.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ChamsSettings {
    pub enemy: ChamsPreset,
    pub team: ChamsPreset,
    pub local: ChamsPreset,
    pub weapon: ChamsPreset,
    pub arms: ChamsPreset,
    pub ragdoll: Vec<ChamsPreset>,
}

impl Default for ChamsSettings {
    fn default() -> Self {
        Self {
            enemy: ChamsPreset::default(),
            team: ChamsPreset::default(),
            local: ChamsPreset::default(),
            weapon: ChamsPreset::default(),
            arms: ChamsPreset::default(),
            ragdoll: vec![ChamsPreset::default(); 3],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct GrenadeSettings {
    /// Preview the flight path, bounces and landing point before release.
    pub trajectory: bool,
    pub trajectory_color: Rgba,
    /// Warn about dangerous grenades near the local player.
    pub warnings: bool,
    pub warning_radius: f32,
}

impl Default for GrenadeSettings {
    fn default() -> Self {
        Self {
            trajectory: false,
            trajectory_color: [255, 196, 92, 220],
            warnings: false,
            warning_radius: 6.0,
        }
    }
}

/// Ambience controls. Values are stored even when the engine backend is not
/// active, so switching to an available backend never loses a user's setup.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct AmbienceSettings {
    pub skybox: bool,
    pub skybox_idx: usize,
    pub skybox_color: bool,
    pub sky_color: Rgba,
    pub cloud_color: Rgba,
    pub sun_color: Rgba,
    pub world_material: bool,
    pub world_material_type: usize,
    pub world_material_color: Rgba,
    pub lighting: bool,
    pub lighting_intensity: f32,
    pub lighting_color: Rgba,
    pub fog: bool,
    pub fog_density: f32,
    pub fog_color: Rgba,
    /// Burning-world ambience (fire barrels, molotov residue). Stored even
    /// while the backend is inactive, same as the rest of this block.
    pub fire_area: bool,
    pub weather_on: bool,
    /// 0 snow, 1 rain, 2 stars (matches the Visuals combo).
    pub weather: usize,
    pub weather_color: Rgba,
}

impl Default for AmbienceSettings {
    fn default() -> Self {
        Self {
            skybox: false,
            skybox_idx: 0,
            skybox_color: false,
            sky_color: [120, 165, 220, 255],
            cloud_color: [220, 225, 235, 255],
            sun_color: [255, 235, 190, 255],
            world_material: false,
            world_material_type: 0,
            world_material_color: [200, 200, 200, 255],
            lighting: false,
            lighting_intensity: 1.0,
            lighting_color: [255, 255, 255, 255],
            fog: false,
            fog_density: 0.25,
            fog_color: [180, 195, 215, 255],
            fire_area: false,
            weather_on: false,
            weather: 0,
            weather_color: [220, 230, 245, 255],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ViewmodelSettings {
    /// Viewmodel FOV override used while `adjust` is on.
    pub fov: f32,
    /// x, y, z, fov — same layout as the C++ viewmodel offset helper.
    pub offset: [f32; 4],
    pub thirdperson: bool,
    pub tp_dist: f32,
    pub tp_hull: f32,
    /// Celerity `fov_override`/`fov_value`: world FOV override (not while scoped).
    pub fov_override: bool,
    pub fov_value: f32,
    /// Scoped FOV override.
    pub scoped_fov_override: bool,
    pub scoped_fov: f32,
    /// Celerity `aspect_ratio_enabled`/`aspect_ratio_value`.
    pub aspect_enabled: bool,
    pub aspect_value: f32,
    /// Celerity `position_enabled`/`enabled`: viewmodel adjust master.
    pub adjust: bool,
}

impl Default for ViewmodelSettings {
    fn default() -> Self {
        Self {
            fov: 68.0,
            offset: [0.0, 0.0, 0.0, 68.0],
            thirdperson: false,
            tp_dist: 120.0,
            tp_hull: 6.0,
            fov_override: false,
            fov_value: 90.0,
            scoped_fov_override: false,
            scoped_fov: 45.0,
            aspect_enabled: false,
            aspect_value: 1.333,
            adjust: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct TracerSettings {
    pub bullet_tracers: bool,
    pub tracer_color: Rgba,
    pub tracer_length: f32,
    pub hit_marker: bool,
    pub hit_marker_type: usize,
    pub hit_color: Rgba,
    pub hit_effect: bool,
}

impl Default for TracerSettings {
    fn default() -> Self {
        Self {
            bullet_tracers: false,
            tracer_color: [164, 204, 232, 220],
            tracer_length: 24.0,
            hit_marker: false,
            hit_marker_type: 0,
            hit_color: [255, 236, 160, 255],
            hit_effect: false,
        }
    }
}

/// HUD overlays. These stay off until a real data source is wired.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct HudSettings {
    pub radar: bool,
    pub binds: bool,
    pub spectators: bool,
    pub events: bool,
    pub connection: bool,
    pub performance: bool,
}

impl Default for HudSettings {
    fn default() -> Self {
        Self {
            radar: false,
            binds: false,
            spectators: false,
            events: false,
            connection: false,
            performance: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ItemEspSettings {
    /// One entry per `wallbang_resources::WEAPON_GROUPS`.
    pub groups: Vec<bool>,
    pub enabled: bool,
    pub max_distance: f32,
    pub display: usize,
    pub color: Rgba,
}

impl Default for ItemEspSettings {
    fn default() -> Self {
        Self {
            groups: vec![false; 6],
            enabled: false,
            max_distance: 20.0,
            display: 0,
            color: [180, 200, 230, 220],
        }
    }
}

/// Everything under the Visuals tab.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct VisualSettings {
    pub chams: ChamsSettings,
    pub grenade: GrenadeSettings,
    pub ambience: AmbienceSettings,
    pub viewmodel: ViewmodelSettings,
    pub tracers: TracerSettings,
    pub hud: HudSettings,
    pub items: ItemEspSettings,
}

impl Default for VisualSettings {
    fn default() -> Self {
        Self {
            chams: ChamsSettings::default(),
            grenade: GrenadeSettings::default(),
            ambience: AmbienceSettings::default(),
            viewmodel: ViewmodelSettings::default(),
            tracers: TracerSettings::default(),
            hud: HudSettings::default(),
            items: ItemEspSettings::default(),
        }
    }
}

/// One equipped inventory item. `def_index` is an item definition index,
/// `paint_index` indexes `PAINT_KITS` in the skin catalog.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct AppliedSkin {
    pub def_index: i16,
    pub paint_index: usize,
    pub wear: f32,
    pub seed: i32,
    pub stattrak: bool,
}

impl Default for AppliedSkin {
    fn default() -> Self {
        Self {
            def_index: 0,
            paint_index: 0,
            wear: 0.01,
            seed: 0,
            stattrak: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StickerSlot {
    pub sticker_id: i32,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub scale: f32,
}

/// Everything under the Skins/Inventory tab.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct SkinSettings {
    pub applied: Vec<AppliedSkin>,
    pub stickers: Vec<StickerSlot>,
    pub charms: Vec<StickerSlot>,
    pub knife: AppliedSkin,
    pub gloves: AppliedSkin,
    pub agent: String,
    pub agent_voice: String,
    pub inspect_yaw: f32,
    pub inspect_zoom: f32,
}

impl Default for SkinSettings {
    fn default() -> Self {
        Self {
            applied: Vec::new(),
            stickers: vec![StickerSlot::default(); 5],
            charms: vec![StickerSlot::default(); 2],
            knife: AppliedSkin::default(),
            gloves: AppliedSkin::default(),
            agent: "tm_phoenix".to_owned(),
            agent_voice: "default".to_owned(),
            inspect_yaw: 0.0,
            inspect_zoom: 1.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct MovementSettings {
    pub acceleration: f32,
    pub turn_boost: f32,
    pub obstacle_avoidance: bool,
    pub jumpbug: bool,
    /// Celerity `bhop.mini_jump`: crouch on the jump tick for lower jumps.
    pub mini_jump: bool,
    pub edgejump: bool,
    pub edgestop: bool,
    pub edgebug: bool,
    pub strafe_assist: bool,
    pub last_input_priority: bool,
    pub fast_ladder: bool,
    pub super_toss: bool,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            acceleration: 1.0,
            turn_boost: 1.0,
            obstacle_avoidance: false,
            jumpbug: false,
            mini_jump: false,
            edgejump: false,
            edgestop: false,
            edgebug: false,
            strafe_assist: false,
            last_input_priority: true,
            fast_ladder: false,
            super_toss: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct GrenadeAutoSettings {
    pub he_release: bool,
    pub he_min_damage: i32,
    pub molotov_release: bool,
    pub molotov_max_distance: f32,
}

impl Default for GrenadeAutoSettings {
    fn default() -> Self {
        Self {
            he_release: false,
            he_min_damage: 40,
            molotov_release: false,
            molotov_max_distance: 8.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct AutobuySettings {
    pub enabled: bool,
    pub primary: usize,
    pub secondary: usize,
    pub armor: bool,
    pub defuser: bool,
    pub taser: bool,
    pub grenades: [bool; 5],
}

impl Default for AutobuySettings {
    fn default() -> Self {
        Self {
            enabled: false,
            primary: 0,
            secondary: 0,
            armor: true,
            defuser: true,
            taser: false,
            grenades: [true, true, true, true, false],
        }
    }
}

/// Hit/death sounds. The sound assets are ported from wallbang-main-clone.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct AudioSettings {
    pub hit_sound: bool,
    pub hit_sound_type: usize,
    pub hit_sound_volume: f32,
    pub death_sound: bool,
    pub death_sound_type: usize,
    pub death_sound_volume: f32,
    pub custom_hit_sound: String,
    pub custom_death_sound: String,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            hit_sound: false,
            hit_sound_type: 3,
            hit_sound_volume: 25.0,
            death_sound: false,
            death_sound_type: 2,
            death_sound_volume: 20.0,
            custom_hit_sound: "hit.wav".to_owned(),
            custom_death_sound: "kill.wav".to_owned(),
        }
    }
}

/// Complete user configuration, serialized as a single compact JSON object.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub menu: MenuSettings,
    pub combat: CombatSettings,
    pub esp: EspSettings,
    pub misc: MiscSettings,
    pub visual: VisualSettings,
    pub skins: SkinSettings,
    /// Serialized mirror of the legacy C++ menu controls.
    pub ui: ExtendedUiState,
    /// Name of the profile that owns these settings, without `.wb`.
    #[serde(default = "default_active_profile")]
    pub active_profile: String,
}

fn default_active_profile() -> String {
    DEFAULT_PROFILE.to_owned()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            menu: MenuSettings::default(),
            combat: CombatSettings::default(),
            esp: EspSettings::default(),
            misc: MiscSettings::default(),
            visual: VisualSettings::default(),
            skins: SkinSettings::default(),
            ui: ExtendedUiState::default(),
            active_profile: default_active_profile(),
        }
    }
}

/// Menu position, size, accent colour and open-state.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct MenuSettings {
    pub open: bool,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    /// RGBA accent colour.
    pub accent: [u8; 4],
    /// Virtual-key code of the menu toggle (VK_INSERT by default).
    pub toggle_key: u32,
}

impl Default for MenuSettings {
    fn default() -> Self {
        Self {
            open: false,
            x: 0.0,
            y: 0.0,
            w: DEFAULT_MENU_W,
            h: DEFAULT_MENU_H,
            accent: [120, 180, 255, 255],
            toggle_key: 0x2D,
        }
    }
}

/// Per-weapon-group ragebot settings (как settings::combat::ragebot::group в C++ меню).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct WeaponGroup {
    pub max_fov: f32,
    pub min_damage: i32,
    /// head, chest, stomach, arms, legs, feet.
    pub hitboxes: [bool; 6],
    pub pointscale: f32,
    pub dynamic_pointscale: bool,
    pub hitchance: i32,
    pub hitchance_override: bool,
    pub hitchance_override_value: i32,
    pub min_damage_override: bool,
    pub min_damage_override_value: i32,
    pub lethal_damage: bool,
    pub autostop: bool,
    pub air_autostop: bool,
    pub extrapolation: bool,
    pub extrapolation_ticks: i32,
    /// Стрельба по записанной позиции из прошлого (backtrack).
    pub backtrack: bool,
    pub backtrack_ticks: i32,
}

impl Default for WeaponGroup {
    fn default() -> Self {
        Self {
            max_fov: 180.0,
            min_damage: 0,
            hitboxes: [true, true, false, false, false, false],
            pointscale: 0.0,
            dynamic_pointscale: false,
            hitchance: 0,
            hitchance_override: false,
            hitchance_override_value: 60,
            min_damage_override: false,
            min_damage_override_value: 0,
            lethal_damage: false,
            autostop: false,
            air_autostop: false,
            extrapolation: false,
            extrapolation_ticks: 1,
            backtrack: false,
            backtrack_ticks: 6,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct RagebotSettings {
    pub enabled: bool,
    pub silent: bool,
    pub no_spread: bool,
    pub weapon_group: usize,
    pub groups: Vec<WeaponGroup>,
}

impl Default for RagebotSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            silent: false,
            no_spread: false,
            weapon_group: 0,
            groups: vec![WeaponGroup::default(); WEAPON_GROUP_COUNT],
        }
    }
}

impl RagebotSettings {
    #[inline]
    pub fn group(&self) -> &WeaponGroup {
        self.groups
            .get(self.weapon_group)
            .or_else(|| self.groups.first())
            .unwrap_or(&EMPTY_GROUP)
    }
}

static EMPTY_GROUP: WeaponGroup = WeaponGroup {
    max_fov: 180.0,
    min_damage: 0,
    hitboxes: [false; 6],
    pointscale: 0.0,
    dynamic_pointscale: false,
    hitchance: 0,
    hitchance_override: false,
    hitchance_override_value: 0,
    min_damage_override: false,
    min_damage_override_value: 0,
    lethal_damage: false,
    autostop: false,
    air_autostop: false,
    extrapolation: false,
    extrapolation_ticks: 0,
    backtrack: false,
    backtrack_ticks: 0,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct AntiAimSettings {
    pub enabled: bool,
    /// 0 none, 1 down, 2 up.
    pub pitch: i32,
    pub auto_yaw_adjust: bool,
    pub manual_left: bool,
    pub manual_right: bool,
    pub avoid_backstab: bool,
    pub direction_indicator: bool,
}

impl Default for AntiAimSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            pitch: 0,
            auto_yaw_adjust: false,
            manual_left: false,
            manual_right: false,
            avoid_backstab: false,
            direction_indicator: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct QuickPeekSettings {
    pub enabled: bool,
    pub color: [u8; 4],
    pub retrack_color: [u8; 4],
}

impl Default for QuickPeekSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            color: [120, 180, 255, 160],
            retrack_color: [255, 120, 120, 160],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct DuckPeekSettings {
    pub enabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ZeusSettings {
    pub enabled: bool,
    pub max_fov: f32,
    pub drop_after: bool,
}

impl Default for ZeusSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            max_fov: 180.0,
            drop_after: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct KnifeSettings {
    pub enabled: bool,
    pub max_fov: f32,
}

impl Default for KnifeSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            max_fov: 180.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AutosSettings {
    pub scope: bool,
    pub revolver: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct PenetrationCrosshairSettings {
    pub enabled: bool,
    pub glow: bool,
    pub glow_strength: f32,
    pub can_penetrate_fill: [u8; 4],
    pub can_penetrate_outline: [u8; 4],
    pub blocked_fill: [u8; 4],
    pub blocked_outline: [u8; 4],
}

impl Default for PenetrationCrosshairSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            glow: false,
            glow_strength: 0.5,
            can_penetrate_fill: [120, 255, 160, 60],
            can_penetrate_outline: [120, 255, 160, 240],
            blocked_fill: [255, 90, 90, 60],
            blocked_outline: [255, 90, 90, 240],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct LegitGroup {
    pub aimbot: bool,
    pub autostop: bool,
    pub autoscope: bool,
    pub fov: f32,
    pub smooth: i32,
    /// head, chest, stomach, arms, legs.
    pub hitboxes: [bool; 5],
    pub visualize_fov: bool,
    pub fov_color: [u8; 4],
    pub rcs: bool,
    pub rcs_min: i32,
    pub rcs_max: i32,
    pub standalone_rcs: bool,
    pub standalone_rcs_strength: i32,
    pub standalone_rcs_min: i32,
    pub standalone_rcs_max: i32,
    pub triggerbot: bool,
    pub trigger_delay: i32,
    pub trigger_hitchance: i32,
    pub give_me_your_seed: bool,
    pub autowall: bool,
    pub min_damage: i32,
}

impl Default for LegitGroup {
    fn default() -> Self {
        Self {
            aimbot: false,
            autostop: false,
            autoscope: false,
            fov: 4.0,
            smooth: 8,
            hitboxes: [true, true, false, false, false],
            visualize_fov: false,
            fov_color: [120, 180, 255, 160],
            rcs: false,
            rcs_min: 50,
            rcs_max: 150,
            standalone_rcs: false,
            standalone_rcs_strength: 100,
            standalone_rcs_min: 50,
            standalone_rcs_max: 150,
            triggerbot: false,
            trigger_delay: 20,
            trigger_hitchance: 60,
            give_me_your_seed: false,
            autowall: false,
            min_damage: 1,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct LegitSettings {
    pub enabled: bool,
    pub weapons: usize,
    pub groups: Vec<LegitGroup>,
}

impl Default for LegitSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            weapons: 0,
            groups: vec![LegitGroup::default(); WEAPON_GROUP_COUNT],
        }
    }
}

impl LegitSettings {
    #[inline]
    pub fn group(&self) -> &LegitGroup {
        self.groups
            .get(self.weapons)
            .or_else(|| self.groups.first())
            .unwrap_or(&EMPTY_LEGIT_GROUP)
    }
}

static EMPTY_LEGIT_GROUP: LegitGroup = LegitGroup {
    aimbot: false,
    autostop: false,
    autoscope: false,
    fov: 0.0,
    smooth: 0,
    hitboxes: [false; 5],
    visualize_fov: false,
    fov_color: [0; 4],
    rcs: false,
    rcs_min: 0,
    rcs_max: 0,
    standalone_rcs: false,
    standalone_rcs_strength: 0,
    standalone_rcs_min: 0,
    standalone_rcs_max: 0,
    triggerbot: false,
    trigger_delay: 0,
    trigger_hitchance: 0,
    give_me_your_seed: false,
    autowall: false,
    min_damage: 0,
};

/// Aim assist / rage and legit bot settings.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct CombatSettings {
    pub ragebot: RagebotSettings,
    pub legit: LegitSettings,
    pub antiaim: AntiAimSettings,
    pub quickpeek: QuickPeekSettings,
    pub duckpeek: DuckPeekSettings,
    pub zeusbot: ZeusSettings,
    pub knifebot: KnifeSettings,
    pub autos: AutosSettings,
    pub penetration_crosshair: PenetrationCrosshairSettings,
}

impl Default for CombatSettings {
    fn default() -> Self {
        Self {
            ragebot: RagebotSettings::default(),
            legit: LegitSettings::default(),
            antiaim: AntiAimSettings::default(),
            quickpeek: QuickPeekSettings::default(),
            duckpeek: DuckPeekSettings::default(),
            zeusbot: ZeusSettings::default(),
            knifebot: KnifeSettings::default(),
            autos: AutosSettings::default(),
            penetration_crosshair: PenetrationCrosshairSettings::default(),
        }
    }
}

/// ESP option bit masks, 1:1 with Celerity `esp_bits` (`src/config.hpp`).
pub mod esp_bits {
    // health/ammo element types (shared mask layout)
    pub const TEXT: usize = 1 << 0;
    pub const BAR: usize = 1 << 1;
    pub const GRADIENT: usize = 1 << 2;

    // weapon display types
    pub const WEAPON_TEXT: usize = 1 << 0;
    pub const WEAPON_ICON: usize = 1 << 1;

    // right-side info flags
    pub const FLAG_DISTANCE: usize = 1 << 0;
    pub const FLAG_CASH: usize = 1 << 1;
    pub const FLAG_ARMOR: usize = 1 << 2;
    pub const FLAG_SCOPED: usize = 1 << 3;
    pub const FLAG_BOMB: usize = 1 << 4;
}

#[inline]
pub const fn has_flag(mask: usize, flag: usize) -> bool {
    mask & flag != 0
}

/// One ESP target config, 1:1 with Celerity `esp_player_cfg`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct EspPlayerSettings {
    pub enabled: bool,
    /// Serialized as the JSON key `box` (`box` is a Rust keyword).
    #[serde(rename = "box")]
    pub r#box: bool,
    /// Corner length as a fraction of the smaller box side (0.005..=0.5).
    /// 0.5 draws a full rectangle.
    pub corner_length: f32,
    pub name: bool,
    pub weapon: bool,
    /// `esp_bits::WEAPON_TEXT` / `esp_bits::WEAPON_ICON`.
    pub weapon_type: usize,
    pub health_bar: bool,
    /// `esp_bits::BAR` / `esp_bits::GRADIENT` / `esp_bits::TEXT`.
    pub health_type: usize,
    pub skeleton: bool,
    pub ammo: bool,
    pub ammo_type: usize,
    pub flags: bool,
    pub flags_type: usize,
    pub box_color: Rgba,
    pub name_color: Rgba,
    pub weapon_color: Rgba,
    pub health_low_color: Rgba,
    pub health_high_color: Rgba,
    pub skeleton_color: Rgba,
    pub ammo_low_color: Rgba,
    pub ammo_high_color: Rgba,
    pub flags_color: Rgba,
}

impl Default for EspPlayerSettings {
    fn default() -> Self {
        Self {
            // Same defaults as Celerity, except that nothing draws until the
            // user enables the target class explicitly.
            enabled: false,
            r#box: false,
            corner_length: 0.25,
            name: false,
            weapon: false,
            weapon_type: esp_bits::WEAPON_TEXT,
            health_bar: false,
            health_type: esp_bits::BAR | esp_bits::GRADIENT | esp_bits::TEXT,
            skeleton: false,
            ammo: false,
            ammo_type: esp_bits::BAR | esp_bits::GRADIENT | esp_bits::TEXT,
            flags: false,
            flags_type: 0,
            box_color: [255, 255, 255, 255],
            name_color: [255, 255, 255, 255],
            weapon_color: [255, 255, 255, 255],
            health_low_color: [255, 102, 170, 255],
            health_high_color: [255, 255, 255, 255],
            skeleton_color: [255, 255, 255, 255],
            ammo_low_color: [255, 102, 102, 255],
            ammo_high_color: [102, 204, 255, 255],
            flags_color: [255, 255, 255, 255],
        }
    }
}

/// Player ESP overlays.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct EspSettings {
    pub enabled: bool,
    /// Celerity-style per-target configs driving the world overlay.
    pub enemies: EspPlayerSettings,
    pub teammates: EspPlayerSettings,
    /// Serialized as the JSON key `box` (`box` is a Rust keyword).
    #[serde(rename = "box")]
    pub r#box: bool,
    pub skeleton: bool,
    pub name: bool,
    pub health: bool,
    pub armor: bool,
    pub distance: bool,
    /// Current weapon name and remaining ammo.
    pub weapon: bool,
    pub ammo: bool,
    pub visible_only: bool,
    /// 0 corner, 1 full; shared by the world overlay and preview.
    pub box_style: usize,
    pub corner_size: f32,
    pub box_color: Rgba,
    pub skeleton_thickness: f32,
    pub skeleton_visible_color: Rgba,
    pub skeleton_occluded_color: Rgba,
    pub health_bar: bool,
    /// 0 above box, 1 left, 2 right.
    pub health_pos: usize,
    pub health_full_color: Rgba,
    pub health_low_color: Rgba,
    pub name_color: Rgba,
    pub armor_color: Rgba,
    pub distance_color: Rgba,
    /// Ammo bar colour (the bar is drawn right of the box).
    pub ammo_color: Rgba,
    /// Use the optional Panorama engine model instead of the built-in skinned render.
    pub preview_engine: bool,
    /// Show raw preview state values in the preview panel.
    pub preview_debug: bool,
}

impl Default for EspSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            enemies: EspPlayerSettings::default(),
            teammates: EspPlayerSettings::default(),
            r#box: false,
            skeleton: false,
            name: false,
            health: false,
            armor: false,
            distance: false,
            weapon: false,
            ammo: false,
            visible_only: false,
            box_style: 0,
            corner_size: 10.0,
            box_color: [255, 122, 110, 240],
            skeleton_thickness: 1.4,
            skeleton_visible_color: [236, 240, 248, 245],
            skeleton_occluded_color: [255, 112, 104, 225],
            health_bar: true,
            health_pos: 0,
            health_full_color: [110, 228, 145, 255],
            health_low_color: [235, 85, 85, 255],
            name_color: [255, 142, 128, 255],
            armor_color: [164, 204, 232, 255],
            distance_color: [190, 194, 202, 255],
            ammo_color: [190, 194, 202, 255],
            preview_engine: false,
            preview_debug: false,
        }
    }
}

/// Motion blur controls from the Miscellaneous tab.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct MotionBlurSettings {
    pub enabled: bool,
    /// 0.0..1.0; scales the blur trail length.
    pub strength: f32,
    /// 0 screen, 1 viewmodel.
    pub mode: usize,
    /// Only blur while the local player is moving.
    pub on_velocity: bool,
}

impl Default for MotionBlurSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            strength: 0.5,
            mode: 0,
            on_velocity: true,
        }
    }
}

/// Miscellaneous quality-of-life features.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct MiscSettings {
    pub watermark: bool,
    /// Reveal all enemies on the radar (writes m_bSpotted + mask; needs allow_writes).
    pub reveal_radar: bool,
    pub bhop: bool,
    pub autostrafe: bool,
    /// Разрешает запись в игру (движение/углы/кнопки). По умолчанию выключено.
    pub allow_writes: bool,
    pub hit_log: bool,
    pub miss_log: bool,
    pub motion_blur: MotionBlurSettings,
    /// UI language index. Only English resources are compiled at present.
    pub language: usize,
    pub movement: MovementSettings,
    pub grenade_auto: GrenadeAutoSettings,
    pub autobuy: AutobuySettings,
    pub audio: AudioSettings,
}

impl Default for MiscSettings {
    fn default() -> Self {
        Self {
            watermark: false,
            reveal_radar: false,
            bhop: false,
            autostrafe: false,
            allow_writes: false,
            hit_log: false,
            miss_log: false,
            motion_blur: MotionBlurSettings::default(),
            language: 0,
            movement: MovementSettings::default(),
            grenade_auto: GrenadeAutoSettings::default(),
            autobuy: AutobuySettings::default(),
            audio: AudioSettings::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_visual_and_inventory_fields_round_trip() {
        let mut settings = Settings::default();
        settings.visual.chams.enemy.enabled = true;
        settings.visual.chams.enemy.primary.material = 6;
        settings.visual.chams.enemy.primary.color = [12, 34, 56, 78];
        settings.visual.ambience.skybox_idx = 3;
        settings.visual.ambience.fire_area = true;
        settings.visual.ambience.weather_on = true;
        settings.visual.tracers.hit_marker = true;
        settings.skins.applied.push(AppliedSkin {
            def_index: 9,
            paint_index: 4,
            wear: 0.07,
            seed: 42,
            stattrak: true,
        });
        settings.skins.stickers[0].sticker_id = 7;
        settings.misc.movement.edgejump = true;
        settings.misc.audio.hit_sound = true;
        settings.esp.weapon = true;
        settings.esp.ammo = true;

        let json = serde_json::to_vec(&settings).expect("serialize");
        let loaded: Settings = serde_json::from_slice(&json).expect("deserialize");
        assert!(loaded.visual.chams.enemy.enabled);
        assert_eq!(loaded.visual.chams.enemy.primary.material, 6);
        assert_eq!(loaded.visual.chams.enemy.primary.color, [12, 34, 56, 78]);
        assert_eq!(loaded.visual.ambience.skybox_idx, 3);
        assert!(loaded.visual.ambience.fire_area);
        assert!(loaded.visual.tracers.hit_marker);
        assert_eq!(loaded.skins.applied.len(), 1);
        assert_eq!(loaded.skins.applied[0].seed, 42);
        assert_eq!(loaded.skins.stickers[0].sticker_id, 7);
        assert!(loaded.misc.movement.edgejump);
        assert!(loaded.misc.audio.hit_sound);
        assert!(loaded.esp.weapon && loaded.esp.ammo);
    }

    #[test]
    fn old_profile_missing_new_fields_uses_defaults() {
        let json = br#"{"esp":{"enabled":true},"misc":{"bhop":true}}"#;
        let settings: Settings = serde_json::from_slice(json).expect("deserialize");
        assert!(settings.esp.enabled);
        assert!(settings.misc.bhop);
        assert!(!settings.visual.chams.enemy.enabled);
        assert_eq!(settings.skins.agent, "tm_phoenix");
        assert!(settings.misc.audio.custom_hit_sound.ends_with(".wav"));
        assert_eq!(settings.active_profile, DEFAULT_PROFILE);
    }
}
