// Минимальный мост к Panorama: HUD/MainMenu root + CUIEngine::RunScript.
// Только чтение памяти и вызовы движка; прямые записи в игру не делает.
use crate::{memory, modules, pattern};
use std::ffi::{c_char, CString};
use std::sync::OnceLock;
use wallbang_config::signatures::{
    SIG_HUD, SIG_HUD_DISP, SIG_MAIN_MENU, SIG_MAIN_MENU_DISP, SIG_PUI_ENGINE, SIG_PUI_ENGINE_DISP,
};

type GetUiEngineFn = unsafe extern "C" fn(usize) -> usize;
type RunScriptFn = unsafe extern "C" fn(usize, usize, *const c_char, *const c_char, u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanelMode {
    Hud,
    MainMenu,
}

impl PanelMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Hud => "hud",
            Self::MainMenu => "menu",
        }
    }
}

pub struct Panorama {
    engine: usize,
    preferred: PanelMode,
    mode: PanelMode,
    panel: usize,
}

/// Результат отправки скрипта. `changed` означает смену HUD <-> MainMenu.
#[derive(Clone, Copy, Debug)]
pub struct RunResult {
    pub mode: PanelMode,
    pub panel: usize,
    pub changed: bool,
}

#[inline]
unsafe fn wrapper_panel(global: usize) -> Option<usize> {
    let object = memory::read_ptr(global)?;
    if object == 0 {
        return None;
    }
    // pMainMenuPanel использует ту же обёртку, что и pHudPanel:
    // global -> object -> +0x8 CUIPanel*. Проверить в live-меню, если panel=0.
    let panel = memory::read_ptr(object + 0x8)?;
    if panel == 0 || memory::read_ptr(panel)? == 0 {
        return None;
    }
    Some(panel)
}

#[inline]
unsafe fn hud_panel() -> Option<usize> {
    static GLOBAL: OnceLock<Option<usize>> = OnceLock::new();
    let global = *GLOBAL.get_or_init(|| signature_global(SIG_HUD, SIG_HUD_DISP));
    let global = global?;
    wrapper_panel(global)
}

#[inline]
unsafe fn menu_panel() -> Option<usize> {
    static GLOBAL: OnceLock<Option<usize>> = OnceLock::new();
    let global = *GLOBAL.get_or_init(|| signature_global(SIG_MAIN_MENU, SIG_MAIN_MENU_DISP));
    let global = global?;
    wrapper_panel(global)
}

fn signature_global(signature: &str, displacement: usize) -> Option<usize> {
    let address = pattern::scan_module("client.dll", signature)?;
    let disp = unsafe { memory::read::<i32>(address + displacement)? } as isize;
    Some((address + displacement + 4).wrapping_add_signed(disp))
}

/// Ищет живой UI root. `preferred` решает, какой root использовать, когда
/// одновременно валидны и HUD, и MainMenu (например, на переходе в меню).
fn resolve_panel(preferred: PanelMode) -> Option<(PanelMode, usize)> {
    let (first, second) = match preferred {
        PanelMode::Hud => (PanelMode::Hud, PanelMode::MainMenu),
        PanelMode::MainMenu => (PanelMode::MainMenu, PanelMode::Hud),
    };
    for mode in [first, second] {
        let panel = unsafe { match mode {
            PanelMode::Hud => hud_panel(),
            PanelMode::MainMenu => menu_panel(),
        }};
        if let Some(panel) = panel {
            return Some((mode, panel));
        }
    }
    None
}

fn ui_engine_interface() -> Option<usize> {
    if let Some(interface) = modules::create_interface("panorama.dll", "PanoramaUIEngine001") {
        return Some(interface);
    }
    let address = pattern::scan_module("client.dll", SIG_PUI_ENGINE)?;
    let disp = unsafe { memory::read::<i32>(address + SIG_PUI_ENGINE_DISP)? } as isize;
    let global = (address + SIG_PUI_ENGINE_DISP + 4).wrapping_add_signed(disp);
    unsafe { memory::read_ptr(global) }.filter(|value| *value != 0)
}

fn run_script_address(engine: usize) -> Result<usize, &'static str> {
    let run = unsafe { memory::vfunc::<usize>(engine, 77) }.ok_or("RunScript vfunc missing")?;
    let base = modules::module_base("panorama.dll").ok_or("panorama.dll missing")?;
    let size = modules::module_size("panorama.dll").ok_or("panorama.dll size missing")?;
    if run < base || run >= base.saturating_add(size) {
        return Err("RunScript outside panorama.dll");
    }
    Ok(run)
}

impl Panorama {
    pub fn attach(preferred: PanelMode) -> Result<Self, &'static str> {
        let ui_engine = ui_engine_interface().ok_or("pUiEngine not found")?;
        let get = unsafe { memory::vfunc::<usize>(ui_engine, 13) }.ok_or("GetUiEngine vfunc missing")?;
        let get: GetUiEngineFn = unsafe { std::mem::transmute(get) };
        let engine = unsafe { get(ui_engine) };
        if engine == 0 {
            return Err("GetUiEngine returned null");
        }
        run_script_address(engine)?;
        let (mode, panel) = resolve_panel(preferred).ok_or("no live HUD/menu panel")?;
        Ok(Self { engine, preferred, mode, panel })
    }

    #[inline]
    pub fn mode(&self) -> PanelMode {
        self.mode
    }

    #[inline]
    pub fn panel(&self) -> usize {
        self.panel
    }

    #[inline]
    pub fn set_preferred(&mut self, preferred: PanelMode) {
        self.preferred = preferred;
    }

    /// Реальный живой root без изменения кэша (нужен для dispose на смене режима).
    #[inline]
    pub fn live_root(&self) -> Option<(PanelMode, usize)> {
        resolve_panel(self.preferred)
    }

    /// Перепроверяет root каждый вызов; кэш используется только для логов.
    #[inline]
    pub fn is_ready(&self) -> bool {
        self.engine != 0
            && resolve_panel(self.preferred)
                .map(|(mode, panel)| mode == self.mode && panel == self.panel)
                .unwrap_or(false)
    }

    /// Возвращает false с одной строкой причины, если скрипт не отправлен.
    pub fn run_script(&mut self, script: &str) -> Result<RunResult, &'static str> {
        let (mode, panel) = resolve_panel(self.preferred).ok_or("no live HUD/menu panel")?;
        let changed = mode != self.mode || panel != self.panel;
        self.mode = mode;
        self.panel = panel;
        let run = run_script_address(self.engine)?;
        let run: RunScriptFn = unsafe { std::mem::transmute(run) };
        let script = CString::new(script).map_err(|_| "script contains NUL")?;
        let origin = CString::new("").map_err(|_| "origin contains NUL")?;
        unsafe {
            run(
                self.engine,
                self.panel,
                script.as_ptr(),
                origin.as_ptr(),
                1,
            );
        }
        Ok(RunResult { mode, panel, changed })
    }
}
