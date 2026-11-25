// Config block: TICK_SLEEP_MS в ядре, MUTATIONS_ENABLED = false по умолчанию.
mod diag;

use parking_lot::Mutex;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::OnceLock;
use wallbang_config::constants::DEFAULT_PROFILE;
use wallbang_config::storage::{default_dir, Storage};
use wallbang_core::hooks::{parse_prefix, PrologueHook, VTableHook};
use wallbang_core::usercmd::UserCmd;
use wallbang_lua::Runtime;

const DLL_PROCESS_ATTACH: u32 = 1;
const DLL_PROCESS_DETACH: u32 = 0;
const WT_EXECUTEDEFAULT: u32 = 0;

static INITIALIZED: OnceLock<()> = OnceLock::new();
static LUA: Mutex<Option<Runtime>> = Mutex::new(None);
static CREATE_MOVE: Mutex<Option<PrologueHook>> = Mutex::new(None);
static CREATE_MOVE_TRAMPOLINE: AtomicUsize = AtomicUsize::new(0);
static FRAME_STAGE: Mutex<Option<VTableHook>> = Mutex::new(None);
static FRAME_STAGE_ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static FRAMES: AtomicU32 = AtomicU32::new(0);

type CreateMoveFn = unsafe extern "system" fn(*mut c_void, u32, *mut c_void) -> f64;
type FrameStageNotifyFn = unsafe extern "C" fn(usize, i32);

#[link(name = "kernel32")]
extern "system" {
    fn QueueUserWorkItem(
        callback: unsafe extern "system" fn(*mut c_void) -> u32,
        param: *mut c_void,
        flags: u32,
    ) -> i32;
    fn Sleep(milliseconds: u32);
}

unsafe extern "system" fn deferred_init(_: *mut c_void) -> u32 {
    let _ = initialize();
    0
}

/// Detour CCSGOInput::CreateMove: сначала оригинал (игра заполняет cmd), потом фичи.
unsafe extern "system" fn hk_create_move(this: *mut c_void, slot: u32, cmd: *mut c_void) -> f64 {
    let trampoline = CREATE_MOVE_TRAMPOLINE.load(Ordering::Relaxed);
    if trampoline == 0 {
        return 0.0;
    }
    let original: CreateMoveFn = std::mem::transmute(trampoline);
    let result = original(this, slot, cmd);
    if let Some(cmd) = UserCmd::from_raw(cmd as usize) {
        wallbang_feature::combat::cmd::on_create_move(cmd, &wallbang_core::state());
    }
    result
}

/// Panorama-скрипт отправляется из UI-потока: stage 6 в матче, любой stage в меню.
unsafe extern "C" fn hk_frame_stage_notify(this: usize, stage: i32) {
    if stage == 6 || wallbang_render::native_preview::wants_menu_tick() {
        wallbang_render::native_preview::tick();
    }
    let original = FRAME_STAGE_ORIGINAL.load(Ordering::Relaxed);
    if original != 0 {
        let original: FrameStageNotifyFn = std::mem::transmute(original);
        original(this, stage);
    }
}

fn install_frame_stage_notify() {
    let Some(instance) = wallbang_core::modules::create_interface("client.dll", "Source2Client002") else {
        diag::log("FrameStageNotify: Source2Client002 not found");
        return;
    };
    let sig = wallbang_core::pattern::scan_module(
        "client.dll",
        wallbang_config::signatures::SIG_FRAME_STAGE_NOTIFY,
    ).unwrap_or(0);
    let vtable = unsafe { wallbang_core::memory::read_ptr(instance) }.unwrap_or(0);
    let mut idx = usize::MAX;
    for i in 0..128 {
        if unsafe { wallbang_core::memory::read_ptr(vtable + i * 8) } == Some(sig) {
            idx = i;
            break;
        }
    }
    if idx == usize::MAX {
        diag::log(&format!(
            "FrameStageNotify: slot not found sig=0x{:X} vtable=0x{:X}",
            sig, vtable
        ));
        return;
    }
    let Some(hook) = (unsafe { VTableHook::install(instance, idx, hk_frame_stage_notify as *const () as usize) }) else {
        diag::log("FrameStageNotify: vtable hook failed");
        return;
    };
    diag::log(&format!(
        "FrameStageNotify: hooked slot={} original=0x{:X} sig=0x{:X}",
        idx,
        hook.original(),
        sig
    ));
    FRAME_STAGE_ORIGINAL.store(hook.original(), Ordering::SeqCst);
    *FRAME_STAGE.lock() = Some(hook);
}

