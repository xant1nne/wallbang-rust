// Config: native process + window discovery; no game offsets or privilege changes.
use std::{
    collections::BTreeSet,
    mem::size_of,
    path::Path,
    process::Command,
    time::{Duration, Instant},
};
use windows::{
    core::{BOOL, PWSTR},
    Win32::{
        Foundation::{CloseHandle, HANDLE, HWND, LPARAM},
        Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
        System::{
            Diagnostics::ToolHelp::{
                CreateToolhelp32Snapshot, Module32FirstW, Module32NextW, Process32FirstW,
                Process32NextW, MODULEENTRY32W, PROCESSENTRY32W, TH32CS_SNAPMODULE,
                TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS,
            },
            Threading::{
                GetCurrentProcess, GetCurrentProcessId, GetExitCodeProcess, OpenProcess, OpenProcessToken, QueryFullProcessImageNameW, PROCESS_ACCESS_RIGHTS,
                PROCESS_CREATE_THREAD, PROCESS_NAME_WIN32, PROCESS_QUERY_INFORMATION,
                PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ,
                PROCESS_VM_WRITE,
            },
        },
        UI::WindowsAndMessaging::{
            EnumWindows, GetClassNameW, GetWindowTextW, GetWindowThreadProcessId,
        },
    },
};
const POLL_INTERVAL: Duration = Duration::from_millis(50);
const ERROR_NO_MORE_FILES: u32 = 18;
const ERROR_BAD_LENGTH: u32 = 24;
const ERROR_PARTIAL_COPY: u32 = 299;
const STILL_ACTIVE: u32 = 259;
const IMAGE_PATH_SIZE: usize = 32768;
const CS2_WINDOW_TITLE: &str = "Counter-Strike 2";
const CS2_WINDOW_CLASS: &str = "SDL_app";
const MAP_RIGHTS: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(
    PROCESS_CREATE_THREAD.0
        | PROCESS_QUERY_INFORMATION.0
        | PROCESS_VM_OPERATION.0
        | PROCESS_VM_READ.0
        | PROCESS_VM_WRITE.0,
);

