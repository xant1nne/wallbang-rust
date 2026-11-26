// Config block: the remote entry point is called with the mapped image base as its only
// argument. wb_entry never returns on success, so a finite wait is treated as success.

use std::{ffi::c_void, mem::transmute};

use windows::core::s;
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0},
    System::{
        Diagnostics::Debug::WriteProcessMemory,
        LibraryLoader::{GetModuleHandleA, GetProcAddress},
        Memory::{
            VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        },
        Threading::{
            CreateRemoteThread, GetExitCodeThread, WaitForSingleObject,
            LPTHREAD_START_ROUTINE, INFINITE,
        },
    },
};

/// wb_entry держит remote-поток живым после инициализации (иначе Rust-FLS падает
/// в ntdll при выходе), поэтому ждём только старт, а не завершение потока.
const START_TIMEOUT_MS: u32 = 3_000;
const TLS_OUT_OF_INDEXES: u32 = u32::MAX;

/// Start `entry` in the target process. A timeout means wb_entry is still running
/// after successful initialization, which is the expected path for manual mapping.
pub unsafe fn call_remote_entry(handle: HANDLE, entry: usize, base: *mut c_void) -> Result<(), String> {
    // usize -> LPTHREAD_START_ROUTINE is a plain pointer-width cast on x64.
    let start: LPTHREAD_START_ROUTINE = transmute(entry);

    let thread = CreateRemoteThread(
        handle,
        None,
        0,
        start,
        Some(base),
        0,
        None,
    )
    .map_err(|error| format!("CreateRemoteThread failed: {error}"))?;

    let wait_result = WaitForSingleObject(thread, START_TIMEOUT_MS);
    let mut exit_code = 0u32;
    let exit_result = GetExitCodeThread(thread, &mut exit_code)
        .map(|_| exit_code)
        .map_err(|error| format!("GetExitCodeThread failed: {error}"));
    let _ = CloseHandle(thread);

    if wait_result == windows::Win32::Foundation::WAIT_TIMEOUT {
        return Ok(());
    }
    if wait_result != WAIT_OBJECT_0 {
        return Err(format!("entry thread wait failed: {:?}", wait_result));
    }
    let exit_code = exit_result?;
    if exit_code != 0 {
        return Err(format!("wb_entry returned non-zero status: {exit_code}"));
    }
    Ok(())
}

/// `TlsAlloc` must run inside the target process. Allocating the slot in the
/// loader and copying the index into the mapped image makes the Rust CRT read
/// and write somebody else's TLS slot, which corrupts the game heap.
pub unsafe fn alloc_remote_tls_index(handle: HANDLE) -> Result<u32, String> {
    let kernel32 = GetModuleHandleA(s!("kernel32.dll"))
        .map_err(|error| format!("GetModuleHandleA(kernel32) failed: {error:?}"))?;
    // Kernel32 is mapped at the same address in every process on Windows.
    let tls_alloc = GetProcAddress(kernel32, s!("TlsAlloc"))
        .ok_or_else(|| "GetProcAddress(TlsAlloc) failed".to_string())?;
    let start: LPTHREAD_START_ROUTINE = transmute(tls_alloc);

    let thread = CreateRemoteThread(handle, None, 0, start, None, 0, None)
        .map_err(|error| format!("CreateRemoteThread(TlsAlloc) failed: {error:?}"))?;
    let wait = WaitForSingleObject(thread, INFINITE);
    let mut index = TLS_OUT_OF_INDEXES;
    let index_result = GetExitCodeThread(thread, &mut index)
        .map(|_| index)
        .map_err(|error| format!("GetExitCodeThread(TlsAlloc) failed: {error:?}"));
    let _ = CloseHandle(thread);

    if wait != WAIT_OBJECT_0 {
        return Err(format!("TlsAlloc thread wait failed: {wait:?}"));
    }
    let index = index_result?;
    if index == TLS_OUT_OF_INDEXES {
        return Err("TlsAlloc returned TLS_OUT_OF_INDEXES".to_string());
    }
    Ok(index)
}

/// Runs the image's TLS callbacks with DLL_PROCESS_ATTACH before wb_entry.
/// CreateRemoteThread supplies only one argument, so a tiny x64 stub sets the
/// remaining two arguments and tail-jumps to each callback.
pub unsafe fn run_remote_tls_callbacks(
    handle: HANDLE,
    image_base: usize,
    callbacks: &[usize],
) -> Result<(), String> {
    // mov edx, 1        (Reason = DLL_PROCESS_ATTACH)
    // xor r8d, r8d      (Reserved = NULL)
    // jmp rel32         (direct branch, no CFG-checked indirect target)
    // RCX already holds image_base from CreateRemoteThread's lpParameter.
    let mut stub = [
        0xBA, 0x01, 0x00, 0x00, 0x00, 0x45, 0x31, 0xC0, 0xE9, 0x00, 0x00, 0x00, 0x00,
    ];

    for &callback in callbacks {
        if callback == 0 {
            continue;
        }
        let remote = VirtualAllocEx(
            handle,
            None,
            stub.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        );
        if remote.is_null() {
            return Err("VirtualAllocEx for TLS stub failed".to_string());
        }
        let rel = callback as isize - (remote as usize + stub.len()) as isize;
        let Ok(rel) = i32::try_from(rel) else {
            let _ = VirtualFreeEx(handle, remote, 0, MEM_RELEASE);
            return Err("TLS callback is too far from the remote stub".to_string());
        };
        stub[9..13].copy_from_slice(&rel.to_le_bytes());
        let write = WriteProcessMemory(
            handle,
            remote,
            stub.as_ptr() as *const c_void,
            stub.len(),
            None,
        );
        if let Err(error) = write {
            let _ = VirtualFreeEx(handle, remote, 0, MEM_RELEASE);
            return Err(format!("WriteProcessMemory(TLS stub) failed: {error:?}"));
        }
        let start: LPTHREAD_START_ROUTINE = transmute(remote);
        let thread = match CreateRemoteThread(
            handle,
            None,
            0,
            start,
            Some(image_base as *mut c_void),
            0,
            None,
        ) {
            Ok(thread) => thread,
            Err(error) => {
                let _ = VirtualFreeEx(handle, remote, 0, MEM_RELEASE);
                return Err(format!("CreateRemoteThread(TLS callback) failed: {error:?}"));
            }
        };
        let wait = WaitForSingleObject(thread, INFINITE);
        let _ = CloseHandle(thread);
        let _ = VirtualFreeEx(handle, remote, 0, MEM_RELEASE);
        if wait != WAIT_OBJECT_0 {
            return Err(format!("TLS callback thread wait failed: {wait:?}"));
        }
    }
    Ok(())
}
