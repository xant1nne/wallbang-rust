// Config block:
// MAX_SCHEDULED = 16384
// MAX_DATABASE_KEYS = 65536
// MAX_DATABASE_DEPTH = 16
// MAX_DATABASE_BYTES = 16 * 1024 * 1024

use mlua::{Error, Lua, LuaString, MultiValue, Table, Value, Variadic};
use serde_json::Value as JsonValue;

use crate::api::{json_to_lua, lua_to_json};

fn vector_table(lua: &Lua, x: f64, y: f64, z: f64) -> Result<Table, Error> {
    let table = lua.create_table_with_capacity(3, 0)?;
    table.raw_set("x", x)?;
    table.raw_set("y", y)?;
    table.raw_set("z", z)?;
    Ok(table)
}

fn vector2_table(lua: &Lua, x: f64, y: f64) -> Result<Table, Error> {
    let table = lua.create_table_with_capacity(2, 0)?;
    table.raw_set("x", x)?;
    table.raw_set("y", y)?;
    Ok(table)
}

fn angle_table(lua: &Lua, pitch: f64, yaw: f64, roll: f64) -> Result<Table, Error> {
    let table = lua.create_table_with_capacity(3, 0)?;
    table.raw_set("p", pitch)?;
    table.raw_set("y", yaw)?;
    table.raw_set("r", roll)?;
    Ok(table)
}

fn color_table(lua: &Lua, r: u8, g: u8, b: u8, a: u8) -> Result<Table, Error> {
    let table = lua.create_table_with_capacity(4, 0)?;
    table.raw_set("r", r)?;
    table.raw_set("g", g)?;
    table.raw_set("b", b)?;
    table.raw_set("a", a)?;
    Ok(table)
}

fn metatable(lua: &Lua, name: &str, methods: Table) -> Result<(), Error> {
    methods.raw_set("__index", methods.clone())?;
    methods.raw_set("__name", name)?;
    let type_name = name.to_owned();
    methods.raw_set(
        "__tostring",
        lua.create_function(move |_, _: Table| Ok(type_name.clone()))?,
    )?;
    Ok(())
}

