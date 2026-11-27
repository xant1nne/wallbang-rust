// Config block: Present=8 в IDXGISwapChain vtable. Оверлей рисуется прямо в
// swapchain игры (как старый xdraw в on_present), в потоке самого Present.
// Отдельное layered-окно убрано: оно давало чёрный экран поверх игры.
use crate::draw::DrawList;
use crate::dx11::Dx11;
use crate::menu::Menu;
use parking_lot::Mutex;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicIsize, AtomicU32, AtomicU64, Ordering};
use wallbang_core::hooks::VTableHook;
use windows::core::Interface;
use windows::Win32::Graphics::Direct3D11::{
    D3D11CreateDeviceAndSwapChain, D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION,
};
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL_11_0};
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_MODE_DESC, DXGI_RATIONAL, DXGI_SAMPLE_DESC,
};
use windows::Win32::Graphics::Dxgi::{
    IDXGIAdapter, IDXGISwapChain, DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_EFFECT_DISCARD,
    DXGI_USAGE_RENDER_TARGET_OUTPUT,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CallWindowProcW, CreateWindowExW, DefWindowProcW, RegisterClassExW, SetWindowLongPtrW,
    WNDCLASSEXW, CS_HREDRAW, CS_VREDRAW, GWLP_WNDPROC, WS_EX_LAYERED, WS_EX_NOACTIVATE,
    WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_POPUP,
};
use windows::core::w;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleA(name: *const std::ffi::c_char) -> *mut c_void;
}

#[link(name = "user32")]
extern "system" {
    fn ShowCursor(show: i32) -> i32;
    fn ClipCursor(rect: *const c_void) -> i32;
    fn GetCursorInfo(info: *mut CursorInfo) -> i32;
    fn ScreenToClient(window: *mut c_void, point: *mut wallbang_core::input::Point) -> i32;
    fn GetClientRect(window: *mut c_void, rect: *mut [i32; 4]) -> i32;
    fn PostMessageW(window: *mut c_void, message: u32, wparam: usize, lparam: isize) -> i32;
    fn LoadCursorW(instance: *mut c_void, name: *const u16) -> *mut c_void;
    fn SetCursor(cursor: *mut c_void) -> *mut c_void;
}

const MENU_CURSOR_MESSAGE: u32 = 0x8000 + 0x57A;
static SDL_RELATIVE_SAVED: Mutex<Option<(usize, bool)>> = Mutex::new(None);

// SDL owns relative input. Run its UI API on the window thread, never Present.
unsafe fn native_menu_cursor(open: bool) {
    let export = |name: &'static [u8]| {
        wallbang_core::modules::export("SDL3.dll", std::ffi::CStr::from_bytes_with_nul(name).unwrap())
    };
    if let (Some(focus), Some(get), Some(set)) = (
        export(b"SDL_GetMouseFocus\0"), export(b"SDL_GetWindowRelativeMouseMode\0"),
        export(b"SDL_SetWindowRelativeMouseMode\0")) {
        let focus: unsafe extern "C" fn() -> usize = std::mem::transmute(focus);
        let get: unsafe extern "C" fn(usize) -> bool = std::mem::transmute(get);
        let set: unsafe extern "C" fn(usize, bool) -> bool = std::mem::transmute(set);
        let mut saved = SDL_RELATIVE_SAVED.lock();
        if open {
            let window = saved.as_ref().map(|v| v.0).unwrap_or_else(|| focus());
            if window != 0 {
                if saved.is_none() { *saved = Some((window, get(window))); }
                set(window, false);
            }
        } else if let Some((window, relative)) = saved.take() { set(window, relative); }
    }
    if open {
        ClipCursor(std::ptr::null());
        SetCursor(LoadCursorW(std::ptr::null_mut(), 32512usize as *const u16));
        let mut info = CursorInfo { size: std::mem::size_of::<CursorInfo>() as u32,
            flags: 0, cursor: std::ptr::null_mut(), position: [0, 0] };
        if GetCursorInfo(&mut info) != 0 && info.flags & CURSOR_SHOWING == 0 {
            for _ in 0..16 { if ShowCursor(1) >= 0 { break; } }
        }
    }
}

