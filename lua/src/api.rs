// Config block:
// MAX_SCHEDULED = 16384
// MAX_DATABASE_KEYS = 65536
// MAX_DATABASE_DEPTH = 16
// MAX_DATABASE_BYTES = 16 * 1024 * 1024

use std::path::PathBuf;

use mlua::{Error, Function, Lua, Table, Value, Variadic};
use serde_json::{Map as JsonMap, Number as JsonNumber, Value as JsonValue};

use crate::runtime::{epoch_millis, epoch_seconds, local_time, Deferred, Runtime, Scheduled};
use crate::{MAX_DATABASE_BYTES, MAX_DATABASE_DEPTH, MAX_DATABASE_KEYS, MAX_SCHEDULED};

const MAX_DATABASE_KEY_LEN: usize = 1024;

/// Writes a line to stdout with the standard wallbang prefix.
pub(crate) fn print_line(message: &str) {
    println!("wallbang · {}", message);
}

fn database_path() -> PathBuf {
    let base = std::env::var_os("LOCALAPPDATA")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    base.join("wallbang").join("lua").join("database.json")
}

pub(crate) struct Database {
    values: JsonMap<String, JsonValue>,
    bytes: usize,
    key_count: usize,
    path: PathBuf,
    dirty: bool,
}

fn encoded_len(key: &str, value: &JsonValue) -> usize {
    let key_len = serde_json::to_vec(key).map_or(usize::MAX, |bytes| bytes.len());
    key_len.saturating_add(serde_json::to_vec(value).map_or(usize::MAX, |bytes| bytes.len()))
}

fn json_keys(value: &JsonValue) -> usize {
    match value {
        JsonValue::Object(map) => {
            map.len() + map.values().map(json_keys).sum::<usize>()
        }
        JsonValue::Array(items) => items.iter().map(json_keys).sum(),
        _ => 0,
    }
}

impl Database {
    pub(crate) fn load() -> Self {
        let path = database_path();
        let mut database = Self {
            values: JsonMap::new(),
            bytes: 0,
            key_count: 0,
            path,
            dirty: false,
        };

        if let Ok(text) = std::fs::read_to_string(&database.path) {
            if let Ok(JsonValue::Object(map)) = serde_json::from_str::<JsonValue>(&text) {
                for (key, value) in map {
                    let size = encoded_len(&key, &value);
                    if key.len() <= MAX_DATABASE_KEY_LEN
                        && json_keys(&value) + 1 <= MAX_DATABASE_KEYS
                        && database.bytes.saturating_add(size) <= MAX_DATABASE_BYTES
                    {
                        database.bytes += size;
                        database.key_count += json_keys(&value) + 1;
                        database.values.insert(key, value);
                    }
                }
            }
        }
        database
    }

    fn set(&mut self, key: String, value: JsonValue, keys: usize) -> Result<(), Error> {
        if key.is_empty() || key.len() > MAX_DATABASE_KEY_LEN {
            return Err(Error::runtime("database key must be 1..1024 bytes long"));
        }
        if keys > MAX_DATABASE_KEYS {
            return Err(Error::runtime("database key limit exceeded"));
        }

        let old = self.values.get(&key);
        let had_old = old.is_some();
        let old_size = old.map_or(0, |old| encoded_len(&key, old));
        let old_keys = old.map_or(0, json_keys);
        let new_size = encoded_len(&key, &value);

        let next_bytes = self
            .bytes
            .checked_sub(old_size)
            .and_then(|bytes| bytes.checked_add(new_size))
            .ok_or_else(|| Error::runtime("database size overflow"))?;
        if next_bytes > MAX_DATABASE_BYTES {
            return Err(Error::runtime("database size limit exceeded"));
        }
        let old_key_count = if had_old { old_keys + 1 } else { 0 };
        let next_keys = self
            .key_count
            .checked_sub(old_key_count)
            .and_then(|count| count.checked_add(keys))
            .ok_or_else(|| Error::runtime("database key overflow"))?;
        if next_keys > MAX_DATABASE_KEYS {
            return Err(Error::runtime("database key limit exceeded"));
        }

        self.values.insert(key, value);
        self.bytes = next_bytes;
        self.key_count = next_keys;
        self.dirty = true;
        self.flush()
    }

    fn remove(&mut self, key: &str) -> Result<(), Error> {
        if let Some(old) = self.values.remove(key) {
            self.bytes = self.bytes.saturating_sub(encoded_len(key, &old));
            self.key_count = self.key_count.saturating_sub(json_keys(&old) + 1);
            self.dirty = true;
            self.flush()?;
        }
        Ok(())
    }

