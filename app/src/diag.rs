// Минимальный файловый лог для smoke-теста: Desktop\wallbang_log.txt.
use parking_lot::Mutex;
use std::ffi::c_void;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

static LOG: Mutex<Option<File>> = Mutex::new(None);
static DLL_BASE: AtomicUsize = AtomicUsize::new(0);

pub fn set_base(base: usize) {
    DLL_BASE.store(base, Ordering::Relaxed);
}

#[link(name = "kernel32")]
extern "system" {
    fn AddVectoredExceptionHandler(
        first: u32,
        handler: unsafe extern "system" fn(*mut c_void) -> i32,
    ) -> *mut c_void;
    fn GetModuleHandleA(name: *const std::ffi::c_char) -> *mut c_void;
}

fn module_bases() -> String {
    let names = [
        "client.dll\0",
        "d3d11.dll\0",
        "dxgi.dll\0",
        "user32.dll\0",
        "kernel32.dll\0",
        "ntdll.dll\0",
    ];
    names
        .iter()
        .map(|name| {
            let base = unsafe { GetModuleHandleA(name.as_ptr() as *const std::ffi::c_char) };
            format!("{}=0x{:X}", name.trim_end_matches('\0'), base as usize)
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[repr(C)]
struct ExceptionRecord {
    code: u32,
    flags: u32,
    next: *mut ExceptionRecord,
    address: *mut c_void,
    number_parameters: u32,
    _padding: u32,
    information: [usize; 15],
}

#[repr(C)]
struct ExceptionPointers {
    record: *mut ExceptionRecord,
    context: *mut c_void,
}

/// Векторный обработчик: пишет напрямую в файл, без LOG-мьютекса, чтобы не
/// zadeadlock'иться, если исключение случилось внутри самого `log`.
unsafe extern "system" fn on_exception(pointers: *mut c_void) -> i32 {
    if !pointers.is_null() {
        let pointers = &*(pointers as *const ExceptionPointers);
        if !pointers.record.is_null() {
            let record = &*pointers.record;
            // MSVC задаёт имя потока через исключение 0x406D1388 — это не краш.
            if record.code == 0x406D1388 {
                return 0;
            }
            let secs = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let base = DLL_BASE.load(Ordering::Relaxed);
            let address = record.address as usize;
            // В EXCEPTION_RECORD.address лежит адрес доступа, а не инструкции.
            // RIP в CONTEXT_AMDX64 находится по смещению 248.
            let rip = if pointers.context.is_null() {
                0
            } else {
                (pointers.context as *const u8)
                    .add(248)
                    .cast::<u64>()
                    .read_unaligned() as usize
            };
            let line = format!(
                "[{secs}] EXCEPTION code=0x{:X} address=0x{:X} rip=0x{:X} rip_rva=0x{:X} dll_base=0x{:X} flags=0x{:X} info0=0x{:X} modules: {}\r\n",
                record.code,
                address,
                rip,
                rip.wrapping_sub(base),
                base,
                record.flags,
                record.information.first().copied().unwrap_or(0),
                module_bases()
            );
            if let Ok(mut file) = OpenOptions::new().append(true).open(log_path()) {
                let _ = file.write_all(line.as_bytes());
                let _ = file.flush();
            }
        }
    }
    0 // EXCEPTION_CONTINUE_SEARCH — крэш остаётся настоящим, но адрес залогирован
}

pub fn install_exception_logger() {
    static ONCE: OnceLock<()> = OnceLock::new();
    ONCE.get_or_init(|| unsafe {
        AddVectoredExceptionHandler(1, on_exception);
    });
}

fn log_path() -> PathBuf {
    let Some(profile) = std::env::var_os("USERPROFILE") else {
        return PathBuf::from("wallbang_log.txt");
    };
    let profile = PathBuf::from(profile);
    // Desktop может быть перенаправлен в OneDrive.
    let onedrive = profile.join("OneDrive").join("Desktop");
    if onedrive.is_dir() {
        return onedrive.join("wallbang_log.txt");
    }
    profile.join("Desktop").join("wallbang_log.txt")
}

pub fn init() {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path())
        .ok();
    *LOG.lock() = file;
    log("=== wallbang started ===");
}

pub fn log(message: &str) {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let line = format!("[{secs}] {message}\r\n");
    if let Some(file) = LOG.lock().as_mut() {
        let _ = file.write_all(line.as_bytes());
        let _ = file.flush();
    }
}
