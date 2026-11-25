// Config block: process discovery settings are passed by the CLI (exe path, module name,
// timeout in ms); polling interval is fixed at 50 ms and uses the ToolHelp snapshot API.

use std::{
    mem::size_of,
    path::Path,
    process::Command,
    time::{Duration, Instant},
};

use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Module32FirstW, Module32NextW, Process32FirstW,
            Process32NextW, MODULEENTRY32W, PROCESSENTRY32W, TH32CS_SNAPMODULE,
            TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS,
        },
        Threading::{OpenProcess, PROCESS_ALL_ACCESS},
    },
};

const POLL_INTERVAL_MS: u64 = 50;

/// Launch cs2.exe directly. `-insecure` is only used when explicitly requested;
/// the manual map loader does not need it.
pub fn launch_cs2(steam_path: &Path, insecure: bool) -> std::io::Result<u32> {
    let mut command = Command::new(steam_path);
    if insecure {
        command.arg("-insecure");
    }
    Ok(command.spawn()?.id())
}

/// Compare a null-terminated UTF-16 name from a ToolHelp entry with an ASCII name.
fn name_eq(buffer: &[u16], expected: &str) -> bool {
    let len = buffer.iter().position(|&unit| unit == 0).unwrap_or(buffer.len());
    String::from_utf16_lossy(&buffer[..len]).eq_ignore_ascii_case(expected)
}

/// Search one process snapshot for `name`, returning its pid.
#[allow(dead_code)]
unsafe fn pid_in_snapshot(snapshot: HANDLE, name: &str) -> Option<u32> {
    let mut entry = PROCESSENTRY32W {
        dwSize: size_of::<PROCESSENTRY32W>() as u32,
        ..Default::default()
    };

    if Process32FirstW(snapshot, &mut entry as *mut PROCESSENTRY32W).is_err() {
        return None;
    }

    loop {
        if name_eq(&entry.szExeFile, name) {
            return Some(entry.th32ProcessID);
        }
        if Process32NextW(snapshot, &mut entry as *mut PROCESSENTRY32W).is_err() {
            return None;
        }
    }
}

/// Search one module snapshot of `pid` for `module`.
unsafe fn module_in_snapshot(snapshot: HANDLE, module: &str) -> bool {
    let mut entry = MODULEENTRY32W {
        dwSize: size_of::<MODULEENTRY32W>() as u32,
        ..Default::default()
    };

    if Module32FirstW(snapshot, &mut entry as *mut MODULEENTRY32W).is_err() {
        return false;
    }

    loop {
        if name_eq(&entry.szModule, module) {
            return true;
        }
        if Module32NextW(snapshot, &mut entry as *mut MODULEENTRY32W).is_err() {
            return false;
        }
    }
}

/// Poll the process list until a process with this executable name exists.
#[allow(dead_code)] // utility API kept for callers that attach instead of launching
pub fn wait_for_process(name: &str, timeout_ms: u32) -> Option<u32> {
    let started = Instant::now();

    loop {
        if let Ok(snapshot) = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) } {
            let found = unsafe { pid_in_snapshot(snapshot, name) };
            unsafe {
                let _ = CloseHandle(snapshot);
            }
            if let Some(pid) = found {
                return Some(pid);
            }
        }

        if started.elapsed().as_millis() >= u128::from(timeout_ms) {
            return None;
        }
        std::thread::sleep(Duration::from_millis(POLL_INTERVAL_MS));
    }
}

/// Full-access handle for the target process. A null handle means OpenProcess failed.
pub fn process_handle(pid: u32) -> HANDLE {
    unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, pid).unwrap_or_default() }
}

/// Poll the module list of a process until `module` (for example `client.dll`) is loaded.
pub fn wait_for_module(pid: u32, module: &str, timeout_ms: u32) -> bool {
    let started = Instant::now();

    loop {
        let flags = TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32;
        if let Ok(snapshot) = unsafe { CreateToolhelp32Snapshot(flags, pid) } {
            let found = unsafe { module_in_snapshot(snapshot, module) };
            unsafe {
                let _ = CloseHandle(snapshot);
            }
            if found {
                return true;
            }
        }

        if started.elapsed().as_millis() >= u128::from(timeout_ms) {
            return false;
        }
        std::thread::sleep(Duration::from_millis(POLL_INTERVAL_MS));
    }
}
