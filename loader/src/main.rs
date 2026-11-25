// Config block: CLI defaults live here (cs2.exe path, wallbang.dll next to our exe,
// 30 s wait for client.dll). No config file is parsed by the loader.

#[allow(dead_code)] // explicit teardown API, intentionally unused while the game runs
mod cleanup;
mod entry_call;
mod find_process;
mod loadlib;
mod map;

use std::{env, fs, path::PathBuf, process};

use windows::Win32::Foundation::CloseHandle;

use find_process::{launch_cs2, process_handle, wait_for_module};

const DEFAULT_CS2: &str = r"C:\Program Files (x86)\Steam\steamapps\common\Counter-Strike Global Offensive\game\bin\win64\cs2.exe";
const DEFAULT_WAIT_MS: u32 = 30_000;

fn print_usage() {
    eprintln!(
        "usage: wallbang_loader [--steam <path>] [--dll <path>] [--wait <ms>]\n\
         defaults: steam = {DEFAULT_CS2}\n\
                   dll    = <exe directory>\\wallbang.dll\n\
                   wait   = {DEFAULT_WAIT_MS} ms"
    );
}

fn parse_args() -> Result<(PathBuf, PathBuf, u32, bool, bool), String> {
    let mut steam = PathBuf::from(DEFAULT_CS2);
    let mut dll = env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(|parent| parent.join("wallbang.dll")))
        .unwrap_or_else(|| PathBuf::from("wallbang.dll"));
    let mut wait_ms = DEFAULT_WAIT_MS;
    let mut attach = false;
    let mut loadlib = false;

    let mut args = env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--steam" => {
                steam = PathBuf::from(args.next().ok_or("--steam requires a path")?);
            }
            "--dll" => {
                dll = PathBuf::from(args.next().ok_or("--dll requires a path")?);
            }
            "--wait" => {
                let raw = args.next().ok_or("--wait requires milliseconds")?;
                wait_ms = raw
                    .parse::<u32>()
                    .map_err(|_| format!("invalid --wait value: {raw}"))?;
            }
            "--attach" => attach = true,
            "--loadlib" => loadlib = true,
            "--help" | "-h" => {
                print_usage();
                process::exit(0);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    Ok((steam, dll, wait_ms, attach, loadlib))
}

fn main() {
    // Game offsets must be resolved before any process is launched or mapped.
    if !wallbang_config::offsets::is_resolved() {
        println!("OFFSETS_NOT_RESOLVED");
        process::exit(2);
    }

    let (steam_path, dll_path, wait_ms, attach, loadlib) = match parse_args() {
        Ok(values) => values,
        Err(error) => {
            eprintln!("{error}");
            print_usage();
            process::exit(1);
        }
    };

    let pid = if attach {
        match find_process::wait_for_process("cs2.exe", wait_ms) {
            Some(pid) => pid,
            None => {
                eprintln!("cs2.exe was not running within {wait_ms} ms");
                process::exit(1);
            }
        }
    } else {
        match launch_cs2(&steam_path, false) {
            Ok(pid) => pid,
            Err(error) => {
                eprintln!("failed to launch {}: {error}", steam_path.display());
                process::exit(1);
            }
        }
    };
    println!("CS2_PID={pid}");

    if !wait_for_module(pid, "client.dll", wait_ms) {
        eprintln!("client.dll was not loaded within {wait_ms} ms");
        process::exit(3);
    }
    println!("client.dll ready");

    let dll_bytes = match fs::read(&dll_path) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("failed to read {}: {error}", dll_path.display());
            process::exit(4);
        }
    };

    let handle = process_handle(pid);
    if handle.0.is_null() {
        eprintln!("OpenProcess({pid}) failed");
        process::exit(4);
    }

    if loadlib {
        let path = match std::ffi::CString::new(dll_path.to_string_lossy().as_bytes()) {
            Ok(path) => path,
            Err(error) => {
                eprintln!("invalid DLL path: {error}");
                process::exit(4);
            }
        };
        match unsafe { loadlib::load_library(handle, &path) } {
            Ok(module) => println!("LOADLIB_MODULE=0x{module:016X}"),
            Err(error) => {
                eprintln!("loadlib failed: {error}");
                unsafe {
                    let _ = CloseHandle(handle);
                }
                process::exit(6);
            }
        }
        unsafe {
            let _ = CloseHandle(handle);
        }
        return;
    }

    let image = unsafe { map::map_image(handle, &dll_bytes) };
    let image = match image {
        Ok(image) => image,
        Err(error) => {
            eprintln!("manual map failed: {error}");
            unsafe {
                let _ = CloseHandle(handle);
            }
            process::exit(5);
        }
    };

    // Manual map должен вызывать экспорт wb_entry, а не PE EntryPoint (DllMain).
    let Some(export_rva) = (unsafe { map::find_export_rva(&dll_bytes, "wb_entry") }) else {
        eprintln!("wb_entry export not found in {}", dll_path.display());
        unsafe {
            let _ = CloseHandle(handle);
        }
        process::exit(6);
    };
    let entry = image.base as usize + export_rva as usize;

    if let Err(error) = unsafe { entry_call::call_remote_entry(handle, entry, image.base) } {
        eprintln!("entry call failed: {error}");
        unsafe {
            let _ = CloseHandle(handle);
        }
        process::exit(6);
    }

    println!("MAPPED_BASE=0x{:016X}", image.base as usize);
    println!("ENTRY=0x{:016X} (wb_entry)", entry);
    println!("TLS_CALLBACKS={}", image.tls.len());

    // Intentionally do not unmap on exit: the DLL stays live in the game process.
    // cleanup::unmap(handle, &image) remains available for explicit teardown.
    unsafe {
        let _ = CloseHandle(handle);
    }
}
