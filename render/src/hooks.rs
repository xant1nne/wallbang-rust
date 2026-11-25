// Config block: Present=8 в IDXGISwapChain vtable. Оверлей рисуется прямо в
// swapchain игры (как старый xdraw в on_present), в потоке самого Present.
// Отдельное layered-окно убрано: оно давало чёрный экран поверх игры.
use crate::draw::DrawList;
use crate::dx11::Dx11;
use crate::menu::Menu;
use parking_lot::Mutex;
use std::ffi::c_void;
use std::sync::atomic::{AtomicI32, AtomicIsize, AtomicU32, AtomicU64, Ordering};
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
    fn GetTickCount64() -> u64;
}

const PRESENT_SLOT: usize = 8;

static DX: Mutex<Option<Dx11>> = Mutex::new(None);
static MENU: Mutex<Option<Menu>> = Mutex::new(None);
static DRAW: Mutex<Option<DrawList>> = Mutex::new(None);
static ESP_OVERLAY: Mutex<Option<DrawList>> = Mutex::new(None);
static PRESENT_HOOK: Mutex<Option<VTableHook>> = Mutex::new(None);
static OVERLAY: Mutex<Option<OverlayFn>> = Mutex::new(None);
static FRAME: Mutex<Option<fn()>> = Mutex::new(None);
static TRACE: Mutex<Option<fn(&str)>> = Mutex::new(None);
static PRESENT_TRACES: AtomicU32 = AtomicU32::new(0);
static LAST_REFRESH_MS: AtomicU64 = AtomicU64::new(0);
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
    const WM_MOUSEWHEEL: u32 = 0x020A;
    if msg == WM_MOUSEWHEEL {
        let delta = ((wparam.0 >> 16) as u16) as i16 as i32;
        WHEEL_DELTA.fetch_add(delta, Ordering::Relaxed);
    }
    // While the menu is open the game must not see mouse input at all.
    if crate::menu::is_menu_open() && (0x0200..=0x020E).contains(&msg) {
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
                // Снимок мира не обязан обновляться на каждый Present: при 300+ FPS
                // это лишний проход по entity list и лишние локи.
                let now = GetTickCount64();
                let last = LAST_REFRESH_MS.load(Ordering::Relaxed);
                if now.saturating_sub(last) >= 16 {
                    LAST_REFRESH_MS.store(now, Ordering::Relaxed);
                    let t = std::time::Instant::now();
                    state.refresh();
                    REFRESH_US.fetch_add(t.elapsed().as_micros() as u64, Ordering::Relaxed);
                }
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
                renderer.draw(list.vertices());
                let t = std::time::Instant::now();
                if let Some(srv) = model_srv.as_ref() {
                    renderer.draw_preview(model_request.rect, Some(srv));
                } else if let Some(rect) = preview_rect {
                    renderer.draw_preview(rect, None);
                }
                if model_srv.is_some() {
                    let mut overlay_guard = ESP_OVERLAY.lock();
                    let overlay = overlay_guard.get_or_insert_with(DrawList::new);
                    overlay.begin(w, h);
                    crate::preview::draw_esp_overlay(overlay, model_request.rect);
                    renderer.draw(overlay.vertices());
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
                    crate::trace(&format!(
                        "timing: hook={hook}us refresh={refresh}us esp={esp}us menu={menu}us model={model}us preview={preview}us | {profile}"
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