/// Ставит inline-хук по сигнатуре; если байты пролога не совпали — тихо пропускает.
fn install_create_move() {
    let Some(address) = wallbang_core::pattern::scan_module(
        "client.dll",
        wallbang_config::signatures::SIG_CREATE_MOVE,
    ) else {
        diag::log("CreateMove: signature not found");
        return;
    };
    diag::log(&format!("CreateMove: 0x{address:X}"));
    let Some(prefix) = parse_prefix(wallbang_config::signatures::SIG_CREATE_MOVE_PROLOGUE) else {
        diag::log("CreateMove: bad prologue prefix");
        return;
    };
    if prefix.len() != wallbang_config::signatures::CREATE_MOVE_STEAL_BYTES {
        diag::log("CreateMove: wrong stolen byte count");
        return;
    }
    let detour = hk_create_move as *const () as usize;
    let Some(hook) = (unsafe { PrologueHook::install(address, detour, &prefix) }) else {
        diag::log("CreateMove: hook install failed");
        return;
    };
    diag::log(&format!("CreateMove: hooked, trampoline=0x{:X}", hook.trampoline()));
    CREATE_MOVE_TRAMPOLINE.store(hook.trampoline(), Ordering::SeqCst);
    *CREATE_MOVE.lock() = Some(hook);
}

fn log_signatures() {
    use wallbang_config::signatures as sig;
    let checks = [
        ("SIG_WORLD_SPACE_BONES", sig::SIG_WORLD_SPACE_BONES),
        ("SIG_CMD_SERIALIZE", sig::SIG_CMD_SERIALIZE),
        ("SIG_CMD_SET_MESSAGE_DATA", sig::SIG_CMD_SET_MESSAGE_DATA),
        ("SIG_CMD_WRITE_MESSAGE", sig::SIG_CMD_WRITE_MESSAGE),
        ("SIG_FRAME_STAGE_NOTIFY", sig::SIG_FRAME_STAGE_NOTIFY),
        ("SIG_OVERRIDE_VIEW", sig::SIG_OVERRIDE_VIEW),
        ("SIG_INPUT_SYSTEM_SVC", sig::SIG_INPUT_SYSTEM_SVC),
        ("SIG_HUD", sig::SIG_HUD),
        ("SIG_MAIN_MENU", sig::SIG_MAIN_MENU),
        ("SIG_PUI_ENGINE", sig::SIG_PUI_ENGINE),
    ];
    for (name, pattern) in checks {
        let module = if name == "SIG_INPUT_SYSTEM_SVC" { "inputsystem.dll" } else { "client.dll" };
        let found = wallbang_core::pattern::scan_module(module, pattern);
        diag::log(&format!(
            "SIG {name}: {}",
            found.map(|a| format!("0x{a:X}")).unwrap_or_else(|| "NOT FOUND".to_string())
        ));
    }
}

/// Точка входа для manual map: маппер вызывает её как LPTHREAD_START_ROUTINE(base).
#[no_mangle]
pub extern "system" fn wb_entry(_base: *mut c_void) -> u32 {
    diag::set_base(_base as usize);
    diag::log(&format!("wb_entry base=0x{:X}", _base as usize));
    if initialize().is_err() {
        return 1;
    }
    // Manual-map: выход из remote-потока запускает FLS/CRT-деструкторы Rust
    // и падает в ntdll с 0xC0000409. Держим поток живым, лоадер считает таймаут успехом.
    diag::log("wb_entry: parking remote thread");
    loop {
        if let Some(runtime) = LUA.lock().as_ref() {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0);
            let _ = runtime.tick(now);
        }
        // Тик Lua выполняется в том же потоке, где создавался LuaJIT, — без
        // отдельного Rust-потока и его TLS-деструкторов.
        unsafe { Sleep(16) };
    }
}

pub fn initialize() -> Result<(), String> {
    if INITIALIZED.set(()).is_err() {
        return Ok(());
    }
    diag::init();
    wallbang_render::set_trace(diag::log);
    wallbang_core::state::set_trace(diag::log);
    diag::install_exception_logger();
    diag::log("initialize: start");
    diag::log(&format!(
        "modules: client=0x{:X} engine2=0x{:X} inputsystem=0x{:X}",
        wallbang_core::modules::module_base("client.dll").unwrap_or(0),
        wallbang_core::modules::module_base("engine2.dll").unwrap_or(0),
        wallbang_core::modules::module_base("inputsystem.dll").unwrap_or(0),
    ));
    diag::log(&format!(
        "build={:?} offsets_resolved={} input_system=0x{:X} csgo_input=0x{:X}",
        wallbang_core::modules::build_number(),
        wallbang_config::offsets::is_resolved(),
        wallbang_core::modules::input_system().unwrap_or(0),
        wallbang_core::modules::csgo_input().unwrap_or(0),
    ));
    log_signatures();
    load_startup_profile();

    wallbang_core::mutation::set_allowed(false);
    if let Err(error) = unsafe { wallbang_render::init() } {
        diag::log(&format!("render init FAILED: {error}"));
        return Err(error);
    }
    diag::log("render init: ok (overlay thread started)");
    wallbang_render::native_preview::init();
    install_frame_stage_notify();
    wallbang_render::set_overlay(wallbang_feature::draw_esp);
    wallbang_render::set_frame_callback(on_frame);
    install_create_move();

    let state = wallbang_core::state();
    state.refresh();
    diag::log(&format!(
        "snapshot: valid={} players={}",
        state.snapshot().valid,
        state.snapshot().players.len()
    ));
    // Раньше здесь был отдельный Rust-поток с Lua-tick — он первым инициализировал
    // TLS-деструкторы в manual-map DLL и ронял процесс. Тик перенесён в wb_entry.
    diag::log("initialize: before Lua");

    match Runtime::new() {
        Ok(runtime) => {
            let scripts = {
                let dir = std::env::var_os("LOCALAPPDATA")
                    .map(|p| std::path::PathBuf::from(p).join("wallbang").join("lua").join("scripts"))
                    .unwrap_or_else(|| std::path::PathBuf::from("wallbang/lua/scripts"));
                std::fs::read_dir(dir)
                    .map(|entries| entries.filter_map(Result::ok).count())
                    .unwrap_or(0)
            };
            diag::log(&format!("LuaJIT runtime: ok, scripts={scripts}"));
            diag::log("initialize: before load_scripts");
            load_scripts(&runtime);
            diag::log("initialize: before LUA store");
            *LUA.lock() = Some(runtime);
            diag::log("initialize: after LUA store");
        }
        Err(error) => diag::log(&format!("LuaJIT runtime FAILED: {error}")),
    }
    diag::log("initialize: done");
    Ok(())
}

