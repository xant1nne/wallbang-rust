// Загрузка DLL через LoadLibraryA в целевом процессе. Используется для старого
// C++ wallbang.dll: он инициализируется в DllMain и не экспортирует wb_entry.
use std::ffi::{c_void, CStr};
use std::mem::transmute;

use windows::core::s;
use windows::Win32::Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0};
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};
use windows::Win32::System::Memory::{
    VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE,
};
use windows::Win32::System::Threading::{
    CreateRemoteThread, GetExitCodeThread, LPTHREAD_START_ROUTINE,
};

pub unsafe fn load_library(handle: HANDLE, path: &CStr) -> Result<usize, String> {
    let kernel32 = GetModuleHandleA(s!("kernel32.dll"))
        .map_err(|error| format!("GetModuleHandleA(kernel32) failed: {error:?}"))?;
    let load_library = GetProcAddress(kernel32, s!("LoadLibraryA"))
        .ok_or_else(|| "GetProcAddress(LoadLibraryA) failed".to_string())?;

    let bytes = path.to_bytes_with_nul();
    let remote_path = VirtualAllocEx(
        handle,
        None,
        bytes.len(),
        MEM_COMMIT | MEM_RESERVE,
        PAGE_READWRITE,
    );
    if remote_path.is_null() {
        return Err("VirtualAllocEx for DLL path failed".to_string());
    }
    windows::Win32::System::Diagnostics::Debug::WriteProcessMemory(
        handle,
        remote_path,
        bytes.as_ptr() as *const c_void,
        bytes.len(),
        None,
    )
    .map_err(|error| format!("WriteProcessMemory failed: {error:?}"))?;

    let start: LPTHREAD_START_ROUTINE = transmute(load_library);
    let thread = CreateRemoteThread(
        handle,
        None,
        0,
        start,
        Some(remote_path as *mut c_void),
        0,
        None,
    )
    .map_err(|error| format!("CreateRemoteThread(LoadLibraryA) failed: {error:?}"))?;

    let wait = windows::Win32::System::Threading::WaitForSingleObject(
        thread,
        windows::Win32::System::Threading::INFINITE,
    );
    let mut module = 0u32;
    let module_result = GetExitCodeThread(thread, &mut module)
        .map(|_| module as usize)
        .map_err(|error| format!("GetExitCodeThread failed: {error:?}"));
    let _ = CloseHandle(thread);
    let _ = VirtualFreeEx(handle, remote_path, 0, MEM_RELEASE);

    if wait != WAIT_OBJECT_0 {
        return Err(format!("LoadLibraryA thread wait failed: {wait:?}"));
    }
    let module = module_result?;
    if module == 0 {
        return Err("LoadLibraryA returned NULL in target process".to_string());
    }
    Ok(module)
}