/// Registers value constructors, the `bit` library, `table.new`/`table.clear` and json/msgpack.
pub fn register_mathlib(lua: &Lua) -> Result<(), Error> {
    let globals = lua.globals();

    // ---- Vector ----
    let vector_methods = lua.create_table()?;
    vector_methods.raw_set(
        "Length",
        lua.create_function(|_, table: Table| {
            let x: f64 = table.get("x")?;
            let y: f64 = table.get("y")?;
            let z: f64 = table.get("z")?;
            Ok((x * x + y * y + z * z).sqrt())
        })?,
    )?;
    vector_methods.raw_set(
        "Length2D",
        lua.create_function(|_, table: Table| {
            let x: f64 = table.get("x")?;
            let y: f64 = table.get("y")?;
            Ok((x * x + y * y).sqrt())
        })?,
    )?;
    vector_methods.raw_set(
        "Dot",
        lua.create_function(|_, (left, right): (Table, Table)| {
            let ax: f64 = left.get("x")?;
            let ay: f64 = left.get("y")?;
            let az: f64 = left.get("z")?;
            let bx: f64 = right.get("x")?;
            let by: f64 = right.get("y")?;
            let bz: f64 = right.get("z")?;
            Ok(ax * bx + ay * by + az * bz)
        })?,
    )?;
    vector_methods.raw_set(
        "Cross",
        lua.create_function(|lua, (left, right): (Table, Table)| {
            let ax: f64 = left.get("x")?;
            let ay: f64 = left.get("y")?;
            let az: f64 = left.get("z")?;
            let bx: f64 = right.get("x")?;
            let by: f64 = right.get("y")?;
            let bz: f64 = right.get("z")?;
            vector_table(
                lua,
                ay * bz - az * by,
                az * bx - ax * bz,
                ax * by - ay * bx,
            )
        })?,
    )?;
    vector_methods.raw_set(
        "Normalize",
        lua.create_function(|lua, table: Table| {
            let x: f64 = table.get("x")?;
            let y: f64 = table.get("y")?;
            let z: f64 = table.get("z")?;
            let length = (x * x + y * y + z * z).sqrt();
            if length == 0.0 {
                vector_table(lua, 0.0, 0.0, 0.0)
            } else {
                vector_table(lua, x / length, y / length, z / length)
            }
        })?,
    )?;
    vector_methods.raw_set(
        "__add",
        lua.create_function(|lua, (left, right): (Table, Table)| {
            vector_table(
                lua,
                left.get::<f64>("x")? + right.get::<f64>("x")?,
                left.get::<f64>("y")? + right.get::<f64>("y")?,
                left.get::<f64>("z")? + right.get::<f64>("z")?,
            )
        })?,
    )?;
    vector_methods.raw_set(
        "__sub",
        lua.create_function(|lua, (left, right): (Table, Table)| {
            vector_table(
                lua,
                left.get::<f64>("x")? - right.get::<f64>("x")?,
                left.get::<f64>("y")? - right.get::<f64>("y")?,
                left.get::<f64>("z")? - right.get::<f64>("z")?,
            )
        })?,
    )?;
    vector_methods.raw_set(
        "__mul",
        lua.create_function(|lua, (table, factor): (Table, Value)| {
            let x: f64 = table.get("x")?;
            let y: f64 = table.get("y")?;
            let z: f64 = table.get("z")?;
            match factor {
                Value::Number(number) => vector_table(lua, x * number, y * number, z * number),
                Value::Integer(integer) => {
                    let number = integer as f64;
                    vector_table(lua, x * number, y * number, z * number)
                }
                Value::Table(other) => vector_table(
                    lua,
                    x * other.get::<f64>("x")?,
                    y * other.get::<f64>("y")?,
                    z * other.get::<f64>("z")?,
                ),
                _ => Err(Error::runtime("Vector multiplication needs a number or Vector")),
            }
        })?,
    )?;
    vector_methods.raw_set(
        "__unm",
        lua.create_function(|lua, table: Table| {
            vector_table(
                lua,
                -table.get::<f64>("x")?,
                -table.get::<f64>("y")?,
                -table.get::<f64>("z")?,
            )
        })?,
    )?;
    metatable(lua, "Vector", vector_methods.clone())?;
    globals.set(
        "Vector",
        lua.create_function(move |lua, (x, y, z): (f64, f64, f64)| {
            let table = vector_table(lua, x, y, z)?;
            table.set_metatable(Some(vector_methods.clone()))?;
            Ok(table)
        })?,
    )?;

    // ---- Vector2D ----
    let vector2_methods = lua.create_table()?;
    vector2_methods.raw_set(
        "Length",
        lua.create_function(|_, table: Table| {
            let x: f64 = table.get("x")?;
            let y: f64 = table.get("y")?;
            Ok((x * x + y * y).sqrt())
        })?,
    )?;
    vector2_methods.raw_set(
        "Dot",
        lua.create_function(|_, (left, right): (Table, Table)| {
            Ok(left.get::<f64>("x")? * right.get::<f64>("x")?
                + left.get::<f64>("y")? * right.get::<f64>("y")?)
        })?,
    )?;
    vector2_methods.raw_set(
        "Normalize",
        lua.create_function(|lua, table: Table| {
            let x: f64 = table.get("x")?;
            let y: f64 = table.get("y")?;
            let length = (x * x + y * y).sqrt();
            if length == 0.0 {
                vector2_table(lua, 0.0, 0.0)
            } else {
                vector2_table(lua, x / length, y / length)
            }
        })?,
    )?;
    vector2_methods.raw_set(
        "__add",
        lua.create_function(|lua, (left, right): (Table, Table)| {
            vector2_table(
                lua,
                left.get::<f64>("x")? + right.get::<f64>("x")?,
                left.get::<f64>("y")? + right.get::<f64>("y")?,
            )
        })?,
    )?;
    metatable(lua, "Vector2D", vector2_methods.clone())?;
    globals.set(
        "Vector2D",
        lua.create_function(move |lua, (x, y): (f64, f64)| {
            let table = vector2_table(lua, x, y)?;
            table.set_metatable(Some(vector2_methods.clone()))?;
            Ok(table)
        })?,
    )?;

    // ---- QAngle ----
    let angle_methods = lua.create_table()?;
    metatable(lua, "QAngle", angle_methods.clone())?;
    globals.set(
        "QAngle",
        lua.create_function(move |lua, (pitch, yaw, roll): (f64, f64, f64)| {
            let table = angle_table(lua, pitch, yaw, roll)?;
            table.set_metatable(Some(angle_methods.clone()))?;
            Ok(table)
        })?,
    )?;

    // ---- Color ----
    let color_methods = lua.create_table()?;
    metatable(lua, "Color", color_methods.clone())?;
    globals.set(
        "Color",
        lua.create_function(move |lua, (r, g, b, a): (u8, u8, u8, Option<u8>)| {
            let table = color_table(lua, r, g, b, a.unwrap_or(255))?;
            table.set_metatable(Some(color_methods.clone()))?;
            Ok(table)
        })?,
    )?;

    register_bit(lua)?;
    register_table_helpers(lua)?;
    register_json(lua)?;
    register_msgpack(lua)?;
    Ok(())
}