pub struct ProcessHandle(HANDLE);
impl ProcessHandle {
    pub fn raw(&self) -> HANDLE {
        self.0
    }
}
impl Drop for ProcessHandle {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.0);
        }
    }
}
pub fn open_process(pid: u32) -> Result<ProcessHandle, String> {
    unsafe { OpenProcess(MAP_RIGHTS, false, pid) }
        .map(ProcessHandle)
        .map_err(|error| {
            let target = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) }
                .map(ProcessHandle);
            let target_elevation = target.as_ref().map(|p| elevation(p.raw()))
                .unwrap_or_else(|e| format!("query failed ({})", code(e)));
            format!(
                "OpenProcess PID {pid} failed ({}): {error}; rights=0x{:X}; loader PID {} elevated={}; CS2 elevated={target_elevation}; image={}",
                code(&error), MAP_RIGHTS.0, unsafe { GetCurrentProcessId() },
                elevation(unsafe { GetCurrentProcess() }),
                image_name(pid).unwrap_or_else(|e| format!("query failed ({e})")),
            )
        })
}
// Read-only diagnostics. Never changes privileges or retries with broader access.
fn elevation(process: HANDLE) -> String {
    unsafe {
        let mut token = HANDLE::default();
        if let Err(e) = OpenProcessToken(process, TOKEN_QUERY, &mut token) {
            return format!("unknown ({})", code(&e));
        }
        let token = ProcessHandle(token);
        let mut value = TOKEN_ELEVATION::default();
        let mut returned = 0;
        match GetTokenInformation(token.raw(), TokenElevation,
            Some((&mut value as *mut TOKEN_ELEVATION).cast()),
            size_of::<TOKEN_ELEVATION>() as u32, &mut returned) {
            Ok(()) => (value.TokenIsElevated != 0).to_string(),
            Err(e) => format!("unknown ({})", code(&e)),
        }
    }
}
fn code(error: &windows::core::Error) -> u32 {
    error.code().0 as u32 & 0xffff
}
fn name_eq(buffer: &[u16], expected: &str) -> bool {
    let n = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
    String::from_utf16_lossy(&buffer[..n]).eq_ignore_ascii_case(expected)
}
fn snapshot_pids(name: &str) -> Result<Vec<u32>, String> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
            .map(ProcessHandle)
            .map_err(|e| format!("Process snapshot failed ({})", code(&e)))?;
        let mut entry = PROCESSENTRY32W {
            dwSize: size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };
        if let Err(e) = Process32FirstW(snapshot.raw(), &mut entry) {
            return if code(&e) == ERROR_NO_MORE_FILES {
                Ok(Vec::new())
            } else {
                Err(format!("Process32First failed ({})", code(&e)))
            };
        };
        let mut pids = Vec::new();
        loop {
            if name_eq(&entry.szExeFile, name) {
                pids.push(entry.th32ProcessID);
            }
            if let Err(e) = Process32NextW(snapshot.raw(), &mut entry) {
                if code(&e) == ERROR_NO_MORE_FILES {
                    break;
                }
                return Err(format!("Process32Next failed ({})", code(&e)));
            }
        }
        Ok(pids)
    }
}
fn image_name(pid: u32) -> Result<String, u32> {
    unsafe {
        let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid)
            .map(ProcessHandle)
            .map_err(|e| code(&e))?;
        let mut path = vec![0u16; IMAGE_PATH_SIZE];
        let mut size = path.len() as u32;
        QueryFullProcessImageNameW(
            process.raw(),
            PROCESS_NAME_WIN32,
            PWSTR(path.as_mut_ptr()),
            &mut size,
        )
        .map_err(|e| code(&e))?;
        Ok(String::from_utf16_lossy(&path[..size as usize]))
    }
}
struct WindowSearch<'a> {
    name: &'a str,
    seen: BTreeSet<u32>,
    pids: Vec<u32>,
    blocked: Vec<(u32, u32)>,
}
unsafe extern "system" fn visit_window(window: HWND, context: LPARAM) -> BOOL {
    let search = &mut *(context.0 as *mut WindowSearch<'_>);
    let mut pid = 0;
    GetWindowThreadProcessId(window, Some(&mut pid));
    if pid == 0 {
        return BOOL(1);
    }
    // Failed PID queries are not cached: a later visible SDL window can provide
    // the evidence that an existing CS2 is inaccessible instead of absent.
    if search.seen.contains(&pid) {
        return BOOL(1);
    }
    match image_name(pid) {
        Ok(path) => {
            search.seen.insert(pid);
            if Path::new(&path)
                .file_name()
                .is_some_and(|n| n.to_string_lossy().eq_ignore_ascii_case(search.name))
            {
                search.pids.push(pid);
            }
        }
        Err(error) => {
            if search.name.eq_ignore_ascii_case("cs2.exe") {
                let mut title = [0u16; 256];
                let mut class = [0u16; 128];
                GetWindowTextW(window, &mut title);
                GetClassNameW(window, &mut class);
                if name_eq(&title, CS2_WINDOW_TITLE) && name_eq(&class, CS2_WINDOW_CLASS) {
                    search.blocked.push((pid, error));
                }
            }
        }
    }
    BOOL(1)
}
fn choose_existing(
    snapshot: Result<Vec<u32>, String>,
    windows: Vec<u32>,
    blocked: Vec<(u32, u32)>,
) -> Result<Option<u32>, String> {
    let windows: BTreeSet<_> = windows.into_iter().collect();
    if windows.len() == 1 {
        return Ok(windows.first().copied());
    }
    if windows.len() > 1 {
        return Err(format!("Multiple CS2 windows: {windows:?}"));
    }
    if let Some((pid, error)) = blocked.first() {
        return Err(format!("Existing CS2 PID {pid} is inaccessible ({error})"));
    }
    let pids: BTreeSet<_> = snapshot?.into_iter().collect();
    match pids.len() {
        0 => Ok(None),
        1 => Ok(pids.first().copied()),
        _ => Err(format!("Multiple CS2 processes: {pids:?}")),
    }
}
/// ToolHelp can miss a process in a different launch context. Independently
/// resolve top-level window PIDs and validate their executable with query-only
/// rights. Inaccessible/snapshot-failed is never silently treated as absent.
pub fn find_existing(name: &str) -> Result<Option<u32>, String> {
    let snapshot = snapshot_pids(name);
    let mut windows = WindowSearch {
        name,
        seen: BTreeSet::new(),
        pids: Vec::new(),
        blocked: Vec::new(),
    };
    unsafe {
        EnumWindows(
            Some(visit_window),
            LPARAM((&mut windows as *mut WindowSearch<'_>) as isize),
        )
    }
    .map_err(|e| format!("Window enumeration failed ({})", code(&e)))?;
    choose_existing(snapshot, windows.pids, windows.blocked)
}
pub fn wait_for_process(name: &str, timeout_ms: u32) -> Result<u32, String> {
    let start = Instant::now();
    loop {
        if let Some(pid) = find_existing(name)? {
            return Ok(pid);
        }
        if start.elapsed().as_millis() >= timeout_ms as u128 {
            return Err(format!("Timed out waiting for {name}"));
        }
        std::thread::sleep(POLL_INTERVAL);
    }
}
/// Steam owns game lifetime. Its launcher PID is never used as the CS2 PID.
pub fn launch_cs2(steam_path: &Path, insecure: bool) -> Result<(), String> {
    let mut command = Command::new(steam_path);
    command.args(["-silent", "-applaunch", "730"]);
    if insecure {
        command.arg("-insecure");
    }
    command
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("Steam launch failed: {e}"))
}
#[derive(Debug, PartialEq, Eq)]
pub struct Selection {
    pub pid: u32,
    pub already_running: bool,
}
pub fn attach_or_launch(
    find: impl FnOnce() -> Result<Option<u32>, String>,
    launch: impl FnOnce() -> Result<(), String>,
    wait: impl FnOnce() -> Result<u32, String>,
) -> Result<Selection, String> {
    if let Some(pid) = find()? {
        return Ok(Selection {
            pid,
            already_running: true,
        });
    }
    launch()?;
    Ok(Selection {
        pid: wait()?,
        already_running: false,
    })
}
unsafe fn module_in_snapshot(snapshot: HANDLE, module: &str) -> Result<bool, String> {
    let mut entry = MODULEENTRY32W {
        dwSize: size_of::<MODULEENTRY32W>() as u32,
        ..Default::default()
    };
    if let Err(e) = Module32FirstW(snapshot, &mut entry) {
        return if code(&e) == ERROR_NO_MORE_FILES {
            Ok(false)
        } else {
            Err(format!("Module32First failed ({})", code(&e)))
        };
    };
    loop {
        if name_eq(&entry.szModule, module) {
            return Ok(true);
        }
        if let Err(e) = Module32NextW(snapshot, &mut entry) {
            return if code(&e) == ERROR_NO_MORE_FILES {
                Ok(false)
            } else {
                Err(format!("Module32Next failed ({})", code(&e)))
            };
        }
    }
}
pub fn wait_for_module(
    pid: u32,
    process: &ProcessHandle,
    module: &str,
    timeout_ms: u32,
) -> Result<(), String> {
    let start = Instant::now();
    loop {
        unsafe {
            let mut exit = 0;
            GetExitCodeProcess(process.raw(), &mut exit)
                .map_err(|e| format!("CS2 PID {pid} state failed ({})", code(&e)))?;
            if exit != STILL_ACTIVE {
                return Err(format!("CS2 PID {pid} exited while waiting for {module}"));
            }
            match CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid) {
                Ok(handle) => {
                    let snapshot = ProcessHandle(handle);
                    if module_in_snapshot(snapshot.raw(), module)? {
                        return Ok(());
                    }
                }
                Err(e) if matches!(code(&e), ERROR_BAD_LENGTH | ERROR_PARTIAL_COPY) => {}
                Err(e) => {
                    return Err(format!(
                        "CS2 PID {pid} module snapshot failed ({})",
                        code(&e)
                    ))
                }
            }
        }
        if start.elapsed().as_millis() >= timeout_ms as u128 {
            return Err(format!("CS2 PID {pid}: timed out waiting for {module}"));
        }
        std::thread::sleep(POLL_INTERVAL);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn existing_cs2_never_launches_a_second_game() {
        let result = attach_or_launch(
            || Ok(Some(42)),
            || panic!("must not launch"),
            || panic!("must not wait for a launcher PID"),
        )
        .unwrap();
        assert_eq!(
            result,
            Selection {
                pid: 42,
                already_running: true
            }
        );
    }
    #[test]
    fn lookup_failure_never_launches_a_second_game() {
        assert!(attach_or_launch(
            || Err("access denied".into()),
            || panic!("must not launch after an error"),
            || panic!("must not wait")
        )
        .is_err());
    }
    #[test]
    fn a_new_launch_attaches_to_the_discovered_game_pid() {
        let called = std::cell::Cell::new(false);
        let result = attach_or_launch(
            || Ok(None),
            || {
                called.set(true);
                Ok(())
            },
            || Ok(730),
        )
        .unwrap();
        assert!(called.get());
        assert_eq!(
            result,
            Selection {
                pid: 730,
                already_running: false
            }
        );
    }
    #[test]
    fn window_pid_wins_over_incomplete_snapshot_or_bootstrap_pid() {
        assert_eq!(
            choose_existing(Ok(vec![1, 2]), vec![2, 2], vec![]).unwrap(),
            Some(2)
        );
        assert_eq!(
            choose_existing(Err("snapshot failed".into()), vec![3], vec![]).unwrap(),
            Some(3)
        );
        assert_eq!(
            choose_existing(Ok(vec![]), vec![4], vec![]).unwrap(),
            Some(4)
        );
    }
    #[test]
    fn inaccessible_or_ambiguous_is_not_absent() {
        assert!(choose_existing(Ok(vec![]), vec![], vec![(4, 5)]).is_err());
        assert!(choose_existing(Ok(vec![1, 2]), vec![], vec![]).is_err());
        assert!(choose_existing(Err("snapshot failed".into()), vec![], vec![]).is_err());
    }
}
