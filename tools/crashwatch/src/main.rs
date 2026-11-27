// Config block: read-only debugger capture; never launches or injects the game.
// Waits for cs2.exe, records fatal exceptions and lets the OS handle them.
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::mem::size_of;
use std::os::windows::io::AsRawHandle;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use windows::Win32::Foundation::{CloseHandle, HANDLE, DBG_CONTINUE, DBG_EXCEPTION_NOT_HANDLED};
use windows::Win32::System::Diagnostics::Debug::*;
use windows::Win32::System::Diagnostics::ToolHelp::*;
use windows::Win32::System::Memory::*;
use windows::Win32::System::Threading::*;

const WAIT_PROCESS_SECONDS: u64 = 600;
const DEBUG_SECONDS: u64 = 300;
const STACK_WORDS: usize = 256;

fn log(file: &mut File, text: impl AsRef<str>) {
    let text = text.as_ref();
    println!("{text}");
    let _ = writeln!(file, "{text}");
    let _ = file.flush();
}

unsafe fn find_cs2() -> Option<u32> {
    let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).ok()?;
    let mut entry = PROCESSENTRY32W { dwSize: size_of::<PROCESSENTRY32W>() as u32, ..Default::default() };
    let mut result = None;
    if Process32FirstW(snapshot, &mut entry).is_ok() {
        loop {
            let len = entry.szExeFile.iter().position(|&c| c == 0).unwrap_or(entry.szExeFile.len());
            if String::from_utf16_lossy(&entry.szExeFile[..len]).eq_ignore_ascii_case("cs2.exe") {
                result = Some(entry.th32ProcessID);
                break;
            }
            if Process32NextW(snapshot, &mut entry).is_err() { break; }
        }
    }
    let _ = CloseHandle(snapshot);
    result
}

