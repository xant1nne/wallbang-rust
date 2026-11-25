// Config block:
// MAX_SCHEDULED = 16384
// MAX_DATABASE_KEYS = 65536
// MAX_DATABASE_DEPTH = 16
// MAX_DATABASE_BYTES = 16 * 1024 * 1024

use std::path::Path;
use std::sync::{Arc, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use mlua::{Error, Function, Lua, LuaOptions, MultiValue, RegistryKey, StdLib, Value};
use parking_lot::Mutex;

use crate::api::{self, Database};
use crate::bindings;
use crate::events::EventBus;

/// A callback scheduled by `time.Schedule`.
pub struct Scheduled {
    pub due_ms: u64,
    pub callback: RegistryKey,
    pub args: Vec<Value>,
}

/// A cleanup callback registered by `defer`.
pub(crate) struct Deferred {
    pub(crate) callback: RegistryKey,
    pub(crate) args: Vec<Value>,
}

/// One Lua state is shared by every `Runtime` instance in the process.
static SHARED_LUA: OnceLock<Result<Lua, String>> = OnceLock::new();

/// The Lua runtime: owns the single Lua state and all script callbacks.
#[derive(Clone)]
pub struct Runtime {
    lua: Lua,
    pub(crate) scheduled: Arc<Mutex<Vec<Scheduled>>>,
    pub(crate) event_table: Arc<EventBus>,
    pub(crate) deferred: Arc<Mutex<Vec<Deferred>>>,
    pub(crate) database: Arc<Mutex<Database>>,
}

#[allow(dead_code)]
fn assert_send_sync() {
    fn assert<T: Send + Sync>() {}
    assert::<Runtime>();
}

fn create_lua() -> Result<Lua, Error> {
    // LuaJIT 2.1 with the Lua 5.2 compatibility layer: no io/os/debug/package/ffi.
    let libs = StdLib::TABLE | StdLib::STRING | StdLib::MATH | StdLib::BIT | StdLib::JIT;
    let lua = Lua::new_with(libs, LuaOptions::default())?;

    let globals = lua.globals();
    for name in ["io", "os", "debug", "package", "require", "module", "ffi"] {
        globals.set(name, Value::Nil)?;
    }
    Ok(lua)
}

/// Milliseconds since the Unix epoch (UTC).
pub(crate) fn epoch_millis() -> i64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_millis().min(i64::MAX as u128) as i64,
        Err(error) => -(error.duration().as_millis().min(i64::MAX as u128) as i64),
    }
}

/// Seconds since the Unix epoch (UTC).
pub(crate) fn epoch_seconds() -> i64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs().min(i64::MAX as u64) as i64,
        Err(error) => -(error.duration().as_secs().min(i64::MAX as u64) as i64),
    }
}

/// Current local time as (hour, minute, second, millisecond).
pub(crate) fn local_time() -> (u32, u32, u32, u32) {
    #[cfg(windows)]
    {
        // No external crates: a direct declaration of the Win32 SYSTEMTIME layout.
        #[repr(C)]
        #[derive(Default)]
        struct WinSystemTime {
            year: u16,
            month: u16,
            day_of_week: u16,
            day: u16,
            hour: u16,
            minute: u16,
            second: u16,
            milliseconds: u16,
        }

        unsafe extern "system" {
            fn GetLocalTime(lp_system_time: *mut WinSystemTime);
        }

        let mut time = WinSystemTime::default();
        unsafe { GetLocalTime(&mut time) };
        let millis = epoch_millis().rem_euclid(1_000) as u32;
        (
            time.hour as u32,
            time.minute as u32,
            time.second as u32,
            millis,
        )
    }

    #[cfg(not(windows))]
    {
        // Fallback for non-Windows builds: UTC time of day from SystemTime.
        let millis = epoch_millis().rem_euclid(86_400_000) as u32;
        (
            millis / 3_600_000,
            (millis / 60_000) % 60,
            (millis / 1_000) % 60,
            millis % 1_000,
        )
    }
}

