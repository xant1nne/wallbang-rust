// Config block: TICK_SLEEP_MS, MUTATIONS_ENABLED, CHUNK_SLOT_*, BONE_STRIDE, MATRIX_SIZE,
// W2S_MIN_W, CONFIRM_TIMEOUT_S, DEFAULT_MENU_*, DEFAULT_FONT_SIZE, MAX_CONFIG_NAME_BYTES.

/// Main loop sleep per tick (milliseconds).
pub const TICK_SLEEP_MS: u64 = 4;

/// Runtime memory mutation patching is disabled by default.
pub const MUTATIONS_ENABLED: bool = false;

/// Количество слотов в одном чанке entity list.
pub const CHUNK_SLOT_COUNT: usize = 512;

/// Размер одной CEntityIdentity (build 14184; 0x78 — старый билд, проверять при апдейте).
pub const CHUNK_SLOT_SIZE: usize = 0x70;

/// Stride between bone entries in the model-state bone array.
pub const BONE_STRIDE: usize = 32;

/// Number of floats in a 4x4 view/world matrix.
pub const MATRIX_SIZE: usize = 16;

/// Minimum clip-space W considered valid by world-to-screen projection.
pub const W2S_MIN_W: f32 = 0.001f32;

/// Seconds before a delete/reset confirmation dialog times out.
pub const CONFIRM_TIMEOUT_S: f32 = 3.0f32;

/// Default menu width.
pub const DEFAULT_MENU_W: f32 = 720.0f32;

/// Default menu height.
pub const DEFAULT_MENU_H: f32 = 520.0f32;

/// Default font size used by the menu.
pub const DEFAULT_FONT_SIZE: f32 = 14.0f32;

/// Maximum byte length of a config profile name.
pub const MAX_CONFIG_NAME_BYTES: usize = 64;

/// Extension used by all wallbang config profiles. Payload stays JSON.
pub const CONFIG_EXTENSION: &str = "wb";

/// Profile loaded on startup and created on first run.
pub const DEFAULT_PROFILE: &str = "default";
