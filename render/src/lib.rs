// Config block: рендер не знает офсетов; все чтения состояния идут через wallbang_core::state.
pub mod draw;
pub mod dx11;
pub mod esp_draw;
pub mod font;
pub mod hooks;
pub mod menu;
pub mod model;
pub mod native_preview;
pub mod pixel_font;
pub mod preview;
pub mod shaders;
pub mod ui;

pub use draw::{Color, DrawList};
pub use hooks::{set_frame_callback, set_overlay, set_trace};
pub use menu::is_menu_open;
pub(crate) use hooks::trace;

pub unsafe fn init() -> Result<(), String> {
    hooks::install()
}

pub unsafe fn shutdown() {
    hooks::shutdown();
}