fn bit_value(value: &Value) -> Result<u32, Error> {
    match value {
        Value::Integer(integer) => Ok(*integer as u32),
        Value::Number(number) if number.is_finite() => Ok(*number as u32),
        Value::String(string) => String::from_utf8_lossy(&string.as_bytes())
            .parse::<i64>()
            .map(|number| number as u32)
            .map_err(|_| Error::runtime("bit argument must be a number")),
        _ => Err(Error::runtime("bit argument must be a number")),
    }
}

fn register_bit(lua: &Lua) -> Result<(), Error> {
    let bit = lua.create_table()?;

    bit.raw_set(
        "bor",
        lua.create_function(|_, args: Variadic<Value>| {
            let mut result = 0u32;
            for value in args.iter() {
                result |= bit_value(value)?;
            }
            Ok(result as i32)
        })?,
    )?;
    bit.raw_set(
        "band",
        lua.create_function(|_, args: Variadic<Value>| {
            let mut result = u32::MAX;
            for value in args.iter() {
                result &= bit_value(value)?;
            }
            Ok(result as i32)
        })?,
    )?;
    bit.raw_set(
        "bxor",
        lua.create_function(|_, args: Variadic<Value>| {
            let mut result = 0u32;
            for value in args.iter() {
                result ^= bit_value(value)?;
            }
            Ok(result as i32)
        })?,
    )?;
    bit.raw_set(
        "bnot",
        lua.create_function(|_, value: Value| Ok((!bit_value(&value)?) as i32))?,
    )?;
    bit.raw_set(
        "lshift",
        lua.create_function(|_, (value, shift): (Value, Value)| {
            Ok((bit_value(&value)? << (bit_value(&shift)? & 31)) as i32)
        })?,
    )?;
    bit.raw_set(
        "rshift",
        lua.create_function(|_, (value, shift): (Value, Value)| {
            Ok((bit_value(&value)? >> (bit_value(&shift)? & 31)) as i32)
        })?,
    )?;
    bit.raw_set(
        "rol",
        lua.create_function(|_, (value, shift): (Value, Value)| {
            Ok(bit_value(&value)?.rotate_left(bit_value(&shift)? & 31) as i32)
        })?,
    )?;
    bit.raw_set(
        "ror",
        lua.create_function(|_, (value, shift): (Value, Value)| {
            Ok(bit_value(&value)?.rotate_right(bit_value(&shift)? & 31) as i32)
        })?,
    )?;
    bit.raw_set(
        "tobit",
        lua.create_function(|_, value: Value| Ok(bit_value(&value)? as i32))?,
    )?;
    bit.raw_set(
        "tohex",
        lua.create_function(|_, (value, height): (Value, Option<u32>)| {
            let value = bit_value(&value)?;
            let width = height.unwrap_or(8) as usize;
            Ok(format!("{:0width$x}", value, width = width.min(16)))
        })?,
    )?;

    lua.globals().set("bit", bit)?;
    Ok(())
}