    pub(crate) fn flush(&self) -> Result<(), Error> {
        if !self.dirty {
            return Ok(());
        }
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| {
                Error::runtime(format!("cannot create database directory: {}", error))
            })?;
        }
        let text = serde_json::to_string(&self.values)
            .map_err(|error| Error::runtime(format!("cannot encode database: {}", error)))?;
        let temporary = self.path.with_extension("json.tmp");
        std::fs::write(&temporary, text)
            .map_err(|error| Error::runtime(format!("cannot write database: {}", error)))?;
        std::fs::rename(&temporary, &self.path)
            .map_err(|error| Error::runtime(format!("cannot commit database: {}", error)))?;
        Ok(())
    }
}

/// Converts a Lua value to a JSON value, counting table keys on the way.
fn lua_to_json_inner(
    value: &Value,
    depth: usize,
    keys: &mut usize,
) -> Result<JsonValue, Error> {
    if depth > MAX_DATABASE_DEPTH {
        return Err(Error::runtime("database value is nested too deeply"));
    }
    match value {
        Value::Nil => Ok(JsonValue::Null),
        Value::Boolean(boolean) => Ok(JsonValue::Bool(*boolean)),
        Value::Integer(integer) => Ok(JsonValue::Number(JsonNumber::from(*integer))),
        Value::Number(number) => {
            if number.is_finite() {
                JsonNumber::from_f64(*number)
                    .map(JsonValue::Number)
                    .ok_or_else(|| Error::runtime("invalid database number"))
            } else {
                Err(Error::runtime("database numbers must be finite"))
            }
        }
        Value::String(string) => Ok(JsonValue::String(
            String::from_utf8_lossy(&string.as_bytes()).into_owned(),
        )),
        Value::Table(table) => {
            let mut entries = Vec::new();
            let mut array_like = true;
            let mut max_index = 0usize;
            for pair in table.pairs::<Value, Value>() {
                let (key, item) = pair?;
                match &key {
                    Value::Integer(index) if *index >= 1 => {
                        max_index = max_index.max(*index as usize);
                    }
                    _ => array_like = false,
                }
                entries.push((key, item));
            }

            if array_like && max_index == entries.len() {
                let mut items = Vec::with_capacity(entries.len());
                for (_, item) in entries {
                    *keys = keys.saturating_add(1);
                    items.push(lua_to_json_inner(&item, depth + 1, keys)?);
                }
                Ok(JsonValue::Array(items))
            } else {
                let mut map = JsonMap::new();
                for (key, item) in entries {
                    let key = key
                        .as_string()
                        .ok_or_else(|| Error::runtime("database table keys must be strings"))?
                        .to_string_lossy();
                    *keys = keys.saturating_add(1);
                    map.insert(
                        key,
                        lua_to_json_inner(&item, depth + 1, keys)?,
                    );
                }
                Ok(JsonValue::Object(map))
            }
        }
        _ => Err(Error::runtime(
            "database values must be numbers, strings, booleans or tables",
        )),
    }
}

pub(crate) fn lua_to_json(value: &Value, depth: usize) -> Result<JsonValue, Error> {
    let mut keys = 0;
    lua_to_json_inner(value, depth, &mut keys)
}

pub(crate) fn json_to_lua(lua: &Lua, value: &JsonValue) -> Result<Value, Error> {
    match value {
        JsonValue::Null => Ok(Value::Nil),
        JsonValue::Bool(boolean) => Ok(Value::Boolean(*boolean)),
        JsonValue::Number(number) => {
            if let Some(integer) = number.as_i64() {
                Ok(Value::Integer(integer))
            } else if let Some(float) = number.as_f64() {
                Ok(Value::Number(float))
            } else {
                Err(Error::runtime("invalid JSON number"))
            }
        }
        JsonValue::String(string) => Ok(Value::String(lua.create_string(string.as_bytes())?)),
        JsonValue::Array(items) => {
            let table = lua.create_table_with_capacity(items.len(), 0)?;
            for (index, item) in items.iter().enumerate() {
                table.set(index + 1, json_to_lua(lua, item)?)?;
            }
            Ok(Value::Table(table))
        }
        JsonValue::Object(map) => {
            let table = lua.create_table_with_capacity(0, map.len())?;
            for (key, item) in map {
                table.set(key.as_str(), json_to_lua(lua, item)?)?;
            }
            Ok(Value::Table(table))
        }
    }
}

fn args_to_string(lua: &Lua, args: &Variadic<Value>) -> Result<String, Error> {
    let tostring: Function = lua.globals().get("tostring")?;
    let mut output = String::new();
    for value in args.iter() {
        output.push_str(&tostring.call::<String>(value.clone())?);
    }
    Ok(output)
}

