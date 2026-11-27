// Config block: TICK_SLEEP_MS в ядре, MUTATIONS_ENABLED = false по умолчанию.
mod diag;

use parking_lot::Mutex;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::OnceLock;
use wallbang_config::constants::DEFAULT_PROFILE;
use wallbang_config::storage::{default_dir, Storage};
use wallbang_core::hooks::{CallHook, VTableHook};
use wallbang_core::usercmd::UserCmd;
use wallbang_lua::Runtime;

const DLL_PROCESS_ATTACH: u32 = 1;
const DLL_PROCESS_DETACH: u32 = 0;
const WT_EXECUTEDEFAULT: u32 = 0;
const SKIN_SHUTDOWN_WAIT_MS: u32 = 250;
const SKIN_PROFILE_INTERVAL:std::time::Duration=std::time::Duration::from_secs(5);

static INITIALIZED: OnceLock<()> = OnceLock::new();
static LUA: Mutex<Option<Runtime>> = Mutex::new(None);
static CREATE_MOVE: Mutex<Option<VTableHook>> = Mutex::new(None);
static CREATE_MOVE_ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static CAPTURE_THREAD: AtomicU32 = AtomicU32::new(0);
static CAPTURED_COMMAND: AtomicUsize = AtomicUsize::new(0);
static MOVE_CALLS: AtomicUsize = AtomicUsize::new(0);
static MOVE_CAPTURED: AtomicUsize = AtomicUsize::new(0);
static MOVE_CHANGED: AtomicUsize = AtomicUsize::new(0);
static AUTO_MAKE_USER_CMD: Mutex<Option<CallHook>> = Mutex::new(None);
static AUTO_MAKE_USER_CMD_ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static FRAME_STAGE: Mutex<Option<VTableHook>> = Mutex::new(None);
static FRAME_STAGE_ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static FRAMES: AtomicU32 = AtomicU32::new(0);
static LUA_RENDER_PENDING: AtomicBool = AtomicBool::new(false);
static LUA_SHUTDOWN_REQUESTED: AtomicBool = AtomicBool::new(false);
static LUA_SHUTDOWN_COMPLETE: AtomicBool = AtomicBool::new(false);

type CreateMoveFn = unsafe extern "system" fn(*mut c_void, i32, bool);
type FrameStageNotifyFn = unsafe extern "C" fn(usize, i32);
/// IDA (client.dll build 14201): `sub_1809447B0(rcx, edx)` — edx is the
/// command number, and the return is the `CUserCmd*` stored by the caller.
type AutoMakeUserCmdFn = unsafe extern "system" fn(usize, usize) -> usize;

#[link(name = "kernel32")]
extern "system" {
    fn QueueUserWorkItem(
        callback: unsafe extern "system" fn(*mut c_void) -> u32,
        param: *mut c_void,
        flags: u32,
    ) -> i32;
    fn Sleep(milliseconds: u32);
    fn GetCurrentThreadId() -> u32;
}

unsafe extern "system" fn deferred_init(_: *mut c_void) -> u32 {
    let _ = initialize();
    0
}

