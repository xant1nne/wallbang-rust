// Config block: WINDOW_W/H, TICK_SLEEP_MS, CS2_PATH, CLIENT_WAIT_MS, palette.
// Frameless borderless GUI loader in the wallbang menu style: Win32 popup window
// (no OS frame), own D3D11 swapchain, drawn with the same DrawList as the game
// menu. LOAD runs the manual-map pipeline on a worker thread.
#![windows_subsystem = "windows"]

use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use wallbang_loader::{entry_call, find_process, map};
use wallbang_render::dx11::Dx11;
use wallbang_render::DrawList;

use windows::core::w;
use windows::Win32::Foundation::{COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL_11_0};
use windows::Win32::Graphics::Direct3D11::{
    D3D11CreateDeviceAndSwapChain, D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION,
};
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_UNKNOWN, DXGI_MODE_DESC, DXGI_RATIONAL,
    DXGI_SAMPLE_DESC,
};
use windows::Win32::Graphics::Dxgi::{
    IDXGIAdapter, IDXGISwapChain, DXGI_PRESENT, DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_CHAIN_FLAG,
    DXGI_SWAP_EFFECT_DISCARD, DXGI_USAGE_RENDER_TARGET_OUTPUT,
};
use windows::Win32::Graphics::Gdi::{CreateSolidBrush, ScreenToClient};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetClientRect,
    GetSystemMetrics, PeekMessageW, RegisterClassExW, SetProcessDPIAware, ShowWindow,
    TranslateMessage, HTCLIENT, HTCAPTION, MSG, PM_REMOVE, SW_SHOW, WINDOW_EX_STYLE, WINDOW_STYLE,
    WNDCLASSEXW, WM_DESTROY, WM_ERASEBKGND, WM_KEYDOWN, WM_LBUTTONDOWN, WM_NCHITTEST, WM_QUIT,
    WM_SIZE, WS_POPUP,
};

const WINDOW_W: i32 = 400;
const WINDOW_H: i32 = 380;
const TICK_SLEEP_MS: u64 = 8;
const CLIENT_WAIT_MS: u32 = 120_000;
const CS2_PATH: &str =
    r"C:\Program Files (x86)\Steam\steamapps\common\Counter-Strike Global Offensive\game\bin\win64\cs2.exe";
const VK_ESCAPE: usize = 0x1B;

const BG: [u8; 4] = [8, 9, 11, 255];
const FRAME: [u8; 4] = [96, 104, 118, 170];
const LOGO_BOX: [u8; 4] = [0, 0, 0, 255];
const TEXT: [u8; 4] = [235, 238, 244, 255];
const TEXT_DIM: [u8; 4] = [112, 118, 130, 255];
const TEXT_FAINT: [u8; 4] = [70, 74, 84, 255];
const ACCENT: [u8; 4] = [172, 210, 238, 255];
const BTN: [u8; 4] = [14, 15, 18, 255];
const OK: [u8; 4] = [126, 224, 156, 255];
const ERR: [u8; 4] = [232, 116, 108, 255];
const BG_COLORREF: COLORREF = COLORREF((BG[2] as u32) << 16 | (BG[1] as u32) << 8 | BG[0] as u32);

/// DLL payload is embedded at build time so the resulting `wallbang.exe` is
/// fully self-contained — no external files are required next to it.
const EMBEDDED_DLL: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../target/release/wallbang.dll"
));

fn dll_bytes() -> Result<Vec<u8>, String> {
    Ok(EMBEDDED_DLL.to_vec())
}

static STATUS: OnceLock<Mutex<String>> = OnceLock::new();
static BUSY: AtomicBool = AtomicBool::new(false);
static DONE: AtomicBool = AtomicBool::new(false);
static FAILED: AtomicBool = AtomicBool::new(false);
static PROGRESS: AtomicI32 = AtomicI32::new(0);
static ELAPSED_MS: AtomicI32 = AtomicI32::new(0);
static CLICK: AtomicBool = AtomicBool::new(false);
static CLIENT_W: AtomicI32 = AtomicI32::new(WINDOW_W);
static CLIENT_H: AtomicI32 = AtomicI32::new(WINDOW_H);
static RESIZED: AtomicBool = AtomicBool::new(false);
static STARTED: OnceLock<Instant> = OnceLock::new();