fn event_set(
    runtime: &Runtime,
    lua: &Lua,
    name: &str,
    callback: Function,
) -> Result<bool, Error> {
    // `Set` with an already registered function is a no-op.
    let existing = runtime.event_table.take(name);
    let mut duplicate = false;
    for key in existing {
        match lua.registry_value::<Function>(&key) {
            Ok(function) if function == callback => {
                duplicate = true;
                runtime.event_table.subscribe(name, key);
            }
            Ok(_) => runtime.event_table.subscribe(name, key),
            Err(_) => {}
        }
    }
    if duplicate {
        return Ok(false);
    }
    let key = lua.create_registry_value(callback)?;
    runtime.event_table.subscribe(name, key);
    Ok(true)
}

fn event_unset(
    runtime: &Runtime,
    lua: &Lua,
    name: &str,
    callback: Function,
) -> Result<bool, Error> {
    let mut removed = false;
    for key in runtime.event_table.take(name) {
        match lua.registry_value::<Function>(&key) {
            Ok(function) if function == callback => removed = true,
            Ok(_) => runtime.event_table.subscribe(name, key),
            Err(_) => {}
        }
    }
    Ok(removed)
}

fn make_event_object(lua: &Lua, runtime: &Runtime, name: &str) -> Result<Table, Error> {
    let object = lua.create_table()?;
    object.set("name", name)?;

    let set_runtime = runtime.clone();
    let set_name = name.to_owned();
    object.set(
        "Set",
        lua.create_function(move |lua, (_, callback): (Table, Function)| {
            event_set(&set_runtime, lua, &set_name, callback)
        })?,
    )?;

    let unset_runtime = runtime.clone();
    let unset_name = name.to_owned();
    object.set(
        "Unset",
        lua.create_function(move |lua, (_, callback): (Table, Function)| {
            event_unset(&unset_runtime, lua, &unset_name, callback)
        })?,
    )?;

    let fire_runtime = runtime.clone();
    let fire_name = name.to_owned();
    object.set(
        "Fire",
        lua.create_function(move |_, (_, args): (Table, Variadic<Value>)| {
            fire_runtime.fire(&fire_name, &args)?;
            Ok(true)
        })?,
    )?;

    let metatable = lua.create_table()?;
    let call_runtime = runtime.clone();
    let call_name = name.to_owned();
    metatable.set(
        "__call",
        lua.create_function(move |lua, (_, args): (Table, Variadic<Value>)| {
            let callback = match args.first().and_then(Value::as_function) {
                Some(callback) => callback.clone(),
                None => return Err(Error::runtime("event callback must be a function")),
            };
            let enabled = args.get(1).and_then(Value::as_boolean).unwrap_or(true);
            if enabled {
                event_set(&call_runtime, lua, &call_name, callback)?;
            } else {
                event_unset(&call_runtime, lua, &call_name, callback)?;
            }
            Ok(true)
        })?,
    )?;
    object.set_metatable(Some(metatable))?;
    Ok(object)
}

fn register_time(lua: &Lua, runtime: &Runtime, globals: &Table) -> Result<(), Error> {
    let time = lua.create_table()?;

    time.set(
        "GetEpochMilliseconds",
        lua.create_function(|_, ()| Ok(epoch_millis()))?,
    )?;
    time.set(
        "GetUnixTime",
        lua.create_function(|_, ()| Ok(epoch_seconds()))?,
    )?;
    time.set("GetLocalTime", lua.create_function(|_, ()| {
        let (hour, minute, second, millisecond) = local_time();
        Ok((hour, minute, second, millisecond))
    })?)?;

    let schedule_runtime = runtime.clone();
    time.set(
        "Schedule",
        lua.create_function(
            move |lua, (delay, callback, args): (f64, Function, Variadic<Value>)| {
                if delay.is_nan() {
                    return Err(Error::runtime("time.Schedule delay must not be NaN"));
                }
                let delay_ms = if delay <= 0.0 {
                    0
                } else if delay.is_infinite() {
                    u64::MAX
                } else {
                    (delay * 1_000.0).floor() as u64
                };

                let mut scheduled = schedule_runtime.scheduled.lock();
                if scheduled.len() >= MAX_SCHEDULED {
                    return Err(Error::runtime("too many scheduled calls"));
                }
                let key = lua.create_registry_value(callback)?;
                scheduled.push(Scheduled {
                    due_ms: (epoch_millis() as u64).saturating_add(delay_ms),
                    callback: key,
                    args: Vec::from(args),
                });
                Ok(true)
            },
        )?,
    )?;

    globals.set("time", time)?;
    Ok(())
}