fn table_capacity(value: Option<&Value>) -> Result<usize, Error> {
    match value {
        None => Ok(0),
        Some(Value::Integer(integer)) if *integer > 0 => Ok(*integer as usize),
        Some(Value::Number(number)) if number.is_finite() && *number > 0.0 => {
            Ok(*number as usize)
        }
        Some(Value::Nil) => Ok(0),
        _ => Err(Error::runtime("table.new expects non-negative capacities")),
    }
}

fn clear_table(table: &Table, seen: &mut Vec<Table>) -> Result<(), Error> {
    for visited in seen.iter() {
        if table.equals(visited)? {
            return Ok(());
        }
    }
    seen.push(table.clone());

    let mut keys = Vec::new();
    for pair in table.pairs::<Value, Value>() {
        keys.push(pair?.0);
    }
    for key in keys {
        if let Value::Table(child) = table.get::<Value>(&key)? {
            clear_table(&child, seen)?;
        }
        table.set(key, Value::Nil)?;
    }
    Ok(())
}

fn register_table_helpers(lua: &Lua) -> Result<(), Error> {
    let table: Table = lua.globals().get("table")?;

    table.raw_set(
        "new",
        lua.create_function(|lua, args: Variadic<Value>| {
            let array = table_capacity(args.first())?;
            let hash = table_capacity(args.get(1))?;
            lua.create_table_with_capacity(array, hash)
        })?,
    )?;
    table.raw_set(
        "clear",
        lua.create_function(|_, table: Table| {
            let mut seen = Vec::new();
            clear_table(&table, &mut seen)?;
            Ok(table)
        })?,
    )?;
    Ok(())
}

fn register_json(lua: &Lua) -> Result<(), Error> {
    let json = lua.create_table()?;
    json.raw_set(
        "encode",
        lua.create_function(|_, value: Value| {
            let json = lua_to_json(&value, 1)?;
            serde_json::to_string(&json)
                .map_err(|error| Error::runtime(format!("json.encode: {}", error)))
        })?,
    )?;
    json.raw_set(
        "decode",
        lua.create_function(|lua, text: LuaString| {
            let json: JsonValue = serde_json::from_slice(&text.as_bytes())
                .map_err(|error| Error::runtime(format!("json.decode: {}", error)))?;
            json_to_lua(lua, &json)
        })?,
    )?;
    lua.globals().set("json", json)?;
    Ok(())
}

fn read_be(data: &[u8], position: &mut usize, size: usize) -> Result<u64, Error> {
    if data.len() < position.saturating_add(size) {
        return Err(Error::runtime("msgpack: truncated data"));
    }
    let mut value = 0u64;
    for _ in 0..size {
        value = (value << 8) | data[*position] as u64;
        *position += 1;
    }
    Ok(value)
}

fn mp_write_uint(output: &mut Vec<u8>, value: u64) {
    if value <= 0x7f {
        output.push(value as u8);
    } else if value <= u8::MAX as u64 {
        output.push(0xcc);
        output.push(value as u8);
    } else if value <= u16::MAX as u64 {
        output.push(0xcd);
        output.extend_from_slice(&(value as u16).to_be_bytes());
    } else if value <= u32::MAX as u64 {
        output.push(0xce);
        output.extend_from_slice(&(value as u32).to_be_bytes());
    } else {
        output.push(0xcf);
        output.extend_from_slice(&value.to_be_bytes());
    }
}

fn mp_write_size(output: &mut Vec<u8>, size: usize, fix: u8, code8: u8, code16: u8, code32: u8) {
    if size < fix as usize {
        output.push(fix | size as u8);
    } else if size <= u8::MAX as usize {
        output.push(code8);
        output.push(size as u8);
    } else if size <= u16::MAX as usize {
        output.push(code16);
        output.extend_from_slice(&(size as u16).to_be_bytes());
    } else {
        output.push(code32);
        output.extend_from_slice(&(size as u32).to_be_bytes());
    }
}