#[repr(C)]
struct CursorInfo {
    size: u32,
    flags: u32,
    cursor: *mut c_void,
    position: [i32; 2],
}

const CURSOR_SHOWING: u32 = 0x0000_0001;

const PRESENT_SLOT: usize = 8;
static MENU_CLICK_PENDING: AtomicBool = AtomicBool::new(false);

pub fn take_menu_click() -> bool {
    MENU_CLICK_PENDING.swap(false, Ordering::Relaxed)
}
pub fn has_menu_input() -> bool {
    WNDPROC_ORIG.load(Ordering::Relaxed) != 0
}

/// UI geometry is in backbuffer pixels, GetCursorPos is in desktop pixels.
pub fn menu_pointer(width: f32, height: f32) -> Option<wallbang_core::math::Vec2> {
    let mut point = wallbang_core::input::cursor_position()?;
    let window = WNDPROC_HWND.load(Ordering::Relaxed) as *mut c_void;
    if window.is_null() { return None; }
    let mut rect = [0i32; 4];
    unsafe {
        if ScreenToClient(window, &mut point) == 0 || GetClientRect(window, &mut rect) == 0 { return None; }
    }
    pointer_to_backbuffer(point.x as f32, point.y as f32,
        (rect[2] - rect[0]) as f32, (rect[3] - rect[1]) as f32, width, height)
}

fn pointer_to_backbuffer(x: f32, y: f32, client_w: f32, client_h: f32,
    width: f32, height: f32) -> Option<wallbang_core::math::Vec2> {
    if ![x,y,client_w,client_h,width,height].iter().all(|v| v.is_finite()) ||
        client_w <= 0.0 || client_h <= 0.0 || width <= 0.0 || height <= 0.0 { return None; }
    Some(wallbang_core::math::Vec2::new(x * width / client_w, y * height / client_h))
}

#[cfg(test)]
mod pointer_tests {
    use super::pointer_to_backbuffer;

    #[test]
    fn maps_client_pixels_to_render_resolution() {
        let point = pointer_to_backbuffer(640.0, 360.0, 1280.0, 720.0, 1920.0, 1080.0).unwrap();
        assert_eq!((point.x, point.y), (960.0, 540.0));
        let outside = pointer_to_backbuffer(-20.0, 0.0, 1280.0, 720.0, 1920.0, 1080.0).unwrap();
        assert_eq!(outside.x, -30.0);
    }

    #[test]
    fn rejects_minimized_and_invalid_surfaces() {
        assert!(pointer_to_backbuffer(0.0, 0.0, 0.0, 720.0, 1920.0, 1080.0).is_none());
        assert!(pointer_to_backbuffer(f32::NAN, 0.0, 1280.0, 720.0, 1920.0, 1080.0).is_none());
    }
}

static DX: Mutex<Option<Dx11>> = Mutex::new(None);
static MENU: Mutex<Option<Menu>> = Mutex::new(None);
static DRAW: Mutex<Option<DrawList>> = Mutex::new(None);
static ESP_OVERLAY: Mutex<Option<DrawList>> = Mutex::new(None);
static PRESENT_HOOK: Mutex<Option<VTableHook>> = Mutex::new(None);
static OVERLAY: Mutex<Option<OverlayFn>> = Mutex::new(None);
static FRAME: Mutex<Option<fn()>> = Mutex::new(None);
static TRACE: Mutex<Option<fn(&str)>> = Mutex::new(None);
static PRESENT_TRACES: AtomicU32 = AtomicU32::new(0);
static WNDPROC_ORIG: AtomicIsize = AtomicIsize::new(0);
static WNDPROC_HWND: AtomicIsize = AtomicIsize::new(0);
static WHEEL_DELTA: AtomicI32 = AtomicI32::new(0);
// Diagnostic timing, one averaged line per 600 Present calls.
static HOOK_US: AtomicU64 = AtomicU64::new(0);
static REFRESH_US: AtomicU64 = AtomicU64::new(0);
static ESP_US: AtomicU64 = AtomicU64::new(0);
static MENU_US: AtomicU64 = AtomicU64::new(0);
static MODEL_US: AtomicU64 = AtomicU64::new(0);
static PREVIEW_US: AtomicU64 = AtomicU64::new(0);
static STAT_FRAMES: AtomicU32 = AtomicU32::new(0);
static FRAME_TIMING: Mutex<Option<(std::time::Instant, f32)>> = Mutex::new(None);

