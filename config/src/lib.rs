// Config block: exposes constants, offsets, settings, registry, storage and hotkeys
// built on DEFAULT_MENU_*, MAX_CONFIG_NAME_BYTES and the VK_INSERT default.

pub mod constants;
pub mod collections;
pub mod offsets;
pub mod settings;
pub mod skin_catalog;
pub mod ui_state;
pub mod registry;
pub mod signatures;
pub mod storage;
pub mod hotkeys;

pub use settings::Settings;