fn mp_encode(value: &Value, output: &mut Vec<u8>, depth: usize) -> Result<(), Error> {
    if depth > 64 {
        return Err(Error::runtime("msgpack: value is nested too deeply"));
    }
    match value {
        Value::Nil => output.push(0xc0),
        Value::Boolean(true) => output.push(0xc3),
        Value::Boolean(false) => output.push(0xc2),
        Value::Integer(integer) => {
            if *integer >= 0 {
                mp_write_uint(output, *integer as u64);
            } else if *integer >= -32 {
                output.push(*integer as i8 as u8);
            } else if *integer >= i8::MIN as i64 {
                output.push(0xd0);
                output.push(*integer as i8 as u8);
            } else if *integer >= i16::MIN as i64 {
                output.push(0xd1);
                output.extend_from_slice(&(*integer as i16).to_be_bytes());
            } else if *integer >= i32::MIN as i64 {
                output.push(0xd2);
                output.extend_from_slice(&(*integer as i32).to_be_bytes());
            } else {
                output.push(0xd3);
                output.extend_from_slice(&integer.to_be_bytes());
            }
        }
        Value::Number(number) => {
            output.push(0xcb);
            output.extend_from_slice(&number.to_be_bytes());
        }
        Value::String(string) => {
            let bytes = string.as_bytes();
            mp_write_size(output, bytes.len(), 0xa0, 0xd9, 0xda, 0xdb);
            output.extend_from_slice(&bytes);
        }
        Value::Table(table) => {
            let mut entries = Vec::new();
            let mut array_like = true;
            let mut max_index = 0usize;
            for pair in table.pairs::<Value, Value>() {
                let (key, item) = pair?;
                if let Value::Integer(index) = key {
                    if index >= 1 {
                        max_index = max_index.max(index as usize);
                    } else {
                        array_like = false;
                    }
                } else {
                    array_like = false;
                }
                entries.push((key, item));
            }
            if array_like && max_index == entries.len() {
                mp_write_size(output, entries.len(), 0x90, 0xdc, 0xdc, 0xdd);
                for (_, item) in entries {
                    mp_encode(&item, output, depth + 1)?;
                }
            } else {
                mp_write_size(output, entries.len(), 0x80, 0xde, 0xde, 0xdf);
                for (key, item) in entries {
                    match key {
                        Value::String(string) => {
                            let bytes = string.as_bytes();
                            mp_write_size(output, bytes.len(), 0xa0, 0xd9, 0xda, 0xdb);
                            output.extend_from_slice(&bytes);
                        }
                        Value::Integer(integer) => mp_write_uint(output, integer as u64),
                        _ => return Err(Error::runtime("msgpack map keys must be strings")),
                    }
                    mp_encode(&item, output, depth + 1)?;
                }
            }
        }
        _ => return Err(Error::runtime("msgpack cannot encode functions or userdata")),
    }
    Ok(())
}