pub fn frame_fps() -> f32 {
    FRAME_TIMING.lock().as_ref().map(|(_, fps)| *fps).unwrap_or(0.0)
}

pub type OverlayFn = fn(&mut DrawList, &wallbang_core::state::GameState);

/// Consumes accumulated WM_MOUSEWHEEL notches for this frame.
pub(crate) fn take_wheel() -> f32 {
    WHEEL_DELTA.swap(0, Ordering::Relaxed) as f32
}

/// Subclasses the game window so the menu can scroll its child panels and so
/// mouse clicks on the menu never reach the game (same as the C++ WndProc hook).
unsafe fn install_wndproc(swapchain: &IDXGISwapChain) {
    if WNDPROC_ORIG.load(Ordering::Relaxed) != 0 {
        return;
    }
    let Ok(desc) = swapchain.GetDesc() else {
        return;
    };
    let hwnd = desc.OutputWindow;
    if hwnd.0.is_null() {
        return;
    }
    let old = SetWindowLongPtrW(
        hwnd,
        GWLP_WNDPROC,
        hk_wndproc as *const () as usize as isize,
    );
    if old != 0 {
        WNDPROC_ORIG.store(old, Ordering::Relaxed);
        WNDPROC_HWND.store(hwnd.0 as isize, Ordering::Relaxed);
        crate::trace("wndproc: subclass installed");
    }
}

unsafe extern "system" fn hk_wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == MENU_CURSOR_MESSAGE {
        native_menu_cursor(crate::menu::is_menu_open());
        return LRESULT(0);
    }
    if msg == 0x0020 && crate::menu::is_menu_open() {
        native_menu_cursor(true);
        return LRESULT(1);
    }
    const WM_MOUSEWHEEL: u32 = 0x020A;
    const WM_INPUT: u32 = 0x00FF;
    if msg == 0x0201 && crate::menu::is_menu_open() {
        MENU_CLICK_PENDING.store(true, Ordering::Relaxed);
    }
    if msg == WM_MOUSEWHEEL {
        let delta = ((wparam.0 >> 16) as u16) as i16 as i32;
        WHEEL_DELTA.fetch_add(delta, Ordering::Relaxed);
    }
    // While the menu is open the game must not see mouse input at all. Raw input
    // arrives as WM_INPUT, so blocking only WM_MOUSEMOVE left the camera turning.
    if crate::menu::is_menu_open()
        && (msg == WM_INPUT || (0x0200..=0x0210).contains(&msg))
    {
        return LRESULT(0);
    }
    let old = WNDPROC_ORIG.load(Ordering::Relaxed);
    if old != 0 {
        type Proc = unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT;
        let proc: Proc = std::mem::transmute(old);
        CallWindowProcW(Some(proc), hwnd, msg, wparam, lparam)
    } else {
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}

/// On menu open: release the game cursor clip and make the OS cursor visible.
/// On close: hide it again so the game returns to its raw-input look.
pub fn update_menu_cursor(open: bool) {
    static MENU_CURSOR: AtomicBool = AtomicBool::new(false);
    if MENU_CURSOR.swap(open, Ordering::Relaxed) == open {
        return;
    }
    if !open { MENU_CLICK_PENDING.store(false, Ordering::Relaxed); }
    enforce_menu_cursor();
}