/// Detour CCSGOInput::CreateMove. NOTE: третьим аргументом идёт `bool active`,
/// а не CUserCmd*. AutoMakeUserCmd captures; features run after command construction.
unsafe extern "system" fn hk_create_move(this: *mut c_void, slot: i32, active: bool) {
    MOVE_CALLS.fetch_add(1, Ordering::Relaxed);
    let original = CREATE_MOVE_ORIGINAL.load(Ordering::Relaxed);
    if original == 0 {
        return;
    }
    let original: CreateMoveFn = std::mem::transmute(original);
    CAPTURED_COMMAND.store(0, Ordering::Release);
    CAPTURE_THREAD.store(GetCurrentThreadId(), Ordering::Release);
    original(this, slot, active);
    CAPTURE_THREAD.store(0, Ordering::Release);
    let captured = CAPTURED_COMMAND.swap(0, Ordering::AcqRel);
    if active && slot == 0 {
        if let Some(cmd) = UserCmd::from_raw(captured) {
            MOVE_CAPTURED.fetch_add(1, Ordering::Relaxed);
            let before = (cmd.buttons(), cmd.side_move());
            wallbang_feature::combat::cmd::on_create_move(cmd, &wallbang_core::state());
            if before != (cmd.buttons(), cmd.side_move()) { MOVE_CHANGED.fetch_add(1, Ordering::Relaxed); }
        }
    }
    static CM_LOGS: AtomicUsize = AtomicUsize::new(0);
    if CM_LOGS.fetch_add(1, Ordering::Relaxed) < 3 {
        let settings = wallbang_core::state().settings().clone();
        diag::log(&format!(
            "CreateMove: input=0x{:X} slot={} active={} allow_writes={} bhop={} autostrafe={} jumpbug={} edgejump={}",
            this as usize,
            slot,
            active as usize,
            settings.misc.allow_writes,
            settings.misc.bhop,
            settings.misc.autostrafe,
            settings.misc.movement.jumpbug,
            settings.misc.movement.edgejump,
        ));
    }
}

/// Captures the real `CUserCmd*` returned by AutoMakeUserCmd and runs the
/// Celerity movement port on the same game tick, before the caller serializes
/// and sends the command. The five-byte call hook keeps the caller's stack
/// intact; calling the original by address avoids copying a relative prologue.
unsafe extern "system" fn hk_auto_make_user_cmd(input: usize, command_number: usize) -> usize {
    let original = AUTO_MAKE_USER_CMD_ORIGINAL.load(Ordering::Relaxed);
    if original == 0 {
        return 0;
    }
    let original: AutoMakeUserCmdFn = std::mem::transmute(original);
    let address = original(input, command_number);
    if address != 0 {
        if CAPTURE_THREAD.load(Ordering::Acquire) == GetCurrentThreadId() {
            CAPTURED_COMMAND.store(address, Ordering::Release);
        }
        static CAPTURE_LOGS: AtomicUsize = AtomicUsize::new(0);
        if CAPTURE_LOGS.fetch_add(1, Ordering::Relaxed) < 3 {
            let state = wallbang_core::state();
            let settings = state.settings();
            diag::log(&format!(
                "AutoMakeUserCmd: cmd=0x{:X} command_number={} base=0x{:X} allow_writes={} mutations={} bhop={} autostrafe={} jumpbug={} edgejump={}",
                address,
                command_number,
                cmd_base(address),
                settings.misc.allow_writes,
                wallbang_core::mutation::is_allowed(),
                settings.misc.bhop,
                settings.misc.autostrafe,
                settings.misc.movement.jumpbug,
                settings.misc.movement.edgejump,
            ));
        }
    }
    address
}