fn register_log(lua: &Lua, globals: &Table) -> Result<(), Error> {
    let log = lua.create_table()?;

    log.set("Print", lua.create_function(|lua, args: Variadic<Value>| {
        print_line(&args_to_string(lua, &args)?);
        Ok(())
    })?)?;
    log.set("Console", lua.create_function(|lua, args: Variadic<Value>| {
        print_line(&args_to_string(lua, &args)?);
        Ok(())
    })?)?;
    log.set("Event", lua.create_function(|lua, args: Variadic<Value>| {
        print_line(&format!("[event] {}", args_to_string(lua, &args)?));
        Ok(())
    })?)?;

    globals.set("log", log)?;
    // `print` is an alias of `log.Print` and also writes to the log.
    globals.set(
        "print",
        lua.create_function(|lua, args: Variadic<Value>| {
            print_line(&args_to_string(lua, &args)?);
            Ok(())
        })?,
    )?;
    Ok(())
}

fn register_events(lua: &Lua, runtime: &Runtime, globals: &Table) -> Result<(), Error> {
    let events = lua.create_table()?;
    let index_runtime = runtime.clone();
    let metatable = lua.create_table()?;
    metatable.set(
        "__index",
        lua.create_function(move |lua, (_, key): (Table, Value)| {
            let name = key
                .as_string()
                .ok_or_else(|| Error::runtime("event name must be a string"))?
                .to_string_lossy();
            make_event_object(lua, &index_runtime, &name)
        })?,
    )?;
    events.set_metatable(Some(metatable))?;
    globals.set("events", events)?;
    Ok(())
}

fn register_defer(lua: &Lua, runtime: &Runtime, globals: &Table) -> Result<(), Error> {
    let defer_runtime = runtime.clone();
    globals.set(
        "defer",
        lua.create_function(move |lua, (callback, args): (Function, Variadic<Value>)| {
            let key = lua.create_registry_value(callback)?;
            defer_runtime.deferred.lock().push(Deferred {
                callback: key,
                args: Vec::from(args),
            });
            Ok(true)
        })?,
    )?;
    Ok(())
}

fn register_database(lua: &Lua, runtime: &Runtime, globals: &Table) -> Result<(), Error> {
    let database = lua.create_table()?;
    let metatable = lua.create_table()?;

    let index_database = runtime.database.clone();
    metatable.set(
        "__index",
        lua.create_function(move |lua, (_, key): (Table, Value)| {
            let key = key
                .as_string()
                .ok_or_else(|| Error::runtime("database key must be a string"))?
                .to_string_lossy();
            match index_database.lock().values.get(&key) {
                Some(value) => json_to_lua(lua, value),
                None => Ok(Value::Nil),
            }
        })?,
    )?;

    let newindex_database = runtime.database.clone();
    metatable.set(
        "__newindex",
        lua.create_function(move |_, (_, key, value): (Table, Value, Value)| {
            let key = key
                .as_string()
                .ok_or_else(|| Error::runtime("database key must be a string"))?
                .to_string_lossy();
            let mut database = newindex_database.lock();
            if value == Value::Nil {
                database.remove(&key)?;
            } else {
                let mut nested_keys = 0;
                let json = lua_to_json_inner(&value, 1, &mut nested_keys)?;
                let count = json_keys(&json) + 1;
                database.set(key, json, count)?;
            }
            Ok(())
        })?,
    )?;

    let pairs_database = runtime.database.clone();
    metatable.set(
        "__pairs",
        lua.create_function(move |lua, _: Table| {
            let snapshot = lua.create_table()?;
            {
                let database = pairs_database.lock();
                for (key, value) in database.values.iter() {
                    snapshot.set(key.as_str(), json_to_lua(lua, value)?)?;
                }
            }
            let next: Function = lua.globals().get("next")?;
            Ok((next, snapshot, Value::Nil))
        })?,
    )?;

    database.set_metatable(Some(metatable))?;
    globals.set("database", database)?;
    Ok(())
}

/// Registers all wallbang API globals on the shared Lua state.
pub fn register(runtime: &Runtime) -> Result<(), Error> {
    let lua = runtime.lua();
    let globals = lua.globals();

    register_time(lua, runtime, &globals)?;
    register_log(lua, &globals)?;
    register_events(lua, runtime, &globals)?;
    register_defer(lua, runtime, &globals)?;
    register_database(lua, runtime, &globals)?;
    Ok(())
}