/// Must run every frame while the menu is open: the game re-applies
/// `ClipCursor(RECT)` and hides the cursor from its own CreateMove/input code,
/// so a one-shot release is not enough to keep the menu usable.
pub fn enforce_menu_cursor() {
    let hwnd = WNDPROC_HWND.load(Ordering::Relaxed) as *mut c_void;
    if !hwnd.is_null() {
        unsafe { PostMessageW(hwnd, MENU_CURSOR_MESSAGE, 0, 0); }
    }
}

pub fn set_trace(callback: fn(&str)) {
    *TRACE.lock() = Some(callback);
}

pub(crate) fn trace(message: &str) {
    if let Some(callback) = *TRACE.lock() {
        callback(message);
    }
}

fn trace_present(message: &str) {
    let n = PRESENT_TRACES.fetch_add(1, Ordering::Relaxed);
    if n < 24 {
        trace(&format!("present {n}: {message}"));
    }
}

pub fn set_overlay(callback: OverlayFn) {
    *OVERLAY.lock() = Some(callback);
}

pub fn set_frame_callback(callback: fn()) {
    *FRAME.lock() = Some(callback);
}

unsafe extern "system" fn dummy_wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    DefWindowProcW(hwnd, msg, wparam, lparam)
}

/// Swapchain только для получения адреса vtable Present. Он не презентуется и
/// не рисуется — нужен исключительно как «образец» IDXGISwapChain.
unsafe fn create_vtable_probe() -> Result<IDXGISwapChain, String> {
    let class = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(dummy_wndproc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: Default::default(),
        hIcon: Default::default(),
        hCursor: Default::default(),
        hbrBackground: Default::default(),
        lpszMenuName: windows::core::PCWSTR::null(),
        lpszClassName: w!("WallbangVtableProbe"),
        hIconSm: Default::default(),
    };
    let instance = HINSTANCE(GetModuleHandleA(std::ptr::null()));
    let class = WNDCLASSEXW { hInstance: instance, ..class };
    RegisterClassExW(&class);
    let hwnd = CreateWindowExW(
        WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST | WS_EX_NOACTIVATE,
        w!("WallbangVtableProbe"),
        w!("WallbangProbe"),
        WS_POPUP,
        0,
        0,
        1,
        1,
        None,
        None,
        Some(instance),
        None,
    )
    .map_err(|error| format!("probe window failed: {error:?}"))?;

    let desc = DXGI_SWAP_CHAIN_DESC {
        BufferDesc: DXGI_MODE_DESC {
            Width: 1,
            Height: 1,
            RefreshRate: DXGI_RATIONAL { Numerator: 60, Denominator: 1 },
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            ScanlineOrdering: Default::default(),
            Scaling: Default::default(),
        },
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        BufferCount: 2,
        OutputWindow: hwnd,
        Windowed: true.into(),
        SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
        Flags: 0,
    };
    let mut swapchain = None;
    let mut device = None;
    let mut context = None;
    let mut feature = D3D_FEATURE_LEVEL_11_0;
    D3D11CreateDeviceAndSwapChain(
        None::<&IDXGIAdapter>,
        D3D_DRIVER_TYPE_HARDWARE,
        windows::Win32::Foundation::HMODULE::default(),
        D3D11_CREATE_DEVICE_FLAG(0),
        Some(&[D3D_FEATURE_LEVEL_11_0]),
        D3D11_SDK_VERSION,
        Some(&desc),
        Some(&mut swapchain),
        Some(&mut device),
        Some(&mut feature),
        Some(&mut context),
    )
    .map_err(|error| format!("probe swapchain failed: {error:?}"))?;
    swapchain.ok_or_else(|| "probe swapchain missing".to_string())
}