fn status() -> &'static Mutex<String> {
    STATUS.get_or_init(|| Mutex::new("Ready to load".to_owned()))
}

fn set_status(text: &str) {
    if let Ok(mut guard) = status().lock() {
        *guard = text.to_owned();
    }
}

#[inline]
fn button_rect(width: f32, height: f32) -> (f32, f32, f32, f32) {
    let w = 148.0;
    let h = 34.0;
    (width - 26.0 - w, height - 22.0 - h, w, h)
}

unsafe fn create_swapchain(hwnd: HWND, width: u32, height: u32) -> Option<IDXGISwapChain> {
    let desc = DXGI_SWAP_CHAIN_DESC {
        BufferDesc: DXGI_MODE_DESC {
            Width: width,
            Height: height,
            RefreshRate: DXGI_RATIONAL {
                Numerator: 60,
                Denominator: 1,
            },
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            ScanlineOrdering: Default::default(),
            Scaling: Default::default(),
        },
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
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
    .ok()?;
    swapchain
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_NCHITTEST => {
            // Frameless window: everything drags except the LOAD button.
            let mut point = POINT {
                x: (lparam.0 & 0xFFFF) as u16 as i16 as i32,
                y: ((lparam.0 >> 16) & 0xFFFF) as u16 as i16 as i32,
            };
            let _ = ScreenToClient(hwnd, &mut point);
            let width = CLIENT_W.load(Ordering::Relaxed) as f32;
            let height = CLIENT_H.load(Ordering::Relaxed) as f32;
            let (bx, by, bw, bh) = button_rect(width, height);
            let inside_button = point.x as f32 >= bx
                && point.x as f32 <= bx + bw
                && point.y as f32 >= by
                && point.y as f32 <= by + bh;
            return LRESULT(if inside_button {
                HTCLIENT as isize
            } else {
                HTCAPTION as isize
            });
        }
        WM_LBUTTONDOWN => {
            CLICK.store(true, Ordering::Relaxed);
            LRESULT(0)
        }
        WM_KEYDOWN => {
            if wparam.0 == VK_ESCAPE {
                let _ = DestroyWindow(hwnd);
            }
            LRESULT(0)
        }
        WM_SIZE => {
            let width = (lparam.0 & 0xFFFF) as u16 as i32;
            let height = ((lparam.0 >> 16) & 0xFFFF) as u16 as i16 as i32;
            if width > 0 && height > 0 {
                CLIENT_W.store(width, Ordering::Relaxed);
                CLIENT_H.store(height, Ordering::Relaxed);
                RESIZED.store(true, Ordering::Relaxed);
            }
            LRESULT(0)
        }
        WM_ERASEBKGND => LRESULT(1),
        WM_DESTROY => {
            windows::Win32::UI::WindowsAndMessaging::PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe fn create_window() -> Result<HWND, String> {
    let _ = SetProcessDPIAware();
    let instance = HINSTANCE(GetModuleHandleW(None).map_err(|_| "module handle")?.0);
    let class = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        lpfnWndProc: Some(wndproc),
        hInstance: instance,
        hbrBackground: CreateSolidBrush(BG_COLORREF),
        lpszClassName: w!("WallbangGuiLoader"),
        ..Default::default()
    };
    RegisterClassExW(&class);
    let screen_w = GetSystemMetrics(windows::Win32::UI::WindowsAndMessaging::SM_CXSCREEN);
    let screen_h = GetSystemMetrics(windows::Win32::UI::WindowsAndMessaging::SM_CYSCREEN);
    let hwnd = CreateWindowExW(
        WINDOW_EX_STYLE(0),
        w!("WallbangGuiLoader"),
        w!("WALLBANG"),
        WINDOW_STYLE(WS_POPUP.0),
        (screen_w - WINDOW_W) / 2,
        (screen_h - WINDOW_H) / 2,
        WINDOW_W,
        WINDOW_H,
        None,
        None,
        Some(instance),
        None,
    )
    .map_err(|error| format!("CreateWindowExW: {error:?}"))?;
    let _ = ShowWindow(hwnd, SW_SHOW);
    let mut rect = Default::default();
    let _ = GetClientRect(hwnd, &mut rect);
    let (width, height) = (rect.right - rect.left, rect.bottom - rect.top);
    if width > 0 && height > 0 {
        CLIENT_W.store(width, Ordering::Relaxed);
        CLIENT_H.store(height, Ordering::Relaxed);
    }
    Ok(hwnd)
}

fn start_injection() {
    if BUSY.swap(true, Ordering::SeqCst) {
        return;
    }
    DONE.store(false, Ordering::Relaxed);
    FAILED.store(false, Ordering::Relaxed);
    PROGRESS.store(2, Ordering::Relaxed);
    set_status("Starting...");
    thread::spawn(|| {
        let result = run_injection();
        match result {
            Ok(()) => {
                PROGRESS.store(100, Ordering::Relaxed);
                DONE.store(true, Ordering::Relaxed);
                set_status("Injected. Good luck.");
            }
            Err(error) => {
                PROGRESS.store(0, Ordering::Relaxed);
                FAILED.store(true, Ordering::Relaxed);
                set_status(&format!("Failed: {error}"));
            }
        }
        BUSY.store(false, Ordering::SeqCst);
    });
}

fn run_injection() -> Result<(), String> {
    let pid = match find_process::wait_for_process("cs2.exe", 0) {
        Some(pid) => {
            PROGRESS.store(25, Ordering::Relaxed);
            set_status("CS2 already running - attaching");
            pid
        }
        None => {
            PROGRESS.store(10, Ordering::Relaxed);
            set_status("Launching CS2...");
            find_process::launch_cs2(Path::new(CS2_PATH), false)
                .map_err(|error| format!("launch: {error}"))?
        }
    };
    PROGRESS.store(35, Ordering::Relaxed);
    set_status("Waiting for client.dll...");
    if !find_process::wait_for_module(pid, "client.dll", CLIENT_WAIT_MS) {
        return Err("client.dll not loaded".to_owned());
    }
    PROGRESS.store(62, Ordering::Relaxed);
    set_status("Mapping wallbang.dll...");
    let bytes = dll_bytes()?;
    let handle = find_process::process_handle(pid);
    if handle.0.is_null() {
        return Err("OpenProcess failed".to_owned());
    }
    let image = unsafe { map::map_image(handle, &bytes) }
        .map_err(|error| format!("manual map: {error}"))?;
    PROGRESS.store(85, Ordering::Relaxed);
    let export = unsafe { map::find_export_rva(&bytes, "wb_entry") }
        .ok_or_else(|| "wb_entry export missing".to_owned())?;
    let entry = image.base as usize + export as usize;
    set_status("Calling wb_entry...");
    unsafe { entry_call::call_remote_entry(handle, entry, image.base) }
        .map_err(|error| format!("entry call: {error}"))
}

fn draw_frame(dl: &mut DrawList, width: f32, height: f32) {
    dl.begin(width, height);
    dl.rect(0.0, 0.0, width, height, BG);

    // Thin painted frame replaces the OS window border.
    dl.outline(6.0, 6.0, width - 12.0, height - 12.0, 1.0, FRAME);

    // Header: black logo tile + brand wordmark.
    let logo_size = 64.0;
    let logo_x = 24.0;
    let logo_y = 24.0;
    dl.rect(logo_x, logo_y, logo_size, logo_size, LOGO_BOX);
    dl.draw_wallbang_logo(logo_x + 8.0, logo_y + 14.0, logo_size - 16.0, logo_size - 28.0);
    dl.text_brand(logo_x + logo_size + 20.0, logo_y + 10.0, "WALLBANG", TEXT, 1.55);
    dl.text(
        logo_x + logo_size + 20.0,
        logo_y + 46.0,
        "COUNTER-STRIKE 2 · MANUAL MAP",
        TEXT_FAINT,
        0.62,
    );

    // Tabs like the reference layout; RAGE is the active one.
    let tab_y = logo_y + logo_size + 30.0;
    let tabs = ["RAGE", "LEGIT"];
    let mut tab_x = width * 0.5 - 66.0;
    for (index, label) in tabs.iter().enumerate() {
        let active = index == 0;
        dl.text(tab_x, tab_y, label, if active { ACCENT } else { TEXT_FAINT }, 0.78);
        if active {
            dl.rect(tab_x, tab_y + 14.0, dl.text_width(label, 0.78), 1.5, ACCENT);
        }
        tab_x += 78.0;
    }

    // Big progress percentage + phase text.
    let progress = PROGRESS.load(Ordering::Relaxed).clamp(0, 100);
    let percent = format!("{progress}%");
    let scale = 2.2;
    let pw = dl.brand_text_width(&percent, scale);
    let ph = dl.brand_text_height(scale);
    let percent_y = (height * 0.44).floor();
    let color = if FAILED.load(Ordering::Relaxed) {
        ERR
    } else if DONE.load(Ordering::Relaxed) {
        OK
    } else {
        TEXT
    };
    dl.text_brand((width - pw) * 0.5, percent_y, &percent, color, scale);
    let status_text = status()
        .lock()
        .map(|guard| guard.clone())
        .unwrap_or_else(|_| "Ready".to_owned());
    let status_text = dl.truncate(&status_text, width - 60.0, 0.72);
    let sw = dl.text_width(&status_text, 0.72);
    dl.text(
        (width - sw) * 0.5,
        percent_y + ph + 8.0,
        &status_text,
        TEXT_DIM,
        0.72,
    );

    // Footer: elapsed timer left, LOAD button right.
    let elapsed_ms = ELAPSED_MS.load(Ordering::Relaxed);
    let secs = (elapsed_ms / 1000).max(0);
    let timer = format!("{:02}:{:02}", secs / 60, secs % 60);
    dl.text(26.0, height - 38.0, &timer, TEXT_FAINT, 0.76);

    let (bx, by, bw, bh) = button_rect(width, height);
    let busy = BUSY.load(Ordering::Relaxed);
    dl.rect(bx, by, bw, bh, BTN);
    dl.outline(bx, by, bw, bh, 1.0, if busy { TEXT_FAINT } else { ACCENT });
    let label = if busy {
        "LOADING..."
    } else if FAILED.load(Ordering::Relaxed) {
        "RETRY"
    } else if DONE.load(Ordering::Relaxed) {
        "RELOAD"
    } else {
        "LOAD"
    };
    let lw = dl.text_width(label, 0.92);
    dl.text(
        bx + (bw - lw) * 0.5,
        by + (bh - dl.text_height(0.92)) * 0.5,
        label,
        if busy { TEXT_DIM } else { TEXT },
        0.92,
    );
    if CLICK.swap(false, Ordering::Relaxed) && !busy {
        start_injection();
    }
}

fn main() {
    STARTED.get_or_init(Instant::now);
    unsafe {
        let hwnd = match create_window() {
            Ok(hwnd) => hwnd,
            Err(_) => return,
        };
        let width = CLIENT_W.load(Ordering::Relaxed).max(320) as u32;
        let height = CLIENT_H.load(Ordering::Relaxed).max(240) as u32;
        let Some(mut swapchain) = create_swapchain(hwnd, width, height) else {
            return;
        };
        let Some(mut renderer) = Dx11::new(&swapchain) else {
            return;
        };
        let mut draw_list = DrawList::new();
        let mut message = MSG::default();
        loop {
            while PeekMessageW(&mut message, None, 0, 0, PM_REMOVE).as_bool() {
                if message.message == WM_QUIT {
                    return;
                }
                let _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            }
            if RESIZED.swap(false, Ordering::Relaxed) {
                let width = CLIENT_W.load(Ordering::Relaxed).max(320) as u32;
                let height = CLIENT_H.load(Ordering::Relaxed).max(240) as u32;
                if swapchain
                    .ResizeBuffers(0, width, height, DXGI_FORMAT_UNKNOWN, DXGI_SWAP_CHAIN_FLAG(0))
                    .is_ok()
                {
                    renderer.recreate(&swapchain);
                }
            }
            ELAPSED_MS.store(STARTED.get().map(|s| s.elapsed().as_millis() as i32).unwrap_or(0), Ordering::Relaxed);
            let width = CLIENT_W.load(Ordering::Relaxed).max(320) as f32;
            let height = CLIENT_H.load(Ordering::Relaxed).max(240) as f32;
            draw_frame(&mut draw_list, width, height);
            renderer.draw(draw_list.vertices());
            let _ = swapchain.Present(1, DXGI_PRESENT(0));
            thread::sleep(Duration::from_millis(TICK_SLEEP_MS));
        }
    }
}
