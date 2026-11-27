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
    fn RtlCaptureStackBackTrace(
        frames_to_skip: u32,
        frames_to_capture: u32,
        back_trace: *mut *mut c_void,
        back_trace_hash: *mut u32,
    ) -> u16;
}

fn modules() -> Vec<(&'static str, usize)> {
    let names = [
        "client.dll\0",
        "engine2.dll\0",
        "scenesystem.dll\0",
        "materialsystem2.dll\0",
        "filesystem.dll\0",
        "panorama.dll\0",
        "inputsystem.dll\0",
        "soundsystem.dll\0",
        "vphysics2.dll\0",
        "tier0.dll\0",
        "vstdlib.dll\0",
        "d3d11.dll\0",
        "dxgi.dll\0",
        "d3dcompiler_47.dll\0",
        "user32.dll\0",
        "kernel32.dll\0",
        "kernelbase.dll\0",
        "ntdll.dll\0",
    ];
    let mut list = vec![("wallbang.dll", DLL_BASE.load(Ordering::Relaxed))];
    list.extend(names.iter().map(|name| {
        let base = unsafe { GetModuleHandleA(name.as_ptr() as *const std::ffi::c_char) };
        (name.trim_end_matches('\0'), base as usize)
    }));
    list
}

fn module_bases() -> String {
    modules()
        .iter()
        .map(|(name, base)| format!("{name}=0x{base:X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn symbol_for(address: usize) -> String {
    let mut best: Option<(&'static str, usize)> = None;
    for (name, base) in modules() {
        if base != 0 && address >= base {
            let delta = address - base;
            if delta < 0x1000_0000 && best.map_or(true, |(_, old)| delta < old) {
                best = Some((name, delta));
            }
        }
    }
    best.map_or_else(
        || format!("0x{address:X}"),
        |(name, delta)| format!("{name}+0x{delta:X}"),
    )
}

fn stack_trace() -> String {
    let mut frames = [std::ptr::null_mut(); 24];
    let mut hash = 0u32;
    // В VEH skip=1 иногда возвращает 0 кадров (нет unwind-информации у
    // manual-map обратных вызовов), поэтому захватываем и нулевой кадр.
    let count = unsafe { RtlCaptureStackBackTrace(0, frames.len() as u32, frames.as_mut_ptr(), &mut hash) };
    frames[..count as usize]
        .iter()
        .map(|frame| symbol_for(*frame as usize))
        .collect::<Vec<_>>()
        .join(" -> ")
}

/// CONTEXT AMD64: Rsp по смещению 152, Rip по смещению 248. Когда stackwalk
/// молчит, сырые слова вершины стека всё равно показывают владельца кадра.
fn raw_stack_trace(context: *mut c_void) -> String {
    if context.is_null() {
        return String::new();
    }
    let rsp = unsafe { (context as *const u8).add(152).cast::<u64>().read_unaligned() } as usize;
    let mut words = Vec::with_capacity(12);
    unsafe {
        for i in 0..12usize {
            let address = (rsp as *const u8).add(i * 8).cast::<u64>().read_unaligned() as usize;
            if address != 0 {
                words.push(format!("+0x{:X}:{}", i * 8, symbol_for(address)));
            }
        }
    }
    format!("rsp=0x{rsp:X} {}", words.join(" | "))
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
            let register = |offset: usize| {
                if pointers.context.is_null() {
                    0
                } else {
                    (pointers.context as *const u8)
                        .add(offset)
                        .cast::<u64>()
                        .read_unaligned() as usize
                }
            };
            let line = format!(
                "[{secs}] EXCEPTION code=0x{:X} address=0x{:X} rip=0x{:X} rip_sym={} dll_base=0x{:X} flags=0x{:X} info0=0x{:X} info1=0x{:X} rbx=0x{:X} rcx=0x{:X} rdx=0x{:X} stack: {} raw_stack: {} modules: {}\r\n",
                record.code,
                address,
                rip,
                symbol_for(rip),
                base,
                record.flags,
                record.information.first().copied().unwrap_or(0),
                record.information.get(1).copied().unwrap_or(0),
                register(0x90),
                register(0x80),
                register(0x88),
                stack_trace(),
                raw_stack_trace(pointers.context),
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