unsafe extern "system" fn hk_present(swapchain: *mut c_void, sync: u32, flags: u32) -> i32 {
    type PresentFn = unsafe extern "system" fn(*mut c_void, u32, u32) -> i32;
    let original = PRESENT_HOOK.lock().as_ref().map(|h| h.original());
    let now = std::time::Instant::now();
    {
        let mut timing = FRAME_TIMING.lock();
        match timing.as_mut() {
            Some((last, rolling)) => {
                let dt = now.saturating_duration_since(*last).as_secs_f32();
                if dt > 0.000_1 && dt < 1.0 {
                    let sample = 1.0 / dt;
                    *rolling = if *rolling > 0.0 { *rolling * 0.9 + sample * 0.1 } else { sample };
                }
                *last = now;
            }
            None => *timing = Some((now, 0.0)),
        }
    }

    if !swapchain.is_null() {
        if let Some(sc) = IDXGISwapChain::from_raw_borrowed(&swapchain) {
            install_wndproc(sc);
            let mut dx = DX.lock();
            if dx.is_none() {
                *dx = Dx11::new(sc);
                if dx.is_none() {
                    trace_present("Dx11::new failed");
                }
            }
            if let Some(renderer) = dx.as_mut() {
                let hook_start = std::time::Instant::now();
                // После ResizeBuffers старый RTV недействителен — пересоздаём.
                renderer.recreate(sc);
                let preview_rect = crate::native_preview::last_rect();
                if let Some(rect) = preview_rect {
                    let ok = renderer.capture_preview(sc, rect);
                    crate::native_preview::report_capture(ok);
                }
                let (w, h) = (renderer.width as f32, renderer.height as f32);
                let state = wallbang_core::state();
                // Проекция ESP обязана идти по размеру backbuffer'а, а не по
                // размеру рабочего стола: GetSystemMetrics даёт desktop, и при
                // неродном разрешении/растяжке кадр «уезжает».
                wallbang_core::state::set_viewport(renderer.width, renderer.height);
                // Снимок мира занимает ~4us; обновляем каждый кадр, чтобы
                // view_matrix для ESP не отставал на 60 Гц при 200+ FPS.
                let t = std::time::Instant::now();
                state.refresh();
                REFRESH_US.fetch_add(t.elapsed().as_micros() as u64, Ordering::Relaxed);
                let mut list_guard = DRAW.lock();
                let list = list_guard.get_or_insert_with(DrawList::new);
                list.begin(w, h);
                if let Some(frame) = *FRAME.lock() {
                    frame();
                }
                let t = std::time::Instant::now();
                if let Some(overlay) = *OVERLAY.lock() {
                    overlay(list, &state);
                }
                ESP_US.fetch_add(t.elapsed().as_micros() as u64, Ordering::Relaxed);
                let t = std::time::Instant::now();
                let mut menu_guard = MENU.lock();
                let menu = menu_guard.get_or_insert_with(Menu::new);
                menu.frame(list, &state, w, h);
                MENU_US.fetch_add(t.elapsed().as_micros() as u64, Ordering::Relaxed);
                // Request is produced by this frame's menu. Closing the menu
                // clears it before any preview draw, without an engine heartbeat.
                let inspect = crate::skin_inspect::request().and_then(|request| {
                    renderer.render_weapon(&request).map(|srv| (request.rect, srv))
                });
                crate::skin_inspect::report_capture(inspect.is_some());
                let model_request = crate::preview::model_request();
                let t = std::time::Instant::now();
                let model_srv = if model_request.active {
                    renderer.render_model(
                        model_request.rect,
                        model_request.clock,
                        model_request.enemy,
                        model_request.material,
                    )
                } else {
                    None
                };
                MODEL_US.fetch_add(t.elapsed().as_micros() as u64, Ordering::Relaxed);
                let pp_cfg = state.settings().postprocess;
                renderer.apply_postprocess(swapchain, &pp_cfg);
                renderer.draw(list.vertices());
                renderer.draw_music_art();
                if let Some((rect, srv)) = inspect.as_ref() {
                    renderer.draw_weapon_preview((rect[0],rect[1],rect[2],rect[3]), srv);
                }
                renderer.draw_inventory_icons(list.inventory_icons());
                let t = std::time::Instant::now();
                if let Some(srv) = model_srv.as_ref() {
                    renderer.draw_preview(model_request.rect, Some(srv));
                } else if let Some(rect) = preview_rect {
                    renderer.draw_preview(rect, None);
                }
                if model_srv.is_some() && model_request.show_esp {
                    let mut overlay_guard = ESP_OVERLAY.lock();
                    let overlay = overlay_guard.get_or_insert_with(DrawList::new);
                    overlay.begin(w, h);
                    crate::preview::draw_esp_overlay(overlay, model_request.rect);
                    renderer.draw(overlay.vertices());
                }
                if !list.inventory_icons().is_empty() || inspect.is_some() {
                    renderer.draw(list.flushed_top_vertices());
                }
                PREVIEW_US.fetch_add(t.elapsed().as_micros() as u64, Ordering::Relaxed);
                HOOK_US.fetch_add(
                    hook_start.elapsed().as_micros() as u64,
                    Ordering::Relaxed,
                );
                if STAT_FRAMES.fetch_add(1, Ordering::Relaxed) >= 599 {
                    let n = STAT_FRAMES.swap(0, Ordering::Relaxed).max(1) as u64;
                    let hook = HOOK_US.swap(0, Ordering::Relaxed) / n;
                    let refresh = REFRESH_US.swap(0, Ordering::Relaxed) / n;
                    let esp = ESP_US.swap(0, Ordering::Relaxed) / n;
                    let menu = MENU_US.swap(0, Ordering::Relaxed) / n;
                    let model = MODEL_US.swap(0, Ordering::Relaxed) / n;
                    let preview = PREVIEW_US.swap(0, Ordering::Relaxed) / n;
                    let profile = wallbang_core::state::profile_summary();
                    let (inspect_avg, inspect_max) = crate::skin_inspect::take_profile();
                    crate::trace(&format!(
                        "timing: hook={hook}us refresh={refresh}us esp={esp}us menu={menu}us model={model}us preview={preview}us inspect_dx11={inspect_avg}us inspect_max={inspect_max}us | {profile}"
                    ));
                }
            }
        }
    }

    match original {
        Some(addr) => {
            let call: PresentFn = std::mem::transmute(addr);
            call(swapchain, sync, flags)
        }
        None => 0,
    }
}

