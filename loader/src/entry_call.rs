// Config block: the remote entry point is called with the mapped image base as its only
// argument. wb_entry never returns on success, so a finite wait is treated as success.

use std::{ffi::c_void, mem::transmute};

use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0, WAIT_TIMEOUT},
    System::Threading::{
        CreateRemoteThread, GetExitCodeThread, WaitForSingleObject, LPTHREAD_START_ROUTINE,
    },
};

/// wb_entry держит remote-поток живым после инициализации (иначе Rust-FLS падает
/// в ntdll при выходе), поэтому ждём только старт, а не завершение потока.
const START_TIMEOUT_MS: u32 = 3_000;

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

    if wait_result == WAIT_TIMEOUT {
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