unsafe fn capture(process: HANDLE, pid: u32, tid: u32, exception: &EXCEPTION_DEBUG_INFO,
    output: &std::path::Path, log_file: &mut File) {
    let code = exception.ExceptionRecord.ExceptionCode.0 as u32;
    log(log_file, format!("EXCEPTION code=0x{code:08X} thread={tid} first_chance={} address={:p} params={:?}",
        exception.dwFirstChance, exception.ExceptionRecord.ExceptionAddress,
        &exception.ExceptionRecord.ExceptionInformation[..exception.ExceptionRecord.NumberParameters.min(15) as usize]));
    let Ok(thread) = OpenThread(THREAD_GET_CONTEXT | THREAD_QUERY_INFORMATION, false, tid) else { return; };
    let mut context = CONTEXT { ContextFlags: CONTEXT_ALL_AMD64, ..Default::default() };
    if let Err(error) = GetThreadContext(thread, &mut context) {
        log(log_file, format!("GetThreadContext failed: {error}"));
        let _ = CloseHandle(thread);
        return;
    }
    log(log_file, format!("RIP=0x{:X} RSP=0x{:X} RBP=0x{:X} RAX=0x{:X} RBX=0x{:X} RCX=0x{:X} RDX=0x{:X} R8=0x{:X} R9=0x{:X}",
        context.Rip, context.Rsp, context.Rbp, context.Rax, context.Rbx, context.Rcx, context.Rdx, context.R8, context.R9));
    let mut stack = [0usize; STACK_WORDS];
    let mut read = 0;
    let _ = ReadProcessMemory(process, context.Rsp as *const _, stack.as_mut_ptr().cast(), size_of_val(&stack), Some(&mut read));
    for (i, &word) in stack[..(read / 8).min(STACK_WORDS)].iter().enumerate() {
        log(log_file, format!("STACK +0x{:03X}: 0x{word:016X}", i * 8));
    }
    // Record private PE allocations too: a manual-mapped DLL is absent from
    // the normal module list but its image base is needed to resolve RVAs.
    let mut address = 0usize;
    let mut info = MEMORY_BASIC_INFORMATION::default();
    while VirtualQueryEx(process, Some(address as *const _), &mut info, size_of::<MEMORY_BASIC_INFORMATION>()) != 0 {
        let next = (info.BaseAddress as usize).checked_add(info.RegionSize);
        if info.State == MEM_COMMIT && info.BaseAddress == info.AllocationBase {
            let mut header = [0u8; 4096];
            let mut got = 0;
            if ReadProcessMemory(process, info.BaseAddress, header.as_mut_ptr().cast(), header.len(), Some(&mut got)).is_ok()
                && got >= 64 && &header[..2] == b"MZ" {
                log(log_file, format!("PE allocation base={:p} region_size=0x{:X} type=0x{:X}", info.BaseAddress, info.RegionSize, info.Type.0));
            }
        }
        match next { Some(next) if next > address => address = next, _ => break }
    }
    let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid);
    if let Ok(snapshot) = snapshot {
        let mut module = MODULEENTRY32W { dwSize: size_of::<MODULEENTRY32W>() as u32, ..Default::default() };
        if Module32FirstW(snapshot, &mut module).is_ok() {
            loop {
                let len = module.szModule.iter().position(|&c| c == 0).unwrap_or(module.szModule.len());
                log(log_file, format!("MODULE {} base={:p} size=0x{:X}", String::from_utf16_lossy(&module.szModule[..len]), module.modBaseAddr, module.modBaseSize));
                if Module32NextW(snapshot, &mut module).is_err() { break; }
            }
        }
        let _ = CloseHandle(snapshot);
    }
    if let Ok(dump) = File::create(output.join(format!("cs2-{pid}-0x{code:08x}.dmp"))) {
        let mut record = exception.ExceptionRecord;
        let mut pointers = EXCEPTION_POINTERS { ExceptionRecord: &mut record, ContextRecord: &mut context };
        let information = MINIDUMP_EXCEPTION_INFORMATION { ThreadId: tid, ExceptionPointers: &mut pointers, ClientPointers: false.into() };
        let flags = MINIDUMP_TYPE(MiniDumpWithThreadInfo.0 | MiniDumpWithUnloadedModules.0);
        let result = MiniDumpWriteDump(process, pid, HANDLE(dump.as_raw_handle()), flags, Some(&information), None, None);
        log(log_file, format!("MiniDumpWriteDump: {result:?}"));
    }
    let _ = CloseHandle(thread);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::env::args_os().nth(1).map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"C:\wallbang-rust\target\tmp\crashwatch"));
    std::fs::create_dir_all(&output)?;
    let mut file = OpenOptions::new().create(true).append(true).open(output.join("capture.log"))?;
    log(&mut file, "Waiting for cs2.exe; start the game, wait for Attached, then press LOAD.");
    let deadline = Instant::now() + Duration::from_secs(WAIT_PROCESS_SECONDS);
    let pid = loop {
        if let Some(pid) = unsafe { find_cs2() } { break pid; }
        if Instant::now() >= deadline { return Err("Timed out waiting for cs2.exe".into()); }
        std::thread::sleep(Duration::from_millis(200));
    };
    unsafe {
        DebugActiveProcess(pid)?;
        DebugSetProcessKillOnExit(false)?;
        let process = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid)?;
        log(&mut file, format!("Attached to PID {pid}; LOAD can be pressed now."));
        let deadline = Instant::now() + Duration::from_secs(DEBUG_SECONDS);
        let mut attached_breakpoint = false;
        while Instant::now() < deadline {
            let mut event = DEBUG_EVENT::default();
            if WaitForDebugEvent(&mut event, 500).is_err() { continue; }
            let mut status = DBG_CONTINUE;
            let mut fatal = false;
            match event.dwDebugEventCode {
                EXCEPTION_DEBUG_EVENT => {
                    let exception = event.u.Exception;
                    let code = exception.ExceptionRecord.ExceptionCode.0 as u32;
                    if code == 0x80000003 && !attached_breakpoint { attached_breakpoint = true; }
                    else {
                        status = DBG_EXCEPTION_NOT_HANDLED;
                        if code == 0xC0000409 || code == 0xC0000602 || exception.dwFirstChance == 0 {
                            capture(process, pid, event.dwThreadId, &exception, &output, &mut file);
                            fatal = true;
                        }
                    }
                }
                CREATE_PROCESS_DEBUG_EVENT => {
                    let info = event.u.CreateProcessInfo;
                    if !info.hFile.is_invalid() { let _ = CloseHandle(info.hFile); }
                    if !info.hProcess.is_invalid() { let _ = CloseHandle(info.hProcess); }
                    if !info.hThread.is_invalid() { let _ = CloseHandle(info.hThread); }
                }
                CREATE_THREAD_DEBUG_EVENT => {
                    let info = event.u.CreateThread;
                    if !info.hThread.is_invalid() { let _ = CloseHandle(info.hThread); }
                }
                LOAD_DLL_DEBUG_EVENT => {
                    let info = event.u.LoadDll;
                    if !info.hFile.is_invalid() { let _ = CloseHandle(info.hFile); }
                }
                EXIT_PROCESS_DEBUG_EVENT => {
                    log(&mut file, format!("Process exit: 0x{:08X}", event.u.ExitProcess.dwExitCode));
                    let _ = ContinueDebugEvent(event.dwProcessId, event.dwThreadId, status);
                    break;
                }
                _ => {}
            }
            ContinueDebugEvent(event.dwProcessId, event.dwThreadId, status)?;
            if fatal { break; }
        }
        let _ = DebugActiveProcessStop(pid);
        let _ = CloseHandle(process);
    }
    log(&mut file, "Capture stopped; debugger does not terminate the game.");
    Ok(())
}