#[inline]
fn cmd_base(address: usize) -> usize {
    unsafe { wallbang_core::memory::read_ptr(address + wallbang_config::offsets::CMD_BASE_CMD) }
        .unwrap_or(0)
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
    wallbang_feature::skins::on_frame_stage(stage, &wallbang_core::state());
    if stage == 6 {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        wallbang_feature::clantag::sync_name(&wallbang_core::state(), now);
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

/// Build 14201: CCSGOInput::CreateMove is vtable slot 5 (Celerity reference).
/// The slot target is still cross-checked against the local signature before patching.
fn install_create_move() {
    let Some(address) = wallbang_core::pattern::scan_module(
        "client.dll",
        wallbang_config::signatures::SIG_CREATE_MOVE,
    ) else {
        diag::log("CreateMove: signature not found");
        return;
    };
    let Some(input) = wallbang_core::modules::csgo_input() else {
        diag::log("CreateMove: CSGOInput unavailable");
        return;
    };
    let Some(vtable) = (unsafe { wallbang_core::memory::read_ptr(input) }) else {
        diag::log("CreateMove: CSGOInput vtable unavailable");
        return;
    };
    const CREATE_MOVE_SLOT: usize = 5;
    let slot_target = unsafe {
        wallbang_core::memory::read_ptr(vtable + CREATE_MOVE_SLOT * std::mem::size_of::<usize>())
    }.unwrap_or(0);
    if slot_target != address {
        diag::log(&format!(
            "CreateMove: slot mismatch slot=0x{slot_target:X} sig=0x{address:X}"
        ));
        return;
    }
    let Some(hook) = (unsafe {
        VTableHook::install(input, CREATE_MOVE_SLOT, hk_create_move as *const () as usize)
    }) else {
        diag::log("CreateMove: vtable hook install failed");
        return;
    };
    diag::log(&format!(
        "CreateMove: hooked slot={CREATE_MOVE_SLOT} original=0x{:X}",
        hook.original()
    ));
    CREATE_MOVE_ORIGINAL.store(hook.original(), Ordering::SeqCst);
    *CREATE_MOVE.lock() = Some(hook);
}

/// Hooks the verified `AutoMakeUserCmd` call site. The call target is derived
/// from the instruction itself and cross-checked against the offline-verified
/// RVA, so a client update fails closed instead of calling a random address.
fn install_auto_make_user_cmd() {
    let module_base = wallbang_core::modules::module_base("client.dll").unwrap_or(0);
    let Some(call_site) = wallbang_core::pattern::scan_module(
        "client.dll",
        wallbang_config::signatures::SIG_AUTO_MAKE_USER_CMD_CALL,
    ) else {
        diag::log("AutoMakeUserCmd: call site signature not found");
        return;
    };
    let rel = match unsafe { wallbang_core::memory::read::<i32>(call_site + 1) } {
        Some(rel) => rel,
        None => {
            diag::log("AutoMakeUserCmd: cannot read call rel32");
            return;
        }
    };
    let original = (call_site + 5).wrapping_add(rel as usize);
    let expected = module_base.wrapping_add(wallbang_config::signatures::AUTO_MAKE_USER_CMD_RVA);
    if module_base != 0 && original != expected {
        diag::log(&format!(
            "AutoMakeUserCmd: target mismatch original=0x{:X} expected=0x{:X}",
            original, expected
        ));
        return;
    }
    let hook = unsafe {
        CallHook::install_result(call_site, hk_auto_make_user_cmd as *const () as usize)
    };
    let hook = match hook {
        Ok(hook) => hook,
        Err(error) => {
            diag::log(&format!(
                "AutoMakeUserCmd: call hook install failed: {error} (call=0x{:X}, detour=0x{:X})",
                call_site,
                hk_auto_make_user_cmd as *const () as usize
            ));
            return;
        }
    };
    let stub = hook.uses_stub();
    AUTO_MAKE_USER_CMD_ORIGINAL.store(original, Ordering::SeqCst);
    *AUTO_MAKE_USER_CMD.lock() = Some(hook);
    diag::log(&format!(
        "AutoMakeUserCmd: hooked call=0x{:X} original=0x{:X} near_stub={}",
        call_site, original, stub
    ));
}

fn log_signatures() {
    use wallbang_config::signatures as sig;
    let checks = [
        ("SIG_WORLD_SPACE_BONES", sig::SIG_WORLD_SPACE_BONES),
        ("SIG_AUTO_MAKE_USER_CMD_CALL", sig::SIG_AUTO_MAKE_USER_CMD_CALL),
        ("SIG_CMD_SERIALIZE", sig::SIG_CMD_SERIALIZE),
        ("SIG_CMD_SET_MESSAGE_DATA", sig::SIG_CMD_SET_MESSAGE_DATA),
        ("SIG_CMD_STRING_COPY", sig::SIG_CMD_STRING_COPY),
        ("SIG_CMD_STRING_COPY_CALL", sig::SIG_CMD_STRING_COPY_CALL),
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
    let mut last_skin_profile=std::time::Instant::now();
    loop {
        if LUA_SHUTDOWN_REQUESTED.load(Ordering::Acquire) {
            if !LUA_SHUTDOWN_COMPLETE.load(Ordering::Acquire) {
                if let Some(runtime) = LUA.lock().take() {
                    let _ = runtime.shutdown();
                }
                LUA_SHUTDOWN_COMPLETE.store(true, Ordering::Release);
            }
            unsafe { Sleep(16) };
            continue;
        }
        if last_skin_profile.elapsed()>=SKIN_PROFILE_INTERVAL {
            diag::log(&wallbang_feature::skins::take_profile());
            last_skin_profile=std::time::Instant::now();
        }
        if let Some(runtime) = LUA.lock().as_ref() {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0);
            let _ = runtime.tick(now);
            // Keep Lua calls on its owner thread. Present only queues a signal.
            if LUA_RENDER_PENDING.swap(false, Ordering::AcqRel) {
                let _ = runtime.fire("render", &[]);
            }
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
    match wallbang_core::modules::build_supported() {
        Some(true) => diag::log(&format!(
            "build guard: ok, build {} matches expected {}",
            wallbang_core::modules::build_number().unwrap_or(0),
            wallbang_config::offsets::EXPECTED_BUILD_NUMBER
        )),
        _ => diag::log(&format!(
            "build guard: BLOCKED actual={:?} expected={} — snapshot reads disabled until offsets are updated",
            wallbang_core::modules::build_number(),
            wallbang_config::offsets::EXPECTED_BUILD_NUMBER
        )),
    }
    log_signatures();
    load_startup_profile();
    diag::log("build marker: unicode-hud-gpu-profile-v1");
    wallbang_core::state().settings_mut().misc.allow_writes = true;

    wallbang_core::mutation::set_allowed(wallbang_core::state().settings().misc.allow_writes);
    let cvar_count = wallbang_feature::cvar::init();
    diag::log(&format!(
        "cvar backend: {} cvars, overrides={}",
        cvar_count,
        wallbang_feature::cvar::active_overrides()
    ));
    if let Err(error) = unsafe { wallbang_render::init() } {
        diag::log(&format!("render init FAILED: {error}"));
        return Err(error);
    }
    diag::log("render init: ok (overlay thread started)");
    wallbang_render::native_preview::init();
    install_frame_stage_notify();
    // The unverified inputsystem vfunc 76 hook is disabled: offline inspection
    // showed slot 76 contains inline code bytes, not a function pointer, and a
    // live run logged original=0x48000025708F8B48. Menu cursor is handled by
    // hk_wndproc + update_menu_cursor through Win32 instead.

    wallbang_render::set_overlay(wallbang_feature::draw_esp);
    wallbang_render::set_frame_callback(on_frame);
    install_create_move();
    install_auto_make_user_cmd();
    diag::log(&format!("movement subtick: {}", if wallbang_core::usercmd::initialize_subtick() { "ready" } else { "unavailable" }));
    diag::log(&format!("skin changer backend: {}", if wallbang_feature::skins::initialize() { "ready" } else { "unavailable" }));
    // Branch-safe scene-draw vtable slot hook. It no longer copies the
    // scenesystem.dll prologue, and it fails closed on an unexpected slot value.
    wallbang_feature::world::install();
    diag::log(&format!("vote revealer hook: {}", wallbang_feature::votes::install()));
    diag::log(&format!(
        "world hooks: color={} skybox={} fog={}",
        if wallbang_feature::world::installed() { "ok" } else { "off" },
        if wallbang_feature::world::skybox_installed() { "ok" } else { "off" },
        if wallbang_feature::world::fog_installed() { "ok" } else { "off" },
    ));

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
    if frame % 120 == 0 {
        static LAST_SKIN_REPORT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);
        let report=wallbang_core::skin_status::snapshot();
        if LAST_SKIN_REPORT.swap(report,Ordering::Relaxed)!=report {
            diag::log(&format!("skin backend: {}",wallbang_core::skin_status::message()));
        }
    }
    if frame == 0 || frame % 600 == 0 {
        let state = wallbang_core::state();
        diag::log(&format!("movement diag: calls={} captured={} changed={} allow_writes={}",
            MOVE_CALLS.load(Ordering::Relaxed), MOVE_CAPTURED.load(Ordering::Relaxed),
            MOVE_CHANGED.load(Ordering::Relaxed), state.settings().misc.allow_writes));
        diag::log(&format!(
            "frame {frame}: menu_open={} valid={} players={} | {}",
            wallbang_render::is_menu_open(),
            state.snapshot().valid,
            state.snapshot().players.len(),
            wallbang_feature::world::diag_summary(),
        ));
    }
    // Пользовательский тумблер "allow writes" в Misc -> MutationGuard.
    let state = wallbang_core::state();
    state.settings_mut().misc.allow_writes = true;
    let allowed = true;
    wallbang_core::mutation::set_allowed(allowed);
    if !allowed {
        wallbang_feature::cvar::restore_all();
    }
    wallbang_feature::world::sync_settings(&state);
    // Client-side visuals that only need entity writes (radar reveal).
    wallbang_feature::visuals::on_frame(&state);
    wallbang_feature::event_feed::tick(&state);
    {
        let ct_cfg = state.settings().misc.clantag.clone();
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        let _ = wallbang_feature::clantag::tick(&ct_cfg, now_ms);
    }
    if let Some(report) = wallbang_feature::c3::sync(&state) {
        diag::log(&report);
    }
    LUA_RENDER_PENDING.store(true, Ordering::Release);
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
    wallbang_feature::skins::request_shutdown();
    wallbang_feature::clantag::request_shutdown();
    wallbang_render::skin_inspect::set_request(None);
    for _ in 0..SKIN_SHUTDOWN_WAIT_MS {
        if wallbang_feature::skins::shutdown_complete() { break; }
        unsafe { Sleep(1) };
    }
    if !wallbang_feature::skins::shutdown_complete() {
        diag::log("skins: shutdown stage timeout; native restoration skipped on worker thread");
    }
    for _ in 0..500 {
        if wallbang_feature::clantag::restored() { break; }
        unsafe { Sleep(1); }
    }
    if !wallbang_feature::clantag::restored() { diag::log("name prefix: restore timeout, no game-thread stage available"); }
    if let Some(hook) = FRAME_STAGE.lock().take() {
        unsafe { hook.restore() };
    }
    FRAME_STAGE_ORIGINAL.store(0, Ordering::SeqCst);
    LUA_RENDER_PENDING.store(false, Ordering::Release);
    LUA_SHUTDOWN_REQUESTED.store(true, Ordering::Release);
    for _ in 0..SKIN_SHUTDOWN_WAIT_MS {
        if LUA_SHUTDOWN_COMPLETE.load(Ordering::Acquire) { break; }
        unsafe { Sleep(1) };
    }
    if !LUA_SHUTDOWN_COMPLETE.load(Ordering::Acquire) {
        diag::log("Lua: owner-thread shutdown timeout; no Lua call made from shutdown worker");
    }
    CREATE_MOVE_ORIGINAL.store(0, Ordering::SeqCst);
    if let Some(hook) = CREATE_MOVE.lock().take() {
        unsafe { hook.restore() };
    }
    AUTO_MAKE_USER_CMD_ORIGINAL.store(0, Ordering::SeqCst);
    if let Some(mut hook) = AUTO_MAKE_USER_CMD.lock().take() {
        unsafe { hook.remove() };
    }
    wallbang_feature::cvar::shutdown();
    wallbang_feature::world::shutdown();
    wallbang_feature::votes::shutdown();
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