fn mp_decode(lua: &Lua, data: &[u8], position: &mut usize, depth: usize) -> Result<Value, Error> {
    if depth > 64 || *position >= data.len() {
        return Err(Error::runtime("msgpack: truncated data"));
    }
    let code = data[*position];
    *position += 1;

    match code {
        0x00..=0x7f => Ok(Value::Integer(code as i64)),
        0xe0..=0xff => Ok(Value::Integer((code as i8) as i64)),
        0xa0..=0xbf => {
            let length = (code & 0x1f) as usize;
            let bytes = read_slice(data, position, length)?;
            Ok(Value::String(lua.create_string(bytes)?))
        }
        0x90..=0x9f => {
            let length = (code & 0x0f) as usize;
            let table = lua.create_table_with_capacity(length, 0)?;
            for index in 0..length {
                table.set(index + 1, mp_decode(lua, data, position, depth + 1)?)?;
            }
            Ok(Value::Table(table))
        }
        0x80..=0x8f => {
            let length = (code & 0x0f) as usize;
            let table = lua.create_table_with_capacity(0, length)?;
            for _ in 0..length {
                let key = mp_decode(lua, data, position, depth + 1)?;
                let value = mp_decode(lua, data, position, depth + 1)?;
                table.set(key, value)?;
            }
            Ok(Value::Table(table))
        }
        0xc0 => Ok(Value::Nil),
        0xc2 => Ok(Value::Boolean(false)),
        0xc3 => Ok(Value::Boolean(true)),
        0xcc => Ok(Value::Integer(read_be(data, position, 1)? as i64)),
        0xcd => Ok(Value::Integer(read_be(data, position, 2)? as i64)),
        0xce => Ok(Value::Integer(read_be(data, position, 4)? as i64)),
        0xcf => Ok(Value::Integer(read_be(data, position, 8)? as i64)),
        0xd0 => Ok(Value::Integer((read_be(data, position, 1)? as i8) as i64)),
        0xd1 => Ok(Value::Integer((read_be(data, position, 2)? as i16) as i64)),
        0xd2 => Ok(Value::Integer((read_be(data, position, 4)? as i32) as i64)),
        0xd3 => Ok(Value::Integer(read_be(data, position, 8)? as i64)),
        0xca => Ok(Value::Number(f32::from_bits(read_be(data, position, 4)? as u32) as f64)),
        0xcb => Ok(Value::Number(f64::from_bits(read_be(data, position, 8)?))),
        0xd9 | 0xda | 0xdb => {
            let size = match code {
                0xd9 => 1,
                0xda => 2,
                _ => 4,
            };
            let length = read_be(data, position, size)? as usize;
            let bytes = read_slice(data, position, length)?;
            Ok(Value::String(lua.create_string(bytes)?))
        }
        0xc4 | 0xc5 | 0xc6 => {
            let size = match code {
                0xc4 => 1,
                0xc5 => 2,
                _ => 4,
            };
            let length = read_be(data, position, size)? as usize;
            let bytes = read_slice(data, position, length)?;
            Ok(Value::String(lua.create_string(bytes)?))
        }
        0xdc | 0xdd => {
            let size = if code == 0xdc { 2 } else { 4 };
            let length = read_be(data, position, size)? as usize;
            let table = lua.create_table_with_capacity(length, 0)?;
            for index in 0..length {
                table.set(index + 1, mp_decode(lua, data, position, depth + 1)?)?;
            }
            Ok(Value::Table(table))
        }
        0xde | 0xdf => {
            let size = if code == 0xde { 2 } else { 4 };
            let length = read_be(data, position, size)? as usize;
            let table = lua.create_table_with_capacity(0, length)?;
            for _ in 0..length {
                let key = mp_decode(lua, data, position, depth + 1)?;
                let value = mp_decode(lua, data, position, depth + 1)?;
                table.set(key, value)?;
            }
            Ok(Value::Table(table))
        }
        _ => Err(Error::runtime(format!("msgpack: unknown code 0x{:02x}", code))),
    }
}

fn read_slice<'a>(data: &'a [u8], position: &mut usize, length: usize) -> Result<&'a [u8], Error> {
    let end = position.saturating_add(length);
    if end > data.len() {
        return Err(Error::runtime("msgpack: truncated data"));
    }
    let slice = &data[*position..end];
    *position = end;
    Ok(slice)
}

fn register_msgpack(lua: &Lua) -> Result<(), Error> {
    let msgpack = lua.create_table()?;

    msgpack.raw_set(
        "pack",
        lua.create_function(|lua, values: Variadic<Value>| {
            let mut bytes = Vec::new();
            for value in values.iter() {
                mp_encode(value, &mut bytes, 1)?;
            }
            Ok(lua.create_string(&bytes)?)
        })?,
    )?;
    msgpack.raw_set(
        "unpack",
        lua.create_function(|lua, bytes: LuaString| {
            let bytes = bytes.as_bytes();
            let mut position = 0;
            let mut values = Vec::new();
            while position < bytes.len() {
                values.push(mp_decode(lua, &bytes, &mut position, 1)?);
            }
            Ok(MultiValue::from_vec(values))
        })?,
    )?;
    msgpack.raw_set("encode", msgpack.raw_get::<mlua::Function>("pack")?)?;
    msgpack.raw_set("decode", msgpack.raw_get::<mlua::Function>("unpack")?)?;
    lua.globals().set("msgpack", msgpack)?;
    Ok(())
}