/// Loads `default.wb` on startup and creates it on first run.
fn load_startup_profile() {
    let storage = Storage::new(default_dir());
    let state = wallbang_core::state();
    if storage.exists(DEFAULT_PROFILE) {
        match storage.load(DEFAULT_PROFILE) {
            Ok(settings) => {
                *state.settings_mut() = settings;
                diag::log(&format!("config: loaded {DEFAULT_PROFILE}.wb"));
            }
            Err(error) => diag::log(&format!("config: load failed: {error}")),
        }
        return;
    }
    if let Err(error) = storage.ensure_dir() {
        diag::log(&format!("config: create dir failed: {error}"));
        return;
    }
    let mut settings = wallbang_config::Settings::default();
    settings.active_profile = DEFAULT_PROFILE.to_owned();
    if let Err(error) = storage.save(DEFAULT_PROFILE, &settings) {
        diag::log(&format!("config: create default failed: {error}"));
        return;
    }
    *state.settings_mut() = settings;
    diag::log(&format!("config: created {DEFAULT_PROFILE}.wb"));
}

/// Persists the active profile on explicit shutdown. Menu edits only touch
/// in-memory settings; the loader calls `wb_shutdown()` before unmapping.
fn save_active_profile() {
    let settings = wallbang_core::state().settings().clone();
    let name = if Storage::validate_name(&settings.active_profile) {
        settings.active_profile.clone()
    } else {
        DEFAULT_PROFILE.to_owned()
    };
    let storage = Storage::new(default_dir());
    match storage.save(&name, &settings) {
        Ok(()) => diag::log(&format!("config: saved {name}.wb")),
        Err(error) => diag::log(&format!("config: save failed: {error}")),
    }
}

fn on_frame() {
    let frame = FRAMES.fetch_add(1, Ordering::Relaxed);
    if frame == 0 || frame % 600 == 0 {
        let state = wallbang_core::state();
        diag::log(&format!(
            "frame {frame}: menu_open={} valid={} players={}",
            wallbang_render::is_menu_open(),
            state.snapshot().valid,
            state.snapshot().players.len()
        ));
    }
    // Пользовательский тумблер "allow writes" в Misc -> MutationGuard.
    let allowed = wallbang_core::state().settings().misc.allow_writes;
    wallbang_core::mutation::set_allowed(allowed);
    if let Some(runtime) = LUA.lock().as_ref() {
        let _ = runtime.fire("render", &[]);
    }
}

fn load_scripts(runtime: &Runtime) {
    let dir = std::env::var_os("LOCALAPPDATA")
        .map(|p| std::path::PathBuf::from(p).join("wallbang").join("lua").join("scripts"))
        .unwrap_or_else(|| std::path::PathBuf::from("wallbang/lua/scripts"));
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut scripts: Vec<_> = entries.filter_map(Result::ok).collect();
    scripts.sort_by_key(|e| e.path());
    for entry in scripts {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("lua") {
            let _ = runtime.load_script(&path);
        }
    }
}

#[no_mangle]
pub extern "system" fn wb_shutdown() {
    save_active_profile();
    if let Some(runtime) = LUA.lock().take() {
        let _ = runtime.shutdown();
    }
    CREATE_MOVE_TRAMPOLINE.store(0, Ordering::SeqCst);
    if let Some(mut hook) = CREATE_MOVE.lock().take() {
        unsafe { hook.remove() };
    }
    unsafe { wallbang_render::shutdown() };
}

#[no_mangle]
pub extern "system" fn DllMain(
    _module: *mut c_void,
    reason: u32,
    _reserved: *mut c_void,
) -> i32 {
    if reason == DLL_PROCESS_ATTACH {
        unsafe {
            QueueUserWorkItem(deferred_init, std::ptr::null_mut(), WT_EXECUTEDEFAULT);
        }
    } else if reason == DLL_PROCESS_DETACH {
        // Manual map не использует DllMain; явный вызов — только через wb_shutdown.
    }
    1
}