pub unsafe fn install() -> Result<(), String> {
    trace("render::install: enter");
    if PRESENT_HOOK.lock().is_some() {
        trace("render::install: already installed");
        return Ok(());
    }
    trace("render::install: DrawList::new");
    *DRAW.lock() = Some(DrawList::new());
    *ESP_OVERLAY.lock() = Some(DrawList::new());
    trace("render::install: Menu::new");
    *MENU.lock() = Some(Menu::new());
    trace("render::install: vtable probe");
    let probe = create_vtable_probe()?;
    let raw = probe.as_raw() as usize;
    trace(&format!("render::install: probe swapchain=0x{raw:X}"));
    let hook = VTableHook::install(raw, PRESENT_SLOT, hk_present as *const () as usize)
        .ok_or_else(|| "Present vtable hook failed".to_string())?;
    trace(&format!(
        "render::install: Present hooked, original=0x{:X}",
        hook.original()
    ));
    *PRESENT_HOOK.lock() = Some(hook);
    // Probe больше не нужен: vtable находится в dxgi.dll и остаётся пропатченной.
    drop(probe);
    trace("render::install: done");
    Ok(())
}

pub unsafe fn shutdown() {
    if let Some(hook) = PRESENT_HOOK.lock().take() {
        hook.restore();
    }
    let hwnd = WNDPROC_HWND.swap(0, Ordering::Relaxed);
    let old = WNDPROC_ORIG.swap(0, Ordering::Relaxed);
    if hwnd != 0 && old != 0 {
        SetWindowLongPtrW(HWND(hwnd as *mut c_void), GWLP_WNDPROC, old);
    }
    DX.lock().take();
    MENU.lock().take();
    DRAW.lock().take();
    ESP_OVERLAY.lock().take();
}