fn report_callback_error(context: &str, error: Error) {
    api::print_line(&format!("{}: {}", context, error));
}

impl Runtime {
    pub fn new() -> Result<Self, Error> {
        let lua = SHARED_LUA
            .get_or_init(|| create_lua().map_err(|error| error.to_string()))
            .as_ref()
            .map_err(|error| Error::runtime(error.clone()))?
            .clone();

        let runtime = Self {
            lua,
            scheduled: Arc::new(Mutex::new(Vec::new())),
            event_table: Arc::new(EventBus::new()),
            deferred: Arc::new(Mutex::new(Vec::new())),
            database: Arc::new(Mutex::new(Database::load())),
        };

        bindings::register_mathlib(&runtime.lua)?;
        api::register(&runtime)?;
        Ok(runtime)
    }

    /// Loads and executes a Lua script file.
    pub fn load_script(&self, path: &Path) -> Result<(), Error> {
        let source = std::fs::read(path).map_err(|error| {
            Error::runtime(format!("cannot read {}: {}", path.display(), error))
        })?;
        let name = format!("@{}", path.to_string_lossy());
        self.lua.load(&source).set_name(name).exec()
    }

    /// Executes every scheduled callback whose deadline has passed.
    pub fn tick(&self, now_ms: u64) -> Result<(), Error> {
        let mut due = Vec::new();
        {
            let mut scheduled = self.scheduled.lock();
            let mut index = 0;
            while index < scheduled.len() {
                if scheduled[index].due_ms <= now_ms {
                    due.push(scheduled.remove(index));
                } else {
                    index += 1;
                }
            }
        }

        for job in due {
            let callback: Function = match self.lua.registry_value(&job.callback) {
                Ok(callback) => callback,
                Err(error) => {
                    report_callback_error("scheduled callback", error);
                    continue;
                }
            };
            if let Err(error) = callback.call::<()>(MultiValue::from_vec(job.args)) {
                report_callback_error("scheduled callback", error);
            }
        }
        Ok(())
    }

    /// Calls every callback subscribed to `name` with the given arguments.
    pub fn fire(&self, name: &str, args: &[Value]) -> Result<(), Error> {
        // Take the keys so callbacks can be invoked without holding the event lock.
        let keys = self.event_table.take(name);
        for key in &keys {
            let callback: Function = match self.lua.registry_value(key) {
                Ok(callback) => callback,
                Err(error) => {
                    report_callback_error(&format!("event {}", name), error);
                    continue;
                }
            };
            if let Err(error) = callback.call::<()>(MultiValue::from_vec(args.to_vec())) {
                report_callback_error(&format!("event {}", name), error);
            }
        }
        // Subscriptions survive the event; only `Unset` or `shutdown` removes them.
        for key in keys {
            self.event_table.subscribe(name, key);
        }
        Ok(())
    }

    /// Fires `events.shutdown`, runs deferred callbacks in reverse order and clears state.
    pub fn shutdown(&self) -> Result<(), Error> {
        self.fire("shutdown", &[])?;

        let deferred: Vec<Deferred> = self.deferred.lock().drain(..).collect();
        for job in deferred.into_iter().rev() {
            let callback: Function = match self.lua.registry_value(&job.callback) {
                Ok(callback) => callback,
                Err(error) => {
                    report_callback_error("deferred callback", error);
                    continue;
                }
            };
            if let Err(error) = callback.call::<()>(MultiValue::from_vec(job.args)) {
                report_callback_error("deferred callback", error);
            }
        }

        self.scheduled.lock().clear();
        self.event_table.clear();
        if let Err(error) = self.database.lock().flush() {
            report_callback_error("database flush", error);
        }
        Ok(())
    }

    pub fn lua(&self) -> &Lua {
        &self.lua
    }
}
