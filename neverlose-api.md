# Neverlose CS2 API Reference

Source: https://docs.neverlose.cc/cs2/api_reference/introduction/overview

Compiled from 76 documentation pages. Custom site-only API tags and encoded metadata were converted to readable Markdown.

## Contents

- [Database](#data-database)
- [Filesystem](#data-filesystem)
- [Json](#data-json)
- [Msgpack](#data-msgpack)
- [Console](#engine-console)
- [Engine](#engine-engine)
- [Entity](#engine-entity)
- [Globals](#engine-globals)
- [Memory](#engine-memory)
- [Nav](#engine-nav)
- [Random](#engine-random)
- [Schema](#engine-schema)
- [Sounds](#engine-sounds)
- [Trace](#engine-trace)
- [Value](#engine-value)
- [Cheat](#events-cheat)
- [Game](#events-game)
- [Anatomy](#extensions-anatomy)
- [Globals And Modules](#extensions-globals-and-modules)
- [Lifecycle And Events](#extensions-lifecycle-and-events)
- [Localization](#extensions-localization)
- [Requirements](#extensions-requirements)
- [Cmd](#input-cmd)
- [Input](#input-input)
- [Motion](#input-motion)
- [Ragebot](#input-ragebot)
- [Editor Setup](#introduction-editor-setup)
- [Getting Started](#introduction-getting-started)
- [Lua Intelligence](#introduction-lua-intelligence)
- [Overview](#introduction-overview)
- [Reading These Docs](#introduction-reading-these-docs)
- [Base](#lua-base)
- [Bit](#lua-bit)
- [Ffi](#lua-ffi)
- [Jit](#lua-jit)
- [Math](#lua-math)
- [String](#lua-string)
- [Table.Clear](#lua-table.clear)
- [Table](#lua-table)
- [Table.New](#lua-table.new)
- [Aabb](#mathlib-aabb)
- [Angle](#mathlib-angle)
- [Color](#mathlib-color)
- [Euler](#mathlib-euler)
- [Mathlib](#mathlib-mathlib)
- [Matrix3X4](#mathlib-matrix3x4)
- [Quaternion](#mathlib-quaternion)
- [Rect](#mathlib-rect)
- [Transform](#mathlib-transform)
- [Vector](#mathlib-vector)
- [Vector2](#mathlib-vector2)
- [Vector4D](#mathlib-vector4d)
- [Vectoraligned](#mathlib-vectoraligned)
- [Vmatrix](#mathlib-vmatrix)
- [Vplane](#mathlib-vplane)
- [Http](#networking-http)
- [Netchan](#networking-netchan)
- [Protobuf](#networking-protobuf)
- [Websocket](#networking-websocket)
- [Esp](#rendering-esp)
- [Indicators](#rendering-indicators)
- [Panorama](#rendering-panorama)
- [Particles](#rendering-particles)
- [Projection](#rendering-projection)
- [Render](#rendering-render)
- [Ui](#rendering-ui)
- [View](#rendering-view)
- [Cloud](#runtime-cloud)
- [Events](#runtime-events)
- [Hwmon](#runtime-hwmon)
- [Log](#runtime-log)
- [Time](#runtime-time)
- [Utils](#runtime-utils)
- [Advanced Luajit](#tutorials-advanced-luajit)
- [Ffi Crash Course](#tutorials-ffi-crash-course)
- [Lua Crash Course](#tutorials-lua-crash-course)

---


## Database

<!-- Source: cs2/api_reference/data/database.md -->

---
title: Database
icon: database
---

# Database

<Heading level={2} id="database-1">
  database

   _()_
</Heading>

A key and value store that is saved to disk. The data stays after the game restarts.

Use `database` like a table. `database[key] = value` saves the value, and `database[key]`
returns the saved value. If the key was never set, it returns **nil**. Assign **nil** to a key
to remove it.

Keys must be strings. Any other key type throws an error. You can save numbers,
strings, booleans, and tables of these values. Functions,
userdata, and cdata are saved as **nil**. Tables that are nested more than 16 levels deep are
also saved as **nil**.

Each script has its own keys. A function of your script reads and writes the keys of your
script, even when another script calls it. In a library, `database` uses the keys of the
script that runs the library's code.

When a script is removed in the menu, its keys are removed too. When a developer script is
renamed in the menu, its keys are kept. If you delete or rename the folder of a script outside
the menu, its keys are not removed.

A key can be at most 1 KB long. A value can be at most 4 MB after it is encoded. The keys and
values of one script together can be at most 16 MB, and one script can have at most 65536
keys.

An assignment that goes over a limit throws an error, and the saved value does not change.
An assignment that makes the total size smaller always works, and assigning **nil** always
works.


  ### `database[string]`
**Kind:** `prop`

**Source:** `data/framework/database.lua`

---


## Filesystem

<!-- Source: cs2/api_reference/data/filesystem.md -->

---
title: File System
icon: folder-tree
---

# File System

## filesystem _()_

Provides access to the game's working directory at `%steamdir%/steamapps/cs2/game/bin/win64/`,
the SearchPaths at `game/csgo/`, `game/core/`, `csgo_addons`, and `csgo_community_addons`,
and the directory used by `cloud.ReadResource`. Attempting to access any other paths will throw an error.

Paths for the game's directories should be relative paths.

See [https://developer.valvesoftware.com/wiki/VPK\_(file\_format)](https://developer.valvesoftware.com/wiki/VPK_\(file_format\)) for how game content is managed by CS2.

The files in the path returned by `cloud.ReadResource` cannot be modified. `filesystem.List` will show
cloud files that are not yet extracted, and `filesystem.Info` will return the time the cloud script was modified at.


  ### `filesystem.Read`
**Kind:** `fn`

**Arguments:** `path, is_binary?`

Reads a file and returns all of its contents. If the file is not in the working directory, the SearchPaths are checked. Returns **nil** if the file cannot be opened.

**Source:** `data/engine/filesystem.lua`

  ### `filesystem.Write`
**Kind:** `fn`

**Arguments:** `path, contents, is_binary?`

Writes `contents` to a file and replaces any content that the file already has. If the file cannot be opened in the working directory, the SearchPaths are tried. Returns `true` if all of the data was written, and `false` otherwise.

**Source:** `data/engine/filesystem.lua`

  ### `filesystem.Delete`
**Kind:** `fn`

**Arguments:** `path`

Deletes a file. If the file does not exist, nothing happens. Throws an error if the file exists but cannot be deleted.

**Source:** `data/engine/filesystem.lua`

  ### `filesystem.Folder`
**Kind:** `fn`

**Arguments:** `path`

Creates a directory at the given path. If the directory already exists, nothing happens. The parent directory must already exist, or an error is thrown.

**Source:** `data/engine/filesystem.lua`

  ### `filesystem.List`
**Kind:** `fn`

**Arguments:** `path, filter?`

Returns the names of the entries in a directory. You can pass a wildcard pattern to keep only the names that match it. In the pattern, `*` matches any number of characters and `?` matches exactly one character.

Throws an error if the path is not an existing directory.
    
  

**Source:** `data/engine/filesystem.lua`

  ### `filesystem.Info`
**Kind:** `fn`

**Arguments:** `path`

Returns the times and the size of a file. Throws an error if the file does not exist.

**Source:** `data/engine/filesystem.lua`


## filesystem.Info.Result _()_

The file information returned by `filesystem.Info`.


  ### `filesystem.Info.Result.atime`
**Kind:** `prop`

The time of the last access, as a Unix timestamp.

**Source:** `data/engine/filesystem.lua`

  ### `filesystem.Info.Result.ctime`
**Kind:** `prop`

The time of creation, as a Unix timestamp.

**Source:** `data/engine/filesystem.lua`

  ### `filesystem.Info.Result.mtime`
**Kind:** `prop`

The time of the last modification, as a Unix timestamp.

**Source:** `data/engine/filesystem.lua`

  ### `filesystem.Info.Result.size`
**Kind:** `prop`

The size of the file in bytes.

**Source:** `data/engine/filesystem.lua`

---


## Json

<!-- Source: cs2/api_reference/data/json.md -->

---
title: JSON
icon: file-braces
---

# JSON

<Heading level={2} id="json-1">
  json

   _()_
</Heading>

Encodes Lua values as JSON and decodes JSON into Lua values.


  ### `json.new`
**Kind:** `fn`

`json.new` can be used to instantiate an independent copy of the Lua CJSON module. The new module has a separate persistent encoding buffer, and default settings.

**Source:** `data/c/json.lua`

  ### `json.decode`
**Kind:** `fn`

**Arguments:** `json_text`

`json.decode` will deserialise any UTF-8 JSON string into a Lua value or table.

UTF-16 and UTF-32 JSON strings are not supported.

      `json.decode` requires that any NULL (ASCII 0) and double quote (ASCII 34)
      characters are escaped within strings. All escape codes will be decoded and
      other bytes will be passed transparently. UTF-8 characters are not validated
      during decoding and should be checked elsewhere if required.

      JSON `null` will be converted to NULL `lightuserdata` value.

      By default, numbers incompatible with the JSON specification (infinity,
      NaN, hexadecimal) can be decoded. This default can be changed with
      `json.decode_invalid_numbers`.

      <Warning>
        Care must be taken after decoding JSON objects with numeric keys. Each
        numeric key will be stored as a Lua `string`. Any subsequent code assuming
        type `number` may break.
      </Warning>
    
  

**Source:** `data/c/json.lua`

  ### `json.decode_invalid_numbers`
**Kind:** `fn`

**Arguments:** `setting?`

Lua CJSON may generate an error when trying to decode numbers not supported by the JSON specification. *Invalid numbers* are defined as:

* infinity
      * not-a-number (NaN)
      * hexadecimal

      Available settings:

      * `true`: Accept and decode *invalid numbers*. This is the default setting.
      * `false`: Throw an error when *invalid numbers* are encountered.

      The current setting is always returned, and is only updated when an argument
      is provided.
    
  

**Source:** `data/c/json.lua`

  ### `json.decode_max_depth`
**Kind:** `fn`

**Arguments:** `depth?`

Lua CJSON will generate an error when parsing deeply nested JSON once the maximum array/object depth has been exceeded. This check prevents unnecessarily complicated JSON from slowing down the application, or crashing the application due to a lack of process stack space.

An error may be generated before the depth limit is hit if Lua is unable to
      allocate more objects on the Lua stack.

      By default, Lua CJSON will reject JSON with arrays and/or objects nested
      more than 1000 levels deep.

      The current setting is always returned, and is only updated when an argument
      is provided.
    
  

**Source:** `data/c/json.lua`

  ### `json.encode`
**Kind:** `fn`

**Arguments:** `value`

`json.encode` will serialise a Lua value into a string containing the JSON representation.

`json.encode` supports the following types:

      * `boolean`
      * `lightuserdata` (NULL value only)
      * `nil`
      * `number`
      * `string`
      * `table`

      The remaining Lua types will generate an error:

      * `function`
      * `lightuserdata` (non-NULL values)
      * `thread`
      * `userdata`

      By default, numbers are encoded with 14 significant digits. Refer to
      `json.encode_number_precision` for details.

      Lua CJSON will escape the following characters within each UTF-8 string:

      * Control characters (ASCII 0 - 31)
      * Double quote (ASCII 34)
      * Forward slash (ASCII 47)
      * Backslash (ASCII 92)
      * Delete (ASCII 127)

      All other bytes are passed transparently.

      <Warning>
        Lua CJSON will successfully encode/decode binary strings, but this is
        technically not supported by JSON and may not be compatible with other
        JSON libraries. To ensure the output is valid JSON, applications should
        ensure all Lua strings passed to `json.encode` are UTF-8.
      </Warning>

      Lua CJSON uses a heuristic to determine whether to encode a Lua table as a
      JSON array or an object. A Lua table with only positive integer keys of type
      `number` will be encoded as a JSON array. All other tables will be encoded
      as a JSON object.

      Lua CJSON does not use metamethods when serialising tables.

      * `rawget` is used to iterate over Lua arrays
      * `next` is used to iterate over Lua objects

      Lua arrays with missing entries (*sparse arrays*) may optionally be encoded
      in several different ways. Refer to `json.encode_sparse_array` for details.

      JSON object keys are always strings. Hence `json.encode` only supports
      table keys which are type `number` or `string`. All other types will
      generate an error.

      By default, encoding the following Lua values will generate errors:

      * Numbers incompatible with the JSON specification (infinity, NaN)
      * Tables nested more than 1000 levels deep
      * Excessively sparse Lua arrays

      These defaults can be changed with:

      * `json.encode_invalid_numbers`
      * `json.encode_max_depth`
      * `json.encode_sparse_array`
    
  

**Source:** `data/c/json.lua`

  ### `json.encode_invalid_numbers`
**Kind:** `fn`

**Arguments:** `setting?`

Lua CJSON may generate an error when encoding floating point numbers not supported by the JSON specification (*invalid numbers*):

* infinity
      * not-a-number (NaN)

      Available settings:

      * `true`: Allow *invalid numbers* to be encoded. This will generate
        non-standard JSON, but this output is supported by some libraries.

      * `"null"`: Encode *invalid numbers* as a JSON `null` value. This allows
        infinity and NaN to be encoded into valid JSON.

      * `false`: Throw an error when attempting to encode *invalid numbers*. This
        is the default setting.

      The current setting is always returned, and is only updated when an argument
      is provided.
    
  

**Source:** `data/c/json.lua`

  ### `json.encode_keep_buffer`
**Kind:** `fn`

**Arguments:** `keep?`

Lua CJSON can reuse the JSON encoding buffer to improve performance.

Available settings:

      * `true`: The buffer will grow to the largest size required and is not freed
        until the Lua CJSON module is garbage collected. This is the default
        setting.

      * `false`: Free the encode buffer after each call to `json.encode`.

      The current setting is always returned, and is only updated when an argument
      is provided.
    
  

**Source:** `data/c/json.lua`

  ### `json.encode_max_depth`
**Kind:** `fn`

**Arguments:** `depth?`

Once the maximum table depth has been exceeded Lua CJSON will generate an error. This prevents deeply nested or recursive data structure from crashing the application.

By default, Lua CJSON will generate an error when trying to encode data
      structures with more than 1000 nested tables.

      The current setting is always returned, and is only updated when an argument
      is provided.
    
  

**Source:** `data/c/json.lua`

  ### `json.encode_number_precision`
**Kind:** `fn`

**Arguments:** `precision?`

Sets the number of significant digits used when encoding numbers. Lower values reduce precision.

By default, Lua CJSON will output 14 significant digits when converting a
      number to text.

      The current setting is always returned, and is only updated when an argument
      is provided.
    
  

**Source:** `data/c/json.lua`

  ### `json.encode_sparse_array`
**Kind:** `fn`

**Arguments:** `convert?, ratio?, safe?`

Lua CJSON classifies a Lua table into one of three kinds when encoding a JSON array. This is determined by the number of values missing from the Lua array as follows:

* **Normal**: All values are available.
      * **Sparse**: At least one value is missing.
      * **Excessively sparse**: The number of values missing exceeds the
        configured ratio.

      Lua CJSON encodes sparse Lua arrays as JSON arrays using JSON `null` for the
      missing entries.

      An array is excessively sparse when all the following conditions are met:

      * `ratio > 0`
      * `maximum_index > safe`
      * `maximum_index > item_count * ratio`

      Lua CJSON will never consider an array to be *excessively sparse* when
      `ratio == 0`. The `safe` limit ensures that small Lua arrays are always
      encoded as sparse arrays.

      By default, attempting to encode an *excessively sparse* array will generate
      an error. If `convert` is set to `true`, *excessively sparse* arrays will be
      converted to a JSON object.

      The current settings are always returned. A particular setting is only
      changed when the argument is provided (non-`nil`).
    
  

**Source:** `data/c/json.lua`

---


## Msgpack

<!-- Source: cs2/api_reference/data/msgpack.md -->

---
title: MessagePack
icon: message-circle-more
---

# MessagePack

## msgpack _()_


  ### `msgpack.pack`
**Kind:** `fn`

**Arguments:** `...?`

Packs one or more Lua values into one string of MessagePack data.

`nil` is packed as MessagePack `nil`, and a boolean is packed as `true` or `false`. A number
      without a fractional part is packed as an integer. Any other number is packed as a 32 bit
      float when that keeps its exact value, and as a 64 bit double otherwise. A string is packed as
      raw bytes.

      A table is packed as an array when its keys are exactly the integers from 1 to N. An empty
      table is packed as an empty array. Any other table is packed as a map. Tables that are nested
      more than 16 levels deep are packed as `nil`.

      Values of any other type, such as functions and userdata, are packed as `nil`.

      You must pass at least one value. If you call `pack` with no values, it throws an error. When
      you pass more than one value, the string holds one MessagePack object for each value, one
      after the other.
    
  

**Source:** `data/c/msgpack.lua`

  ### `msgpack.unpack`
**Kind:** `fn`

**Arguments:** `msgpack_stream`

Unpacks every object in a string of MessagePack data and returns them as separate values. For example, a string made by `msgpack.pack(a, b, c)` returns `a`, `b` and `c`.

Throws an error if the data is cut short, has a bad format, or is nested more than 16 levels
      deep.
    
  

**Source:** `data/c/msgpack.lua`

  ### `msgpack.unpack_one`
**Kind:** `fn`

**Arguments:** `msgpack_stream, offset?`

Unpacks one object from a string of MessagePack data, starting at `offset`.

`offset` is a byte position in `msgpack_stream`. The first byte is at offset `0`, which is
      also the default.

      The function returns `resume_offset` first and the object second. Pass `resume_offset` to the
      next `unpack_one` call to read the next object. When the end of the data is reached,
      `resume_offset` is `-1`.

      Throws an error if `offset` is negative or larger than the length of the data, or if the data
      is invalid.
    
  

**Source:** `data/c/msgpack.lua`

  ### `msgpack.unpack_limit`
**Kind:** `fn`

**Arguments:** `msgpack_stream, limit, offset?`

Unpacks up to `limit` objects from a string of MessagePack data, starting at `offset`.

`offset` is a byte position in `msgpack_stream`. The first byte is at offset `0`, which is
      also the default. If the data ends first, fewer than `limit` objects are returned.

      The function returns `resume_offset` first and the objects after it. Pass `resume_offset` to
      `unpack_limit` or `unpack_one` to continue reading. When the end of the data is reached,
      `resume_offset` is `-1`.

      If `limit` and `offset` are both `0`, the function works like `msgpack.unpack`. It returns
      every object and does not return `resume_offset`.

      Throws an error if `offset` or `limit` is negative, if `offset` is larger than the length of
      the data, or if the data is invalid.
    
  

**Source:** `data/c/msgpack.lua`

---


## Console

<!-- Source: cs2/api_reference/engine/console.md -->

---
title: Console
icon: terminal
---

# Console

<Heading level={2} id="console-1">
  console

   _()_
</Heading>

Gives access to console variables and console commands by their names in the engine.
Indexing this table with a name returns one of three objects.

If a console variable with this name is registered, a `console.Variable` is returned.
Otherwise, if a console command with this name is registered, a `console.Command` is returned.
Otherwise, a `console.CommandLine` is returned. You can call it, but it is not an engine object.
Names are not case sensitive.

Every lookup returns an object, so you cannot check for `nil` to find out whether a name is
registered. Check the `is_valid` field instead.

`pairs(console)` goes over the registered variables and commands only. Each value it returns
is the engine object for that name.

Lookups are not cached. A `console.CommandLine` that you keep in a variable stays a
`console.CommandLine`. If a variable or command with that name is registered later, a new
lookup returns the new engine object.


  ### `console[string]`
**Kind:** `prop`

**Source:** `engine/engine/console.lua`

  ### `console.Execute`
**Kind:** `fn`

**Arguments:** `...?`

Runs a command line in the client console. The arguments are converted to strings and joined with spaces, so `console.Execute("buy", "ak47")` runs `buy ak47`. Calling a `console.CommandLine` does the same thing, with its name added at the start of the line.

**Source:** `engine/engine/console.lua`

  ### `console.ConVarType_t`
**Kind:** `type`

The data types that a console variable can have. The `type` field of `console.Variable` holds one of these values.


  

**Source:** `engine/engine/console.lua`

  ### `console.ConVarFlags_t`
**Kind:** `type`

The flag bits of a console variable or command, as found in the `flags` field of `console.Object`. Use `bit.band` to test them.


  

**Source:** `engine/engine/console.lua`


## console.Object _()_

The base class of console variables and console commands. All of its fields are read-only.


  ### `console.Object.flags`
**Kind:** `prop`

The flag bits, as `console.ConVarFlags_t` values.

**Source:** `engine/engine/console.lua`

  ### `console.Object.help`
**Kind:** `prop`

The registered help text, or `nil` if there is no help text.

**Source:** `engine/engine/console.lua`

  ### `console.Object.info`
**Kind:** `prop`

A pointer to the registration data of the object.

**Source:** `engine/engine/console.lua`

  ### `console.Object.is_valid`
**Kind:** `prop`

Whether the object refers to a valid engine object.

**Source:** `engine/engine/console.lua`

  ### `console.Object.name`
**Kind:** `prop`

The registered name.

**Source:** `engine/engine/console.lua`


## console.Command _()_



A registered console command. When you call it, the registered callback of the command runs
directly. The arguments are converted to strings and passed to the command as its arguments,
so `console.buy("ak47")` runs `buy ak47`.


  ### `console.Command(...)`
**Kind:** `ctor`

**Source:** `engine/engine/console.lua`


## console.CommandLine _()_

A name that has no registered variable or command. When you call it, the line `name args...`
is run in the client console, so `console.drop()` does the same as `console.Execute("drop")`.
The arguments are converted to strings and joined with spaces. The engine decides whether the
line has any effect.

This class does not inherit from `console.Object`. It has no engine object, so it has no
`flags`, `info` or `help` fields.


  ### `console.CommandLine(...)`
**Kind:** `ctor`

**Source:** `engine/engine/console.lua`

  ### `console.CommandLine.is_valid`
**Kind:** `prop`

Always `false`.

**Source:** `engine/engine/console.lua`

  ### `console.CommandLine.name`
**Kind:** `prop`

The name that was looked up.

**Source:** `engine/engine/console.lua`


## console.Variable _()_



A console variable, also called a ConVar.


  ### `console.Variable.default`
**Kind:** `prop`

The default value of the variable. Read-only.

**Source:** `engine/engine/console.lua`

  ### `console.Variable.max`
**Kind:** `prop`

The maximum value of the variable. Read-only.

**Source:** `engine/engine/console.lua`

  ### `console.Variable.min`
**Kind:** `prop`

The minimum value of the variable. Read-only.

**Source:** `engine/engine/console.lua`

  ### `console.Variable.type`
**Kind:** `prop`

The data type of the value, as a `console.ConVarType_t` value. Read-only.

**Source:** `engine/engine/console.lua`

  ### `console.Variable.value`
**Kind:** `prop`

The current value of the variable. Its Lua type depends on the data type of the variable. You can assign a new value to this field.

**Source:** `engine/engine/console.lua`

  ### `console.Variable.Value`
**Kind:** `type`

The Lua type of a console value depends on the registered type of the variable.

**Source:** `engine/engine/console.lua`

---


## Engine

<!-- Source: cs2/api_reference/engine/engine.md -->

---
title: Engine
icon: monitor
sidebar_position: 1
---

# Engine

<Heading level={2} id="engine-1">
  engine

   _()_
</Heading>

Returns information about the state of the engine.


  ### `engine.GetScreenSize`
**Kind:** `fn`

Returns the current render width and height in pixels as a new `Vector2D`. Changing the returned vector does not change the engine state. The render size can be different from the size of the game window. Returns **nil** if the engine state is not available.

**Source:** `engine/engine/engine.lua`

  ### `engine.GetViewAngles`
**Kind:** `fn`

Returns a copy of the current view angles of the engine.

Changing this copy does not move the camera. To change the view angles, use
      `input.InputPollContext:GetViewAngles` and `input.InputPollContext:SetViewAngles` in the
      `events.input_poll` event. The value from `input.InputPollContext:GetViewAngles` includes
      input changes that are not applied yet. This function returns the angles that the engine
      has now.
    
  

**Source:** `engine/game/engine.lua`

  ### `engine.GetLevelName`
**Kind:** `fn`

Returns the full name of the map that is loaded now. Returns nil if there is no network game client.

**Source:** `engine/game/engine.lua`

  ### `engine.GetLevelNameShort`
**Kind:** `fn`

Returns the short name of the map that is loaded now. Returns nil if there is no network game client.

**Source:** `engine/game/engine.lua`

  ### `engine.GetClientTick`
**Kind:** `fn`

Returns the current client tick. Returns nil if there is no network game client.

**Source:** `engine/game/engine.lua`

  ### `engine.GetServerTick`
**Kind:** `fn`

Returns the current server tick. Returns nil if there is no network game client.

**Source:** `engine/game/engine.lua`

  ### `engine.GetDisplayMode`
**Kind:** `fn`

Returns the current display mode of the engine swap chain. Returns nil if the engine has no display mode.

Inside the `display_mode_changed` event, this function already returns the new mode.
    
  

**Source:** `engine/game/engine.lua`


## engine.DisplayMode _()_

A display mode copied from the engine. The object stays valid after the function or callback
that created it returns. All fields are read-only.


  ### `engine.DisplayMode.back_buffer_count`
**Kind:** `prop`

The number of back buffers.

**Source:** `engine/game/engine.lua`

  ### `engine.DisplayMode.height`
**Kind:** `prop`

The height of the mode in pixels.

**Source:** `engine/game/engine.lua`

  ### `engine.DisplayMode.high_dpi`
**Kind:** `prop`

Whether the mode is marked as high DPI.

**Source:** `engine/game/engine.lua`

  ### `engine.DisplayMode.multisample`
**Kind:** `prop`

The MSAA sample count. The value is 0 when multisampling is off.

**Source:** `engine/game/engine.lua`

  ### `engine.DisplayMode.refresh_rate`
**Kind:** `prop`

The refresh rate in Hz in fullscreen. The value is 0 when the mode uses the default rate, for example in a window.

**Source:** `engine/game/engine.lua`

  ### `engine.DisplayMode.vsync`
**Kind:** `prop`

Whether the game waits for vertical sync before it shows a frame.

**Source:** `engine/game/engine.lua`

  ### `engine.DisplayMode.width`
**Kind:** `prop`

The width of the mode in pixels.

**Source:** `engine/game/engine.lua`

  ### `engine.DisplayMode.window_mode`
**Kind:** `prop`

How the game window is shown.

**Source:** `engine/game/engine.lua`

---


## Entity

<!-- Source: cs2/api_reference/engine/entity.md -->

---
title: Entity System
icon: users
sidebar_position: 3
---

# Entity System

## entity _()_

Adds functions for game entities to the engine `entity` namespace. They find the local
player, list the players, and return the game rules and weapon data.

Use this table to choose the function that returns the pawn you need.

| Need                                                                                      | Call                                                |
| ----------------------------------------------------------------------------------------- | --------------------------------------------------- |
| The pawn that you move and shoot with, which is your own pawn or the bot that you control | `GetLocalPlayerPawn()`                              |
| Your own player pawn, also while you control a bot                                        | `GetOwnLocalPlayerPawn()`                           |
| The pawn that the controller points to, which can be an observer pawn                     | `GetLocalPawn()`                                    |
| A pawn that you control and that is not your own                                          | `GetLocalControlledPawn()`                          |
| The player that you spectate                                                              | `GetObservedPawn()`                                 |
| The pawn that the camera renders from                                                     | `GetViewPawn()`                                     |
| The player that you spectate in some observer modes, and your pawn in other cases         | `GetLocalOrObservedPawn()`, `GetLocalOrInEyePawn()` |

`GetLocalPlayerController()` always returns your controller. `GetLocalObserverPawn()` returns
your own observer entity. It can exist while you play, and it is never the player that you
spectate.

A pawn can be missing during a transition, and it can be dead. Get the pawn again in every
callback, and call `pawn:IsAlive()` when you need a living player.


  ### `entity.LookupByIndex`
**Kind:** `fn`

**Arguments:** `index`

Returns the entity with the given entity index. Returns **nil** if no entity has this index.

**Source:** `engine/engine/entity.lua`

  ### `entity.LookupByHandle`
**Kind:** `fn`

**Arguments:** `handle`

Returns the entity with the given handle. Returns **nil** if the handle is not valid, or if the entity of the handle was deleted.

A handle always refers to one entity. If a new entity later gets the same index, the old
      handle does not return the new entity.
    
  

**Source:** `engine/engine/entity.lua`

  ### `entity.LookupPlayerControllerBySlot`
**Kind:** `fn`

**Arguments:** `slot`

Returns the player controller in the given player slot. Returns **nil** if the slot is empty. Throws an error if the slot is less than `-1` or greater than `64`.

**Source:** `engine/engine/entity.lua`

  ### `entity.LookupPlayerControllerByUserId`
**Kind:** `fn`

**Arguments:** `userid`

Returns the player controller of the player with the given user ID. Returns **nil** if no player has this ID.

**Source:** `engine/engine/entity.lua`

  ### `entity.ListByClassName`
**Kind:** `fn`

**Arguments:** `classname, recursive?`

Returns a list of all entities of the given class.

**Source:** `engine/engine/entity.lua`

  ### `entity.Iterate`
**Kind:** `fn`

Returns an iterator over all entities. Use it in a `for` loop. Each step gives the entity index and the entity.

**Source:** `engine/engine/entity.lua`

  ### `entity.IterateByClassName`
**Kind:** `fn`

**Arguments:** `classname, recursive?`

Returns an iterator over the entities of the given class. Use it in a `for` loop. Each step gives the entity index and the entity.

**Source:** `engine/engine/entity.lua`

  ### `entity.FirstByClassName`
**Kind:** `fn`

**Arguments:** `classname, recursive?`

Returns the first entity of the given class. Returns **nil** if there is no such entity.

**Source:** `engine/engine/entity.lua`

  ### `entity.ListByDesignerName`
**Kind:** `fn`

**Arguments:** `designer_name?, recursive?`

Returns a list of all entities with the given designer name.

**Source:** `engine/engine/entity.lua`

  ### `entity.IterateByDesignerName`
**Kind:** `fn`

**Arguments:** `designer_name?, recursive?`

Returns an iterator over the entities with the given designer name. Use it in a `for` loop. Each step gives the entity index and the entity.

**Source:** `engine/engine/entity.lua`

  ### `entity.FirstByDesignerName`
**Kind:** `fn`

**Arguments:** `designer_name, recursive?`

Returns the first entity with the given designer name. Returns **nil** if there is no such entity.

**Source:** `engine/engine/entity.lua`

  ### `entity.WrapInstance`
**Kind:** `fn`

**Arguments:** `instance`

Returns an entity object for a pointer to an entity.

<Warning>
        This function is for advanced users. The pointer must point to an entity that exists. Use it
        only while that entity exists.
      </Warning>
    
  

**Source:** `engine/engine/entity.lua`

  ### `entity.GetLocalPlayer`
**Kind:** `fn`

Returns the controller of the local player and the pawn that the local player uses. The pawn is the player's own pawn, or the pawn of a bot while the player controls one.

Both values are **nil** if there is no local controller. The pawn alone can be **nil** if it is
      not available. To get your own pawn also while you control a bot, use
      `entity.GetOwnLocalPlayerPawn`.
    
  

**Source:** `engine/game/entity.lua`

  ### `entity.GetLocalPlayerController`
**Kind:** `fn`

Returns the controller of the local player. Returns **nil** if you are not connected. The controller stays your own while you control a bot or spectate another player.

**Source:** `engine/game/entity.lua`

  ### `entity.GetLocalPlayerPawn`
**Kind:** `fn`

Returns the pawn that the local player uses. While the local player controls the pawn of another player or a bot, this is that pawn. In other cases this is the local player's own pawn.

Use this function for movement and weapons. While you spectate, it returns your own player
      pawn. `entity.GetOwnLocalPlayerPawn` always returns your own pawn, also while you control a
      bot. Both functions can return a dead pawn.
    
  

**Source:** `engine/game/entity.lua`

  ### `entity.GetViewPawn`
**Kind:** `fn`

Returns the pawn that the camera renders from now. In the in-eye and chase observer modes, this is the player that you spectate. In other cases it is the local player's own pawn.

Returns nil if the pawn or its observer services are not available. Use this function for
      HUD information that follows the camera. During a transition, the result can be different
      from the pawn that `GetLocalOrObservedPawn` returns.
    
  

**Source:** `engine/game/entity.lua`

  ### `entity.GetLocalOrObservedPawn`
**Kind:** `fn`

**Arguments:** `predicate?`

Returns the observed pawn while you spectate. In other cases it returns the same pawn as `entity.GetLocalPlayerPawn`.

Without a predicate, the observed pawn is used in the in-eye and chase observer modes. Pass a
      predicate to decide for each observer mode. The predicate gets the current observer mode as a
      `server.ObserverMode_t` value and the observed pawn. The values of the observer modes are in
      `schema.ObserverMode_t`. The observed pawn is **nil** if there is no observed pawn.

      The predicate is called only while you spectate. If the predicate returns `true` and there is
      no observed pawn, the result is nil. The default selection for the in-eye and chase modes
      works in the same way.
    
  

**Source:** `engine/game/entity.lua`

  ### `entity.GetLocalOrInEyePawn`
**Kind:** `fn`

Returns the observed pawn while you spectate in the in-eye mode. In other cases it returns the same pawn as `entity.GetLocalPlayerPawn`.

**Source:** `engine/game/entity.lua`

  ### `entity.GetLocalControlledPawn`
**Kind:** `fn`

Returns the pawn that the local player controls if it is not the player's own pawn. Returns **nil** in other cases. While you control a bot, this is the same pawn as `GetLocalPlayerPawn()`.

**Source:** `engine/game/entity.lua`

  ### `entity.GetLocalPawn`
**Kind:** `fn`

Returns the pawn that the local controller points to now, of any type. `entity.GetLocalPawnType` returns its type. This pawn can be a `client.C_CSObserverPawn`, so use `GetLocalPlayerPawn()` when you need a player pawn.

**Source:** `engine/game/entity.lua`

  ### `entity.GetOwnLocalPlayerPawn`
**Kind:** `fn`

Returns the local player's own player pawn. The result is the same while you control a bot or spectate another player. The pawn can be dead.

**Source:** `engine/game/entity.lua`

  ### `entity.GetLocalObserverPawn`
**Kind:** `fn`

Returns the observer pawn of the local player. Returns **nil** if there is no observer pawn. The observer pawn can exist while you do not spectate. Use `IsLocalObserving()` to check whether you spectate.

**Source:** `engine/game/entity.lua`

  ### `entity.GetObservedPawn`
**Kind:** `fn`

Returns the current observer target if it is a player pawn. Returns **nil** in other cases. In the free camera observer modes, the target can be different from the pawn that the camera renders from.

**Source:** `engine/game/entity.lua`

  ### `entity.GetObserverMode`
**Kind:** `fn`

Returns the current observer mode. Returns `OBS_MODE_NONE` if you do not spectate. The values of the observer modes are in `schema.ObserverMode_t`.

**Source:** `engine/game/entity.lua`

  ### `entity.GetLocalPawnType`
**Kind:** `fn`

Returns the type of the current pawn as an `entity.LocalPawnType` value. The value is `kNull` if there is no pawn, or if the type of the pawn is not known yet during a transition.

**Source:** `engine/game/entity.lua`

  ### `entity.IsViewInEye`
**Kind:** `fn`

Returns whether the view is a first person view. This is true when the local player spectates in the in-eye mode, or when the pawn of the player is in first person.

**Source:** `engine/game/entity.lua`

  ### `entity.IsLocalPlaying`
**Kind:** `fn`

Returns whether the current pawn of the local player is a player pawn. This can be the player's own pawn or a pawn that the player controls. The pawn type is `entity.LocalPawnType.kPlayer`. This function does not check whether the pawn is alive.

**Source:** `engine/game/entity.lua`

  ### `entity.IsLocalObserving`
**Kind:** `fn`

Returns whether the current pawn of the local player is an observer pawn, which means that the player spectates. The pawn type is `entity.LocalPawnType.kObserver`. An observer pawn that exists while it is not the current pawn does not count.

**Source:** `engine/game/entity.lua`

  ### `entity.IsLocalPawnOwn`
**Kind:** `fn`

Returns whether the current pawn is the local player's own player pawn. Returns false while the player controls another pawn, and while the player spectates.

**Source:** `engine/game/entity.lua`

  ### `entity.IteratePlayers`
**Kind:** `fn`

**Arguments:** `enemies_only?, include_dead?, relative_to?`

Returns an iterator over all players. Each step gives a controller and its pawn.

**Source:** `engine/game/entity.lua`

  ### `entity.GetPlayerControllers`
**Kind:** `fn`

**Arguments:** `enemies_only?, relative_to?`

Returns all player controllers.

**Source:** `engine/game/entity.lua`

  ### `entity.GetPlayerPawns`
**Kind:** `fn`

**Arguments:** `enemies_only?, include_dead?, relative_to?`

Returns all player pawns.

**Source:** `engine/game/entity.lua`

  ### `entity.GetPlayerResource`
**Kind:** `fn`

Returns the player resource entity.

**Source:** `engine/game/entity.lua`

  ### `entity.GetGameRules`
**Kind:** `fn`

Returns the game rules schema object. Returns **nil** if it is not available.

**Source:** `engine/game/entity.lua`

  ### `entity.GetGameModeRules`
**Kind:** `fn`

Returns the game mode rules schema object. Returns **nil** if it is not available.

**Source:** `engine/game/entity.lua`

  ### `entity.GetWeaponDataByClassName`
**Kind:** `fn`

**Arguments:** `classname`

Returns the weapon vdata for an entity class name. Returns nil if the name is not a known weapon class.

**Source:** `engine/game/entity.lua`

  ### `entity.GetWeaponDataByDesignerName`
**Kind:** `fn`

**Arguments:** `designer_name`

Returns the weapon vdata for a designer name. Returns nil if the name is not a known weapon.

**Source:** `engine/game/entity.lua`

  ### `entity.PawnObserveePredicate`
**Kind:** `type`

A function that decides whether to use the observed pawn instead of the local pawn. Return `true` to use the observed pawn.

The function is called only while the local player spectates. In other cases the local pawn
      is used.
    
  

**Source:** `engine/game/entity.lua`

  ### `entity.IteratePlayers.Key`
**Kind:** `type`

**Source:** `engine/game/entity.lua`

  ### `entity.IteratePlayers.Value`
**Kind:** `type`

**Source:** `engine/game/entity.lua`

  ### `entity.IteratePlayers.Table`
**Kind:** `type`

**Source:** `engine/game/entity.lua`

  ### `entity.IteratePlayers.Iterator`
**Kind:** `type`

**Source:** `engine/game/entity.lua`

  ### `entity.LocalPawnType`
**Kind:** `type`

The type of the current pawn of the local player. `entity.GetLocalPawnType` returns this value.


  

**Source:** `engine/game/entity.lua`


<Heading level={2} id="entity-1">
  Entity

   _()_
</Heading>



An entity object refers to one entity through the handle of that entity. After the entity is
deleted, `IsValid` returns `false`, and reading or writing a field throws an error. This is
also true when a new entity gets the same index.

`==` returns `true` if two objects refer to the same entity.

The entities that `entity` returns have their schema fields and the methods below.

Read a field with `pawn.m_iHealth` or `pawn:GetField("m_iHealth")`. Write a field with an
assignment or with `SetField`.

The `self` annotation of each method shows the entity classes that the method works with. For
example, player pawn methods work with `client.C_CSPlayerPawn`, and weapon methods need a
weapon. Use the Schema System and Schema Values namespaces to inspect types and to work with
field values.

### Methods: C\_BaseEntity


  ### `Entity:GetField`
**Kind:** `method`

`. Use it when the name of the field is held in a variable, or when the field has the same name as a method." args="name" sig="[{"s":"Entity:GetField"},{"s":"("},{"s":"name"},{"s":": "},{"s":"string","k":"prim","h":"/cs2/api_reference/lua/base#string"},{"s":")"},{"s":": "},{"s":"any","k":"prim"}]" ret="[[{"s":"any","k":"prim"}]]" params="[{"n":"name","t":[{"s":"string","k":"prim","h":"/cs2/api_reference/lua/base#string"}],"d":"The name of the schema field."}]" self="[{"s":"client.C_BaseEntity","k":"class","h":"/cs2/dumped_data/schema/client/C_BaseEntity"}]" nodiscard source="engine/engine/entity.lua" sourceHref="https://gitlab.com/neverlose/LuaLS-CS2/-/edit/master/library/api/engine/engine/entity.lua#L125" />

   = value`." args="name, value" sig="[{"s":"Entity:SetField"},{"s":"("},{"s":"name"},{"s":": "},{"s":"string","k":"prim","h":"/cs2/api_reference/lua/base#string"},{"s":", "},{"s":"value"},{"s":": "},{"s":"any","k":"prim"},{"s":")"}]" params="[{"n":"name","t":[{"s":"string","k":"prim","h":"/cs2/api_reference/lua/base#string"}],"d":"The name of the schema field."},{"n":"value","t":[{"s":"any","k":"prim"}],"d":"The value to write. It is converted to the schema type of the field."}]" self="[{"s":"client.C_BaseEntity","k":"class","h":"/cs2/dumped_data/schema/client/C_BaseEntity"}]" source="engine/engine/entity.lua" sourceHref="https://gitlab.com/neverlose/LuaLS-CS2/-/edit/master/library/api/engine/engine/entity.lua#L131" />

  

  

  

  

  

  

  

  

  

  

  

  

  

  

  
    
      This change is made on the client only. An update from the server can overwrite it.
    
  

  ### `Entity:GetAbsAngles`
**Kind:** `method`

Returns the absolute angles of the entity in the world.

**Source:** `engine/engine/entity.lua`

  ### `Entity:SetAbsAngles`
**Kind:** `method`

**Arguments:** `new_angles`

Sets the absolute angles of the entity in the world.

This change is made on the client only. An update from the server can overwrite it.
    
  

**Source:** `engine/engine/entity.lua`

  ### `Entity:GetAbsVelocity`
**Kind:** `method`

Returns the absolute velocity of the entity.

**Source:** `engine/engine/entity.lua`

  ### `Entity:GetEntityIdentity`
**Kind:** `method`

Returns the identity object of the entity.

**Source:** `engine/engine/entity.lua`

  ### `Entity:GetHitboxPosition`
**Kind:** `method`

**Arguments:** `i`

Returns the center of a hitbox volume in world space. Returns **nil** if it is not available.

For a sphere hitbox, this is the center of the sphere. For a box or capsule hitbox, this is
      the middle point between its two ends. The hitbox transform moves the point into world
      space. The result is the same as `entity:GetHitbox(i):GetCenter()`. To get the origin of the
      transform, use `Hitbox:GetOrigin()`.
    
  

**Source:** `engine/game/entity.lua`

  ### `Entity:GetHitbox`
**Kind:** `method`

**Arguments:** `i`

Returns the hitbox with the given index. Returns nil if the index is out of range.

Hitbox indices are different for each model. They depend on the hitbox set of the model and
      on the meshes that the bodygroups of the entity enable. Index 0 is the first hitbox of this
      model, and it can be any part of the body.
    
  

**Source:** `engine/game/entity.lua`

  ### `Entity:UpdateBones`
**Kind:** `method`

Calculates the world space bones of this entity again at its current position and pose. After this call, hitbox functions return the data of this frame.

Without this call, hitbox functions use the bones that the game calculated last. These bones
      can be behind the current position of the entity. For example, in `override_view` the bones
      of the local pawn are still from the previous frame. This function calculates all bones, so
      call it only before reads that need the latest data.

      Do not call it on a model whose bones follow the animation of another model, such as a HUD
      weapon that follows the HUD arms. The calculation uses the own pose of the model without the
      animation.
    
  

**Source:** `engine/game/entity.lua`

  ### `Entity:GetBoundingBox`
**Kind:** `method`

Returns the ESP bounds on the screen. The values are the top left corner, the bottom right corner, and the draw alpha.

For players and hostages, the bounds are around the body on the screen. For world entities,
      the bounds use the ESP label positions, so both corners can be the same point.

      Returns **nil** if there is no local or observed pawn, the scene or collision data of the
      entity is missing, the draw alpha of the entity is zero, or the ESP cannot calculate valid
      bounds. Valid bounds can be outside the screen, so a result does not mean that the entity is
      visible.
    
  

**Source:** `engine/game/entity.lua`

  ### `Entity:GetMaxHealth`
**Kind:** `method`

Returns the maximum health of the entity.

**Source:** `engine/game/entity.lua`

  ### `Entity:IsEnemy`
**Kind:** `method`

**Arguments:** `to?`

Returns true if this entity is an enemy of the given entity. Without an argument, it compares with the pawn from `entity.GetLocalPlayerPawn()`, which can be a controlled bot. If there is no local pawn, it returns true.

The default pawn does not change to the spectated player, so the result stays the same when
      the local player dies and spectates an enemy. To follow the camera, pass
      `entity.GetViewPawn()`.
    
  

**Source:** `engine/game/entity.lua`


### Methods: C\_CSPlayerPawnBase


  ### `Entity:GetPlayerController`
**Kind:** `method`

Returns the controller of this pawn.

**Source:** `engine/game/entity.lua`

  ### `Entity:GetActiveWeapon`
**Kind:** `method`

Returns the active weapon of this pawn.

**Source:** `engine/game/entity.lua`

  ### `Entity:GetMyWeapons`
**Kind:** `method`

Returns all weapons that this pawn holds.

The collection shows the current state of the game. Its indices start at 0.
    
  

**Source:** `engine/game/entity.lua`

  ### `Entity:GetEyeAngles`
**Kind:** `method`

Returns the eye angles of this pawn.

**Source:** `engine/game/entity.lua`

  ### `Entity:GetEyePosition`
**Kind:** `method`

Returns the eye position of this pawn.

**Source:** `engine/game/entity.lua`

  ### `Entity:GetAbsViewOffset`
**Kind:** `method`

Returns the view offset of this pawn from its origin.

**Source:** `engine/game/entity.lua`


### Methods: CCSPlayerController


  ### `Entity:GetPlayerSlot`
**Kind:** `method`

Returns the player slot index of this controller.

**Source:** `engine/game/entity.lua`

  ### `Entity:GetPlayerUserId`
**Kind:** `method`

Returns the user ID of this controller.

**Source:** `engine/game/entity.lua`

  ### `Entity:GetPawn`
**Kind:** `method`

Returns the active pawn of this controller. It can be a player pawn or an observer pawn.

**Source:** `engine/game/entity.lua`

  ### `Entity:GetPlayerPawn`
**Kind:** `method`

Returns the player pawn of this controller. The player pawn can exist while the controller spectates.

**Source:** `engine/game/entity.lua`

  ### `Entity:GetObserverPawn`
**Kind:** `method`

Returns the observer pawn of this controller. Returns **nil** if it is not available. The observer pawn can exist while it is not the active pawn of the controller.

**Source:** `engine/game/entity.lua`


### Methods: C\_CSPlayerPawn


  ### `Entity:GetPlayerMaxSpeed`
**Kind:** `method`

Returns the maximum movement speed of this pawn.

**Source:** `engine/game/entity.lua`

  ### `Entity:ScaleDamage`
**Kind:** `method`

**Arguments:** `damage, weapon_data, hitgroup, attacker?`

Scales raw damage by the armor and hitgroup modifiers of this pawn. This function does not fire a shot.

**Source:** `engine/game/entity.lua`

  ### `Entity:GetBombDamage`
**Kind:** `method`

**Arguments:** `bomb?`

Estimates the health that this pawn would lose if the planted bomb exploded now.

On maps with baked bomb damage, the game's own damage table is used, so both values are
      equal. On other maps, the damage falls off with the distance and armor is applied. The
      minimum and maximum are measured over the body of the pawn. Returns `0, 0` if the pawn is
      out of range. The result does not depend on the bomb visuals settings.
    
  

**Source:** `engine/game/entity.lua`

  ### `Entity:GetPlayerSimulation`
**Kind:** `method`

**Arguments:** `optimizations?`

Copies the state of this pawn into a separate simulation snapshot. Use `simulation.Player:RunCommand` to move the snapshot forward. This does not change the real player.

By default, the simulation skips the weapon simulation and the costly movement details. Pass
      `simulation.SimOpt.kNone` for the full simulation.

      For the local player, use the snapshot that the movement callback gives you.
    
  

**Source:** `engine/game/entity.lua`


### Methods: CCSPlayerController / C\_CSPlayerPawnBase


  ### `Entity:GetPlayerName`
**Kind:** `method`

Returns the display name of the player.

**Source:** `engine/game/entity.lua`

  ### `Entity:GetPlayerAvatar`
**Kind:** `method`

Returns the 32x32 Steam avatar of the player. The name ESP and the spectator list draw the same avatar. For a bot, this function returns the icon of its team, which is larger. Pass a size to `render.DrawImage` to draw both at the same size.

The avatar is loaded one or two frames after the player first appears. Until then, and for a
      user without an avatar, this function returns an empty image. Its `failed` field is true, and
      `render.DrawImage` does not draw it. Call this function in every frame, and do not keep the
      result.
    
  

**Source:** `engine/game/entity.lua`


### Methods: C\_CSWeaponBase


  ### `Entity:GetWeaponData`
**Kind:** `method`

Returns the vdata of the weapon.

**Source:** `engine/game/entity.lua`

  ### `Entity:GetInaccuracy`
**Kind:** `method`

Returns the current inaccuracy of this weapon.

**Source:** `engine/game/entity.lua`

  ### `Entity:GetSpread`
**Kind:** `method`

Returns the current spread of this weapon.

**Source:** `engine/game/entity.lua`


### Methods: C\_EconEntity


  ### `Entity:GetItem`
**Kind:** `method`

Returns the econ item view of this entity.

**Source:** `engine/game/entity.lua`


### Methods: C\_BaseModelEntity


  ### `Entity:SetGlowColor`
**Kind:** `method`

**Arguments:** `color`

Adds this entity to the glow list of the current frame with the given color. Call it in every frame while you draw. Entities that cannot be drawn are ignored.

**Source:** `engine/game/entity.lua`


## Hitbox _()_



One hitbox of the skeleton of an entity. Its fields are read from the `modellib.CHitBox` of the
model through the schema, so the field names are the names that the game uses.

Hitbox data is read-only and moves with the entity when the entity is animated. Use it only
while the entity is valid. Reading a field throws an error if the hitbox no longer exists.


  ### `Hitbox:GetOrigin`
**Kind:** `method`

Returns the origin of the world transform of this hitbox. Returns nil if it is not available. The transform can come from a bone or from a shape without a bone. To get the center of the hitbox volume, use `Hitbox:GetCenter()`.

**Source:** `engine/game/entity.lua`

  ### `Hitbox:GetTransform`
**Kind:** `method`

Returns the world transform of this hitbox. Returns nil if it is not available.

The transform places the shape of the hitbox in the world. This also works for shapes that do
      not follow a bone.
    
  

**Source:** `engine/game/entity.lua`

  ### `Hitbox:GetCenter`
**Kind:** `method`

Returns the center of the volume of this hitbox in world space. Returns nil if it is not available. The result is the same as `entity:GetHitboxPosition(index)`.

**Source:** `engine/game/entity.lua`

  ### `Hitbox:GetCapsule`
**Kind:** `method`

Returns this hitbox as a capsule in world space. The values are the centers of the two end spheres and the radius, with the scale of the transform applied. For a sphere hitbox, both ends are the same point. Returns **nil** for other shapes, or if the hitbox is not available.

To draw the capsule, project each end with `projection.SphereEllipse(end, radius)`. To test
      whether a segment hits it, pass the results to `trace.IntersectCapsule`.
    
  

**Source:** `engine/game/entity.lua`

  ### `Hitbox:GetOwnerEntity`
**Kind:** `method`

Returns the entity that this hitbox belongs to.

**Source:** `engine/game/entity.lua`

  ### `Hitbox:GetIndex`
**Kind:** `method`

Returns the index of this hitbox. This is the index that was passed to `entity:GetHitbox`.

**Source:** `engine/game/entity.lua`

---


## Globals

<!-- Source: cs2/api_reference/engine/globals.md -->

---
title: Global Variables
icon: hourglass
sidebar_position: 2
---

# Global Variables

## globals _()_

Global engine variables for timing and tick state.

If there is no network game client, every field returns nil and assignments do nothing.
Assigning a value to `tickinterval` while there is a network game client throws an error.


  ### `globals.absoluteframetime`
**Kind:** `prop`

The render frame time without `host_timescale` and without the server and replay multipliers.

**Source:** `engine/game/globals.lua`

  ### `globals.curtime`
**Kind:** `prop`

The current time of the main world simulation, equal to the server tick multiplied by `tickinterval`. The value is correct only during simulation.

**Source:** `engine/game/globals.lua`

  ### `globals.framecount`
**Kind:** `prop`

The number of frames since the game started. It increases every frame, also while the game is paused.

**Source:** `engine/game/globals.lua`

  ### `globals.frametime`
**Kind:** `prop`

The time of the last server or client frame. This is the time that must be simulated.

**Source:** `engine/game/globals.lua`

  ### `globals.in_simulation`
**Kind:** `prop`

True in the simulation stage, and false in the rendering stage.

**Source:** `engine/game/globals.lua`

  ### `globals.interpolation_amount`
**Kind:** `prop`

The part of the tick that is not complete in this frame, from 0 to 1. The value is correct only in the rendering stage, when `in_simulation` is false.

**Source:** `engine/game/globals.lua`

  ### `globals.interpolation_amount_prediction`
**Kind:** `prop`

The part of the input tick that is not complete in this frame, from 0 to 1. The value is correct only in the rendering stage, when `in_simulation` is false.

**Source:** `engine/game/globals.lua`

  ### `globals.maxplayers`
**Kind:** `prop`

The maximum number of players that the server allows. This is the number of controllers.

**Source:** `engine/game/globals.lua`

  ### `globals.realtime`
**Kind:** `prop`

The time in seconds since the game started. Do not use it to measure short times between frames.

**Source:** `engine/game/globals.lua`

  ### `globals.sim_ticks_this_frame_index`
**Kind:** `prop`

The index of the tick that is simulated now in this frame.

**Source:** `engine/game/globals.lua`

  ### `globals.sim_ticks_this_frame_total`
**Kind:** `prop`

The total number of ticks that are simulated in this frame.

**Source:** `engine/game/globals.lua`

  ### `globals.tickcount`
**Kind:** `prop`

The number of main world simulation ticks. It increases also while the game is paused.

**Source:** `engine/game/globals.lua`

  ### `globals.tickinterval`
**Kind:** `prop`

The length of one tick in seconds. Read-only.

**Source:** `engine/game/globals.lua`

  ### `globals.unfilteredframetime`
**Kind:** `prop`

The render frame time without the minimum and maximum limits.

**Source:** `engine/game/globals.lua`

---


## Memory

<!-- Source: cs2/api_reference/engine/memory.md -->

---
title: Memory
icon: microchip
---

# Memory

<Heading level={2} id="memory-1">
  memory

   _()_
</Heading>

Finds game interfaces and searches the memory of loaded modules for byte patterns.

<Warning>
  These functions are for advanced users. They return raw native addresses. If you use a wrong
  pattern or a wrong address, the game will crash. The script must list `ffi` in the requirements
  of its `extension.json`. Without it, these functions are **nil**.
</Warning>


  ### `memory.CreateInterface`
**Kind:** `fn`

**Arguments:** `module, interface`

Returns a game interface that a loaded module exports. Returns **nil** if the module is not loaded or does not export an interface with this name.

Interface names can change after game updates.
    
  

**Source:** `engine/engine/memory.lua`

  ### `memory.FindPattern`
**Kind:** `fn`

**Arguments:** `module, pattern, find_all?`

Searches a loaded module for a byte pattern and returns the addresses where it is found. By default, only the first match is returned. If the module is not loaded, nothing is returned.

**Source:** `engine/engine/memory.lua`

  ### `memory.BindVirtual`
**Kind:** `fn`

**Arguments:** `module, interface, index, ...?`

Creates a Lua function that calls a virtual function of an interface.

The function finds `interface` in `module`, reads the function pointer at slot `index` of the
      virtual table of the interface, and casts it to the given C type. The interface instance is
      stored in the returned function, so you do not pass the `this` pointer when you call it.

      Throws an error if the interface is not found or the C type is not valid. The slot index and
      the C type must match the interface in the loaded module.

      This function exists only in scripts that have access to `ffi`.
    
  

**Source:** `engine/game/memory.lua`

  ### `memory.VirtualThunk`
**Kind:** `fn`

**Arguments:** `index, ...?`

Creates a Lua function that calls a virtual function on any instance that you pass to it.

The returned function takes the instance pointer as its first argument. When you call it, it
      reads slot `index` of the virtual table of that instance. You can use it with every object
      that has the same virtual table layout. `memory.BindVirtual` instead stores one instance in
      the returned function.

      Throws an error if the C type is not valid. The returned function throws an error if the
      instance is nil or a null pointer. The slot index and the C type must match every instance
      that you pass.

      This function exists only in scripts that have access to `ffi`.
    
  

**Source:** `engine/game/memory.lua`

  ### `memory.BindPattern`
**Kind:** `fn`

**Arguments:** `module, interface, signature, ...?`

Creates a Lua function that calls a function found by an IDA style byte pattern.

The function finds `interface` in `module` to get the instance. Then it searches `module` for
      `signature` to find the address of the function and casts that address to the given C type.
      The interface instance is stored in the returned function and is passed as the first argument
      when you call it.

      Throws an error if the interface is not found, the pattern is not found, or the C type is not
      valid. The pattern and the C type must match the function in the loaded module.

      This function exists only in scripts that have access to `ffi`.
    
  

**Source:** `engine/game/memory.lua`

---


## Nav

<!-- Source: cs2/api_reference/engine/nav.md -->

---
title: Nav
icon: footprints
---

# Nav

<Heading level={2} id="nav-1">
  nav

   _()_
</Heading>

Finds walking paths on the navigation mesh of the current map.

The mesh is read from the `.nav` file of the map on the first call after a level loads. It is
removed when the map changes or when you disconnect. Outside a game, or on a map without a
`.nav` file, `nav.IsLoaded` returns `false` and the other functions return `nil`.

Paths are for a player on foot. They can use steps, drops and jumps up to the height of a
crouch jump.

Distances are in walk units. One walk unit is the time a player needs to run one world unit
with a knife at 250 units per second. Drops and jumps take more time, so they count as more
than their length. Divide a distance by 250 to estimate the time in seconds.


  ### `nav.IsLoaded`
**Kind:** `fn`

Returns whether the navigation mesh of the current map is loaded. The first call on a new level builds the mesh.


      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/engine/nav:0:7b9b84c6e519"
      if not nav.IsLoaded() then print("no nav mesh on this map") end
      ```
    
  

**Source:** `engine/game/nav.lua`

  ### `nav.Snap`
**Kind:** `fn`

**Arguments:** `pos, max_distance?`

Finds the point on the navigation mesh that is closest to a world position.

Returns the point on the mesh and its distance from `pos`. Returns `nil` if there is no
      walkable area within `max_distance`. Throws an error if `pos` is not a finite vector or if
      `max_distance` is negative or not finite.
    

    
      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/engine/nav:1:9f9a465a39ff"
      local point, distance = nav.Snap(Vector(-300, 1200, 100))
      if point then
          print("nearest walkable point ", point, ", ", distance, " units away")
      end
      ```
    
  

**Source:** `engine/game/nav.lua`

  ### `nav.GetDistance`
**Kind:** `fn`

**Arguments:** `from, to`

Returns the shortest walking distance between two world positions, in walk units.

Both positions are moved to the closest point on the mesh within 256 units. Returns `nil` if a
      position has no mesh within 256 units, or if no walkable route connects the two positions.
      Throws an error if a position is not a finite vector.
    

    
      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/engine/nav:2:acb6f9fe0826"
      local pawn = entity.GetLocalPlayerPawn()
      local distance = pawn and nav.GetDistance(pawn:GetAbsOrigin(), Vector(-300, 1200, 100))
      if distance then print(("about %.1f s away"):format(distance / 250)) end
      ```
    
  

**Source:** `engine/game/nav.lua`

  ### `nav.FindPath`
**Kind:** `fn`

**Arguments:** `from, to`

Returns the shortest walking path between two world positions, as a list of points from `from` to `to`.

Both positions are moved to the closest point on the mesh within 256 units to find the
      route. The path is smoothed, so it cuts corners where the mesh allows it. Returns `nil` if a
      position has no mesh within 256 units, or if no walkable route connects the two positions.
      Throws an error if a position is not a finite vector.
    

    
      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/engine/nav:3:2b73dbfcbb19"
      local pawn = entity.GetLocalPlayerPawn()
      local path = pawn and nav.FindPath(pawn:GetAbsOrigin(), Vector(-300, 1200, 100))
      if path then
          for i = 2, #path do
              print(("segment %d: %.0f units"):format(i - 1, path[i - 1]:DistTo(path[i])))
          end
      end
      ```
    
  

**Source:** `engine/game/nav.lua`

---


## Random

<!-- Source: cs2/api_reference/engine/random.md -->

---
title: Random
icon: dices
---

# Random

<Heading level={2} id="random-1">
  random

   _()_
</Heading>

Uses the random number generator of the engine. It is a different generator from the one
used by `math.random`.

The functions `random.Int`, `random.Float`, `random.FloatExp` and `random.GaussianFloat` use
the shared generator of the game. You cannot set a seed for it, because a new seed would change
the random results of the game itself. Use `random.Stream` to get a sequence of numbers that
you can repeat with a seed.


  ### `random.Int`
**Kind:** `fn`

**Arguments:** `min, max`

Returns a random integer from `min` to `max`. Both `min` and `max` can be returned. If `min` is greater than `max`, the two values are swapped.

**Source:** `engine/engine/random.lua`

  ### `random.Float`
**Kind:** `fn`

**Arguments:** `min, max`

Returns a random floating point number from `min` to `max`. If `min` is greater than `max`, the two values are swapped.

**Source:** `engine/engine/random.lua`

  ### `random.FloatExp`
**Kind:** `fn`

**Arguments:** `min, max, exponent?`

Returns a random floating point number from `min` to `max`. First, a uniform random number from 0 up to 1 is made. This number is raised to the power of `exponent`, and then it is scaled to the range. If `min` is greater than `max`, the two values are swapped.

**Source:** `engine/engine/random.lua`

  ### `random.GaussianFloat`
**Kind:** `fn`

**Arguments:** `mean?, stddev?`

Returns a random number with a normal distribution.

**Source:** `engine/engine/random.lua`

  ### `random.Stream`
**Kind:** `fn`

**Arguments:** `seed?`

Creates a separate generator that uses the same algorithm as the engine, `CUniformRandomStream`. A seed always gives the same sequence of numbers as the engine gives for that seed. Using a stream does not change the shared generator of the game.

`seed` can be any signed or unsigned 32 bit value, from `-2^31` to `2^32 - 1`. Any other value
      throws an error. Unsigned values are converted to signed values in the same way as the engine
      converts them. The engine gives the same sequence for `seed` and `-seed`, and it uses the seed
      `1` in place of `0`.
    

    
      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/engine/random:0:3909439b8c7b"
      local a, b = random.Stream(1337), random.Stream(1337)
      assert(a:Float(0, 360) == b:Float(0, 360)) -- same seed, same sequence
      ```
    
  

**Source:** `engine/engine/random.lua`


## random.Stream _()_

A generator with a seed, created by `random.Stream`. Its methods pass `min` and `max` to the
engine exactly as they are given. They never swap the two values.


  ### `random.Stream:SetSeed`
**Kind:** `method`

**Arguments:** `seed`

Starts the stream again with a new seed. The seed follows the same rules as the seed of `random.Stream`.

**Source:** `engine/engine/random.lua`

  ### `random.Stream:Int`
**Kind:** `method`

**Arguments:** `min, max`

Returns a random integer from `min` to `max`. Both `min` and `max` can be returned. If `max` is less than `min`, `min` is returned and the stream does not move on to the next number.

**Source:** `engine/engine/random.lua`

  ### `random.Stream:Float`
**Kind:** `method`

**Arguments:** `min?, max?`

Returns a random floating point number from `min` to `max`.

**Source:** `engine/engine/random.lua`

  ### `random.Stream:FloatExp`
**Kind:** `method`

**Arguments:** `min?, max?, exponent?`

Returns a random floating point number from `min` to `max`. First, a uniform random number from 0 up to 1 is made. This number is raised to the power of `exponent`, and then it is scaled to the range.

**Source:** `engine/engine/random.lua`

  ### `random.Stream:GaussianFloat`
**Kind:** `method`

**Arguments:** `mean?, stddev?`

Returns a random number with a normal distribution.

**Source:** `engine/engine/random.lua`

  ### `random.Stream:Clone`
**Kind:** `method`

Returns a copy of the stream in its current state. The copy continues the same sequence. Using one of them does not change the other.

**Source:** `engine/engine/random.lua`

---


## Schema

<!-- Source: cs2/api_reference/engine/schema.md -->

---
title: Schema System
icon: list-tree
sidebar_position: 4
---

# Schema System

## schema _()_

Gives access to the schema system of the game. The schema system holds the information about
every networked class, its fields, and the types of the fields. Use it to find class types and
enums by name, so that you do not need to write offsets in your code.

Index this table with a name. If a class with this name exists, a `schema.Type` is returned.
If an enum with this name exists, a table that maps the names of the enum values to their
numbers is returned. If the name is the start of the name of a scope, such as `client`, a
`schema.Scope` is returned. Otherwise, **nil** is returned.

This namespace describes types. It does not hold objects of these types. When you read a
value, a structured field can return a `value.*` view, which holds a data pointer and a schema
type. Other fields return plain Lua values or cdata.


  ### `schema[string]`
**Kind:** `prop`

**Source:** `engine/engine/schema.lua`


## schema.Scope _()_

A schema scope. A scope is a game module, for example `client` or `server`. Index it with the
name of a class to get the `schema.Type` of that class in this scope. Index it with the name of
an enum to get a table that maps the names of the enum values to their numbers.


  ### `schema.Scope[string]`
**Kind:** `prop`

**Source:** `engine/engine/schema.lua`


## schema.Type _()_

Describes a schema type. The length operator `#` returns the size of the type in bytes.

Call the type with a pointer to data of this type to read the value at that address. This
does not create a new object. The result can be an entity, a number, cdata or a schema value.
Use the returned views only while the object that owns the data is still valid.


  ### `schema.Type(pointer)`
**Kind:** `ctor`

**Source:** `engine/engine/schema.lua`

  ### `#schema.Type`
**Kind:** `meta`

**Source:** `engine/engine/schema.lua`

  ### `schema.Type:GetSize`
**Kind:** `method`

Returns the size of the type in bytes.

**Source:** `engine/engine/schema.lua`

  ### `schema.Type:GetAlignment`
**Kind:** `method`

Returns the alignment of the type in bytes.

**Source:** `engine/engine/schema.lua`

  ### `schema.Type:GetInstance`
**Kind:** `method`

Returns the raw `CSchemaType` pointer.

<Warning>
        This function is for advanced users. You can only use the pointer through FFI.
      </Warning>
    
  

**Source:** `engine/engine/schema.lua`

  ### `schema.Type:GetName`
**Kind:** `method`

Returns the name of the schema type.

**Source:** `engine/engine/schema.lua`

  ### `schema.Type:GetMetadata`
**Kind:** `method`

Returns the metadata of the type. Class, atomic and enum types have metadata. For other types, an empty table is returned. Use `Category` to find the category of a type.

<Warning>
        This function is for advanced users. The metadata names can change between game builds. The
        values are native pointers, and you can only use them through FFI.
      </Warning>
    
  

**Source:** `engine/engine/schema.lua`

  ### `schema.Type:GetMetadata`
**Kind:** `method`

**Arguments:** `classfield`

Returns the metadata of a field of a class type. If the type is not a class or has no field with this name, an empty table is returned.

**Source:** `engine/engine/schema.lua`

  ### `schema.Type:GetFieldOffset`
**Kind:** `method`

**Arguments:** `classfield`

Returns the offset in bytes of a field of a class type. Returns **nil** if the class has no field with this name, or if the type is not a class.

**Source:** `engine/engine/schema.lua`

  ### `schema.Type:GetBaseClasses`
**Kind:** `method`

Returns one table for each direct base class of a class type. Each table is a separate return value, in the order in which the base classes are declared. Each table holds the type of the base class and its offset in bytes. Use `{type:GetBaseClasses()}` to put all of them in a table. For other types, nothing is returned.

**Source:** `engine/engine/schema.lua`

  ### `schema.Type:GetFields`
**Kind:** `method`

**Arguments:** `no_recurse?`

Returns the names of the fields of a class type. The fields of the class itself come first, and then the fields of its base classes. Returns **nil** if the type is not a class.

**Source:** `engine/engine/schema.lua`

  ### `schema.Type:GetFieldType`
**Kind:** `method`

**Arguments:** `classfield`

Returns the schema type of a field of a class type. Returns **nil** if the class has no field with this name, or if the type is not a class.

**Source:** `engine/engine/schema.lua`

  ### `schema.Type:Category`
**Kind:** `method`

Returns the category of the type. The category is one of `builtin`, `pointer`, `bitfield`, `array`, `atomic`, `class`, `enum` and `invalid`.

**Source:** `engine/engine/schema.lua`

  ### `schema.Type:Atomic`
**Kind:** `method`

Returns the kind of an atomic type. Returns **nil** for other types. The kind is one of these values.

`plain` is a type without type arguments.
      `t` is a type with one type argument.
      `collection_of_t` is a collection of one type argument.
      `tt` is a type with two type arguments.
      `i` is a type with an integer argument.
    
  

**Source:** `engine/engine/schema.lua`

  ### `schema.Type:Inner`
**Kind:** `method`

Returns the inner types of the type, each as a separate return value. For a pointer, this is the type that it points to. For a fixed array, this is the type of the elements. For an atomic type with type arguments, these are the type arguments in order. For other types, nothing is returned.

**Source:** `engine/engine/schema.lua`

  ### `schema.Type:Count`
**Kind:** `method`

Returns the number of elements of a fixed array type. Returns **nil** for other types.

**Source:** `engine/engine/schema.lua`

  ### `schema.Type:NumBits`
**Kind:** `method`

Returns the width in bits of a bitfield type. Returns **nil** for other types.

**Source:** `engine/engine/schema.lua`

  ### `schema.Type:Flags`
**Kind:** `method`

Returns the flags of a class type. Returns **nil** for other types. The `value` field holds all flag bits as one number, and each boolean field holds one of the bits. The optional fields exist only in game builds whose schema defines them. In other builds they are **nil**.

**Source:** `engine/engine/schema.lua`

  ### `schema.Type:GetScope`
**Kind:** `method`

Returns the schema scope that the type belongs to.

**Source:** `engine/engine/schema.lua`

  ### `schema.Type.Meta`
**Kind:** `type`

A table that maps metadata names to native pointers.

**Source:** `engine/engine/schema.lua`

---


## Sounds

<!-- Source: cs2/api_reference/engine/sounds.md -->

---
title: Sounds
icon: volume-2
---

# Sounds

## sounds.PlayEvent.Options _()_

Options for `sounds.PlayEvent`.


  ### `sounds.PlayEvent.Options.lifetime`
**Kind:** `prop`

The time in seconds until the emitter is removed. The default is `30`. Pass `0` to keep the emitter until you call `Destroy`, the script unloads, or the map ends. Use `0` for looping sounds. A negative value throws an error.

**Source:** `engine/game/sounds.lua`

  ### `sounds.PlayEvent.Options.stop_on_new`
**Kind:** `prop`

Whether to stop the previous event when the emitter starts again. The default is `false`.

**Source:** `engine/game/sounds.lua`


## sounds.Sound _()_

A soundevent that this script started at a world position.

The sound does not stop when no variable refers to this object anymore. It plays until you
call `Destroy`, its `lifetime` ends, the script unloads, or the map ends. After that, every
method returns `false` or `nil`.

The game does not tell when a sound that plays once has finished, so `IsValid` returns `true`
until one of the events above happens.


  ### `sounds.Sound:IsValid`
**Kind:** `method`

Returns whether the sound still plays from the emitter of this object.

**Source:** `engine/game/sounds.lua`

  ### `sounds.Sound:GetName`
**Kind:** `method`

Returns the name of the soundevent that started the sound. Returns `nil` after the sound is removed.

**Source:** `engine/game/sounds.lua`

  ### `sounds.Sound:SetOrigin`
**Kind:** `method`

**Arguments:** `origin`

Moves the emitter to a new position. A looping sound moves with the emitter. A sound that plays once and has finished does not play again. Throws an error if `origin` is not a finite vector.

**Source:** `engine/game/sounds.lua`

  ### `sounds.Sound:Destroy`
**Kind:** `method`

Stops the sound and removes its emitter.

**Source:** `engine/game/sounds.lua`


<Heading level={2} id="sounds-1">
  sounds

   _()_
</Heading>

Plays sound files in 2D and soundevents at world positions.

A script can have at most 32 world space sounds at the same time.


  ### `sounds.PlayFile`
**Kind:** `fn`

**Arguments:** `path, volume?`

Plays a sound file by name in 2D. The volume is multiplied by the master volume of the game.

The file must be in a sound search path of the game. A sound file cannot play at a world
      position. To play a sound at a world position, use `sounds.PlayEvent` with a soundevent.
    

    
      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/engine/sounds:0:f9b06593884d"
      sounds.PlayFile("hitmarker", 0.5)
      ```
    
  

**Source:** `engine/game/sounds.lua`

  ### `sounds.PlayEvent`
**Kind:** `fn`

**Arguments:** `event, origin, options?`

Plays a soundevent at a world position. The definition of the event sets its volume and falloff. The game changes the direction and the volume of the sound by the position of the listener, in the same way as for other world sounds.

Only soundevents can play at a world position, for example `"Weapon_AK47.Single"` or
      `"Inferno.Loop"`. The sound file names that `sounds.PlayFile` takes do not work here.

      Returns `nil` outside a game, for an unknown soundevent, or when the game cannot create the
      emitter in this frame. Throws an error if `event` is empty, `origin` is not a finite vector,
      `options` is not a table, or the script already has 32 sounds.
    

    
      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/engine/sounds:1:5273519775ed"
      local pawn = entity.GetLocalPlayerPawn()
      if pawn then
          local origin = pawn:GetAbsOrigin()
          sounds.PlayEvent("Weapon_AK47.Single", origin + Vector(300, 0, 0))

          local loop = sounds.PlayEvent("Inferno.Loop", origin, { lifetime = 0 })
          events.render:Set(function()
              if loop and loop:IsValid() and pawn:IsValid() then loop:SetOrigin(pawn:GetAbsOrigin()) end
          end)
      end
      ```
    
  

**Source:** `engine/game/sounds.lua`

---


## Trace

<!-- Source: cs2/api_reference/engine/trace.md -->

---
title: Trace
icon: squares-intersect
---

# Trace

<Heading level={2} id="trace-1">
  trace

   _()_
</Heading>

Traces rays and shapes through the world and returns what they hit.


  ### `trace.Filter`
**Kind:** `fn`

**Arguments:** `ignore_entities?, interacts_with?, collision_group?, object_set?`

Creates a trace filter. You can give entities that the trace skips, and the layers, collision group and object types that the trace uses.

To skip entities, pass one entity or a table of entities. Up to two entities are fast. Each
      entity after the second makes traces slower.
    
  

**Source:** `engine/engine/trace.lua`

  ### `trace.Filter`
**Kind:** `fn`

**Arguments:** `params`

Creates a trace filter from a table of settings. The keys of the table are the writable fields of `trace.Filter`.

Fields that you do not set keep the values of an empty filter. In an empty filter,
      `interacts_with` is `0`, so the trace hits nothing. `collision_group` is `CONDITIONALLY_SOLID`
      and `object_set_mask` is `ALL`.
    
  

**Source:** `engine/engine/trace.lua`

  ### `trace.Ray`
**Kind:** `fn`

**Arguments:** `startpos, endpos, filter`

Traces a ray, which is a single point, from `startpos` to `endpos`.

**Source:** `engine/engine/trace.lua`

  ### `trace.Sphere`
**Kind:** `fn`

**Arguments:** `startpos, endpos, radius, filter`

Moves a sphere from `startpos` to `endpos` and returns what it hits.

**Source:** `engine/engine/trace.lua`

  ### `trace.Capsule`
**Kind:** `fn`

**Arguments:** `startpos, endpos, p1, p2, radius, filter`

Moves a capsule from `startpos` to `endpos` and returns what it hits. The end points of the capsule are relative to the position of the moving capsule.

**Source:** `engine/engine/trace.lua`

  ### `trace.AABB`
**Kind:** `fn`

**Arguments:** `startpos, endpos, mins, maxs, filter`

Moves an axis aligned box from `startpos` to `endpos` and returns what it hits. The corners of the box are relative to the position of the moving box.

**Source:** `engine/engine/trace.lua`

  ### `trace.ConvexHull`
**Kind:** `fn`

**Arguments:** `startpos, endpos, verts, filter`

Moves a convex hull from `startpos` to `endpos` and returns what it hits. The vertices are relative to the position of the moving hull.

**Source:** `engine/engine/trace.lua`

  ### `trace.Bullet`
**Kind:** `fn`

**Arguments:** `startpos, endpos, attacker, vdata?, filter?`

Simulates a bullet and calculates its damage through surfaces and penetration. This function does not fire a shot.

The bullet starts at `startpos` and moves in the direction of `endpos`. It travels the full
      range of the weapon, so it can go past `endpos`. It can penetrate up to 4 surfaces.

      Returns one value for each impact, starting with the first impact. Use `{trace.Bullet(...)}`
      to put the values in a table. Throws an error if there is no weapon data.
    
  

**Source:** `engine/game/trace.lua`

  ### `trace.IntersectCapsule`
**Kind:** `fn`

**Arguments:** `startpos, endpos, p1, p2, radius`

Tests whether the segment from `startpos` to `endpos` hits a capsule in world space.

This function only does math on the shape. It does not check the world, so it works for
      shapes that do not exist in the game, at any position and rotation. To rotate a capsule,
      transform the centers of its ends, for example with
      `CTransform(origin, rotation):TransformVector(Vector(0, 0, 8))`.

      Returns **nil** if the segment does not hit the capsule. A fraction of 0 means that `startpos`
      is already inside the capsule.

      You can pass the capsule of a hitbox directly, for example
      `trace.IntersectCapsule(startpos, endpos, hitbox:GetCapsule())`.

      World geometry is ignored. To check whether a bullet reaches the shape, compare the fraction
      with the result of `trace.Ray` or `trace.Bullet` for the same segment.
    
  

**Source:** `engine/game/trace.lua`

  ### `trace.IntersectSphere`
**Kind:** `fn`

**Arguments:** `startpos, endpos, center, radius`

Tests whether the segment from `startpos` to `endpos` hits a sphere in world space. In the same way as `trace.IntersectCapsule`, this function only does math on the shape and does not check the world.

Returns **nil** if the segment does not hit the sphere. A fraction of 0 means that `startpos`
      is already inside the sphere.
    
  

**Source:** `engine/game/trace.lua`

  ### `trace.CONTENTS`
**Kind:** `type`

The interaction layer bits for the `interacts_with`, `interacts_as` and `interacts_exclude` fields of a trace filter. Use `bit.bor` to combine bits, or use a predefined `trace.MASK`.


  

**Source:** `engine/engine/trace.lua`

  ### `trace.MASK`
**Kind:** `type`

Predefined combinations of `trace.CONTENTS` bits for common traces.


  

**Source:** `engine/engine/trace.lua`

  ### `trace.RnQueryObjectSet`
**Kind:** `type`

The object types that a trace checks. Use them for the `object_set` argument of `trace.Filter` and for the `object_set_mask` field of a filter. Use `bit.bor` to combine bits.



    
      <Note>
        The members and their values depend on the game build. `STATIC`, `DYNAMIC`,
        `ALL_GAME_ENTITIES`, `ALL`, `KEYFRAMED_ONLY` and `DYNAMIC_ONLY` exist in every build.
      </Note>
    
  

**Source:** `engine/engine/trace.lua`

  ### `trace.BuiltInCollisionGroup_t`
**Kind:** `type`

The collision groups that the engine defines for every game. Use them for the `collision_group` argument of `trace.Filter` and for the `collision_group` field of a filter.


  

**Source:** `engine/engine/trace.lua`

  ### `trace.StandardCollisionGroups_t`
**Kind:** `type`

The collision groups that Source 2 games share. Their values start at the value of `BuiltInCollisionGroup_t.FIRST_USER`.


  

**Source:** `engine/engine/trace.lua`


## trace.Trace _()_

The result of a trace. All trace functions return it. It describes what the trace hit and
where. All fields are read-only.


  ### `trace.Trace.attributes`
**Kind:** `prop`

The collision attributes of the shape that was hit.

**Source:** `engine/engine/trace.lua`

  ### `trace.Trace.body`
**Kind:** `prop`

The physics body that was hit.

**Source:** `engine/engine/trace.lua`

  ### `trace.Trace.contents`
**Kind:** `prop`

A bit mask of the interaction layers at the hit point, as `trace.CONTENTS` bits.

**Source:** `engine/engine/trace.lua`

  ### `trace.Trace.damage`
**Kind:** `prop`

The damage after the surface and penetration modifiers are applied.

**Source:** `engine/game/trace.lua`

  ### `trace.Trace.damage_is_valid`
**Kind:** `prop`

True if the damage fields have values.

**Source:** `engine/game/trace.lua`

  ### `trace.Trace.damage_unscaled`
**Kind:** `prop`

The damage before any modifiers are applied.

**Source:** `engine/game/trace.lua`

  ### `trace.Trace.endpos`
**Kind:** `prop`

The position where the trace ended. If something was hit, this is the point of impact. If nothing was hit, this is the end position that you passed.

**Source:** `engine/engine/trace.lua`

  ### `trace.Trace.enthit`
**Kind:** `prop`

The entity that was hit. It is **nil** if only world geometry was hit.

**Source:** `engine/engine/trace.lua`

  ### `trace.Trace.fraction`
**Kind:** `prop`

How far along the trace the hit happened. `0` is the start. `1` means that nothing was hit, or that the hit is at the end.

**Source:** `engine/engine/trace.lua`

  ### `trace.Trace.hitbox`
**Kind:** `prop`

The hitbox that was hit. It is **nil** if the hitbox cannot be found on the entity that was hit.

**Source:** `engine/engine/trace.lua`

  ### `trace.Trace.hitpos`
**Kind:** `prop`

The exact point of contact on the surface that was hit.

**Source:** `engine/engine/trace.lua`

  ### `trace.Trace.hitpos_is_valid`
**Kind:** `prop`

`true` if the `hitpos` field holds valid data.

**Source:** `engine/engine/trace.lua`

  ### `trace.Trace.normal`
**Kind:** `prop`

The normal of the surface at the hit point.

**Source:** `engine/engine/trace.lua`

  ### `trace.Trace.offset`
**Kind:** `prop`

The plane constant of the plane that was hit. It is the dot product of the normal and a position on the plane.

**Source:** `engine/engine/trace.lua`

  ### `trace.Trace.physicsbone`
**Kind:** `prop`

The index of the physics bone, for skeletal meshes.

**Source:** `engine/engine/trace.lua`

  ### `trace.Trace.shape`
**Kind:** `prop`

The physics shape that was hit.

**Source:** `engine/engine/trace.lua`

  ### `trace.Trace.startpos`
**Kind:** `prop`

The start position of the trace.

**Source:** `engine/engine/trace.lua`

  ### `trace.Trace.startsolid`
**Kind:** `prop`

`true` if the trace started inside solid geometry.

**Source:** `engine/engine/trace.lua`

  ### `trace.Trace.surface`
**Kind:** `prop`

The surface properties of the surface that was hit, such as friction and density.

**Source:** `engine/engine/trace.lua`

  ### `trace.Trace.transform`
**Kind:** `prop`

The transform of the shape that was hit.

**Source:** `engine/engine/trace.lua`

  ### `trace.Trace.triangle`
**Kind:** `prop`

The index of the face or triangle on the surface that was hit. It is `-1` if there is no such index.

**Source:** `engine/engine/trace.lua`

  ### `trace.Trace.type`
**Kind:** `prop`

The type of shape that was traced. `0` is a ray, `1` is a sphere, `2` is a box, `3` is a capsule and `4` is a hull.

**Source:** `engine/engine/trace.lua`


## trace.Filter _()_

Decides which objects and interaction layers a trace collides with.

`ignore_entities` and `should_hit_entity` are write-only. Reading them throws an error. Writing a
field that does not exist also throws an error.

`ignore_entities` takes a table of entities. Up to two entities are fast. Each entity after the
second makes traces slower. Setting `ignore_entities` again replaces the entities that were set
before.


  ### `trace.Filter.collision_group`
**Kind:** `prop`

The collision group of the trace. See `trace.BuiltInCollisionGroup_t` and `trace.StandardCollisionGroups_t`.

**Source:** `engine/engine/trace.lua`

  ### `trace.Filter.ignore_entities`
**Kind:** `prop`

Write-only. A table of entities that the trace skips. Up to two entities are fast. Each entity after the second makes traces slower.

**Source:** `engine/engine/trace.lua`

  ### `trace.Filter.interacts_as`
**Kind:** `prop`

A bit mask of the layers that the trace itself belongs to. Other objects collide with the trace as these layers.

**Source:** `engine/engine/trace.lua`

  ### `trace.Filter.interacts_exclude`
**Kind:** `prop`

A bit mask of the layers that the trace never collides with.

**Source:** `engine/engine/trace.lua`

  ### `trace.Filter.interacts_with`
**Kind:** `prop`

A bit mask of the interaction layers that the trace hits.

**Source:** `engine/engine/trace.lua`

  ### `trace.Filter.object_set_mask`
**Kind:** `prop`

A bit mask of the `trace.RnQueryObjectSet` object types that the trace checks.

**Source:** `engine/engine/trace.lua`

  ### `trace.Filter.should_hit_entity`
**Kind:** `prop`

Write-only. A function that is called for each entity that the trace touches. Return `true` to hit the entity. Return `false` or any other value to skip it. Setting this function makes traces slower.

**Source:** `engine/engine/trace.lua`

---


## Value

<!-- Source: cs2/api_reference/engine/value.md -->

---
title: Schema Values
icon: binary
sidebar_position: 5
---

# Schema Values

## value.Object _()_

A value that is read from game memory through the schema system.

Structured schema values are views of the memory of the game. A view reads and writes the
memory directly, and it is valid only while the object that owns the memory exists.
`value.QuantizedFloat` values are copies of the data at the time of the read.

`Entity` and `value.View` have the methods below, which return the data pointer and the
schema type.


  ### `value.Object:GetInstance`
**Kind:** `method`

Returns the data pointer.

<Warning>
        This function is for advanced users. You can only use the pointer through FFI. The pointer is
        valid only while the object that owns the data exists. Do not keep it after the entity is
        deleted.
      </Warning>
    
  

**Source:** `engine/engine/value.lua`

  ### `value.Object:GetSchemaType`
**Kind:** `method`

Returns the schema type.

**Source:** `engine/engine/value.lua`


## value.View _()_



A `value.Object` that holds a pointer together with its type. Use `Cast` to read the same
address as a different `schema.Type`.


  ### `value.View:Cast`
**Kind:** `method`

**Arguments:** `type`

Returns a view of the same address with the given schema type. The type must describe the data at that address. A cast does not change the data. If `type` is **nil**, the view keeps its current type.

**Source:** `engine/engine/value.lua`


## value.AtomicCollection _()_



A dynamic array of the engine, such as `CUtlVector`. The length operator `#` returns the
number of elements. When you index the array directly, the first element has the index `0`.
An index outside the array reads as **nil**.

`Value`, `pairs` and `ipairs` use a copy of the array in a Lua table, where the first element
has the index `1`.


  ### `value.AtomicCollection:Value`
**Kind:** `method`

Copies the elements into a Lua table, where the first element has the index `1`. Entities in the collection are not copied. The table holds references to them.

**Source:** `engine/engine/value.lua`


## value.FixedArray _()_



A fixed size array of the engine, such as `uint32[2]`. The length operator `#` returns the
number of elements. When you index the array directly, the first element has the index `0`.
An index outside the array reads as **nil**.

`Value`, `pairs` and `ipairs` use a copy of the array in a Lua table, where the first element
has the index `1`.


  ### `value.FixedArray:Value`
**Kind:** `method`

Copies the elements into a Lua table, where the first element has the index `1`.

**Source:** `engine/engine/value.lua`


## value.Named _()_

Adds the `GetClassName` method to values whose schema type is a declared class of the engine.


  ### `value.Named:GetClassName`
**Kind:** `method`

Returns the name of the class.

**Source:** `engine/engine/value.lua`


## value.DeclaredClass _()_



An object of a declared class of the engine. Read and write its schema fields by name with
`self.<name>`.


  ### `value.DeclaredClass:GetField`
**Kind:** `method`

`. Use it when the name of the field is held in a variable, or when the field has the same name as a method." args="name" sig="[{"s":"value.DeclaredClass:GetField"},{"s":"("},{"s":"name"},{"s":": "},{"s":"string","k":"prim","h":"/cs2/api_reference/lua/base#string"},{"s":")"},{"s":": "},{"s":"any","k":"prim"}]" ret="[[{"s":"any","k":"prim"}]]" params="[{"n":"name","t":[{"s":"string","k":"prim","h":"/cs2/api_reference/lua/base#string"}],"d":"The name of the schema field."}]" nodiscard source="engine/engine/value.lua" sourceHref="https://gitlab.com/neverlose/LuaLS-CS2/-/edit/master/library/api/engine/engine/value.lua#L92" />

   = value`." args="name, value" sig="[{"s":"value.DeclaredClass:SetField"},{"s":"("},{"s":"name"},{"s":": "},{"s":"string","k":"prim","h":"/cs2/api_reference/lua/base#string"},{"s":", "},{"s":"value"},{"s":": "},{"s":"any","k":"prim"},{"s":")"}]" params="[{"n":"name","t":[{"s":"string","k":"prim","h":"/cs2/api_reference/lua/base#string"}],"d":"The name of the schema field."},{"n":"value","t":[{"s":"any","k":"prim"}],"d":"The value to write. It is converted to the schema type of the field."}]" source="engine/engine/value.lua" sourceHref="https://gitlab.com/neverlose/LuaLS-CS2/-/edit/master/library/api/engine/engine/value.lua#L100">
    
      It takes the same values as a field assignment. Throws an error if the field has a type that
      cannot be written, such as a bitfield or a fixed array.
    
  


## value.Bitfield _()_



A packed integer field that uses fewer bits than its storage type. You cannot get its value
through the schema. Reading the field returns this object, and writing the field throws an
error.


  ### `value.Bitfield:NumBits`
**Kind:** `method`

Returns the number of bits in the field.

**Source:** `engine/engine/value.lua`


## value.QuantizedVector _()_



An object of a quantized vector class, such as `CNetworkOriginQuantizedVector`.


  ### `value.QuantizedVector:Get`
**Kind:** `method`

Returns the quantized vector as a regular 3D `Vector`.

**Source:** `engine/engine/value.lua`


## value.QuantizedFloat _()_

A copy of one component of a quantized vector. The copy does not change when the game
changes the field. Read the field again to get the current value.


  ### `value.QuantizedFloat:MinValue`
**Kind:** `method`

Returns the lowest value that the real value can have.

**Source:** `engine/engine/value.lua`

  ### `value.QuantizedFloat:MaxValue`
**Kind:** `method`

Returns the highest value that the real value can have.

**Source:** `engine/engine/value.lua`

  ### `value.QuantizedFloat:Get`
**Kind:** `method`

Returns the value.

**Source:** `engine/engine/value.lua`

  ### `value.QuantizedFloat:NumBits`
**Kind:** `method`

Returns the number of bits that the value uses.

**Source:** `engine/engine/value.lua`

---


## Cheat

<!-- Source: cs2/api_reference/events/cheat.md -->

---
title: Cheat Events
icon: zap
asidebar: hide
---

# Cheat Events


  ### `aim_ack`
**Kind:** `event`

**Source:** `events/cheat/game.lua`

  ### `aim_fire`
**Kind:** `event`

**Source:** `events/cheat/game.lua`

  ### `config_state`
**Kind:** `event`

**Source:** `events/cheat/core.lua`

  ### `console_input`
**Kind:** `event`

Fires for a line that is entered in the game console or in VConsole. Return `false` to block the line.

**Source:** `events/cheat/game.lua`

  ### `createmove`
**Kind:** `event`

Fires last for each command, after `post_movement`. Read or change the input here before the command is finalized.

**Source:** `events/cheat/game.lua`

  ### `display_mode_changed`
**Kind:** `event`

Fires when the game applies a new display mode. A new resolution, window mode, vsync setting or MSAA setting changes the display mode. `mode` and `engine.GetDisplayMode()` already hold the new mode. `engine.GetScreenSize()` returns the new size from the next frame.

**Source:** `events/cheat/game.lua`

  ### `entity_created`
**Kind:** `event`

Fires when the first networked data of a new entity has been applied. It fires just before `entity_networked` for the same entity.

**Source:** `events/cheat/game.lua`

  ### `entity_created_early`
**Kind:** `event`

Fires early while an entity is created. At this time, not all networked fields of the entity are available.

**Source:** `events/cheat/game.lua`

  ### `entity_deleted`
**Kind:** `event`

Fires when an entity is deleted. Read the entity only during the callback.

**Source:** `events/cheat/game.lua`

  ### `entity_networked`
**Kind:** `event`

Fires when networked data of an entity has been applied.

**Source:** `events/cheat/game.lua`

  ### `execute_client_cmd`
**Kind:** `event`

Fires when the game or its UI runs a command line, for example `say "hello"` from the chat box. `text` is the whole line. It can contain several commands separated by `;`.

Return `false` to block the line. Return a string to run that string instead. If one listener
      returns `false`, the line is blocked even if other listeners return a string. If several
      listeners return a string, the string of the last listener is used.

      Lines that are typed in the console, key binds and commands that scripts run do not fire this
      event.
    
  

**Source:** `events/cheat/game.lua`

  ### `input_poll`
**Kind:** `event`

Read or change the current input poll inside this callback. Do not keep the context after the callback returns.

**Source:** `events/cheat/game.lua`

  ### `level_init`
**Kind:** `event`

Fires when the client is fully connected to a level. It also fires when the client reconnects to the same map.

**Source:** `events/cheat/game.lua`

  ### `mouse_input`
**Kind:** `event`

Fires for every mouse update. Return `false` to block the input, so that the game and the menu do not receive it. Any other return value lets the input through.

**Source:** `events/cheat/core.lua`

  ### `override_view`
**Kind:** `event`

Read or change the camera inside this callback. Do not keep the context after the callback returns.

**Source:** `events/cheat/game.lua`

  ### `override_viewmodel`
**Kind:** `event`

Read or change the root of the first person viewmodel in each frame. Do not keep the context after the callback returns.

**Source:** `events/cheat/game.lua`

  ### `post_data_update`
**Kind:** `event`

Fires after the networked data of all entities in an update has been applied. It fires after every `entity_networked` event of that update.

**Source:** `events/cheat/game.lua`

  ### `post_movement`
**Kind:** `event`

Fires for each command, after `pre_movement` and before `createmove`.

**Source:** `events/cheat/game.lua`

  ### `post_update_viewmodel`
**Kind:** `event`

Fires after the first person HUD arms are placed and their animation is evaluated for this frame. Attachments that you read here match the current pose. Use `SetAbsAngles` and `SetAbsOrigin` to move both roots.

**Source:** `events/cheat/game.lua`

  ### `pre_movement`
**Kind:** `event`

Fires for each command, before `post_movement`. Changes that you make here can still be overwritten before the command is sent. Clone the simulation that you receive before you advance it.

**Source:** `events/cheat/game.lua`

  ### `ready_for_match`
**Kind:** `event`

Fires when the ready prompt of the lobby appears for a match that was found.

**Source:** `events/cheat/game.lua`

  ### `render`
**Kind:** `event`

**Source:** `events/cheat/core.lua`

  ### `shutdown`
**Kind:** `event`

**Source:** `events/cheat/core.lua`

  ### `usermsg_call_vote_failed`
**Kind:** `event`

**Source:** `events/cheat/game.lua`

  ### `usermsg_report_hit`
**Kind:** `event`

**Source:** `events/cheat/game.lua`

  ### `usermsg_say_text`
**Kind:** `event`

**Source:** `events/cheat/game.lua`

  ### `usermsg_vote_failed`
**Kind:** `event`

**Source:** `events/cheat/game.lua`

  ### `usermsg_vote_pass`
**Kind:** `event`

**Source:** `events/cheat/game.lua`

  ### `usermsg_vote_setup`
**Kind:** `event`

**Source:** `events/cheat/game.lua`

  ### `usermsg_vote_start`
**Kind:** `event`

**Source:** `events/cheat/game.lua`

---


## Game

<!-- Source: cs2/api_reference/events/game.md -->

---
title: Game Events
icon: zap
asidebar: hide
---

# Game Events


  ### `achievement_earned`
**Kind:** `event`

  ### `achievement_earned_local`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `achievement_event`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `achievement_info_loaded`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `achievement_write_failed`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `add_bullet_hit_marker`
**Kind:** `event`

  ### `add_player_sonar_icon`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `ammo_pickup`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `ammo_refill`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `announce_phase_end`
**Kind:** `event`

  ### `begin_new_match`
**Kind:** `event`

  ### `bomb_abortdefuse`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `bomb_abortplant`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `bomb_beep`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `bomb_begindefuse`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `bomb_beginplant`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `bomb_defused`
**Kind:** `event`

  ### `bomb_dropped`
**Kind:** `event`

  ### `bomb_exploded`
**Kind:** `event`

  ### `bomb_pickup`
**Kind:** `event`

  ### `bomb_planted`
**Kind:** `event`

  ### `bonus_updated`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `bot_takeover`
**Kind:** `event`

  ### `break_breakable`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `break_prop`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `broken_breakable`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `bullet_damage`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `bullet_impact`
**Kind:** `event`

  ### `buymenu_close`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `buymenu_open`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `buytime_ended`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `cart_updated`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `choppers_incoming_warning`
**Kind:** `event`

  ### `client_disconnect`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `client_loadout_changed`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `clientside_lesson_closed`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `clientside_reload_custom_econ`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `cs_game_disconnected`
**Kind:** `event`

  ### `cs_intermission`
**Kind:** `event`

  ### `cs_match_end_restart`
**Kind:** `event`

  ### `cs_pre_restart`
**Kind:** `event`

  ### `cs_prev_next_spectator`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `cs_round_final_beep`
**Kind:** `event`

  ### `cs_round_start_beep`
**Kind:** `event`

  ### `cs_win_panel_match`
**Kind:** `event`

  ### `cs_win_panel_round`
**Kind:** `event`

  ### `decoy_detonate`
**Kind:** `event`

  ### `decoy_firing`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `decoy_started`
**Kind:** `event`

  ### `defuser_dropped`
**Kind:** `event`

  ### `defuser_pickup`
**Kind:** `event`

  ### `demo_skip`
**Kind:** `event`

  ### `demo_start`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `demo_stop`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `difficulty_changed`
**Kind:** `event`

  ### `dm_bonus_weapon_start`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `door_break`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `door_close`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `door_closed`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `door_moving`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `door_open`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `drone_above_roof`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `drone_cargo_detached`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `drone_dispatched`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `dronegun_attack`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `drop_rate_modified`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `dynamic_shadow_light_changed`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `dz_item_interaction`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `enable_restart_voting`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `endmatch_cmm_start_reveal_items`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `endmatch_mapvote_selecting_map`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `enter_bombzone`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `enter_buyzone`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `enter_rescue_zone`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `entity_killed`
**Kind:** `event`

  ### `entity_visible`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `event_ticket_modified`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `exit_bombzone`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `exit_buyzone`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `exit_rescue_zone`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `finale_start`
**Kind:** `event`

  ### `firstbombs_incoming_warning`
**Kind:** `event`

  ### `flare_ignite_npc`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `flashbang_detonate`
**Kind:** `event`

  ### `game_end`
**Kind:** `event`

a game ended

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `game_init`
**Kind:** `event`

sent when a new game is started

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `game_message`
**Kind:** `event`

a message send by game logic to everyone

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `game_newmap`
**Kind:** `event`

send when new map is completely loaded

  ### `game_phase_changed`
**Kind:** `event`

  ### `game_start`
**Kind:** `event`

a new game starts

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `gameinstructor_draw`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `gameinstructor_nodraw`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `gameui_hidden`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `gc_connected`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `gg_killed_enemy`
**Kind:** `event`

  ### `grenade_bounce`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `grenade_thrown`
**Kind:** `event`

  ### `guardian_wave_restart`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `hegrenade_detonate`
**Kind:** `event`

  ### `helicopter_grenade_punt_miss`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `hide_deathpanel`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `hltv_cameraman`
**Kind:** `event`

a spectator/player is a cameraman

  ### `hltv_changed_mode`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `hltv_chase`
**Kind:** `event`

shot of a single entity

  ### `hltv_chat`
**Kind:** `event`

a HLTV chat msg sent by spectators

  ### `hltv_fixed`
**Kind:** `event`

show from fixed view

  ### `hltv_message`
**Kind:** `event`

a HLTV message send by moderators

  ### `hltv_rank_camera`
**Kind:** `event`

a camera ranking

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `hltv_rank_entity`
**Kind:** `event`

an entity ranking

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `hltv_replay`
**Kind:** `event`

  ### `hltv_replay_status`
**Kind:** `event`

  ### `hltv_status`
**Kind:** `event`

general HLTV status

  ### `hltv_title`
**Kind:** `event`

  ### `hltv_versioninfo`
**Kind:** `event`

  ### `hostage_call_for_help`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `hostage_follows`
**Kind:** `event`

  ### `hostage_hurt`
**Kind:** `event`

  ### `hostage_killed`
**Kind:** `event`

  ### `hostage_rescued`
**Kind:** `event`

  ### `hostage_rescued_all`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `hostage_stops_following`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `hostname_changed`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `inferno_expire`
**Kind:** `event`

  ### `inferno_extinguish`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `inferno_startburn`
**Kind:** `event`

  ### `inspect_weapon`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `instructor_close_lesson`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `instructor_server_hint_create`
**Kind:** `event`

create a hint using data supplied entirely by the server/map. Intended for hints to smooth playtests before content is ready to make the hint unneccessary. NOT INTENDED AS A SHIPPABLE CRUTCH

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `instructor_server_hint_stop`
**Kind:** `event`

destroys a server/map created hint

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `instructor_start_lesson`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `inventory_updated`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `item_equip`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `item_pickup`
**Kind:** `event`

  ### `item_pickup_failed`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `item_pickup_slerp`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `item_purchase`
**Kind:** `event`

  ### `item_remove`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `item_schema_initialized`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `jointeam_failed`
**Kind:** `event`

  ### `local_player_controller_team`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `local_player_pawn_changed`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `local_player_team`
**Kind:** `event`

  ### `loot_crate_opened`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `loot_crate_visible`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `map_shutdown`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `map_transition`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `match_end_conditions`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `material_default_complete`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `mb_input_lock_cancel`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `mb_input_lock_success`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `molotov_detonate`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `nav_blocked`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `nav_generate`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `nextlevel_changed`
**Kind:** `event`

a game event, name may be 32 characters long

  ### `open_crate_instr`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `other_death`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `parachute_deploy`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `parachute_pickup`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `physgun_pickup`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `player_activate`
**Kind:** `event`

  ### `player_avenged_teammate`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `player_blind`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `player_changename`
**Kind:** `event`

  ### `player_chat`
**Kind:** `event`

a public player chat

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `player_connect`
**Kind:** `event`

a new client connected

  ### `player_connect_full`
**Kind:** `event`

player has sent final message in the connection sequence

  ### `player_death`
**Kind:** `event`

a game event, name may be 32 characters long this extents the original player_death by a new fields

  ### `player_decal`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `player_disconnect`
**Kind:** `event`

a client was disconnected

  ### `player_falldamage`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `player_footstep`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `player_full_update`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `player_given_c4`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `player_hintmessage`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `player_hurt`
**Kind:** `event`

  ### `player_info`
**Kind:** `event`

a player changed his name

  ### `player_jump`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `player_ping`
**Kind:** `event`

  ### `player_ping_stop`
**Kind:** `event`

  ### `player_radio`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `player_reset_vote`
**Kind:** `event`

  ### `player_score`
**Kind:** `event`

players scores changed

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `player_shoot`
**Kind:** `event`

player shoot his weapon

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `player_sound`
**Kind:** `event`

  ### `player_spawn`
**Kind:** `event`

player spawned in game

  ### `player_spawned`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `player_stats_updated`
**Kind:** `event`

  ### `player_team`
**Kind:** `event`

player change his team

  ### `ragdoll_dissolved`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `read_game_titledata`
**Kind:** `event`

read user titledata from profile

  ### `repost_xbox_achievements`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `reset_game_titledata`
**Kind:** `event`

reset user titledata; do not automatically write profile

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `round_announce_final`
**Kind:** `event`

  ### `round_announce_last_round_half`
**Kind:** `event`

  ### `round_announce_match_point`
**Kind:** `event`

  ### `round_announce_match_start`
**Kind:** `event`

  ### `round_announce_warmup`
**Kind:** `event`

  ### `round_end`
**Kind:** `event`

  ### `round_end_upload_stats`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `round_freeze_end`
**Kind:** `event`

  ### `round_mvp`
**Kind:** `event`

  ### `round_officially_ended`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `round_poststart`
**Kind:** `event`

sent after all other round restart actions

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `round_prestart`
**Kind:** `event`

sent before all other round restart actions

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `round_start`
**Kind:** `event`

  ### `round_start_post_nav`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `round_start_pre_entity`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `round_time_warning`
**Kind:** `event`

  ### `seasoncoin_levelup`
**Kind:** `event`

  ### `server_cvar`
**Kind:** `event`

a server console var has changed

  ### `server_message`
**Kind:** `event`

a generic server message

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `server_pre_shutdown`
**Kind:** `event`

server is about to be shut down

  ### `server_shutdown`
**Kind:** `event`

server shut down

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `server_spawn`
**Kind:** `event`

send once a server starts

  ### `set_instructor_group_enabled`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `sfuievent`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `show_deathpanel`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `show_survival_respawn_status`
**Kind:** `event`

  ### `silencer_detach`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `silencer_off`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `silencer_on`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `smoke_beacon_paradrop`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `smokegrenade_detonate`
**Kind:** `event`

  ### `smokegrenade_expired`
**Kind:** `event`

  ### `spec_mode_updated`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `spec_target_updated`
**Kind:** `event`

  ### `start_halftime`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `start_vote`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `store_pricesheet_updated`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `survival_announce_phase`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `survival_no_respawns_final`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `survival_no_respawns_warning`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `survival_paradrop_break`
**Kind:** `event`

  ### `survival_paradrop_spawn`
**Kind:** `event`

  ### `survival_teammate_respawn`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `switch_team`
**Kind:** `event`

  ### `tagrenade_detonate`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `team_info`
**Kind:** `event`

info about team

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `team_intro_end`
**Kind:** `event`

  ### `team_intro_start`
**Kind:** `event`

  ### `team_score`
**Kind:** `event`

team score changed

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `teamchange_pending`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `teamplay_broadcast_audio`
**Kind:** `event`

emits a sound to everyone on a team

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `teamplay_round_start`
**Kind:** `event`

round restart

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `tournament_reward`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `trial_time_expired`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `ugc_file_download_finished`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `ugc_file_download_start`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `ugc_map_download_error`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `ugc_map_info_received`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `ugc_map_unsubscribed`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `update_matchmaking_stats`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `user_data_downloaded`
**Kind:** `event`

fired when achievements/stats are downloaded from Steam or XBox Live

  ### `vip_escaped`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `vip_killed`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `vote_cast`
**Kind:** `event`

  ### `vote_cast_no`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `vote_cast_yes`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `vote_changed`
**Kind:** `event`

  ### `vote_ended`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `vote_failed`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `vote_options`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `vote_passed`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `vote_started`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `warmup_end`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `weapon_fire`
**Kind:** `event`

  ### `weapon_fire_on_empty`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `weapon_reload`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `weapon_zoom`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `weapon_zoom_rifle`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>
    
  

  ### `weaponhud_selection`
**Kind:** `event`

  ### `write_game_titledata`
**Kind:** `event`

write user titledata in profile

  ### `write_profile_data`
**Kind:** `event`

<Warning>
        Nothing in the game listens for this event, so the server does not send it and this callback never runs.
      </Warning>

---


## Anatomy

<!-- Source: cs2/api_reference/extensions/anatomy.md -->

---
title: Anatomy
description: Understand the extension folder, manifest, entry point, and reload model.
icon: folder-tree
sidebar_position: 1
---

# Anatomy

An **extension** contains Lua files that Neverlose loads at runtime. A local
developer extension lives in a folder with a manifest, a `main.lua` entry point,
and any extra files you split your logic across.

<Note>
  **The folder name is your local extension's display name.** It is what shows
  up in the Extensions list. There is no `name` field in the manifest.
</Note>

## Where extensions live

Local developer extensions live on disk in your Neverlose data folder. Enable
**Developer mode** in the cheat's settings to show them in the Extensions list:

```plaintext lines=false
…/Counter-Strike Global Offensive/game/bin/win64/neverlose/extensions
```

Once an extension appears in the list, **Open Folder** in its right-click menu
opens its directory. If Visual Studio Code is installed, **Open with Code**
opens the same folder directly in your editor.

## Minimal layout

A minimal extension is just two files:

<Tree.Folder name="example" icon="folder" defaultOpen>
  <Tree.File name="extension.json" icon="file-cog" />

  <Tree.File name="main.lua" icon="file-braces-corner" />
</Tree.Folder>

<Columns cols={3}>
  <Card title="extension.json" icon="file-cog">
    The manifest. It declares the Lua language and any modules listed
    in [Requirements](/cs2/api_reference/extensions/requirements).
  </Card>

  <Card title="main.lua" icon="file-braces-corner">
    The entry point. Neverlose runs this file each time the extension loads
    or reloads.
  </Card>

  <Card title="extra files" icon="folder-tree">
    Optional Lua modules, assets, localization, or shared helpers. Organize them
    however the extension needs.
  </Card>
</Columns>

An optional `.neverloseignore` file controls which files are included when
publishing. It does not prevent your extension from reading those files during
local development. Directories whose names start with a dot, such as `.git`,
`.vscode`, and the `.nl` snapshot cache, are excluded from published bundles.

## Create the extension

<Steps>
  <Step title="Enable Developer mode" icon="settings">
    Enable **Developer mode** in the cheat's settings so local extensions are
    available in the **Extensions** tab.
  </Step>

  <Step title="Create the folder and files" icon="folder-plus">
    Create a folder under `…/neverlose/extensions/`, then add `extension.json`
    and a nonempty `main.lua`. Use the manifest and entry-point examples below
    to get started.
  </Step>

  <Step title="Enable the extension" icon="power">
    Open the **Extensions** tab and enable your extension. Use **Open Folder**
    or **Open with Code** from its right-click menu for subsequent edits.
  </Step>
</Steps>

Turning off **Developer mode** unloads local extensions. Turn it back on and
enable the extensions you want to run again.

## The manifest

Every extension has an `extension.json` at its root. The loader reads two
fields: the required language and an optional list of requirements.

<Tabs>
  <Tab title="Fields" icon="list">
    <SchemaField name="language" type="string" required>
      The VM the extension runs on. Currently only `"lua"` is supported.
    </SchemaField>

    <SchemaField name="requirements" type="string[]">
      Modules your extension depends on. Omit this field or use an empty array
      if you have none.
    </SchemaField>
  </Tab>

  <Tab title="Minimal" icon="file-cog">
    Use this when the extension has no external dependencies.

    ```json extension.json
    {
      "language": "lua",
      "requirements": []
    }
    ```
  </Tab>

  <Tab title="With requirements" icon="puzzle">
    Declare native, marketplace, or local dependencies here. The marketplace
    name and version below are illustrative; replace them with a published
    library you use, and create the `shared` local extension before loading.

    ```json extension.json
    {
      "language": "lua",
      "requirements": [
        "ffi",
        "neverlose/example-library:5",
        "local/shared"
      ]
    }
    ```
  </Tab>
</Tabs>

<Card title="Manifest rules" icon="list-check">
  * **Keep it valid JSON.** No comments and no trailing commas.
  * **Save as UTF-8.** Neverlose strips a leading byte-order mark from the root
    manifest when reading it locally and preparing an upload.
  * **Leave old metadata out.** Older examples sometimes show `name` and `icon`
    keys; those are ignored.
  * **Edit it directly or through Settings.** The extension's **Settings**
    dialog writes the same `requirements` array.
</Card>

## The entry point

Each time the extension loads, Neverlose runs `main.lua` top to bottom. You
do not write a manual game loop. Register callbacks on
[events](/cs2/api_reference/runtime/events), and the cheat calls them for you:

```lua main.lua
-- my-extension
-- runs once when the extension is enabled

events.render:Set(function()
    -- runs every frame on the render thread
end)
```

## Hot reload

Neverlose checks local extensions for changes every **five seconds**. An
**enabled** extension reloads after a detected change while the **game window
is focused and the menu is open**. After saving in your editor, return to the
game and open the menu to let it reload. Reloading runs `main.lua` again.

Enabling, disabling, or reloading another extension can also rerun `main.lua`
for active extensions sharing its Lua runtime. Keep setup safe to repeat and
use `events.shutdown` to restore temporary changes outside your extension.

Disabled extensions pick up edits when you next enable them. Directories whose
names start with a dot are excluded from reload checks; `.neverloseignore`
rules do not exclude files from these checks.

<Columns cols={2}>
  <Card title="Requirements" icon="puzzle" href="/cs2/api_reference/extensions/requirements">
    Learn how `requirements` map to native modules, marketplace packages, local
    extensions, and your own Lua files.
  </Card>
</Columns>

---


## Globals And Modules

<!-- Source: cs2/api_reference/extensions/globals-and-modules.md -->

---
title: Globals and Modules
icon: package
sidebar_position: 5
---

# Globals and Modules

Some names are available directly in every script. Other features are modules:
declare them in the manifest, and your own files are loaded with `require`.

## Common globals

You can use these without `require`:

<Columns cols={2}>
  <Column>
    **Neverlose namespaces**

    ```lua lines=false
    events.render(function() end)
    local tab = ui.AddTab("Lua")
    local pawn = entity.GetLocalPawn()
    local t = schema.client["C_CSPlayerPawn"]
    ```
  </Column>

  <Column>
    **Value constructors**

    ```lua lines=false
    local p3 = Vector(0, 0, 0)
    local p2 = Vector2D(20, 20)
    local a = QAngle(0, 90, 0)
    local c = Color(255, 255, 255)
    ```
  </Column>
</Columns>

Constructor names match their types: `Vector`, `Vector2D`, `QAngle`, and
`Color`. Functions and methods also use their documented capitalization, such
as `entity.GetLocalPawn()` and `pawn:IsValid()`.

## Standard Lua libraries

Neverlose keeps the Lua helpers you normally use for scripting:

```lua luaintelligence=true lines=false semantic="lua:cs2:cs2/api_reference/extensions/globals-and-modules:2:33ace87d51e5"
local items = { "ak47", "m4a1", "awp" }

for index, name in ipairs(items) do
    print(index, ": ", string.upper(name))
end
```

Standard globals include `math`, `bit`, `string`, `jit`, `table`, and
`coroutine`. `json` and `msgpack` are globals too. Loading `table.new` or
`table.clear` with `require` does not need a manifest requirement. The [Lua
reference](/cs2/api_reference/lua/base) documents the available library surface;
clock functions live in [`time`](/cs2/api_reference/runtime/time).

## Modules

Native capabilities such as [`http`](/cs2/api_reference/networking/http), `ffi`,
and `filesystem`, marketplace libraries, and sibling extensions must be declared
in the manifest. A declared module is also a global, and `require` returns that
same object. Your own files need no declaration and are loaded only with
`require`.

```lua main.lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/extensions/globals-and-modules:3:3eb182ad030e"
local http = require("http")  -- declared in extension.json requirements
local helpers = require("lib/helpers")  -- lib/helpers.lua in this extension
```

[Requirements](/cs2/api_reference/extensions/requirements) covers each kind of
module, path rules, caching, and lookup precedence.

## Keeping globals clean

Prefer locals for your own values. It keeps reloads predictable and avoids
accidentally shadowing public API names.

```lua luaintelligence=true lines=false semantic="lua:cs2:cs2/api_reference/extensions/globals-and-modules:4:28bbd639ade1"
local font = render.LoadFont(render.EspFont.kRegular)
local accent = Color(130, 165, 255)

events.render(function()
    render.DrawText(Vector2D(16, 16), accent, font, nil, "Scoped locals")
end)
```

---


## Lifecycle And Events

<!-- Source: cs2/api_reference/extensions/lifecycle-and-events.md -->

---
title: Lifecycle and Events
icon: radio
sidebar_position: 3
---

# Lifecycle and Events

Neverlose scripts are event-driven. You do not write a permanent `while true`
loop. Put setup in `main.lua`, subscribe to events, and let Neverlose call your
callbacks at the right time.

## main.lua runs on load

Use the top level of `main.lua` for values that should be created once per load:

```lua main.lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/extensions/lifecycle-and-events:0:49a5c7640ddb"
local font = render.LoadFont(render.EspFont.kRegular)
local show_names = ui.AddTab("Lua"):AddGroup("Example"):AddToggle("Show names")

events.render(function()
    if not show_names.value then
        return
    end

    for controller, pawn in entity.IteratePlayers(true, false) do
        if pawn:IsValid() then
            local pos = projection.WorldToScreen(pawn:GetAbsOrigin())
            if pos then
                render.DrawText(pos, Color(255, 255, 255), font, "c", controller:GetPlayerName())
            end
        end
    end
end)
```

Reloading the extension runs the file again. Enabling, disabling, or reloading another extension in the same Lua runtime can also trigger this setup.
Keep it safe to repeat, and restore temporary external changes in `events.shutdown`.

## Render code belongs in events.render

Most functions in [`render`](/cs2/api_reference/rendering/render) write to the
current frame's draw list. Call them from [`events.render`](/cs2/api_reference/events/cheat#render).

```lua luaintelligence=true lines=false semantic="lua:cs2:cs2/api_reference/extensions/lifecycle-and-events:1:3cfa690eed0d"
local font = render.LoadFont(render.EspFont.kBold)

events.render(function()
    local screen = engine.GetScreenSize()
    if not screen then
        return
    end

    render.DrawText(screen / 2, Color(130, 165, 255), font, "c", "Center text")
end)
```

If you need to calculate data outside render, store it in locals and draw the
latest value from the render callback.

## Movement code belongs in movement events

`pre_movement` and `post_movement` surround movement processing;
`createmove` runs before command finalization. Their callbacks receive the
current [`UserCmd`](/cs2/api_reference/input/cmd#usercmd) and a
[`simulation.Player`](/cs2/api_reference/input/cmd#simulationplayer).
Inspect these values inside the callback; clone a simulation before changing
or advancing it.

```lua luaintelligence=true lines=false semantic="lua:cs2:cs2/api_reference/extensions/lifecycle-and-events:2:074377a554ab"
events.pre_movement(function(cmd, player)
    if cmd.is_first_pass and cmd.in_jump.pressed then
        print("Jump pressed at simulation tick ", player.tickcount)
    end
end)
```

Use [`events.input_poll`](/cs2/api_reference/events/cheat#input_poll) for live
input before it becomes a command. Its `input.InputPollContext` uses boolean
`in_*` fields; command fields such as `cmd.in_jump` are button handles with
`down`, `pressed`, and `released` properties.

Open the [Cheat Events](/cs2/api_reference/events/cheat) page for callback names
and payload types.

## Delayed work

For "do this later" behavior, use [`time.Schedule`](/cs2/api_reference/runtime/time#time-Schedule)
instead of blocking the script.

```lua luaintelligence=true lines=false semantic="lua:cs2:cs2/api_reference/extensions/lifecycle-and-events:3:5cae76dd787d"
print("Now")

time.Schedule(1.0, function(message)
    print(message)
end, "One second later")
```

Callbacks should return quickly. Heavy work in a per-frame event will affect the
same frame budget as your game and cheat UI.

## Shutdown cleanup

Use [`events.shutdown`](/cs2/api_reference/events/cheat#shutdown) when your
script changes state that should be restored manually.

```lua luaintelligence=true lines=false semantic="lua:cs2:cs2/api_reference/extensions/lifecycle-and-events:4:4a0929d483fb"
local showfps = console.cl_showfps
if showfps.is_valid then
    local old_value = showfps.value
    showfps.value = 1

    events.shutdown(function()
        if showfps.is_valid then
            showfps.value = old_value
        end
    end)
end
```

Normal event callbacks and UI created by the script are removed when the script
reloads. Shutdown is for things you changed outside that automatic ownership,
such as convars or library state.

## Choosing an event

<CardGroup cols={2}>
  <Card title="Draw every frame" icon="brush" href="/cs2/api_reference/events/cheat#render" arrow>
    Use `events.render`; it is the safe place for most `render.*` calls.
  </Card>

  <Card title="Change movement" icon="move" href="/cs2/api_reference/events/cheat#pre_movement" arrow>
    Use movement events that pass a `UserCmd` and `simulation.Player`.
  </Card>

  <Card title="React to game events" icon="zap" href="/cs2/api_reference/events/game" arrow>
    Use generated game event pages for payload fields such as user IDs, weapon names, or positions.
  </Card>

  <Card title="Library communication" icon="radio" href="/cs2/api_reference/runtime/events#eventseventtraits" arrow>
    Subscribe with `:Set`, detach with `:Unset`, and use `:Fire(...)` for your own event names. Scripts cannot fire host-owned events.
  </Card>
</CardGroup>

---


## Localization

<!-- Source: cs2/api_reference/extensions/localization.md -->

---
title: Localization
description: Translate your extension's menu text with langs/*.json files or ui.Localize.
icon: languages
sidebar_position: 2
---

# Localization

An extension can show its menu text in the language that the user selected.
Put one JSON file per language in a `langs` folder. Neverlose loads these files
for you, so you do not need to read them from Lua.

## Folder layout

<Tree.Folder name="MyExtension" icon="folder" defaultOpen>
  <Tree.File name="extension.json" icon="file-cog" />

  <Tree.File name="main.lua" icon="file-braces-corner" />

  <Tree.Folder name="langs" icon="folder" defaultOpen>
    <Tree.File name="ru.json" icon="file-json" />
  </Tree.Folder>
</Tree.Folder>

The file name is the short name of the language, such as `ru.json` for
Russian. You do not need to list the files in `extension.json`.

## Write the menu in English

Create the menu with readable English text. This text is the key that the
translation files use.

```lua main.lua
local tab = ui.AddTab("My extension")
local group = tab:AddGroup("General")
local enabled = group:AddToggle("Enabled", false)

enabled.tooltip = "Enable the feature"
```

## Add a translation file

Each file is a flat JSON object. A key is the English text from your script,
and the value is the translation.

```json langs/ru.json
{
  "My extension": "Моё расширение",
  "General": "Основное",
  "Enabled": "Включено",
  "Enable the feature": "Включить функцию"
}
```

<Note>
  **No `"strings"` wrapper.** The files that translate the main Neverlose menu
  put their text inside a `"strings"` object. Extension files do not.
</Note>

## How text is chosen

<Steps>
  <Step title="Files load before main.lua" icon="file-down">
    Neverlose reads `langs/*.json` before it runs your script. When the user
    changes the language, the menu switches to the matching text.
  </Step>

  <Step title="Lookup falls back" icon="list-ordered">
    Neverlose looks for the selected language first, then its base language,
    then English.
  </Step>

  <Step title="Your files override your requirements" icon="layers">
    Translations from your [requirements](/cs2/api_reference/extensions/requirements)
    load first, and your own files load last. If both have the same key, your
    translation wins.
  </Step>
</Steps>

Each extension has its own translations. The same text used in two places of
one extension shares one translation.

## Add translations from Lua

[`ui.Localize`](/cs2/api_reference/rendering/ui) adds one translation while
the script runs:

```lua
ui.Localize("ru", "Enabled", "Включено")
```

It keeps the translation in memory only and does not write a JSON file. It
throws `invalid language` if Neverlose does not know the language.

## Rules

<Card title="Keys and values" icon="list-check">
  * **Use readable English text as keys.** Do not use IDs such as
    `"feature_enabled"` with an `en.json` file. Creating tabs, groups, tooltips,
    and some other elements sets the English text back to the key itself.
  * **Change the translation, not the key.** The original text also identifies
    the element and its saved setting. Renaming it in Lua breaks both the
    translations and the user's saved value.
  * **Values must be strings.** A key whose value is not a string is skipped
    with an error in the console.
  * **Keep keys at 1000 bytes or less.** A longer key is skipped with an error.
    A longer translation is cut to 1000 bytes without splitting a UTF-8
    character.
  * **Keep the file valid JSON.** An invalid file is skipped with the error
    `invalid langs/<language>.json`.
</Card>

<Warning>
  **A file does not add a new language.** Neverlose only looks for files named
  after languages that it already knows. `ru.json` works because `ru` is the
  short name of a known language.
</Warning>

## Test and publish

Save the JSON file, then return to the game and open the menu.
[Hot reload](/cs2/api_reference/extensions/anatomy#hot-reload) reloads the whole
extension, translations included. You can also disable and enable the
extension.

When you publish, `langs/*.json` files are sent with the rest of the
extension. Make sure that `.neverloseignore` or the publish window does not
exclude them.

---


## Requirements

<!-- Source: cs2/api_reference/extensions/requirements.md -->

---
icon: puzzle
sidebar_position: 4
---

# Requirements

A **requirement** is a module your extension depends on. List dependencies in the
`requirements` array of your [extension manifest](/cs2/api_reference/extensions/anatomy).
Neverlose resolves and loads them **before** your `main.lua` runs. If your
extension has no dependencies, omit the array or leave it empty.

For example, declare a native module and then get its exported value with `require`:

```json extension.json
{
  "language": "lua",
  "requirements": ["http"]
}
```

```lua main.lua
local http = require("http")
```

## The three kinds of requirement

Which kind a requirement is comes from its **prefix**:

| Kind       | Form                        | Where it comes from                                                                 |
| ---------- | --------------------------- | ----------------------------------------------------------------------------------- |
| **Native** | `ffi`, `http`, `filesystem` | Capabilities built into the runtime. Use the library name without a prefix.         |
| **Market** | `neverlose/<name>`          | A library published on the Neverlose marketplace. Fetched and cached automatically. |
| **Local**  | `local/<name>`              | Another developer extension sitting in your own `extensions/` folder.               |

### Native modules

Declare each native capability your extension uses, then require it by its bare
name:

| Module                                             | Purpose                                                                                  |
| -------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| [`ffi`](/cs2/api_reference/lua/ffi)                | LuaJIT's foreign function interface. Also enables the related memory and vtable helpers. |
| [`http`](/cs2/api_reference/networking/http)       | HTTP requests and cookies.                                                               |
| [`filesystem`](/cs2/api_reference/data/filesystem) | File and directory operations.                                                           |

Calling `require` for one of these modules without a resolved requirement raises
an error. Capabilities needed by dependencies are resolved too, so a library's
`ffi` requirement also makes FFI available to the extension loading it.

Standard libraries such as `math`, `string`, and `table`, plus the built-in
`table.new` and `table.clear` modules, do not need manifest entries.

### Market modules

Reference a published library as `neverlose/<name>`. Neverlose resolves its
dependencies, downloads the required bundles, and caches them by name and
version.

An unversioned entry resolves to the latest version. In a local developer
extension, Neverlose writes that resolved version back into `extension.json`.
The package name below is illustrative; replace it with a published library
you use.

```json extension.json
{
  "language": "lua",
  "requirements": ["neverlose/example-library"]
}
```

You can **pin a published version** yourself by appending `:<number>`. Existing
explicit pins are preserved:

```json extension.json
{
  "language": "lua",
  "requirements": ["neverlose/example-library:5"]
}
```

Lua code can use the bare module name even when the manifest pins a version:

```lua main.lua
local library = require("neverlose/example-library")
```

The full pinned name, such as `require("neverlose/example-library:5")`, also
works when that version is resolved. Declare one version of each library in
your manifest.

Local developer extensions use source Lua and can only load open-source
marketplace dependencies. Closed-source extensions can also use open-source
libraries.

### Local modules

Reference a sibling extension as `local/<folder-name>`. For example,
`local/shared` refers to `extensions/shared/`. Its `main.lua` runs as a
dependency, and `require` returns its exported value. It does not need to be
enabled separately in the Extensions list.

Give the shared extension its own manifest:

```json shared/extension.json
{
  "language": "lua",
  "requirements": []
}
```

```lua shared/main.lua
local M = {}
function M.greet(name) return "hello, " .. name end
return M
```

Declare it in the consuming extension:

```json extension.json
{
  "language": "lua",
  "requirements": ["local/shared"]
}
```

```lua main.lua
local shared = require("local/shared")
print(shared.greet("world"))
```

Local dependencies must also be Lua extensions with a valid manifest and a
nonempty `main.lua`. Keep the dependency graph free of cycles: two libraries
cannot rely on each other's exports during initialization. Replace local
dependencies with published libraries before publishing your extension.

Native capabilities load first, followed by marketplace dependencies and local
dependencies, with dependencies loaded before their consumers. A missing or
unloadable requirement prevents the extension from starting.

## Requiring your own files

To split your extension across multiple `.lua` files, require each file by its
path relative to the extension root, where `main.lua` lives. Omit the `.lua`
extension. These files do not need manifest entries:

<Tree.Folder name="my-extension" defaultOpen>
  <Tree.File name="extension.json" />

  <Tree.File name="main.lua" />

  <Tree.File name="helpers.lua" />

  <Tree.Folder name="lib" defaultOpen>
    <Tree.File name="util.lua" />
  </Tree.Folder>
</Tree.Folder>

```lua main.lua
local helpers = require("helpers")    -- helpers.lua, next to main.lua
local util    = require("lib/util")   -- lib/util.lua, in a subdirectory
```

Each file can return a table of exports:

```lua lib/util.lua
local M = {}
function M.clamp(v, lo, hi) return math.min(math.max(v, lo), hi) end
return M
```

A few rules to keep in mind:

* **Use forward slashes** for subdirectories (`lib/util`), and **omit the `.lua`** extension.
* **Paths are relative to the extension root.** A file in `lib/` refers to a sibling as `require("lib/other")`. Absolute paths and `.` or `..` path segments are rejected.
* **Modules are cached.** Requiring the same file again returns its cached export. Return a table for reusable helpers; a module with no return value yields `true`.
* **Capability and library names take precedence.** Native, marketplace, and local module names are checked before files. Standard-library names are also resolved first, so `ffi.lua` and `math.lua` cannot replace those libraries. An undeclared native capability raises an error even if a same-named file exists.
* **Exports can be shared.** Marketplace and local dependencies are cached across extensions in the same Lua runtime. Avoid storing state specific to one extension in a shared library's export table.

## Managing requirements

Edit the `requirements` array by hand, or use the local extension's **Settings**
dialog. It identifies market, local, and native entries and offers an action to
update market libraries to their latest published version.

Settings saves requirement changes to `extension.json` immediately and reloads
an active extension when you close the dialog. Edits made in your editor follow
the [hot reload rules](/cs2/api_reference/extensions/anatomy#hot-reload).

---


## Cmd

<!-- Source: cs2/api_reference/input/cmd.md -->

---
title: User Command
icon: user-cog
sidebar_position: 2
---

# User Command

## UserCmd _()_

The input command that the client sends to the server every tick. It holds the movement, the
view angles and the button states.

It is passed to the `createmove`, `pre_movement` and `post_movement` events. You can read the
fields and assign new values to change the input of the player. Reading an unknown field
returns nil. Assigning an unknown field throws an error.

To turn the view, use the `events.input_poll` event with `input.InputPollContext:GetViewAngles`
and `input.InputPollContext:SetViewAngles` instead.

The button fields are handles. To change the state of a button, assign a value to a field of the
handle, for example `cmd.in_jump.down = true`. Assigning a value to the button field itself throws
an error.

The user command for a single tick. It holds the input and view state that is sent to the server.


  ### `UserCmd.attack1_start_history_index`
**Kind:** `prop`

The index in the input history at which the primary attack started, or nil if it did not start.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.attack2_start_history_index`
**Kind:** `prop`

The index in the input history at which the secondary attack started, or nil if it did not start.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.client_tick`
**Kind:** `prop`

The client tick at which this command was created.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.forwardmove`
**Kind:** `prop`

The forward and backward movement, as a fraction of the maximum movement speed. Assigned values are normalized when the command is processed.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.has_new_snapshot`
**Kind:** `prop`

True if a new world snapshot arrived before this pass. Read-only.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.impulse`
**Kind:** `prop`

The impulse command number. An impulse triggers an action for a single tick.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.in_attack`
**Kind:** `prop`

Primary fire.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.in_attack2`
**Kind:** `prop`

Secondary fire or scope.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.in_back`
**Kind:** `prop`

Move backward.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.in_duck`
**Kind:** `prop`

Duck.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.in_forward`
**Kind:** `prop`

Move forward.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.in_joyautosprint`
**Kind:** `prop`

Automatic sprint with a gamepad.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.in_jump`
**Kind:** `prop`

Jump.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.in_look_at_weapon`
**Kind:** `prop`

Inspect the weapon.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.in_moveleft`
**Kind:** `prop`

Move left.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.in_moveright`
**Kind:** `prop`

Move right.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.in_reload`
**Kind:** `prop`

Reload.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.in_score`
**Kind:** `prop`

Show the scoreboard.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.in_speed`
**Kind:** `prop`

Walk.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.in_turnleft`
**Kind:** `prop`

Turn left with the keyboard or a gamepad.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.in_turnright`
**Kind:** `prop`

Turn right with the keyboard or a gamepad.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.in_use`
**Kind:** `prop`

Use or interact.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.in_useorreload`
**Kind:** `prop`

The shared button for use and reload.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.in_zoom`
**Kind:** `prop`

Zoom or toggle the scope.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.input_history`
**Kind:** `prop`

The input samples of each frame that are attached to this command. Read-only.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.is_first_pass`
**Kind:** `prop`

True on the first pass over this command, and false while the command is processed again. Read-only.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.leftmove`
**Kind:** `prop`

The left and right movement, as a fraction of the maximum movement speed. Assigned values are normalized when the command is processed.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.legacy_command_number`
**Kind:** `prop`

The sequence number of the command. The server uses it to confirm the command.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.mousedx`
**Kind:** `prop`

The raw horizontal mouse movement in this tick.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.mousedy`
**Kind:** `prop`

The raw vertical mouse movement in this tick.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.moveangles`
**Kind:** `prop`

The angles that give the direction of movement. They are separate from the view angles.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.needs_full_recompute`
**Kind:** `prop`

True if the command must be built again from the start. Read-only.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.random_seed`
**Kind:** `prop`

The random seed that client side prediction uses to get the same results as the server.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.should_set_viewangles`
**Kind:** `prop`

Assign `true` to apply `viewangles` to the local camera after the callback. The angles are copied at the moment of the assignment, so later changes to `viewangles` are not applied. Assign `false` to cancel the request. This moves the camera even when it is set on a copy, so do not set it on copies used for simulation. It does not change what is sent to the server.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.subtick_moves`
**Kind:** `prop`

The subtick input steps. Read-only.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.upmove`
**Kind:** `prop`

The vertical movement, for example when swimming up or down.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.viewangles`
**Kind:** `prop`

The view angles of the player, as pitch, yaw and roll.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.weaponselect`
**Kind:** `prop`

The entity index of the weapon that the player wants to switch to.

**Source:** `input/game/cmd.lua`

  ### `UserCmd:Finalized`
**Kind:** `method`

**Arguments:** `sim`

Returns a copy of this command with the movement resolved and corrected against the given simulation. This command is not changed.

Clone a simulation that an event gave you before you run commands on it. The result is only a
      prediction and is not sent to the server.
    
  

**Source:** `input/game/cmd.lua`

  ### `UserCmd:Clone`
**Kind:** `method`

Returns a full copy of this command.

**Source:** `input/game/cmd.lua`


## UserCmd.InButtonHandle _()_

The state of one button in a command. Reading an unknown field returns nil. Assigning an
unknown field throws an error.


  ### `UserCmd.InButtonHandle.down`
**Kind:** `prop`

True if the button is down. Assigning a value forces the button state.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.InButtonHandle.pressed`
**Kind:** `prop`

True if the button was pressed in this command. Assigning a value throws an error.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.InButtonHandle.released`
**Kind:** `prop`

True if the button was released in this command. Assigning a value throws an error.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.InButtonHandle.state`
**Kind:** `prop`

The raw state of the button. Assigning a value sets the button state.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.InButtonHandle:RemoveButton`
**Kind:** `method`

Removes this button from the command for this tick.

**Source:** `input/game/cmd.lua`


## UserCmd.SubtickMoveStepHandle _()_

One subtick input step. Assigning an unknown field throws an error. If the step was removed from
the command, using the handle throws an error.


  ### `UserCmd.SubtickMoveStepHandle.button`
**Kind:** `prop`

The button that this step presses or releases.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.SubtickMoveStepHandle.index`
**Kind:** `prop`

The index of this step. Read-only.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.SubtickMoveStepHandle.move`
**Kind:** `prop`

The analog movement. Assign nil to remove it.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.SubtickMoveStepHandle.moveangles`
**Kind:** `prop`

The direction used for the movement fix. Assign nil to remove it.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.SubtickMoveStepHandle.preserve`
**Kind:** `prop`

Set it to true to stop the engine from removing this step.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.SubtickMoveStepHandle.pressed`
**Kind:** `prop`

True if the button is pressed, false if it is released.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.SubtickMoveStepHandle.viewangles`
**Kind:** `prop`

The absolute view angles. The roll is ignored. Assign nil to remove them.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.SubtickMoveStepHandle.when`
**Kind:** `prop`

The fraction of the tick at which the step happens.

**Source:** `input/game/cmd.lua`


## CSGOInterpolationInfo" />

An FFI cdata value that describes an interpolation between two recorded ticks.

Use `ffi.new("CSGOInterpolationInfo", { src_tick = 100, dst_tick = 101, frac = 0.5 })` to create
a separate value.

A value read from the command history is a reference into the history. Get it again after you
replace the field or change the history that holds it.


  ### `CSGOInterpolationInfo.dst_tick`
**Kind:** `prop`

The tick at which the interpolation ends.

**Source:** `input/game/cmd.lua`

  ### `CSGOInterpolationInfo.frac`
**Kind:** `prop`

The interpolation amount between the start tick and the end tick.

**Source:** `input/game/cmd.lua`

  ### `CSGOInterpolationInfo.src_tick`
**Kind:** `prop`

The tick at which the interpolation starts.

**Source:** `input/game/cmd.lua`


## TickAndInterpolationAmount\_t" />



An FFI cdata value that stores a tick and a fraction of a tick. It holds an `int tick` followed
by a `float fraction`.

Create one with `TickAndInterpolationAmount_t(tick, fraction)`. Keep the fraction at 0 or more
and less than 1. The constructor does not normalize it.

The tick fields of the command history return separate copies. To update an entry, assign the
edited copy back to the field. Assign nil to clear the field.


  ### `TickAndInterpolationAmount_t.fraction`
**Kind:** `prop`

The fraction of a tick, at 0 or more and less than 1, stored as a 32-bit float.

**Source:** `input/game/cmd.lua`

  ### `TickAndInterpolationAmount_t.tick`
**Kind:** `prop`

The whole number of ticks, stored as a signed 32-bit integer.

**Source:** `input/game/cmd.lua`


<Heading level={2} id="tickandinterpolationamount_t-1">
  TickAndInterpolationAmount_t

  " />
</Heading>


  ### `TickAndInterpolationAmount_t()`
**Kind:** `ctor`

Zero tick and fraction

**Source:** `input/game/cmd.lua`

  ### `TickAndInterpolationAmount_t(tick, fraction)`
**Kind:** `ctor`

**Source:** `input/game/cmd.lua`

  ### `TickAndInterpolationAmount_t(self)`
**Kind:** `ctor`

Copy

**Source:** `input/game/cmd.lua`


## UserCmd.InputHistoryEntryHandle _()_

One recorded input sample of a command. Assigning an unknown field throws an error. If the entry
was removed from the command, using the handle throws an error.

The tick fields return separate FFI cdata copies. To update the entry, edit a copy and assign it
back. Assign nil to clear an optional field.

The interpolation fields return `CSGOInterpolationInfo` references into the entry, and you can
change them in place. Get them again after you replace or clear the field, change the history,
or after the command is no longer valid. Assigning an interpolation value copies it into the entry.


  ### `UserCmd.InputHistoryEntryHandle.cl_interp`
**Kind:** `prop`

The client interpolation amount. Assign nil to clear it.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.InputHistoryEntryHandle.index`
**Kind:** `prop`

The index of this entry, starting at zero. Read-only.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.InputHistoryEntryHandle.player_interp`
**Kind:** `prop`

The player interpolation sample, as a reference into the entry. Assigning a value copies it. Assign nil to clear it.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.InputHistoryEntryHandle.player_tick`
**Kind:** `prop`

The player time, as a separate FFI cdata copy. Assign it back after you edit it. Assign nil to clear it.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.InputHistoryEntryHandle.render_tick`
**Kind:** `prop`

The render time, as a separate FFI cdata copy. Assign it back after you edit it. Assign nil to clear it.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.InputHistoryEntryHandle.sv_interp0`
**Kind:** `prop`

The first server interpolation sample, as a reference into the entry. Assigning a value copies it. Assign nil to clear it.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.InputHistoryEntryHandle.sv_interp1`
**Kind:** `prop`

The second server interpolation sample, as a reference into the entry. Assigning a value copies it. Assign nil to clear it.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.InputHistoryEntryHandle.viewangles`
**Kind:** `prop`

The view angles recorded in this sample. It is a reference into the entry, and you can change it in place.

**Source:** `input/game/cmd.lua`


## UserCmd.SubtickMoves _()_



The subtick steps of a command. Indices start at zero. Reading an index that does not exist
returns nil. A command can hold at most 32 steps.


  ### `UserCmd.SubtickMoves:Size`
**Kind:** `method`

**Source:** `input/game/cmd.lua`

  ### `UserCmd.SubtickMoves:Remove`
**Kind:** `method`

**Arguments:** `i`

Removes the step at index `i`. Indices start at zero. The indices of the steps after it go down by one, so get new handles afterwards. If the index does not exist, the function throws an error.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.SubtickMoves:PopBack`
**Kind:** `method`

Removes the last step. If there are no steps, nothing happens.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.SubtickMoves:Clear`
**Kind:** `method`

Removes all steps.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.SubtickMoves:EmplaceBack`
**Kind:** `method`

**Arguments:** `when?`

Adds a step at the end and returns its handle. If `when` is not given, it is `0`.

A command can hold at most 32 steps. Adding more throws an error. With a button, the value must
      be a single button, otherwise the function throws an error.
    
  

**Source:** `input/game/cmd.lua`


## UserCmd.InputHistory _()_



The input history entries of a command. Indices start at zero. Reading an index that does not
exist returns nil. A command can hold at most 16 entries.


  ### `UserCmd.InputHistory:Size`
**Kind:** `method`

**Source:** `input/game/cmd.lua`

  ### `UserCmd.InputHistory:Remove`
**Kind:** `method`

**Arguments:** `i`

Removes the entry at index `i`. Indices start at zero. The indices of the entries after it go down by one, so get new handles afterwards. If the index does not exist, the function throws an error.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.InputHistory:PopBack`
**Kind:** `method`

Removes the last entry. If there are no entries, nothing happens.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.InputHistory:Clear`
**Kind:** `method`

Removes all entries.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.InputHistory:EmplaceBack`
**Kind:** `method`

Adds an empty entry at the end and returns its handle. A command can hold at most 16 entries. Adding more throws an error. Update the attack history indices of the command when you change the history.

**Source:** `input/game/cmd.lua`

  ### `UserCmd.InputHistory:Resize`
**Kind:** `method`

**Arguments:** `size`

Adds or removes entries at the end, so that the history has exactly `size` entries.

If `size` is less than 0 or more than 16, the function throws an error.
    
  

**Source:** `input/game/cmd.lua`


## simulation _()_

The simulation of players, movement and weapons. It is used by `entity:GetPlayerSimulation` and
the movement events.


  ### `simulation.Grenade`
**Kind:** `fn`

**Arguments:** `source, thrower?`

Creates a grenade simulation.

Pass an item definition index to simulate a grenade without an entity. The supported indices
      are 43 for the flashbang, 44 for the HE grenade, 45 for the smoke grenade, 46 for the molotov,
      47 for the decoy and 48 for the incendiary grenade. You can also pass the `m_iItemDefinitionIndex`
      of a held weapon.

      Pass a held grenade or a grenade projectile in flight to simulate that entity. A projectile
      follows its networked state during `Run`.

      Without a thrower, hits on players are skipped. If the simulation has neither a thrower nor a
      grenade entity, `GetDamage` throws an error for HE grenades. The spread of fire from a molotov
      or incendiary grenade is not simulated.
    
  

**Source:** `input/game/cmd.lua`

  ### `simulation.SimOpt`
**Kind:** `type`

Flags that make a simulation faster by skipping parts of it. Combine them with `bit.bor` into the mask that `entity:GetPlayerSimulation` takes. Each flag skips the named part. Only combine the flags for parts that your simulation does not need.


  

**Source:** `input/game/cmd.lua`


## simulation.Player _()_

A snapshot of a player for prediction.

A snapshot that an event gives you can be write protected. Changing it or running a command on
it then throws an error. Call `Clone` first and change the copy.

Reading an unknown field returns nil. Assigning an unknown field or a read-only field does nothing.


  ### `simulation.Player.controller`
**Kind:** `prop`

The controller of the simulated pawn, or nil if it no longer exists. Read-only.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.curtime`
**Kind:** `prop`

The current simulation time, in seconds.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.frametime`
**Kind:** `prop`

The length of one simulation tick, in seconds.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_MoveType`
**Kind:** `prop`

The current movement type, for example walk, fly, ladder or noclip.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_angEyeAngles`
**Kind:** `prop`

The final eye angles after the subtick and aim punch changes.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_angStashedShootAngles`
**Kind:** `prop`

The saved shoot angles for the queued grenade throw.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_bCanMove`
**Kind:** `prop`

The saved result of whether the player can move.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_bCarryingHostage`
**Kind:** `prop`

True while the player carries a hostage.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_bFreezePeriod`
**Kind:** `prop`

True while the round is in its freeze time.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_bGrenadeParametersStashed`
**Kind:** `prop`

True if the throw parameters of a grenade are saved for a queued throw.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_bIsDefusing`
**Kind:** `prop`

True while the player is defusing the bomb.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_bIsGrabbingHostage`
**Kind:** `prop`

True while the player is picking up a hostage.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_bIsScoped`
**Kind:** `prop`

True while the player is scoped in.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_bIsWalking`
**Kind:** `prop`

True while the player holds the walk key.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_bResumeZoom`
**Kind:** `prop`

True if the scope zooms in again after a shot.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_bWaitForNoAttack`
**Kind:** `prop`

True if the player cannot fire until the attack button is released.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_fFlags`
**Kind:** `prop`

The bit mask of entity flags, for example on ground, ducking or frozen.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_fImmuneToGunGameDamageTime`
**Kind:** `prop`

The time at which the spawn protection in gun game ends.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_fRoundStartTime`
**Kind:** `prop`

The time at which the current round started.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_flFlinchStack`
**Kind:** `prop`

The flinch that has built up from recent damage.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_flFriction`
**Kind:** `prop`

The friction multiplier of the surface for the player.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_flGravityScale`
**Kind:** `prop`

The multiplier for the world gravity for this player.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_flPlayerMaxSpeed`
**Kind:** `prop`

The maximum speed at which the player can move.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_flVelocityModifier`
**Kind:** `prop`

The speed multiplier after the player takes damage.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_flWaterJumpTime`
**Kind:** `prop`

The time that is left in the current water jump.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_flWaterLevel`
**Kind:** `prop`

How deep the player is in water.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_grenadeParameterStashTime`
**Kind:** `prop`

The time at which the throw parameters of the grenade were saved.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_hGroundEntity`
**Kind:** `prop`

The entity the player stands on, or nil. Read-only.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_iShotsFired`
**Kind:** `prop`

The number of shots fired in a row. Recoil grows with it.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_iTeamNum`
**Kind:** `prop`

The team number of the player.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_ignoreLadderJumpTime`
**Kind:** `prop`

The time until the player can jump off a ladder again.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_nInteractsWith`
**Kind:** `prop`

The collision mask of the contents that the player interacts with.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_nRandomSeed`
**Kind:** `prop`

The random seed for spread and recoil.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_nTickBase`
**Kind:** `prop`

The tick base. The prediction time of the player is calculated from it.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_nWaterType`
**Kind:** `prop`

The contents mask of the water the player is in.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_nodeToWorld`
**Kind:** `prop`

The transform from the scene node of the player to world space. Read-only.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_predictableBaseAngle`
**Kind:** `prop`

The base angle of the predictable part of the aim punch.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_predictableBaseAngleVel`
**Kind:** `prop`

The base angular velocity of the predictable part of the aim punch.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_predictableBaseTick`
**Kind:** `prop`

The base tick of the predictable part of the aim punch.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_predictableBaseTickInterpAmount`
**Kind:** `prop`

The interpolation fraction of the base tick of the predictable aim punch.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_unpredictableBaseAngle`
**Kind:** `prop`

The base angle of the unpredictable part of the aim punch.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_unpredictableBaseTick`
**Kind:** `prop`

The base tick of the unpredictable part of the aim punch.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_vecAbsOrigin`
**Kind:** `prop`

The origin of the player in world space.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_vecAbsVelocity`
**Kind:** `prop`

The velocity of the player in world space.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_vecBaseVelocity`
**Kind:** `prop`

The velocity given by the surface the player stands on, for example a conveyor or a moving platform.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_vecRootBoneOffset`
**Kind:** `prop`

The offset of the root bone of the skeleton from the origin.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_vecStashedGrenadeThrowPosition`
**Kind:** `prop`

The saved throw position for the queued grenade throw.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_vecStashedVelocity`
**Kind:** `prop`

The saved velocity of the player for the queued grenade throw.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_vecViewOffset`
**Kind:** `prop`

The offset from the origin to the eye position.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.m_vecWaterJumpVel`
**Kind:** `prop`

The velocity used during the current water jump.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.movement`
**Kind:** `prop`

The movement part, which handles walking, jumping, ducking and collision. Read-only.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.mp_teammates_are_enemies`
**Kind:** `prop`

True if teammates count as enemies for collision and damage.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.optimizations`
**Kind:** `prop`

The mask of enabled `simulation.SimOpt` flags.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.pawn`
**Kind:** `prop`

The pawn this snapshot was taken from, or nil if it no longer exists. Read-only.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.tick_frac`
**Kind:** `prop`

The fraction of the current tick, from 0 to 1.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.tickcount`
**Kind:** `prop`

The current tick of the simulation.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.v_angle`
**Kind:** `prop`

The raw view angles from the command.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player.weapon`
**Kind:** `prop`

The weapon part, which handles firing, accuracy and grenades. Read-only.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player:RunCommand`
**Kind:** `method`

**Arguments:** `cmd`

Advances the simulation by one user command.

Clone a simulation that an event gave you before you run commands on it. If the simulation is
      write protected, this function throws an error. The result is only a prediction and is not sent
      to the server.
    
  

**Source:** `input/game/cmd.lua`

  ### `simulation.Player:Clone`
**Kind:** `method`

Returns a full copy of this simulation snapshot. The copy is not write protected.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player:IsFreezePeriod`
**Kind:** `method`

Returns true if the round is in its freeze time and the player cannot move yet.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player:SetAbsOrigin`
**Kind:** `method`

**Arguments:** `origin`

Moves the player to the given origin in world space. If the simulation is write protected, this function throws an error.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player:IsThrowingGrenade`
**Kind:** `method`

Returns true if the player is throwing a grenade at this moment.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player:IsJumpThrow`
**Kind:** `method`

**Arguments:** `throw_duration`

Checks the timing window of a jump throw. The function adds the given throw delay to the current simulation time and subtracts the time at which the grenade parameters were saved. It returns true if the result is more than 0 and at most 0.2 seconds. The delay is not the time for which a button is held.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player:GetEyePosition`
**Kind:** `method`

Returns the current eye position of the player in world space.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player:IsEnemy`
**Kind:** `method`

**Arguments:** `other`

Returns true if `other` is an enemy of the simulated player. The `mp_teammates_are_enemies` setting is taken into account. Returns false if `other` is nil or the simulated player.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player:IsTeamMate`
**Kind:** `method`

**Arguments:** `other`

Returns true if `other` is a teammate of the simulated player. It always returns the opposite of `player:IsEnemy`, so it also returns true if `other` is nil or the simulated player.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player:GetSimpleFilter`
**Kind:** `method`

**Arguments:** `collision_group?`

Returns a trace filter that is set up in the same way as the filter of the movement code. The filter ignores the simulated player and uses the given collision group.

**Source:** `input/game/cmd.lua`

  ### `simulation.Player:GetShape`
**Kind:** `method`

Returns the current collision shape of the player. Its size depends on the current duck amount.

<Warning>
        The shape is an opaque `GenericShape_t` value for FFI code. It has no fields or methods in Lua.
      </Warning>
    
  

**Source:** `input/game/cmd.lua`


## simulation.Movement _()_

The movement part of a simulated player. It handles walking, movement in the air, jumping,
ducking, ladders, and the console variables that control them.

Reading an unknown field returns nil. Assigning an unknown field does nothing. If the simulation
is write protected, assigning a field throws an error.


  ### `simulation.Movement.host_timescale`
**Kind:** `prop`

The time scale of the simulation.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_bDesiresDuck`
**Kind:** `prop`

True while the duck button is held.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_bDuckOverride`
**Kind:** `prop`

True if ducking is forced, whatever the input is.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_bDucked`
**Kind:** `prop`

True if the player is fully ducked.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_bDucking`
**Kind:** `prop`

True while the player is ducking.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_bHasWalkMovedSinceLastJump`
**Kind:** `prop`

True if the player has walked since the last jump.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_bJumpApexPending`
**Kind:** `prop`

True while the player is still rising toward the highest point of a jump.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_bUseFrictionStashedSpeed`
**Kind:** `prop`

True if the speed saved before friction is used again.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_bUsingGroundTopologyOffset`
**Kind:** `prop`

True if a view offset for the shape of the ground is applied.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_fStashGrenadeParameterWhen`
**Kind:** `prop`

The subtick time at which the grenade parameters should be saved.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_flAccumulatedJumpError`
**Kind:** `prop`

The error in subtick jump timing that has built up.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_flBombPlantViewOffset`
**Kind:** `prop`

The vertical view offset caused by planting the bomb.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_flDuckAmount`
**Kind:** `prop`

The current duck progress. `0` means standing and `1` means fully ducked.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_flDuckRootOffset`
**Kind:** `prop`

The vertical offset of the root bone caused by ducking.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_flDuckSpeed`
**Kind:** `prop`

The speed at which the duck amount changes.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_flDuckViewOffset`
**Kind:** `prop`

The vertical view offset caused by ducking.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_flFallVelocity`
**Kind:** `prop`

The downward speed during the current fall.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_flFrictionStashedSpeed`
**Kind:** `prop`

The speed saved before friction was applied.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_flLastDuckTime`
**Kind:** `prop`

The time of the last change between ducking and standing.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_flLastJumpFrac`
**Kind:** `prop`

The subtick fraction of the last jump.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_flLastJumpVelocityZ`
**Kind:** `prop`

The vertical velocity at the moment of the last jump.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_flMaxspeed`
**Kind:** `prop`

The current maximum movement speed.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_flStamina`
**Kind:** `prop`

The current stamina. Low stamina limits repeated jumps.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_flSurfaceFriction`
**Kind:** `prop`

The friction of the surface the player stands on.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_flUseFrictionStashedSpeedUntilFrac`
**Kind:** `prop`

The subtick fraction until which the saved speed is used.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_flUsingGroundTopologyOffsetTransitionSmoothing`
**Kind:** `prop`

The smoothing amount for changes of the ground shape offset.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_nLastJumpTick`
**Kind:** `prop`

The tick at which the player last jumped.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_vecCmdMove`
**Kind:** `prop`

The movement input of the previous command.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_vecLadderNormal`
**Kind:** `prop`

The surface normal of the ladder the player is on.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_vecLastMovementImpulses`
**Kind:** `prop`

The movement impulses applied in the previous tick.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_vecLastPositionAtFullCrouchSpeed`
**Kind:** `prop`

The last position recorded while the player moved at full crouch speed.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.m_vecWalkWishVel`
**Kind:** `prop`

The walking velocity that the input asks for.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.mp_solid_teammates`
**Kind:** `prop`

The collision mode between teammates.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.mp_tagging_scale`
**Kind:** `prop`

The multiplier for the slowdown when the player is hit by enemy fire.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_accelerate`
**Kind:** `prop`

The acceleration rate on the ground.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_accelerate_use_weapon_speed`
**Kind:** `prop`

True if the acceleration depends on the speed of the active weapon.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_air_max_wishspeed`
**Kind:** `prop`

The maximum wish speed for acceleration in the air.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_airaccelerate`
**Kind:** `prop`

The acceleration rate in the air.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_autobunnyhopping`
**Kind:** `prop`

True if automatic bunny hopping is enabled.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_bhop_time_window`
**Kind:** `prop`

The time window in which a queued jump can start a bunny hop.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_bounce`
**Kind:** `prop`

How much speed is kept when bouncing off surfaces.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_enablebunnyhopping`
**Kind:** `prop`

True if bunny hopping without a speed limit on jumps is allowed.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_falldamage_scale`
**Kind:** `prop`

The multiplier for fall damage.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_friction`
**Kind:** `prop`

The base friction on the ground.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_gravity`
**Kind:** `prop`

The world gravity, in units per second squared.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_jump_impulse`
**Kind:** `prop`

The upward velocity added by a jump.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_jump_precision_enable`
**Kind:** `prop`

True if exact subtick jump timing is enabled.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_jump_spam_penalty_time`
**Kind:** `prop`

The cooldown that punishes pressing jump too often.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_ladder_angle`
**Kind:** `prop`

The view angle limit that affects getting onto a ladder.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_ladder_dampen`
**Kind:** `prop`

How much movement is slowed down on ladders.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_ladder_scale_speed`
**Kind:** `prop`

The speed multiplier for climbing ladders.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_legacy_jump`
**Kind:** `prop`

True if the old jump behavior is used.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_maxspeed`
**Kind:** `prop`

The global maximum speed of players.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_maxvelocity`
**Kind:** `prop`

The maximum velocity along any single axis.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_noclipaccelerate`
**Kind:** `prop`

The acceleration rate in noclip.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_noclipspeed`
**Kind:** `prop`

The movement speed in noclip.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_optimizedmovement`
**Kind:** `prop`

True if the optimized movement code is used.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_quantize_movement_input`
**Kind:** `prop`

True if the movement input is rounded before it is applied.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_staminajumpcost`
**Kind:** `prop`

The stamina that a jump costs.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_staminalandcost`
**Kind:** `prop`

The stamina that a landing costs.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_staminamax`
**Kind:** `prop`

The maximum stamina.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_staminarecoveryrate`
**Kind:** `prop`

The rate at which stamina recovers.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_standable_normal`
**Kind:** `prop`

The minimum Z value of a surface normal for a slope that the player can stand on.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_step_move_vel_min`
**Kind:** `prop`

The minimum velocity needed to try a step move.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_stepsize`
**Kind:** `prop`

The maximum height that the player can step up without a jump.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_stopspeed`
**Kind:** `prop`

The speed below which stronger stopping friction is applied.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_subtick_movement_view_angles`
**Kind:** `prop`

True if the subtick view angles are used for movement.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_timebetweenducks`
**Kind:** `prop`

The minimum time between two duck inputs.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_walkable_normal`
**Kind:** `prop`

The minimum Z value of a surface normal for a slope that the player can walk on.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement.sv_water_slow_amount`
**Kind:** `prop`

How much the player is slowed down in water.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement:CanMove`
**Kind:** `method`

Returns true if the player can move. The player cannot move while frozen, while defusing the bomb, or while arming the bomb.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement:CanJump`
**Kind:** `method`

Returns true if the player can jump. The player must be on the ground, and the jump cooldown must be over.

**Source:** `input/game/cmd.lua`

  ### `simulation.Movement:GetMaxSpeed`
**Kind:** `method`

Returns the current maximum movement speed. The `forwardmove` and `leftmove` input is multiplied by this speed to get the wish velocity.

The value uses the current state. It includes the walk speed limit, the duck speed limit,
      stamina, the slowdown from damage, the surface, the movement speed while attacking, and the
      slowdown after landing.

      To find the `forwardmove` or `leftmove` fraction for a given speed, divide that speed by this
      value, for example `cmd.forwardmove = target_speed / sim.movement:GetMaxSpeed()`.
    
  

**Source:** `input/game/cmd.lua`


## simulation.Weapon _()_

The weapon part of a simulated player. It handles when the weapon can fire, accuracy and recoil,
reloading, and the throw parameters of grenades. Use a full simulation with
`simulation.SimOpt.kNone` to include the weapon behavior.

Reading an unknown field returns nil. Assigning an unknown field or a read-only field does
nothing. If the simulation is write protected, assigning a field throws an error.


  ### `simulation.Weapon.m_bBurstMode`
**Kind:** `prop`

True if the weapon is in burst fire mode.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_bFireOnEmpty`
**Kind:** `prop`

True if the weapon tried to fire with an empty clip.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_bInReload`
**Kind:** `prop`

True while the weapon is reloading.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_bIsHauledBack`
**Kind:** `prop`

True while the hammer of the revolver is pulled back.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_bIsHeldByPlayer`
**Kind:** `prop`

True while the player holds the grenade.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_bJumpThrow`
**Kind:** `prop`

True if the grenade is queued for a jump throw.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_bJustPulledPin`
**Kind:** `prop`

True if the pin of the grenade was pulled in this tick.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_bPinPulled`
**Kind:** `prop`

True if the pin of the grenade is pulled.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_bRedraw`
**Kind:** `prop`

True while the weapon is drawn again after a throw.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_bStartedArming`
**Kind:** `prop`

True if the player has started to arm the bomb.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_fAccuracyPenalty`
**Kind:** `prop`

The inaccuracy penalty that has built up from recent actions.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_fLastShotTime`
**Kind:** `prop`

The time of the last shot.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_fThrowTime`
**Kind:** `prop`

The time at which the grenade is thrown.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_flNextAttack`
**Kind:** `prop`

The earliest time at which the player can attack again.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_flNextHoldFrac`
**Kind:** `prop`

The subtick fraction at which the next grenade hold action is ready.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_flNextPrimaryAttackTickRatio`
**Kind:** `prop`

The subtick fraction at which the next primary attack is ready.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_flNextSecondaryAttackTickRatio`
**Kind:** `prop`

The subtick fraction at which the next secondary attack is ready.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_flPostponeFireReadyFrac`
**Kind:** `prop`

The subtick fraction at which the delayed shot is ready.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_flRecoilIndex`
**Kind:** `prop`

The current recoil index. It goes up with each shot.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_flThrowStrength`
**Kind:** `prop`

The throw strength of the held grenade, from 0 to 1.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_flWatTickOffset`
**Kind:** `prop`

The tick offset of the weapon attack time. It is used to interpolate the shoot position.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_hActiveWeapon`
**Kind:** `prop`

The weapon the player is holding. Read-only.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_iClip1`
**Kind:** `prop`

The ammo in the primary clip.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_iClip2`
**Kind:** `prop`

The ammo in the secondary clip.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_iItemDefinitionIndex`
**Kind:** `prop`

The item definition index, which identifies the weapon type.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_nNextHoldTick`
**Kind:** `prop`

The tick at which the next grenade hold action is ready.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_nNextPrimaryAttackTick`
**Kind:** `prop`

The tick at which the next primary attack is ready.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_nNextSecondaryAttackTick`
**Kind:** `prop`

The tick at which the next secondary attack is ready.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_nOldTotalInputHistoryCount`
**Kind:** `prop`

The previous total number of recorded input history entries.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_nOldTotalShootPositionHistoryCount`
**Kind:** `prop`

The previous total number of recorded shoot positions.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_nPostponeFireReadyTicks`
**Kind:** `prop`

The tick at which a delayed shot, for example of the revolver, is ready.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_weaponMode`
**Kind:** `prop`

The current fire mode of the weapon.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon.m_zoomLevel`
**Kind:** `prop`

The current zoom level of the scope. `0` means not zoomed.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon:GetMaxSpeed`
**Kind:** `method`

Returns the maximum movement speed while the player holds this weapon in its current mode.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon:GetInaccuracy`
**Kind:** `method`

Returns the current inaccuracy of the weapon. It includes movement, jumping and the built up accuracy penalty.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon:GetSpread`
**Kind:** `method`

Returns the base spread of the weapon in its current mode.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon:IsPrimaryAttackReady`
**Kind:** `method`

Returns true if the primary attack of the weapon is ready at the current simulation time.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon:IsSecondaryAttackReady`
**Kind:** `method`

Returns true if the secondary attack of the weapon is ready at the current simulation time.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon:IsPostponeFireReady`
**Kind:** `method`

Returns true if the delayed shot of the revolver is ready at the current simulation time.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon:GetThrowStrength`
**Kind:** `method`

Returns the throw strength of the held grenade, from 0 to 1.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon:GetPlayerShootPosition`
**Kind:** `method`

**Arguments:** `tick`

Returns the position in world space from which shots are fired at the given tick and fraction. The position is interpolated from the recorded shoot position history.

For the shot of the current input poll, read the `shoot_position` field of the input poll
      context instead.
    
  

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon:UpdateAccuracyPenalty`
**Kind:** `method`

Advances the accuracy penalty of the player. If the simulation is write protected, this function throws an error.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon:IsGun`
**Kind:** `method`

Returns true if the active weapon is a gun. Guns are pistols, rifles, submachine guns, shotguns, sniper rifles, machine guns and the taser.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon:IsKnife`
**Kind:** `method`

Returns true if the active weapon is a knife.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon:HasZoom`
**Kind:** `method`

Returns true if the active weapon has a scope with zoom levels.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon:HasSecondaryAttack`
**Kind:** `method`

Returns true if the active weapon has a secondary attack. This is true for weapons with zoom, burst mode, a removable silencer or secondary ammo, and for the revolver.

**Source:** `input/game/cmd.lua`

  ### `simulation.Weapon:CanAttack`
**Kind:** `method`

**Arguments:** `mode`

Returns true if the weapon can attack in the given mode at the current simulation time.

**Source:** `input/game/cmd.lua`


## simulation.GrenadePoint _()_

One point of a simulated grenade path.


  ### `simulation.GrenadePoint.bounced`
**Kind:** `prop`

True if the grenade bounced at this point.

**Source:** `input/game/cmd.lua`

  ### `simulation.GrenadePoint.position`
**Kind:** `prop`

The position in world space.

**Source:** `input/game/cmd.lua`

  ### `simulation.GrenadePoint.tick`
**Kind:** `prop`

The simulation tick of this point.

**Source:** `input/game/cmd.lua`

  ### `simulation.GrenadePoint.time`
**Kind:** `prop`

The time since the start of the simulation, in seconds.

**Source:** `input/game/cmd.lua`

  ### `simulation.GrenadePoint.velocity`
**Kind:** `prop`

The velocity at this point.

**Source:** `input/game/cmd.lua`


## simulation.GrenadeDamage _()_

The predicted damage of an HE grenade to one player.


  ### `simulation.GrenadeDamage.is_enemy`
**Kind:** `prop`

True if the player is an enemy of the thrower.

**Source:** `input/game/cmd.lua`

  ### `simulation.GrenadeDamage.max_damage`
**Kind:** `prop`

The highest predicted damage.

**Source:** `input/game/cmd.lua`

  ### `simulation.GrenadeDamage.min_damage`
**Kind:** `prop`

The lowest predicted damage.

**Source:** `input/game/cmd.lua`

  ### `simulation.GrenadeDamage.passed`
**Kind:** `prop`

True if the lowest damage reaches the health of the player or the quarter range limit of the game. It does not always mean that the grenade kills the player.

**Source:** `input/game/cmd.lua`

  ### `simulation.GrenadeDamage.player`
**Kind:** `prop`

The damaged player.

**Source:** `input/game/cmd.lua`


## simulation.Grenade _()_

A grenade simulation that runs separately from the real game state. Each `Run` traces up to
1000 ticks, so do not run it more often than the result can change. All fields are read-only.


  ### `simulation.Grenade.definition_index`
**Kind:** `prop`

The item definition index of the simulated grenade.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade.detonated`
**Kind:** `prop`

True if the last `Run` reached a detonation.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade.detonation_time`
**Kind:** `prop`

The detonation time in seconds, or nil if the grenade did not detonate.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade.entity`
**Kind:** `prop`

The grenade entity, or nil if there is none or it no longer exists.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade.exploded_from_inferno`
**Kind:** `prop`

The fire that set off this smoke grenade, or nil.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade.extinguished_by_smoke`
**Kind:** `prop`

The smoke that put out this grenade, or nil.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade.ground_entity`
**Kind:** `prop`

The entity that the grenade came to rest on, or nil.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade.hit_enemy`
**Kind:** `prop`

The player that the grenade hit, or nil.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade.initial_position`
**Kind:** `prop`

The start position.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade.initial_velocity`
**Kind:** `prop`

The start velocity.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade.molotov_burn_position`
**Kind:** `prop`

The position at which a molotov starts to burn, or nil.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade.molotov_splash_normal`
**Kind:** `prop`

The surface normal at which the molotov splashed, or nil.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade.origin`
**Kind:** `prop`

The current simulated origin. After `Run`, this is the final origin.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade.simulated_time`
**Kind:** `prop`

The time simulated so far, in seconds.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade.thrower`
**Kind:** `prop`

The thrower, or nil if there is none or it no longer exists.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade.velocity`
**Kind:** `prop`

The current simulated velocity.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade:SetInitial`
**Kind:** `method`

**Arguments:** `position, velocity`

Sets the start position and the start velocity directly. If a value is not a finite vector, the function throws an error.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade:SetThrow`
**Kind:** `method`

**Arguments:** `eye_position, angles, velocity, strength, pawn_center?`

Sets the start state from a throw, with the same calculation as the game. If a value is not finite, the function throws an error.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade:Run`
**Kind:** `method`

Runs the simulation. You can call it again to simulate again.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade:GetPath`
**Kind:** `method`

**Arguments:** `time_offset?`

Returns the simulated path.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade:GetBounces`
**Kind:** `method`

**Arguments:** `time_offset?`

Returns only the points of the simulated path at which the grenade bounced.

**Source:** `input/game/cmd.lua`

  ### `simulation.Grenade:GetDamage`
**Kind:** `method`

**Arguments:** `origin?`

Returns the predicted damage of an HE grenade. For other grenades, it returns an empty table.

If `origin` is given, the damage is calculated at that point and `Run` is not needed. This way
      you can use the end point of a recorded path directly. If `origin` is not given, the damage is
      calculated at the point where the last `Run` detonated. If the grenade did not detonate, the
      table is empty.

      Players are checked at their positions at the time of the call, not at the time of the
      detonation. Dormant, dead and immune players are left out.
    
  

**Source:** `input/game/cmd.lua`

---


## Input

<!-- Source: cs2/api_reference/input/input.md -->

---
title: Input
icon: keyboard
sidebar_position: 1
---

# Input

## input.ButtonHandle _()_

The state of one physical button on the keyboard, mouse or gamepad. All fields are read-only.

`pressed` and `released` are true only during the input poll in which the change happened.


  ### `input.ButtonHandle.binding`
**Kind:** `prop`

The console command that is bound to this button.

**Source:** `input/game/input.lua`

  ### `input.ButtonHandle.down`
**Kind:** `prop`

True while the button is held down.

**Source:** `input/game/input.lua`

  ### `input.ButtonHandle.pressed`
**Kind:** `prop`

True if the button went down in the current input poll.

**Source:** `input/game/input.lua`

  ### `input.ButtonHandle.pressed_tick`
**Kind:** `prop`

The input poll tick of the last press. This is a tick of the input system, so it does not match game ticks or simulation ticks.

**Source:** `input/game/input.lua`

  ### `input.ButtonHandle.released`
**Kind:** `prop`

True if the button went up in the current input poll.

**Source:** `input/game/input.lua`

  ### `input.ButtonHandle.released_tick`
**Kind:** `prop`

The input poll tick of the last release. This is a tick of the input system, so it does not match game ticks or simulation ticks.

**Source:** `input/game/input.lua`


## input.buttons _()_

The physical buttons, indexed by their engine names in lowercase, for example `input.buttons.mouse1`.
Names must be written in lowercase. An unknown name returns nil. All entries are read-only.

Read the `pressed` and `released` states inside the `input_poll` event, because they are only true
during the poll in which the change happened.


  ### `input.buttons[string]`
**Kind:** `prop`

**Source:** `input/game/input.lua`


## input.analog _()_

The physical axes, indexed by their engine names in lowercase. Names must be written in lowercase.
An unknown name returns nil. All entries are read-only.

The mouse wheel value is a running total. To get the movement since your last reading, subtract
the last reading from the current one. A negative result means the wheel moved down, and a
positive result means it moved up. Several wheel steps can add up between two callbacks.


  ### `input.analog[string]`
**Kind:** `prop`

**Source:** `input/game/input.lua`

  ### `input.analog.mouse_wheel`
**Kind:** `prop`

The running total of the mouse wheel position. Subtract the last reading to get the movement. Read-only.

**Source:** `input/game/input.lua`


<Heading level={2} id="input-1">
  input

   _()_
</Heading>


  ### `input.analog`
**Kind:** `prop`

The live values of the physical axes, indexed by their engine names in lowercase. Read-only.

**Source:** `input/game/input.lua`

  ### `input.buttons`
**Kind:** `prop`

The live state of the physical buttons, indexed by their engine names in lowercase. Read-only.

**Source:** `input/game/input.lua`

  ### `input.poll_tick`
**Kind:** `prop`

The current input poll tick. Compare it with `pressed_tick` and `released_tick` of a button. Read-only.

**Source:** `input/game/input.lua`


## input.InputPollContext _()_

The argument of the `input_poll` event. It holds the live input state of the current poll,
before the game turns it into a user command.

To steer the local player, use `SetViewAngles` and the writable button fields. These change
the input in the same way as the mouse and keyboard. Use them instead of writing to the user
command directly.

Enabled legitbot features run after the callback and can override what it sets. When the
aimbot has a target, it replaces the view angles. Movement features override the same buttons.

Use this object only inside the callback. Using it outside the callback throws an error.
Reading or writing an unknown field also throws an error.

To change the view angles, read them with `GetViewAngles`, edit the returned copy, and pass it
to `SetViewAngles`.

The timing, position and shot fields are read-only. The shot fields are nil while the data
that the game last presented is out of date. The writable button fields are listed below.


  ### `input.InputPollContext.aim_punch`
**Kind:** `prop`

The aim punch at the presented player time. It uses the same angle space as `shoot_angles`. The value is nil while the presented data is out of date. Read-only.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.eye_position`
**Kind:** `prop`

The eye position of the rendered player, which is the origin plus the view offset. Aim and field of view calculations use this position. A shot does not start from this position. Read-only.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.frame_start`
**Kind:** `prop`

True if this is the first poll of the frame. Read-only.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.in_attack`
**Kind:** `prop`

Primary fire.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.in_attack2`
**Kind:** `prop`

Secondary fire or scope.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.in_back`
**Kind:** `prop`

Move backward.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.in_duck`
**Kind:** `prop`

Duck.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.in_forward`
**Kind:** `prop`

Move forward.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.in_joyautosprint`
**Kind:** `prop`

Automatic sprint with a gamepad.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.in_jump`
**Kind:** `prop`

Jump.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.in_look_at_weapon`
**Kind:** `prop`

Inspect the weapon.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.in_moveleft`
**Kind:** `prop`

Move left.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.in_moveright`
**Kind:** `prop`

Move right.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.in_reload`
**Kind:** `prop`

Reload.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.in_score`
**Kind:** `prop`

Show the scoreboard.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.in_speed`
**Kind:** `prop`

Walk.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.in_turnleft`
**Kind:** `prop`

Turn left with the keyboard or a gamepad.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.in_turnright`
**Kind:** `prop`

Turn right with the keyboard or a gamepad.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.in_use`
**Kind:** `prop`

Use or interact.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.in_useorreload`
**Kind:** `prop`

The shared button for use and reload.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.in_zoom`
**Kind:** `prop`

Zoom or toggle the scope.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.input_fraction`
**Kind:** `prop`

The fraction of the current tick at which this poll happened. Read-only.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.input_time`
**Kind:** `prop`

The time at which this poll happened, in seconds. Read-only.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.loopback`
**Kind:** `prop`

True if the game is connected to a local server. Read-only.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.player_fraction`
**Kind:** `prop`

The corrected fraction of the player tick of this poll. Read-only.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.player_tick`
**Kind:** `prop`

The corrected player tick of this poll. Read-only.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.pred_fraction`
**Kind:** `prop`

The prediction time fraction of this poll. It is always `0` during the poll. Use `player_tick` and `player_fraction` for the corrected time. Read-only.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.shoot_angles`
**Kind:** `prop`

The view angles that a shot fired from this poll uses, without aim punch. Add `aim_punch` to get the real direction of the bullet. The value is nil while the presented data is out of date. Read-only.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.shoot_position`
**Kind:** `prop`

The position in world space from which a shot fired in this poll starts. It is different from `eye_position`. The value is nil while the presented data is out of date. Read-only.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.shoot_velocity`
**Kind:** `prop`

The speed and direction in which `shoot_position` moves, in units per second. It is the change of `shoot_position` over the one tick before the presented player time. It includes the movement of the view offset when the player ducks or lands, so it is different from `m_vecAbsVelocity` of the pawn. The value is nil while the presented data is out of date. Read-only.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.tick_start`
**Kind:** `prop`

True if this is the first poll of the tick. Read-only.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.tick_start_time`
**Kind:** `prop`

The time at which the current tick started, in seconds. Read-only.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext.time_delta`
**Kind:** `prop`

The time since the last poll, in seconds. Read-only.

**Source:** `input/game/input.lua`

  ### `input.InputPollContext:GetViewAngles`
**Kind:** `method`

Returns a copy of the view angles for this poll. The angles include the mouse input of the user and the angles set by `SetViewAngles` so far. Changing the copy does not change the poll. Pass the copy to `SetViewAngles` to apply it.

Call this function only inside the `input_poll` event.
    
  

**Source:** `input/game/input.lua`

  ### `input.InputPollContext:SetViewAngles`
**Kind:** `method`

**Arguments:** `angles`

Turns the view to the given angles in this poll, in the same way as mouse movement. The difference is converted with the mouse sensitivity, and there is no limit on how far the view turns in one poll. Enabled legitbot features run afterwards and can still change the angles.

Call this function only inside the `input_poll` event. The angles must be finite numbers and
      the roll must be zero, otherwise the function throws an error. While the game does not control
      the mouse, for example while the menu is open, the call is ignored.
    
  

**Source:** `input/game/input.lua`

---


## Motion

<!-- Source: cs2/api_reference/input/motion.md -->

---
title: Motion
icon: footprints
---

# Motion

## motion.WalkTo.Options _()_

Options for `motion.WalkTo`. All fields are optional. If `approach` or `stop` is not a value of
its enum, `motion.WalkTo` throws an error.


  ### `motion.WalkTo.Options.approach`
**Kind:** `prop`

How the path toward the target is shaped. The default is `motion.ApproachMode.kArc`.

**Source:** `input/game/motion.lua`

  ### `motion.WalkTo.Options.nudge`
**Kind:** `prop`

Allows small corrections near the target. The default is `true`.

**Source:** `input/game/motion.lua`

  ### `motion.WalkTo.Options.stop`
**Kind:** `prop`

How the point to start stopping is chosen. The default is `motion.StopMode.kClosest`.

**Source:** `input/game/motion.lua`

  ### `motion.WalkTo.Options.velocity`
**Kind:** `prop`

The velocity that the player should have on arrival, in units per second. Use it to follow a moving target. The default is a zero vector.

**Source:** `input/game/motion.lua`

  ### `motion.WalkTo.Options.walk`
**Kind:** `prop`

Pass `true` to always walk, and `false` to always run. If it is nil, the walk key of the user is kept.

**Source:** `input/game/motion.lua`


## motion.Status _()_

The result of the last command that the planner planned. It is returned by `motion.Status`.


  ### `motion.Status.command_number`
**Kind:** `prop`

The number of the command that this status was planned for.

**Source:** `input/game/motion.lua`

  ### `motion.Status.distance`
**Kind:** `prop`

The distance that is left to the target.

**Source:** `input/game/motion.lua`

  ### `motion.Status.phase`
**Kind:** `prop`

The current phase of the planner.

**Source:** `input/game/motion.lua`

  ### `motion.Status.predicted_miss`
**Kind:** `prop`

The predicted distance by which the player misses the target. It is nil when there is no prediction.

**Source:** `input/game/motion.lua`

  ### `motion.Status.speed`
**Kind:** `prop`

The planned speed for the command.

**Source:** `input/game/motion.lua`

  ### `motion.Status.ticks_to_rest`
**Kind:** `prop`

The predicted number of ticks until the player stops. It is nil when there is no prediction.

**Source:** `input/game/motion.lua`

  ### `motion.Status.Phase`
**Kind:** `type`

The phase of the movement planner, returned in `motion.Status`.

**Source:** `input/game/motion.lua`


<Heading level={2} id="motion-1">
  motion

   _()_
</Heading>


  ### `motion.WalkTo`
**Kind:** `fn`

**Arguments:** `target, options?`

Asks the movement planner to walk the local player to `target`. The request is valid only for the command that is being built, so call this function in every `pre_movement` event.

If `options` is not a table or nil, the function throws an error.
    
  

**Source:** `input/game/motion.lua`

  ### `motion.Active`
**Kind:** `fn`

Returns true if a `motion.WalkTo` request is active for the command that is being built.

**Source:** `input/game/motion.lua`

  ### `motion.Status`
**Kind:** `fn`

Returns the result of the last command that the planner planned.

In the `pre_movement` event, this is the result of the previous command, because the current
      command is planned after the event returns. In the `post_movement` event, this is the result of
      the current command.

      Returns nil if the planner did not control the last command.
    
  

**Source:** `input/game/motion.lua`

  ### `motion.ApproachMode`
**Kind:** `type`

How `motion.WalkTo` shapes the path toward the target.


  

**Source:** `input/game/motion.lua`

  ### `motion.StopMode`
**Kind:** `type`

How `motion.WalkTo` chooses the point at which the player starts stopping.


  

**Source:** `input/game/motion.lua`

---


## Ragebot

<!-- Source: cs2/api_reference/input/ragebot.md -->

---
title: Ragebot
icon: crosshair
sidebar_position: 4
---

# Ragebot

<Heading level={2} id="ragebot-1">
  ragebot

   _()_
</Heading>

Provides the ragebot shot information, the state of the anti-aim mouse override, and the state
of the quick peek assist.


  ### `ragebot.IsMouseOverrideActive`
**Kind:** `fn`

Returns true if the anti-aim mouse override has a yaw selected.

Holding the hotkey while the mouse is inside the deadzone does not select a yaw. A selected yaw
      can stay active after the hotkey is released.
    
  

**Source:** `input/game/ragebot.lua`

  ### `ragebot.GetQuickPeekState`
**Kind:** `fn`

Returns the state of the quick peek assist for the last processed command.

In the `pre_movement` event, this is the state of the previous command, because quick peek runs
      after the event returns. In the `post_movement` and `createmove` events, this is the state of
      the current command.

      Returns nil if the quick peek assist is not active or the local player is dead.
    
  

**Source:** `input/game/ragebot.lua`

  ### `ragebot.QuickPeekPhase`
**Kind:** `type`

The phase of the quick peek assist, returned in `ragebot.GetQuickPeekState`.


  

**Source:** `input/game/ragebot.lua`


## ragebot.QuickPeekState _()_

The state of the quick peek assist, returned by `ragebot.GetQuickPeekState`.


  ### `ragebot.QuickPeekState.interpolated_position`
**Kind:** `prop`

The retreat point, smoothed between commands. The built-in circle is drawn at this point. Use it when you draw.

**Source:** `input/game/ragebot.lua`

  ### `ragebot.QuickPeekState.phase`
**Kind:** `prop`

The current phase of the quick peek assist.

**Source:** `input/game/ragebot.lua`

  ### `ragebot.QuickPeekState.position`
**Kind:** `prop`

The retreat point in world space. It is updated once per command.

**Source:** `input/game/ragebot.lua`


## ragebot.Shot _()_

A shot record, passed to `events.aim_fire` and `events.aim_ack`. All fields are read-only.
Reading an unknown field throws an error.


  ### `ragebot.Shot.aim_point`
**Kind:** `prop`

The chosen point on the target, in world space.

**Source:** `input/game/ragebot.lua`

  ### `ragebot.Shot.backtrack`
**Kind:** `prop`

Absolute backtrack amount in ticks

**Source:** `input/game/ragebot.lua`

  ### `ragebot.Shot.command_number`
**Kind:** `prop`

The command number of the shot.

**Source:** `input/game/ragebot.lua`

  ### `ragebot.Shot.damage`
**Kind:** `prop`

The damage registered on the target. It is `-1` if the shot missed and `0` if a damage rejection was detected.

**Source:** `input/game/ragebot.lua`

  ### `ragebot.Shot.estimated_damage`
**Kind:** `prop`

The predicted damage at the chosen aim point.

**Source:** `input/game/ragebot.lua`

  ### `ragebot.Shot.force_shot`
**Kind:** `prop`

True if the shot was fired with force shot active at perfect accuracy.

**Source:** `input/game/ragebot.lua`

  ### `ragebot.Shot.health_left`
**Kind:** `prop`

The health of the target after the matching hurt event, or `-1` if it is not known.

**Source:** `input/game/ragebot.lua`

  ### `ragebot.Shot.hitchance`
**Kind:** `prop`

The estimated hit chance, from 0 to 1. Multiply it by 100 to get a percentage.

**Source:** `input/game/ragebot.lua`

  ### `ragebot.Shot.hitgroup`
**Kind:** `prop`

The hitgroup that the server reported, or `-1` if the shot missed.

**Source:** `input/game/ragebot.lua`

  ### `ragebot.Shot.manual_shot`
**Kind:** `prop`

True if the user forced the shot with a manual attack.

**Source:** `input/game/ragebot.lua`

  ### `ragebot.Shot.minimum_damage`
**Kind:** `prop`

The minimum damage required for this target and shot.

**Source:** `input/game/ragebot.lua`

  ### `ragebot.Shot.result`
**Kind:** `prop`

The result of the shot, or the reason for the miss.

**Source:** `input/game/ragebot.lua`

  ### `ragebot.Shot.server_shoot_position`
**Kind:** `prop`

The most likely shoot position that the server used for the shot. It can be inaccurate.

**Source:** `input/game/ragebot.lua`

  ### `ragebot.Shot.shoot_position`
**Kind:** `prop`

The predicted position in world space from which the shot was aimed.

**Source:** `input/game/ragebot.lua`

  ### `ragebot.Shot.target`
**Kind:** `prop`

The entity index of the player controller of the target.

**Source:** `input/game/ragebot.lua`

  ### `ragebot.Shot.tickcount`
**Kind:** `prop`

The tick count of the local player when the shot was fired.

**Source:** `input/game/ragebot.lua`

  ### `ragebot.Shot.wanted_hitgroup`
**Kind:** `prop`

The hitgroup of the chosen hitbox of the target.

**Source:** `input/game/ragebot.lua`

  ### `ragebot.Shot.weapon`
**Kind:** `prop`

A handle to the weapon. It is not a weapon entity and it is not an entity index.

**Source:** `input/game/ragebot.lua`

  ### `ragebot.Shot.Result`
**Kind:** `type`

The result of a shot, returned in `ragebot.Shot.result`. A shot that deals damage can still be counted as a miss if the damage is less than the minimum damage of the ragebot.

**Source:** `input/game/ragebot.lua`

---


## Editor Setup

<!-- Source: cs2/api_reference/introduction/editor-setup.md -->

---
title: Editor Setup
description: Get autocompletion, hover docs, and type checking for the CS2 API in VS Code, Zed, JetBrains IDEs, or any editor with Lua Language Server support.
icon: square-terminal
sidebar_position: 3
---

# Editor Setup

The CS2 API ships as type definitions for the
[Lua Language Server](https://luals.github.io/) (LuaLS). With them installed,
your editor completes API names, shows the same docs you read on this site when
you hover a name, and warns you about wrong arguments before you reload the
extension. Any editor with LuaLS support works, including VS Code, Zed, and
JetBrains IDEs.

## 1. Get the definitions

Clone the repository with git, or download it as a ZIP from GitLab and extract
it somewhere permanent:

```bash
git clone https://gitlab.com/neverlose/LuaLS-CS2.git
```

Remember where the folder ends up. The examples below use
`C:/Users/<you>/Downloads/LuaLS-CS2`.

## 2. Add `.luarc.json` to your extension

Local extensions live under:

```text
C:\Program Files (x86)\Steam\steamapps\common\Counter-Strike Global Offensive\game\bin\win64\neverlose\extensions\
```

In your extension's folder (for example `extensions\example`), create a file
named `.luarc.json`:

```json .luarc.json
{
  "$schema": "https://raw.githubusercontent.com/sumneko/vscode-lua/master/setting/schema.json",
  "runtime.version": "LuaJIT",
  "runtime.builtin": {
    "basic": "disable",
    "bit": "disable",
    "bit32": "disable",
    "builtin": "disable",
    "coroutine": "disable",
    "debug": "disable",
    "ffi": "disable",
    "io": "disable",
    "jit": "disable",
    "jit.profile": "disable",
    "jit.util": "disable",
    "math": "disable",
    "os": "disable",
    "package": "disable",
    "string": "disable",
    "string.buffer": "disable",
    "table": "disable",
    "table.clear": "disable",
    "table.new": "disable",
    "utf8": "disable"
  },
  "completion.maxSuggestCount": 500,
  "workspace.preloadFileSize": 2500,
  "workspace.maxPreload": 25000,
  "workspace.library": [
    "C:/Users/<you>/Downloads/LuaLS-CS2/library"
  ]
}
```

Set `workspace.library` to the `library` folder inside your clone or extracted
ZIP, not the repository root. Use forward slashes (`/`) or escaped backslashes
(`\\`); a single `\` is not valid JSON.

<Note>
  All `runtime.builtin` entries are disabled on purpose. The definitions
  include their own versions of the LuaJIT standard library as Neverlose
  exposes it, so the LuaLS built-ins would only add duplicate or wrong entries.
</Note>

## 3. Set up your editor

`.luarc.json` is read by the language server, not the editor, so the same file
works everywhere. LuaLS only reads it from the workspace root: open the
extension folder itself, not a parent folder.

<Tabs>
  <Tab title="VS Code" icon="code">
    1. Install the [Lua extension by sumneko](https://marketplace.visualstudio.com/items?itemName=sumneko.lua). It bundles LuaLS.
    2. Open the extension folder with **File → Open Folder…**.
  </Tab>

  <Tab title="Zed" icon="zap">
    1. Install the **Lua** extension from the Extensions panel. Zed downloads LuaLS for you. See [Zed's Lua docs](https://zed.dev/docs/languages/lua).
    2. Open the extension folder with **File → Open…**.
  </Tab>

  <Tab title="JetBrains" icon="blocks">
    1. Install the [SumnekoLua](https://plugins.jetbrains.com/plugin/22315-sumnekolua) plugin from **Settings → Plugins → Marketplace**.
    2. Open the extension folder as a project.
  </Tab>

  <Tab title="Other editors" icon="terminal">
    Any editor with a Language Server Protocol client can run
    `lua-language-server`. Install the server, start it for `.lua` files, and
    open the extension folder as the workspace root.
  </Tab>
</Tabs>

## 4. Check that it works

Wait for the language server to finish loading the definitions; the first load
takes a few seconds. Then type a global such as `render.` in `main.lua`, and
completions should appear.

<Card title="Getting Started" icon="rocket" href="/cs2/api_reference/introduction/getting-started" arrow>
  Create your first extension and write a script that draws to the screen.
</Card>

---


## Getting Started

<!-- Source: cs2/api_reference/introduction/getting-started.md -->

---
title: Getting Started
description: "From zero to a running extension: unlock Developer mode, create an extension, write your first script, and publish it."
icon: rocket
sidebar_position: 2
---

# Getting Started

This page takes you from zero to your first running extension. No prior
experience with Neverlose scripting is needed. Account, cheat menu, and market
steps are covered in detail in the Product tab; this page links to them where
you need them.

## 1. Unlock Developer mode

To develop local CS2 extensions, you need active Developer mode and a
Neverlose CS2 subscription. After buying Developer mode, restart the cheat.
See [Developer Mode](/cs2/product/developers/developer-mode) for pricing,
renewal, and access rules.

<Card title="Developer Mode" icon="lock-open" href="/cs2/product/developers/developer-mode" arrow>
  Annual access, renewal, and how it relates to your product subscription.
</Card>

## 2. Create an extension

<Steps>
  <Step title="Open Manage Extensions" icon="package">
    A **Developer** section is now at the top: these are your own extensions.
  </Step>

  <Step title="Create the extension" icon="folder-plus">
    Press **Create new developer extension** and give it a name. Scripts and
    libraries are created the same way.
  </Step>

  <Step title="Open its folder" icon="folder-open">
    Use **Open Folder** from the extension's **…** menu and open the folder in
    your editor. To get autocompletion and hover docs for the API there, see
    [Editor Setup](/cs2/api_reference/introduction/editor-setup).
  </Step>
</Steps>

<Frame caption="Your extensions under Developer, downloaded ones under Market">
  <Image src="/images/cs2/developers/manage-extensions.webp?v=674e6c0b" alt="Manage Extensions with the menu of a developer extension open" width="1927" height="1375" placeholder="data:image/webp;base64,UklGRoQAAABXRUJQVlA4WAoAAAAQAAAADwAACgAAQUxQSDkAAAABYBPbVqMnABeowAM1eKGmxAGSchSBAbbcjuAgIiaA220F6H87PhuUHHxVHiwnDqZhBxF4x+22AgAAVlA4ICQAAABQAQCdASoQAAsAAsBMJZwABAAAAP71wDZ7oOULMatHL6BIYAA=" />
</Frame>

## 3. Write your first script

Put this in `main.lua` and save. Enable the extension in **Manage Extensions**,
return to the game, and open the Neverlose menu. When the extension loads,
the text appears in the top-left corner:

```lua main.lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/introduction/getting-started:0:dad41b336484"
local font = render.LoadFont(render.EspFont.kBold)

events.render(function()
    render.DrawText(Vector2D(20, 20), Color(255, 255, 255), font, nil, "Hello from my extension")
end)
```

`main.lua` runs once each time the extension loads, and `events.render` calls
your function every frame. [Lifecycle and Events](/cs2/api_reference/extensions/lifecycle-and-events)
explains what runs when, and [Anatomy](/cs2/api_reference/extensions/anatomy)
covers the other files an extension can have.

After editing, save your changes, return to the game, and open the menu.
See [Manage Extensions](/cs2/product/developers/manage-extensions#check-changes-in-the-game)
for reload conditions.

## 4. Use libraries

To build on a market library, a built-in module such as `ffi`, or another of
your own extensions, add it to **Requirements** in the extension's settings and
load it with `require`:

```lua main.lua
local ffi = require("ffi")
```

<Card title="Requirements" icon="puzzle" href="/cs2/api_reference/extensions/requirements" arrow>
  Native, market, and local modules, and splitting your code across files.
</Card>

## 5. Publish on the market

When the extension is ready, create a market item, upload a build from
**Manage Extensions**, then use the **Update** action on the market item page
to submit it for review. Uploading alone does not submit the build.
Publishing and updating CS2 extensions requires active Developer mode,
including for free items. A library is published the same way, as an item
marked **Make it a library**.

<Columns cols={2}>
  <Card title="Publishing on the Market" icon="store" href="/cs2/product/developers/publishing" arrow>
    Create the item, upload builds, pass moderation, and set the price.
  </Card>

  <Card title="Manage Extensions" icon="package" href="/cs2/product/developers/manage-extensions" arrow>
    The extension menu, settings, requirements, and snapshots.
  </Card>
</Columns>

## Next steps

<CardGroup cols={2}>
  <Card title="Lua Crash Course" icon="graduation-cap" href="/cs2/api_reference/tutorials/lua-crash-course" arrow>
    The parts of Lua you need for extensions, from tables to closures.
  </Card>

  <Card title="Reading API Pages" icon="book-open" href="/cs2/api_reference/introduction/reading-these-docs" arrow>
    How to read signatures, overloads, and types on the reference pages.
  </Card>
</CardGroup>

---


## Lua Intelligence

<!-- Source: cs2/api_reference/introduction/lua-intelligence.md -->

---
title: Lua Intelligence
description: Learn how Lua examples in the docs show hover cards, signatures, diagnostics, and API links.
icon: sparkles
sidebar_position: 5
---

# Lua Intelligence

Lua Intelligence makes code examples on this site behave closer to a small
editor. When a Lua snippet is connected to the CS2 API index, important names in
the code become interactive: you can hover them, pin the card, copy the
signature, or jump to the exact API or Dumped Data page.

<Note>
  You do not need to enable anything. If a code example supports Lua
  Intelligence, the hover cards are already built into that page.
</Note>

## Try the hover

Move your cursor over names like `entity.GetLocalPlayerPawn`, `events.render`,
`pawn:IsValid`, `pawn:GetHealth`, or `m_iHealth` in the example below.

```lua luaintelligence=true luaintelligenceFocus=true lines=false semantic="lua:cs2:cs2/api_reference/introduction/lua-intelligence:0:5297e3dcdf90"
events.render(function()
    local pawn = entity.GetLocalPlayerPawn()

    if not pawn or not pawn:IsValid() or pawn:IsDormant() then
        return
    end

    print(string.format("health: %d", pawn:GetHealth()))
    print(pawn.m_iHealth)
end)
```

<Steps>
  <Step title="Hover a Lua name" icon="mouse-pointer-2">
    Hover a highlighted namespace, function, method, type, or schema field. The
    docs open a compact card with the information known about that symbol.
  </Step>

  <Step title="Click to pin the card" icon="pin">
    A hover card closes when you move away. Click the token to pin it, then move
    the mouse into the card, switch tabs, or use the buttons without losing it.
    Press `Esc` or click outside to close it.
  </Step>

  <Step title="Follow the source of truth" icon="external-link">
    Use **Open documentation** for Lua API pages and **Open schema** for Source2
    fields or enum data. Those links point to the same generated pages used by
    the rest of the docs.
  </Step>
</Steps>

## What can appear in a card

<Columns cols={2}>
  <Card title="Signature" icon="braces">
    Function and method cards show the call shape: argument names, argument
    types, and return types when the API exposes them.
  </Card>

  <Card title="Description" icon="message-circle-more">
    The short explanation comes from the addon documentation, so the hover card
    uses the same notes as the generated API page.
  </Card>

  <Card title="Parameters and returns" icon="list">
    Parameter notes and return values are shown directly in the card, which is
    useful when you only need a quick reminder.
  </Card>

  <Card title="Schema context" icon="network">
    Schema fields can show their owner, type, offset, nearby fields, enum
    members, and a link into Dumped Data.
  </Card>

  <Card title="Diagnostics" icon="circle-alert">
    If an example has a Lua problem, the token can be underlined. Hover it to
    see the message and code reported by LuaLS.
  </Card>

  <Card title="Actions" icon="copy">
    Cards can copy the symbol, copy the signature, copy a diagnostic, or open
    the related documentation page.
  </Card>
</Columns>

## Pinned card tabs

Pinned cards can show extra tabs when there is enough data:

<Tabs>
  <Tab title="Summary" icon="text">
    The default view. It includes the description, signature, type, returns,
    parameters, and useful metadata.
  </Tab>

  <Tab title="Members" icon="list-tree">
    Appears for Lua objects or tables that expose known members. Pinning lets
    you inspect more members than the compact hover preview.
  </Tab>

  <Tab title="Schema" icon="network">
    Appears when the hovered value maps to Source2 schema data. It can preview
    fields, enum members, parent types, and schema metadata.
  </Tab>

  <Tab title="Problems" icon="bug">
    Appears when the token has a diagnostic. This is where warnings and errors
    are easiest to read after pinning the card.
  </Tab>
</Tabs>

<Expandable title="Building pipeline" description="How API sources become generated pages and hover data." icon="workflow">
  This site uses a semi-automatic documentation pipeline. Some pages are written
  manually when they need human explanation, examples, or practical notes. Other
  pages are generated from the current CS2 cheat and addon data, so large
  reference sections can stay close to the real API.

  <Mermaid
    frame={false}
    chart={`%%{init: {"flowchart": {"htmlLabels": true, "curve": "basis", "nodeSpacing": 48, "rankSpacing": 58}}}%%
  flowchart TD
  Maintainer["<div class='nl-mermaid-card'><span class='nl-mermaid-card-title'>Maintainer</span><span class='nl-mermaid-card-desc'>API Docs, Events, Examples</span></div>"]
  Product["<div class='nl-mermaid-card'><span class='nl-mermaid-card-title'>CS2 Cheat</span><span class='nl-mermaid-card-desc'>Schema, ConVars, Game Events</span></div>"]
  Addon["<div class='nl-mermaid-card'><span class='nl-mermaid-card-title'>CS2 LuaCATS Sources</span><span class='nl-mermaid-card-desc'>API documentation and game data</span></div>"]
  LLS["<div class='nl-mermaid-card'><span class='nl-mermaid-card-title'>LuaLS</span><span class='nl-mermaid-card-desc'>Exports documentation</span></div>"]
  Generator["<div class='nl-mermaid-card'><span class='nl-mermaid-card-title'>Doc Generator</span><span class='nl-mermaid-card-desc'>Builds API, Pages and Link References</span></div>"]
  Pages["<div class='nl-mermaid-card'><span class='nl-mermaid-card-title'>Generated API Pages and Source2 Schema</span></div>"]
  LI["<div class='nl-mermaid-card'><span class='nl-mermaid-card-title'>Lua Intelligence</span><span class='nl-mermaid-card-desc'>Second pass over pages and snippets</span></div>"]
  Deploy["<div class='nl-mermaid-card'><span class='nl-mermaid-card-title'>Deploy</span><span class='nl-mermaid-card-desc'>Pages and Interactive Code</span></div>"]

  Maintainer --> Addon
  Product --> Addon
  Addon --> LLS
  LLS --> Generator
  Generator --> Pages
  Pages --> LI
  Addon -.-> LI
  LI --> Deploy

  classDef input fill:#0B0B0D,stroke:#82A5FF,stroke-width:1px,color:#F5F7FA
  classDef build fill:#0B0B0D,stroke:#67D4FF,stroke-width:1px,color:#F5F7FA
  classDef intelligence fill:#0B0B0D,stroke:#D4AA32,stroke-width:1px,color:#F5F7FA
  classDef deploy fill:#0B0B0D,stroke:#65A765,stroke-width:1px,color:#F5F7FA
  class Maintainer,Product,Addon input
  class LLS,Generator,Pages build
  class LI intelligence
  class Deploy deploy
  `}
  />

  <Steps>
    <Step title="API notes live with their source" icon="pencil">
      Maintainers write function overviews, parameter notes, examples, event
      descriptions, and practical explanations in LuaCATS files alongside the
      CS2 implementation and its shared modules. The docs build uses a synchronized
      copy of those sources.
    </Step>

    <Step title="Generated pages are built first" icon="files">
      LuaLS exports structured documentation, then the docs generator creates
      API pages, Source2 schema pages, and link references.
    </Step>

    <Step title="Lua Intelligence runs second" icon="sparkles">
      After pages exist, Lua Intelligence checks Lua examples against the same
      CS2 API index and attaches hover cards, diagnostics, signatures, and
      links to the rendered snippets.
    </Step>
  </Steps>
</Expandable>

---


## Overview

<!-- Source: cs2/api_reference/introduction/overview.md -->

---
title: Overview
description: Write Lua extensions for Counter-Strike 2 with the Neverlose scripting API.
icon: compass
sidebar_position: 1
---

# CS2 Scripting

Neverlose lets you extend Counter-Strike 2 with your own **Lua extensions**:
self-contained scripts that hook into the game loop, draw to the screen, read
the entity system, and add their own controls to the cheat menu. This section
has everything you need to write them, from the extension layout to the exact
name, arguments, and return value of every function you call from `main.lua`.

<Note>
  New here? The fastest path is [Getting Started](/cs2/api_reference/introduction/getting-started) → [Anatomy](/cs2/api_reference/extensions/anatomy) → [Lifecycle and Events](/cs2/api_reference/extensions/lifecycle-and-events) → [Lua Crash Course](/cs2/api_reference/tutorials/lua-crash-course). Then keep the reference pages open while you write.
</Note>

## Start here

<Card title="Getting Started" icon="rocket" href="/cs2/api_reference/introduction/getting-started" arrow>
  Unlock Developer mode, create your first extension, publish it on the market, and set its price.
</Card>

<CardGroup cols={2}>
  <Card title="Anatomy" icon="folder-tree" href="/cs2/api_reference/extensions/anatomy" arrow>
    The extension folder, manifest, `main.lua` entry point, and hot reload model.
  </Card>

  <Card title="Lifecycle and Events" icon="radio" href="/cs2/api_reference/extensions/lifecycle-and-events" arrow>
    How `main.lua`, callbacks, render-only drawing, delayed work, and shutdown fit together.
  </Card>

  <Card title="Requirements" icon="puzzle" href="/cs2/api_reference/extensions/requirements" arrow>
    Native, marketplace, and local modules, and splitting your code across files.
  </Card>
</CardGroup>

## What to open first

<CardGroup cols={2}>
  <Card title="events" icon="bolt" href="/cs2/api_reference/runtime/events" arrow>
    Run code on render, movement, unload, game messages, and custom library events.
  </Card>

  <Card title="render" icon="brush" href="/cs2/api_reference/rendering/render" arrow>
    Draw text, lines, rectangles, images, gradients, clips, and other frame-only UI.
  </Card>

  <Card title="ui" icon="layout-grid" href="/cs2/api_reference/rendering/ui" arrow>
    Add tabs, groups, controls, color pickers, callbacks, and references to menu items.
  </Card>

  <Card title="entity" icon="users" href="/cs2/api_reference/engine/entity" arrow>
    Read players, pawns, weapons, game rules, world entities, and schema-backed fields.
  </Card>

  <Card title="schema" icon="list-tree" href="/cs2/api_reference/engine/schema" arrow>
    Inspect Source 2 classes, fields, offsets, field types, and enum tables from Lua.
  </Card>

  <Card title="console" icon="terminal" href="/cs2/api_reference/engine/console" arrow>
    Read or change convars and call console commands through typed objects.
  </Card>
</CardGroup>

## Browse by category

The reference groups related APIs together:

* **Runtime**: events, delayed work and clocks in [time](/cs2/api_reference/runtime/time), utilities, cloud access, and hardware monitoring.
* **Engine**: entities, [engine state](/cs2/api_reference/engine/engine), schema reflection, [schema values](/cs2/api_reference/engine/value), console, traces, and memory helpers.
* **Input & Movement**: [input polling](/cs2/api_reference/input/input), commands and player simulation in [cmd](/cs2/api_reference/input/cmd), and motion helpers.
* **Rendering & UI**: drawing, menu controls, [indicators](/cs2/api_reference/rendering/indicators), effects, ESP, view overrides, and Panorama.
* **mathlib**: vectors, angles, colors, matrices, and geometry helpers.
* **Data & Storage**: database, filesystem, JSON, and MessagePack.
* **Networking**: HTTP, network-channel statistics, and protobuf messages.
* **Lua & LuaJIT**: standard Lua and LuaJIT libraries and their available extensions.

## Your entry point is main.lua

Neverlose runs your extension's `main.lua` when the extension is loaded or
reloaded. Do setup once at the top of the file, then attach callbacks for work
that must happen later.

```lua main.lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/introduction/overview:0:c8510bc6a5bb"
local font = render.LoadFont(render.EspFont.kRegular)

events.render(function()
    render.DrawText(
        Vector2D(20, 20),
        Color(255, 255, 255, 255),
        font,
        nil,
        "Hello from Neverlose"
    )
end)
```

This is the normal shape of a script:

<Steps>
  <Step title="Create values once" icon="box">
    Load fonts, find UI controls, define helper functions, and keep state in locals.
  </Step>

  <Step title="Subscribe to events" icon="radio">
    Put per-frame and per-tick work in callbacks such as `events.render` and `events.pre_movement`. Scripts can also send each other events under names of their own; see [events](/cs2/api_reference/runtime/events).
  </Step>

  <Step title="Read live data inside callbacks" icon="activity">
    Entities, convars, screen size, and game state can change every frame. Fetch or validate them close to where you use them.
  </Step>

  <Step title="Clean up when needed" icon="refresh-cw">
    When you change something outside your script, such as a convar, register the undo with `defer` right there. It runs when the script unloads or reloads. Use `events.shutdown` when you need to know why the script is stopping.
  </Step>
</Steps>

## Lua names you will see often

Create common value types with the same names shown in API return types:

```lua luaintelligence=true lines=false semantic="lua:cs2:cs2/api_reference/introduction/overview:1:6612e97f69ee"
local pos = Vector(0, 0, 64)
local point = Vector2D(20, 20)
local view = QAngle(0, 90, 0)
local white = Color(255, 255, 255, 255)
```

Lua names are case-sensitive. Use the documented spelling, including
`Vector`, `Vector2D`, `QAngle`, and `Color` for constructors and PascalCase
functions such as `entity.GetLocalPawn()` and `render.DrawText()`. Event names
and data fields keep their documented spelling, such as `events.render` and
`console.cl_showfps.value`.

## API pages vs Dumped Data

Use **API Reference** for callable Lua names: functions, methods, callbacks,
classes, modules, and value types.

Use [Dumped Data](/cs2/dumped_data/overview/overview) when you need Source 2
names: schema classes, raw field names, offsets, enum values, convars, or
commands.

```lua luaintelligence=true lines=false semantic="lua:cs2:cs2/api_reference/introduction/overview:2:f6f09c63abce"
local pawn = entity.GetLocalPlayerPawn()

if pawn and pawn:IsValid() then
    print(pawn:GetHealth())  -- API helper
    print(pawn.m_iHealth) -- raw Schema field
end
```

<Tip>
  Prefer helper methods when they exist. Raw schema fields are useful for data the API does not wrap directly.
</Tip>

## Learn the language

New to Lua, or want to go deeper with FFI and LuaJIT? These guides take you from
the basics to performance-critical native interop.

<CardGroup cols={3}>
  <Card title="Lua Crash Course" icon="graduation-cap" href="/cs2/api_reference/tutorials/lua-crash-course" arrow>
    Variables, control flow, functions, tables, metatables, and modules in one page.
  </Card>

  <Card title="FFI Crash Course" icon="binary" href="/cs2/api_reference/tutorials/ffi-crash-course" arrow>
    Call into C from Lua to reach engine internals the high-level API doesn't expose.
  </Card>

  <Card title="Advanced LuaJIT" icon="zap" href="/cs2/api_reference/tutorials/advanced-luajit" arrow>
    Squeeze performance out of hot paths with LuaJIT-specific techniques.
  </Card>
</CardGroup>

<Tip>
  All your callbacks share a single Lua state. LuaJIT runs your script on one thread, not in parallel, so a slow callback stalls the frame. Keep per-frame work light.
</Tip>

## Next pages

<CardGroup cols={2}>
  <Card title="Reading API Pages" icon="book-open" href="/cs2/api_reference/introduction/reading-these-docs" arrow>
    How to read signatures, tabs, examples, optional arguments, methods, and type links.
  </Card>

  <Card title="Globals and Modules" icon="package" href="/cs2/api_reference/extensions/globals-and-modules" arrow>
    What is available directly, what must be loaded with `require`, and how modules differ from globals.
  </Card>

  <Card title="Dumped Data Overview" icon="database" href="/cs2/dumped_data/overview/overview" arrow>
    How Schema and Console pages turn into practical entity and convar code.
  </Card>

  <Card title="Lua Intelligence" icon="sparkles" href="/cs2/api_reference/introduction/lua-intelligence" arrow>
    Hover cards, signatures, diagnostics, and API links in the code examples on these pages.
  </Card>
</CardGroup>

---


## Reading These Docs

<!-- Source: cs2/api_reference/introduction/reading-these-docs.md -->

---
title: Reading API Pages
icon: book-open
sidebar_position: 4
---

# Reading API Pages

API Reference pages are written for the script you are about to write. Each
page lists its symbols (a global table, an object type, an enum) as sections, and
every function, method, field or operator of a symbol as a card. Scan the card
heads for the name you need, then open a card for its signature, arguments,
return values and examples.

<Note>
  This page describes the generated pages as they appear in the browser. Treat
  the signature block as a readable call guide, not as Lua code that should be
  copied into your script.
</Note>

## Member cards

A card's head shows a kind badge, the member name with its argument names, and
the first sentence of its description. Click the head to open the card; only one
card per group is open at a time. The link icon copies a link to that card, and
the table of contents lists every card on the page.

| Badge                          | What it is                 | How you use it                 |
| ------------------------------ | -------------------------- | ------------------------------ |
|      | function of a global table | `entity.GetLocalPawn()`        |
|  | method of an object        | `pawn:IsValid()`               |
|    | field or property          | `globals.curtime`              |
|    | operator or metamethod     | `a + b`, `#v`                  |
|    | calling the type itself    | `Vector(1, 2, 3)`              |
|    | type alias                 | `entity.PawnObserveePredicate` |
|   | event callback             | `events.render(fn)`            |

An open card shows, in order: the signature line, longer notes, **Applies to**
(the object classes a method accepts), **Parameters**, **Returns** (when results
are named or described), examples, and a footer with tags such as `nodiscard`
and an **Edit** link to the source file.

## Signature line

The signature line inside a card has the callable name, argument names and
types, optional badges, and returned values after the return arrow.

Use that line to answer practical questions:

* what name you call;
* whether you call it with dot syntax (`entity.ListByClassName`) or colon syntax (`pawn:IsValid`);
* which arguments exist and which ones are optional;
* whether the return is a normal value, a nullable value, or an iterator triple;
* which type page you can click next.

For example, [`entity.IteratePlayers`](/cs2/api_reference/engine/entity#entity-IteratePlayers)
shows optional filters and returns values shaped for a Lua generic `for` loop.
That is why the usual script reads controller/pawn pairs like this:

```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/introduction/reading-these-docs:0:7d42e33eea51"
for controller, pawn in entity.IteratePlayers(true, false) do
    print(controller:GetPlayerName(), ": ", pawn:GetHealth(), " hp")
end
```

## Section labels

The quiet label next to a section heading tells you what that symbol is. A
`global table` groups functions; an `object type` describes values the API
returns; a `ctype` is a C type object you can call as a constructor; a `cdata`
section describes the C values created by or returned from the API. `callable`
means the symbol itself can be called. The chips under the description count
the section's members and jump to their group.
For example, `projection.Rect` is a ctype and `projection.Rect()` returns a
`ProjectionRect` cdata value.

See the FFI crash course for an
[explanation of ctype and cdata](/cs2/api_reference/tutorials/ffi-crash-course#ctype-and-cdata).
The labels do not tell you whether storage is owned or borrowed; check the API's lifetime notes.

## Signature tokens

The renderer uses short tokens so the line stays readable even for overloaded
or generic functions:

<Columns cols={2}>
  <Card title="OPT" icon="badge">
    The argument is optional. You may omit it, or pass `nil` when you need to skip to a later optional slot.
  </Card>

  <Card title="Return arrow" icon="arrow-right">
    Values after the arrow are returned by the call. If several values are listed, Lua receives all of them.
  </Card>

  <Card title="or" icon="git-branch">
    A union. `boolean or number or nil` means any one of those values may be returned.
  </Card>

  <Card title="nil" icon="circle-slash">
    A nullable result. Always check it before calling methods on the value.
  </Card>

  <Card title="Clickable types" icon="mouse-pointer-click">
    Colored type names link to API type pages or Dumped Data schema pages when a target exists.
  </Card>

  <Card title="Quoted values" icon="quote">
    A quoted chip, such as `"#"`, is a literal value accepted by that argument.
  </Card>

  <Card title="T and V" icon="braces">
    Generic placeholders. The concrete type depends on the value you passed in or the container you are reading.
  </Card>

  <Card title="T[]" icon="list">
    An array-like Lua table of `T` values. A schema collection's `:Value()` returns a one-based table snapshot.
  </Card>

  <Card title="..." icon="ellipsis">
    Varargs. The call accepts additional values after the listed arguments.
  </Card>

  <Card title="ffi.ct*" icon="binary">
    A C type specification: a declaration string, a ctype object, or a cdata value used as a type template.
  </Card>
</Columns>

```lua luaintelligence=true lines=false semantic="lua:cs2:cs2/api_reference/introduction/reading-these-docs:1:c5a4260678be"
local bomb = entity.FirstByClassName("C_PlantedC4")
if bomb then
    print(bomb:GetAbsOrigin())
end
```

Nullable results are common for lookup-style functions. The docs show that as
`or nil`, and your script should branch before using the returned object.

## Multiple returns

Some signatures return more than one value. The values are shown comma by
comma after the return arrow.

[`time.GetLocalTime`](/cs2/api_reference/runtime/time#time-GetLocalTime)
returns four values: hour, minute, second, and millisecond.

```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/introduction/reading-these-docs:2:d49349ff11e5"
local hour, minute, second, millisecond = time.GetLocalTime()
print(string.format("%02d:%02d:%02d.%03d", hour, minute, second, millisecond))
```

Iterator-style functions also return several values, but you usually do not
store them manually. Lua consumes the iterator, state, and first control value
for you in a generic `for` loop. That is what signatures like `ipairs(...)` and
`entity.IteratePlayers(...)` are showing.

```lua luaintelligence=true lines=false semantic="lua:cs2:cs2/api_reference/introduction/reading-these-docs:3:0b9954dcea4a"
for index, value in ipairs({ "ak47", "awp" }) do
    print(index, ": ", value)
end
```

When the first returned value is shown as a function-shaped chip, it means the
returned value itself is callable. This usually appears on iterator helpers,
where Lua calls the iterator function internally on each loop step.

## Generic arrays and schema vectors

`T[]` means "an array-like table of the same kind of value represented by `T`."
You will see it on generated schema helpers such as `value.AtomicCollection:Value()`.

```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/introduction/reading-these-docs:4:7e0e523047e7"
local pawn = entity.GetLocalPlayerPawn()

if pawn and pawn:IsValid() then
    local weapons = pawn:GetMyWeapons()

    if weapons then
        for _, weapon in ipairs(weapons:Value()) do
            if weapon and weapon:IsValid() then
                print(weapon:GetClassName())
            end
        end
    end
end
```

Direct collection indices start at zero. `:Value()` copies the collection
membership into a one-based Lua table; the entities in it remain live objects.
Validate them again if you keep the table for later.

## Overloads

When one symbol has multiple call shapes, each shape is its own card with the
same name and a different argument list. For example,
[`render.LoadFont`](/cs2/api_reference/rendering/render#render-LoadFont) accepts
a built-in font preset or a font family/path with a size and optional settings.

```lua built-in font luaintelligence=true semantic="lua:cs2:cs2/api_reference/introduction/reading-these-docs:5:053a11a38d8f"
local font = render.LoadFont(render.EspFont.kRegular)
```

```lua family and size luaintelligence=true semantic="lua:cs2:cs2/api_reference/introduction/reading-these-docs:6:396ca5e1bfde"
local font = render.LoadFont("Arial", 16, render.FontEffect.kOutline)
```

<Tip>
  If two cards share a name, do not assume the first one is the only valid call. Compare their arguments before copying an example.
</Tip>

## Inheritance

Some object type sections show an **inherits** row with one or more class pills
under the heading.
That means the value has the documented behavior of those parent types too. For
example, `value.AtomicCollection` inherits `value.View`, which supplies `:Cast`
and inherits `:GetInstance` and `:GetSchemaType` from `value.Object`.
`console.Variable` inherits `console.Object` fields such as `name`, `flags`,
and `is_valid`.

```lua luaintelligence=true lines=false semantic="lua:cs2:cs2/api_reference/introduction/reading-these-docs:7:3e0a395cd612"
local var = console.sv_cheats
if var.is_valid then
    print(var.name, " = ", var.value, " (flags ", var.flags, ")")
end
```

## Events pages

There are two event-related areas:

* [`events`](/cs2/api_reference/runtime/events) documents the event object API:
  `:Set`, `:Unset`, and `:Fire`;
* generated event pages such as [Cheat Events](/cs2/api_reference/events/cheat)
  document concrete callback names such as `render`, `pre_movement`, and
  `shutdown`.

Each event is a card whose signature shows the callback shape. Below it, the
argument's fields or values are listed the way you use them: `e.attacker` for a
payload object, `reason == "unload"` for a value the callback receives. Function
arguments with a fixed set of values list them the same way, as `name = "Text"`.
The `render` event takes a
callback with no arguments; `pre_movement` passes `cmd` (`UserCmd`) and
`player` (`simulation.Player`).

```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/introduction/reading-these-docs:8:8d748de81a51"
events.render(function()
    render.DrawLine(Vector2D(50, 50), Vector2D(100, 100), Color(255, 255, 255))
end)
```

```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/introduction/reading-these-docs:9:b9e5e2eb503b"
events.pre_movement(function(cmd, player)
    if cmd.is_first_pass and cmd.in_jump.pressed then
        print("Jump at tick ", player.tickcount)
    end
end)
```

## Dot, colon, and field access

The displayed name tells you which Lua syntax to use:

<Columns cols={3}>
  <Card title="Dot" icon="dot" href="/cs2/api_reference/engine/entity#entity-GetLocalPawn">
    `entity.GetLocalPawn()` is a namespace function.
  </Card>

  <Card title="Colon" icon="corner-down-right" href="/cs2/api_reference/engine/entity#Entity-IsValid">
    `pawn:IsValid()` is a method on a value.
  </Card>

  <Card title="Field" icon="list" href="/cs2/api_reference/engine/console#consolevariable">
    `console.sv_cheats.value` is a documented object field.
  </Card>
</Columns>

Entity objects also allow raw schema field access. Those field names come from
[Dumped Data](/cs2/dumped_data/overview/overview), not from the API method list.

```lua luaintelligence=true lines=false semantic="lua:cs2:cs2/api_reference/introduction/reading-these-docs:10:a0d17bab541f"
local pawn = entity.GetLocalPlayerPawn()
if pawn and pawn:IsValid() then
    print(pawn:GetHealth())
    print(pawn.m_iHealth)
end
```

---


## Base

<!-- Source: cs2/api_reference/lua/base.md -->

---
title: Environment
icon: globe
sidebar_position: 1
---

# Environment

## \_G _()_

A global variable (not a function) that holds the global environment (that is, `_G._G = _G`).
Lua itself does not use this variable; changing its value does not affect any environment, nor vice-versa.

## Functions _()_


  ### `assert`
**Kind:** `fn`

**Arguments:** `v?, message?, ...?`

Issues an error when the value of its argument `v` is false (i.e., **nil** or **false**); otherwise, returns all its arguments. `message` is an error message; when absent, it defaults to "assertion failed!"

**Source:** `lua/luajit/base.lua`

  ### `collectgarbage`
**Kind:** `fn`

**Arguments:** `opt?, arg?`

This function is a generic interface to the garbage collector. It performs different functions according to its first argument, opt: - **"collect":** performs a full garbage-collection cycle. This is the default option. - **"stop":** This option is ignored, and the collector keeps running. All scripts share one garbage collector, so a script cannot stop it. - **"restart":** restarts the garbage collector. - **"count":** returns the total memory in use by Lua (in Kbytes). - **"step":** performs a garbage-collection step. The step "size" is controlled by arg (larger values mean more steps) in a non-specified way. If you want to control the step size you must experimentally tune the value of arg. Returns true if the step finished a collection cycle. - **"setpause":** This option does not change the pause of the collector. It returns the current pause value. - **"setstepmul":** This option does not change the step multiplier of the collector. It returns the current step multiplier.

The first time a script uses `"stop"`, `"setpause"` or `"setstepmul"`, a warning is added to the
      Errors list of the script and written to the game console.
    
  

**Source:** `lua/luajit/base.lua`

  ### `error`
**Kind:** `fn`

**Arguments:** `message, level?`

Terminates the last protected function called and returns `message` as the error message. Function `error` never returns.

Usually, `error` adds some information about the error position at the beginning of the message.
      The `level` argument specifies how to get the error position.
      With level 1 (the default), the error position is where the `error` function was called.
      Level 2 points the error to where the function that called `error` was called; and so on.
      Passing a level 0 avoids the addition of error position information to the message.
    
  

**Source:** `lua/luajit/base.lua`

  ### `getmetatable`
**Kind:** `fn`

**Arguments:** `object`

If `object` does not have a metatable, returns **nil**. Otherwise, if the object's metatable has a `"__metatable"` field, returns the associated value. Otherwise, returns the metatable of the given object.

**Source:** `lua/luajit/base.lua`

  ### `ipairs`
**Kind:** `fn`

**Arguments:** `t`

Returns three values: an iterator function, the table `t`, and 0, so that the construction

for i,v in ipairs(t) do --\[\[body]] end

      will iterate over the pairs (`1,t[1]`), (`2,t[2]`), ···, up to the first integer key absent from the table.
    
  

**Source:** `lua/luajit/base.lua`

  ### `next`
**Kind:** `fn`

**Arguments:** `table, index?`

Allows a program to traverse all fields of a table. Its first argument is a table and its second argument is an index in this table. `next` returns the next index of the table and its associated value. When called with **nil** as its second argument, `next` returns an initial index and its associated value. When called with the last index, or with **nil** in an empty table, `next` returns **nil**. If the second argument is absent, then it is interpreted as **nil**. In particular, you can use `next(t)` to check whether a table is empty.

The order in which the indices are enumerated is not specified, *even for numeric indices*.
      (To traverse a table in numeric order, use a numerical **for** or the `ipairs` function.)

      The behavior of `next` is *undefined* if, during the traversal, you assign any value to a non-existent field in the table.
      You may however modify existing fields. In particular, you may clear existing fields.
    
  

**Source:** `lua/luajit/base.lua`

  ### `pairs`
**Kind:** `fn`

**Arguments:** `t`

Returns three values: the `next` function, the table `t`, and **nil**, so that the construction

for k,v in pairs(t) do --\[\[body]] end

      will iterate over all key–value pairs of table `t`.

      See function `next` for the caveats of modifying the table during its traversal.
    
  

**Source:** `lua/luajit/base.lua`

  ### `pcall`
**Kind:** `fn`

**Arguments:** `f, arg1?, ...?`

Calls function `f` with the given arguments in _protected mode_. This means that any error inside `f` is not propagated; instead, `pcall` catches the error and returns a status code. Its first result is the status code (a boolean), which is true if the call succeeds without errors. In such case, `pcall` also returns all results from the call, after this first result. In case of any error, `pcall` returns **false** plus the error message.

**Source:** `lua/luajit/base.lua`

  ### `print`
**Kind:** `fn`

**Arguments:** `...?`

The same function as `log.Print`. Converts its arguments to strings, joins them without a separator, and writes them as one line to the game console with the `neverlose ·` prefix. When the Lua Output option of Log Events is enabled in the menu, the line is also added to the event log on the screen. `print` is not intended for formatted output, but only as a quick way to show a value, typically for debugging. For formatted output, use `string.format`.

**Source:** `lua/luajit/base.lua`

  ### `rawequal`
**Kind:** `fn`

**Arguments:** `v1, v2`

Checks whether `v1` is equal to `v2`, without invoking any metamethod. Returns a boolean.

**Source:** `lua/luajit/base.lua`

  ### `rawget`
**Kind:** `fn`

**Arguments:** `table, index`

Gets the real value of `table[index]`, without invoking any metamethod. `table` must be a table; `index` may be any value.

**Source:** `lua/luajit/base.lua`

  ### `rawlen`
**Kind:** `fn`

**Arguments:** `v`

Returns the length of the object `v`, which must be a table or a string, without invoking the `__len` metamethod. Returns an integer.

**Source:** `lua/luajit/base.lua`

  ### `rawset`
**Kind:** `fn`

**Arguments:** `table, index, value`

Sets the real value of `table[index]` to `value`, without invoking any metamethod. `table` must be a table, `index` any value different from **nil**, and `value` any Lua value.

This function returns `table`.
    
  

**Source:** `lua/luajit/base.lua`

  ### `require`
**Kind:** `fn`

**Arguments:** `modname`

Returns the module with the name `modname`. The name can be one of the following.

The name of a module from the `requirements` section of `extension.json`, such as `ffi`,
      `neverlose/name` or `local/name`. These modules are loaded before the script runs, and
      `require` returns the loaded module. If the module is not listed in the requirements,
      `require` throws an error.

      The name of a standard library, which is `math`, `bit`, `string`, `jit`, `table`, `coroutine`,
      `json` or `msgpack`. `require` returns the copy of the library that the script uses.

      The path of a Lua file of the script, without the `.lua` extension. The file runs the first
      time it is required. Later calls return the same result without running the file again.

      `table.new` or `table.clear`.

      If no module has this name, or if the module throws an error while it loads, `require` throws
      an error.
    
  

**Source:** `lua/luajit/base.lua`

  ### `select`
**Kind:** `fn`

**Arguments:** `index, ...?`

If `index` is a number, returns all arguments after argument number `index`. Otherwise, `index` must be the string `"#"`, and `select` returns the total number of extra arguments it received.

**Source:** `lua/luajit/base.lua`

  ### `setmetatable`
**Kind:** `fn`

**Arguments:** `table, metatable?`

Sets the metatable for the given table. (You cannot change the metatable of other types from Lua, only from C.) If `metatable` is **nil**, removes the metatable of the given table. If the original metatable has a `"__metatable"` field, raises an error.

This function returns `table`.
    
  

**Source:** `lua/luajit/base.lua`

  ### `tonumber`
**Kind:** `fn`

**Arguments:** `e`

Tries to convert its argument to a number. If the argument is already a number or a string convertible to a number, then `tonumber` returns this number; otherwise, it returns **nil**.

An optional argument specifies the base to interpret the numeral.
      The base may be any integer between 2 and 36, inclusive.
      In bases above 10, the letter '`A`' (in either upper or lower case) represents 10, '`B`' represents 11, and so forth, with '`Z`' representing 35.
      In base 10 (the default), the number can have a decimal part, as well as an optional exponent part.
      In other bases, only unsigned integers are accepted.
    
  

**Source:** `lua/luajit/base.lua`

  ### `tostring`
**Kind:** `fn`

**Arguments:** `v`

Receives an argument of any type and converts it to a string in a reasonable format. For complete control of how numbers are converted, use `string.format`.

If the metatable of `e` has a `"__tostring"` field, then `tostring` calls the corresponding value with `e` as argument, and uses the result of the call as its result.
    
  

**Source:** `lua/luajit/base.lua`

  ### `xpcall`
**Kind:** `fn`

**Arguments:** `f, msgh, arg1?, ...?`

This function is similar to `pcall`, except that you can set a new error handler.

`xpcall` calls function `f` in protected mode, using `err` as the error handler.
      Any error inside `f` is not propagated; instead, `xpcall` catches the error, calls the `err` function with the original error object, and returns a status code.
      Its first result is the status code (a boolean), which is true if the call succeeds without errors.
      In this case, `xpcall` also returns all results from the call, after this first result.
      In case of any error, `xpcall` returns **false** plus the result from `err`.
    
  

**Source:** `lua/luajit/base.lua`

  ### `unpack`
**Kind:** `fn`

**Arguments:** `list, i?, j?`

Returns the elements from the given table. This function is equivalent to

return list\[i], list\[i+1], ···, list\[j]

      except that the above code can be written only for a fixed number of elements.
      By default, `i` is 1 and `j` is the length of the list, as defined by the length operator.
    
  

**Source:** `lua/luajit/base.lua`

  ### `type`
**Kind:** `fn`

**Arguments:** `v`

Returns the type of its only argument, coded as a string. The possible results of this function are "`nil`" (a string, not the value **nil**), "`number`", "`string`", "`boolean`", "`table`", "`function`", "`thread`", "`userdata`", and "`cdata`".

There are nine basic types in LuaJIT: *nil*, *boolean*, *number*, *string*, *function*, *userdata*, *thread*, *table*, and *cdata*.
    
  

**Source:** `lua/luajit/base.lua`

  ### `inext`
**Kind:** `fn`

**Arguments:** `table, index?`

The iterator function that `ipairs` returns. It walks the array part of a table, in the same way as `next` walks all keys.

Given a table and an index `i`, it returns `i + 1` and the value at index `i + 1`. When that
      value is **nil**, it returns nothing and the loop ends. `for i, v in inext, t, 0 do end` works
      in the same way as `for i, v in ipairs(t) do end`.
    
  

**Source:** `lua/framework/base.lua`

  ### `defer`
**Kind:** `fn`

**Arguments:** `callback, ...?`

Registers a cleanup function for the current script. The arguments are passed to the function exactly as given, including **nil** values at the end.

The cleanup functions run when the script is unloaded, reloaded, or fails to start. They run
      after the `shutdown` callbacks and before the resources of the script are released. The last
      function registered runs first. If one function throws an error, the error is logged and the
      other functions still run.

      If a module fails to load, the cleanup functions that it registered run at once. Functions
      that are registered while the script is already cleaning up are ignored.
    
  

**Source:** `lua/framework/base.lua`

  ### `traceback`
**Kind:** `fn`

**Arguments:** `message?`

Builds a stack traceback for the place where it is called, with `message` in front of it. Use it as the message handler of `xpcall`. The handler runs inside the failing call, so it can still see the stack of that call.

Each frame names its file in the form `My Script/main.lua:12`. A file that comes from a
      dependency is named after the dependency, for example `local/bar/util.lua:5`.

      Deep stacks are shortened. The first 12 frames and the last 10 frames are kept, and a line
      such as `... 30 more frames` replaces the frames between them.

      If `message` already has a traceback in it, it is returned unchanged. If `message` is not a
      string, for example a table or **nil**, it is returned unchanged. If you call `traceback` with
      no arguments, the result is only the traceback.

      Only developer scripts have this function. Market scripts do not have it.
    
  

**Source:** `lua/framework/base.lua`

  ### `perf`
**Kind:** `fn`

Returns how much time and memory the code of this script used in the last measuring window. These are the numbers that the scripts list shows. A window is about one second long.

The values are totals for the whole window. They are not values per second. Divide a total
      by `window_ms` to get a rate. The values change only when a new window ends. Use `window` to
      tell one window from the next.

      `freed_kb` is the memory that was released while the code of this script ran. Most of it is
      released by the garbage collector, and it can include garbage of other scripts. When
      `freed_kb` is high, part of `self_ms` is time that the garbage collector used.

      Returns `nil` until the first window has ended. Only developer scripts have this function,
      in the same way as `traceback`.
    
  

**Source:** `lua/framework/base.lua`

  ### `printf`
**Kind:** `fn`

**Arguments:** `text, ...?`

Formats a message with `string.format` and passes the result to `print`.

**Source:** `lua/game/base.lua`

  ### `toticks`
**Kind:** `fn`

**Arguments:** `time`

Converts a time in seconds to a number of ticks. There are 64 ticks in one second.

The result is rounded in the same way as the engine's `TIME_TO_TICKS` macro. Half a tick is
      added to the value, and then the fractional part is removed, rounding toward zero.
    
  

**Source:** `lua/game/base.lua`

  ### `totime`
**Kind:** `fn`

**Arguments:** `ticks`

Converts a number of ticks to a time in seconds. There are 64 ticks in one second.

**Source:** `lua/game/base.lua`


## nil _()_

*Nil* is the type of the value **nil**, whose main property is to be different from any other value; it usually represents the absence of a useful value.

## boolean _()_

*Boolean* is the type of the values **false** and **true**.
Both **nil** and **false** make a condition false; any other value makes it true.

## number _()_

*Number* represents real (double-precision floating-point) numbers.

## integer _()_



*Integer* isn't an actual Lua type, but it can either mean an integer Lua `number` or LuaJIT `cdata` 64-bit integer, i.e. `cdata<int64_t>`/`cdata<uint64_t>`.

## Types _()_


  ### `int64_t`
**Kind:** `type`

` rather than a plain `number`, so full 64-bit precision is preserved. Can be declared using the `LL` suffix, which stands for `long long`." ret="[[{"s":"integer","k":"prim","h":"/cs2/api_reference/lua/base#integer"}]]" source="lua/luajit/base.lua" sourceHref="https://gitlab.com/neverlose/LuaLS-CS2/-/edit/master/library/api/lua/luajit/base.lua#L319" />

  ### `uint64_t`
**Kind:** `type`

` rather than a plain `number`, so full 64-bit precision is preserved. Can be declared using the `ULL` suffix, which stands for `unsigned long long`." ret="[[{"s":"integer","k":"prim","h":"/cs2/api_reference/lua/base#integer"}]]" source="lua/luajit/base.lua" sourceHref="https://gitlab.com/neverlose/LuaLS-CS2/-/edit/master/library/api/lua/luajit/base.lua#L325" />


## string _()_

*String* represents arrays of characters.
Lua is 8-bit clean: strings can contain any 8-bit character, including embedded zeros ('`\0`')

## userdata _()_

The type *userdata* is provided to allow arbitrary C data to be stored in Lua variables.
This type corresponds to a block of raw memory and has no pre-defined operations in Lua, except assignment and identity test.
However, by using *metatables*, the programmer can define operations for userdata values.
Userdata values cannot be created or modified in Lua, only through the C API.
This guarantees the integrity of data owned by the host program.

Use `ffi.cast` to convert a userdata value to a pointer type. Do this to work with an
address returned by a pattern search or an interface lookup.

## function _()_

A callable Lua function or C function exposed to Lua.

## metatable _()_

Metatable fields that customize operations on Lua values.

`__gc` runs for userdata and cdata only, never for tables.


  ### `metatable.__add`
**Kind:** `prop`

Handles addition.

**Source:** `lua/luajit/base.lua`

  ### `metatable.__call`
**Kind:** `prop`

Handles calls on non-function values.

**Source:** `lua/luajit/base.lua`

  ### `metatable.__concat`
**Kind:** `prop`

Handles concatenation.

**Source:** `lua/luajit/base.lua`

  ### `metatable.__div`
**Kind:** `prop`

Handles division.

**Source:** `lua/luajit/base.lua`

  ### `metatable.__eq`
**Kind:** `prop`

Handles equality comparison.

**Source:** `lua/luajit/base.lua`

  ### `metatable.__gc`
**Kind:** `prop`

Finalizer called when userdata is collected.

**Source:** `lua/luajit/base.lua`

  ### `metatable.__index`
**Kind:** `prop`

Handles reads for missing keys.

**Source:** `lua/luajit/base.lua`

  ### `metatable.__le`
**Kind:** `prop`

Handles less-than-or-equal comparison.

**Source:** `lua/luajit/base.lua`

  ### `metatable.__len`
**Kind:** `prop`

Handles the length operator.

**Source:** `lua/luajit/base.lua`

  ### `metatable.__lt`
**Kind:** `prop`

Handles less-than comparison.

**Source:** `lua/luajit/base.lua`

  ### `metatable.__metatable`
**Kind:** `prop`

Value returned by `getmetatable` and used to protect the metatable.

**Source:** `lua/luajit/base.lua`

  ### `metatable.__mod`
**Kind:** `prop`

Handles modulo.

**Source:** `lua/luajit/base.lua`

  ### `metatable.__mode`
**Kind:** `prop`

Weak table mode for values, keys, or both.

**Source:** `lua/luajit/base.lua`

  ### `metatable.__mul`
**Kind:** `prop`

Handles multiplication.

**Source:** `lua/luajit/base.lua`

  ### `metatable.__newindex`
**Kind:** `prop`

Handles writes to missing keys.

**Source:** `lua/luajit/base.lua`

  ### `metatable.__pow`
**Kind:** `prop`

Handles exponentiation.

**Source:** `lua/luajit/base.lua`

  ### `metatable.__sub`
**Kind:** `prop`

Handles subtraction.

**Source:** `lua/luajit/base.lua`

  ### `metatable.__tostring`
**Kind:** `prop`

Called by `tostring` to produce a string representation.

**Source:** `lua/luajit/base.lua`

  ### `metatable.__unm`
**Kind:** `prop`

Handles unary minus.

**Source:** `lua/luajit/base.lua`


## ScriptPerformance _()_

The table that `perf` returns.


  ### `ScriptPerformance.alloc_kb`
**Kind:** `prop`

The memory that the code of this script allocated during the window, in KB. Memory that was freed is not subtracted.

**Source:** `lua/framework/base.lua`

  ### `ScriptPerformance.calls`
**Kind:** `prop`

The number of callbacks that ran during the window.

**Source:** `lua/framework/base.lua`

  ### `ScriptPerformance.freed_kb`
**Kind:** `prop`

The memory that was freed while the code of this script ran, in KB. Most of it is freed by the garbage collector.

**Source:** `lua/framework/base.lua`

  ### `ScriptPerformance.load_ms`
**Kind:** `prop`

The time that `main.lua` took the last time it ran, in milliseconds. It is not part of the window.

**Source:** `lua/framework/base.lua`

  ### `ScriptPerformance.long_calls`
**Kind:** `prop`

The number of calls whose own code took 50 ms or more. The player sees each of these calls as a frozen frame.

**Source:** `lua/framework/base.lua`

  ### `ScriptPerformance.max_ms`
**Kind:** `prop`

The time of the longest single call, in milliseconds. Only the code of this script is counted.

**Source:** `lua/framework/base.lua`

  ### `ScriptPerformance.self_ms`
**Kind:** `prop`

The time spent in the code of this script during the window, in milliseconds.

**Source:** `lua/framework/base.lua`

  ### `ScriptPerformance.share`
**Kind:** `prop`

`self_ms` divided by the real length of the window. The value is from 0 to 1.

**Source:** `lua/framework/base.lua`

  ### `ScriptPerformance.total_ms`
**Kind:** `prop`

The same as `self_ms`, plus the time spent in code of other scripts that this script called, in milliseconds.

**Source:** `lua/framework/base.lua`

  ### `ScriptPerformance.window`
**Kind:** `prop`

A number that changes each time a new window ends.

**Source:** `lua/framework/base.lua`

  ### `ScriptPerformance.window_ms`
**Kind:** `prop`

The real length of the window, in milliseconds.

**Source:** `lua/framework/base.lua`

  ### `ScriptPerformance.worst_event`
**Kind:** `prop`

The event that the longest call ran for, such as `"render"`. It is `nil` if there were no calls.

**Source:** `lua/framework/base.lua`

---


## Bit

<!-- Source: cs2/api_reference/lua/bit.md -->

---
title: Bit Operations
icon: arrow-up-0-1
---

# Bit Operations

## bit _()_

LuaJIT's `bit` library provides bitwise operations. Numeric operands are normalized to
32 bits, and numeric results use the signed 32-bit range.


  ### `bit.tobit`
**Kind:** `fn`

**Arguments:** `x`

Normalizes a number to the signed 32-bit range used by bit operations.

**Source:** `lua/luajit/bit.lua`

  ### `bit.tohex`
**Kind:** `fn`

**Arguments:** `x, n?`

Converts its first argument to a hex string. The number of hex digits is given by the absolute value of the optional second argument. Negative values produce uppercase hex digits. If omitted, the default is 8 digits.

**Source:** `lua/luajit/bit.lua`

  ### `bit.bnot`
**Kind:** `fn`

**Arguments:** `x`

Returns the bitwise **NOT** of its argument.

**Source:** `lua/luajit/bit.lua`

  ### `bit.bor`
**Kind:** `fn`

**Arguments:** `x, ...?`

Returns the bitwise **OR** of all of its arguments.

**Source:** `lua/luajit/bit.lua`

  ### `bit.band`
**Kind:** `fn`

**Arguments:** `x, ...?`

Returns the bitwise **AND** of all of its arguments.

**Source:** `lua/luajit/bit.lua`

  ### `bit.bxor`
**Kind:** `fn`

**Arguments:** `x, ...?`

Returns the bitwise **XOR** of all of its arguments.

**Source:** `lua/luajit/bit.lua`

  ### `bit.lshift`
**Kind:** `fn`

**Arguments:** `x, n`

Returns the bitwise **logical left-shift** of its first argument by the number of bits given by the second argument. Bits shifted out on either end are lost. Vacant bits are filled with zeros.

**Source:** `lua/luajit/bit.lua`

  ### `bit.rshift`
**Kind:** `fn`

**Arguments:** `x, n`

Returns the bitwise **logical right-shift** of its first argument by the number of bits given by the second argument. Bits shifted out on either end are lost. Vacant bits are filled with zeros.

**Source:** `lua/luajit/bit.lua`

  ### `bit.arshift`
**Kind:** `fn`

**Arguments:** `x, n`

Returns the bitwise **arithmetic right-shift** of its first argument by the number of bits given by the second argument. Vacant bits are filled with the sign bit (bit 31) of the original value, preserving the sign.

**Source:** `lua/luajit/bit.lua`

  ### `bit.rol`
**Kind:** `fn`

**Arguments:** `x, n`

Returns the bitwise **left rotation** of its first argument by the number of bits given by the second argument. Bits rotated out on the left are inserted on the right.

**Source:** `lua/luajit/bit.lua`

  ### `bit.ror`
**Kind:** `fn`

**Arguments:** `x, n`

Returns the bitwise **right rotation** of its first argument by the number of bits given by the second argument. Bits rotated out on the right are inserted on the left.

**Source:** `lua/luajit/bit.lua`

  ### `bit.bswap`
**Kind:** `fn`

**Arguments:** `x`

Returns the **byte-swapped** value of its argument. This reverses the byte order of the 32-bit integer (e.g. for converting between little-endian and big-endian).

**Source:** `lua/luajit/bit.lua`

---


## Ffi

<!-- Source: cs2/api_reference/lua/ffi.md -->

---
title: FFI
icon: memory-stick
---

# FFI

<Heading level={2} id="ffi-1">
  ffi

   _()_
</Heading>

The `ffi` library lets you call external C functions and use C data structures from pure Lua code.
To use this library, add `ffi` to the `requirements` section of `extension.json`.


  ### `ffi.C`
**Kind:** `prop`

Default C namespace. Resolves symbols from the default set of loaded libraries.

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.arch`
**Kind:** `prop`

Target architecture string (e.g. `"x64"`).

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.os`
**Kind:** `prop`

Target OS name string (e.g. `"Windows"`).

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.cdef`
**Kind:** `fn`

**Arguments:** `def, params?, ...?`

Adds multiple C declarations for types, external variables, or external functions. `def` is a string holding one or more C declarations, separated by semicolons. Additional arguments substitute `$` placeholders in parameterized declarations, in order.

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.load`
**Kind:** `fn`

**Arguments:** `name, global?`

Loads the dynamic library at the given path or name and returns a new C library namespace. On POSIX systems, `global=true` makes symbols globally visible. The flag is ignored on Windows.

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.new`
**Kind:** `fn`

**Arguments:** `ct, nelem?, init?, ...?`

Creates a new C data object with the given C type and optional initial value. For VLAs, `nelem` specifies the number of elements.

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.typeof`
**Kind:** `fn`

**Arguments:** `ct, params?, ...?`

Creates a C type object for the given C type specifier. Caching the result of `ffi.typeof` and using it repeatedly is more efficient than passing strings directly.

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.cast`
**Kind:** `fn`

**Arguments:** `ct, init`

Casts a C data object to a different C type. This is a low-level, unsafe operation — no bounds or type checking is performed.

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.metatype`
**Kind:** `fn`

**Arguments:** `ct, metatable`

Attaches a metatable to a C type, enabling Lua-style OOP on C structures. Only one metatable can be attached per type. This cannot be changed afterwards.

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.gc`
**Kind:** `fn`

**Arguments:** `cdata, finalizer?`

Attaches a finalizer to a C data pointer, called when it is garbage collected. Pass `nil` as the finalizer to remove a previously set finalizer.

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.sizeof`
**Kind:** `fn`

**Arguments:** `ct, nelem?`

Returns the size in bytes of the given C type or C data object. Returns `nil` for types of unknown or variable size (e.g. incomplete types).

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.alignof`
**Kind:** `fn`

**Arguments:** `ct`

Returns the minimum required alignment in bytes for the given C type.

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.offsetof`
**Kind:** `fn`

**Arguments:** `ct, field`

Returns the byte offset of a field within a C struct or union. For bit fields, also returns the bit position and bit size.

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.istype`
**Kind:** `fn`

**Arguments:** `ct, obj`

Checks whether `obj` is a C data object compatible with the C type `ct`.

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.errno`
**Kind:** `fn`

**Arguments:** `newerr?`

Gets or sets the OS error number (errno). If `newerr` is given, sets errno to that value and returns the previous value. C runtime `errno` is distinct from Win32 `GetLastError`.

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.string`
**Kind:** `fn`

**Arguments:** `ptr, len?`

Converts a C data pointer to a Lua string. If `len` is given, copies exactly that many bytes; otherwise copies until the first null byte.

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.copy`
**Kind:** `fn`

**Arguments:** `dst, src, len`

Copies `len` bytes from `src` to `dst`. If given a string as `src` and no `len`, copies the full string including the null terminator.

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.fill`
**Kind:** `fn`

**Arguments:** `dst, len, c?`

Fills `len` bytes starting at `dst` with the byte value `c` (default 0).

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.abi`
**Kind:** `fn`

**Arguments:** `param`

Checks for ABI parameters of the target platform. Common params: `"32bit"`, `"64bit"`, `"le"`, `"be"`, `"fpu"`, `"softfp"`, `"hardfp"`, `"eabi"`, `"win"`.

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.ct*`
**Kind:** `type`

**Source:** `lua/luajit/ffi.lua`


## ffi.ctype\* _()_



Represents a C type object created by `ffi.typeof`. Callable to construct new `ffi.cdata*` instances.


  ### `ffi.ctype*(init, ...)`
**Kind:** `ctor`

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.ctype*(nelem, init, ...)`
**Kind:** `ctor`

**Source:** `lua/luajit/ffi.lua`


## ffi.cb\* _()_



Represents a C callback object. Use `:free()` to release it and `:set()` to replace the Lua function.


  ### `ffi.cb*:free`
**Kind:** `method`

Releases the resources associated with a C callback object. The callback must not be called after this — doing so is undefined behavior.

**Source:** `lua/luajit/ffi.lua`

  ### `ffi.cb*:set`
**Kind:** `method`

**Arguments:** `func`

Replaces the Lua function of an existing C callback object with a new one. This avoids the overhead of allocating a new callback.

**Source:** `lua/luajit/ffi.lua`

---


## Jit

<!-- Source: cs2/api_reference/lua/jit.md -->

---
title: JIT
icon: clock-check
---

# JIT

<Heading level={2} id="jit-1">
  jit

   _()_
</Heading>

The `jit` library controls the LuaJIT just-in-time compiler at runtime.
You can use it through the global `jit` or get it with `require("jit")`.


  ### `jit.arch`
**Kind:** `prop`

Target CPU architecture.

**Source:** `lua/luajit/jit.lua`

  ### `jit.os`
**Kind:** `prop`

Target OS name.

**Source:** `lua/luajit/jit.lua`

  ### `jit.version`
**Kind:** `prop`

LuaJIT version string

**Source:** `lua/luajit/jit.lua`

  ### `jit.version_num`
**Kind:** `prop`

Numeric version

**Source:** `lua/luajit/jit.lua`

  ### `jit.on`
**Kind:** `fn`

**Arguments:** `func?, recursive?`

Enables JIT compilation for the function `func`. Pass `true` to select the function that calls `jit.on`.

If `recursive` is `true`, the function and all functions defined inside it are selected. If
      `recursive` is `false`, only the functions defined inside it are selected. If `recursive` is
      omitted, only the function itself is selected.

      All scripts share one JIT compiler, so a call without a function, or with `nil`, is ignored.
      In a developer extension, the first ignored call also adds a warning to the Errors list of the
      script. If `func` is any other value that is not a function, an error is thrown.
    
  

**Source:** `lua/luajit/jit.lua`

  ### `jit.off`
**Kind:** `fn`

**Arguments:** `func?, recursive?`

Disables JIT compilation for the function `func`. Pass `true` to select the function that calls `jit.off`.

If `recursive` is `true`, the function and all functions defined inside it are selected. If
      `recursive` is `false`, only the functions defined inside it are selected. If `recursive` is
      omitted, only the function itself is selected.

      All scripts share one JIT compiler, so a call without a function, or with `nil`, is ignored.
      In a developer extension, the first ignored call also adds a warning to the Errors list of the
      script. If `func` is any other value that is not a function, an error is thrown.
    
  

**Source:** `lua/luajit/jit.lua`

  ### `jit.flush`
**Kind:** `fn`

**Arguments:** `func?, recursive?`

Flushes the compiled JIT code of the function `func`. Pass `true` to select the function that calls `jit.flush`. Pass a trace number to flush only that trace.

If `recursive` is `true`, the function and all functions defined inside it are selected. If
      `recursive` is `false`, only the functions defined inside it are selected. If `recursive` is
      omitted, only the function itself is selected.

      All scripts share one JIT compiler, so a call without a function, or with `nil`, is ignored.
      In a developer extension, the first ignored call also adds a warning to the Errors list of the
      script. If `func` is any other value that is not a function, an error is thrown.
    
  

**Source:** `lua/luajit/jit.lua`

  ### `jit.status`
**Kind:** `fn`

Returns the current JIT status. The first return value is `true` if the JIT compiler is enabled. Subsequent return values are strings describing enabled CPU features and optimizations.

**Source:** `lua/luajit/jit.lua`

---


## Math

<!-- Source: cs2/api_reference/lua/math.md -->

---
title: Math
icon: calculator
---

# Math

<Heading level={2} id="math-1">
  math

   _()_
</Heading>

Standard Lua math library, providing common mathematical functions.


  ### `math.huge`
**Kind:** `prop`

Positive infinity (`1/0`). Useful as an initial value for minimum searches.

**Source:** `lua/luajit/math.lua`

  ### `math.pi`
**Kind:** `prop`

The value of π (3.14159265358979323846…).

**Source:** `lua/luajit/math.lua`

  ### `math.abs`
**Kind:** `fn`

**Arguments:** `x`

Returns the absolute value of `x`.

**Source:** `lua/luajit/math.lua`

  ### `math.acos`
**Kind:** `fn`

**Arguments:** `x`

Returns the arc cosine of `x` in radians. `x` must be in the range `[-1, 1]`.

**Source:** `lua/luajit/math.lua`

  ### `math.asin`
**Kind:** `fn`

**Arguments:** `x`

Returns the arc sine of `x` in radians. `x` must be in the range `[-1, 1]`.

**Source:** `lua/luajit/math.lua`

  ### `math.atan`
**Kind:** `fn`

**Arguments:** `y`

Returns the arc tangent of `y` in radians. Use `math.atan2` to compute the angle from an (x, y) pair with proper quadrant handling.

**Source:** `lua/luajit/math.lua`

  ### `math.atan2`
**Kind:** `fn`

**Arguments:** `y, x`

Returns the arc tangent of `y/x` in radians, using the signs of both arguments to determine the correct quadrant. Handles the case where `x` is zero correctly.

**Source:** `lua/luajit/math.lua`

  ### `math.ceil`
**Kind:** `fn`

**Arguments:** `x`

Returns the smallest integer value greater than or equal to `x`.

**Source:** `lua/luajit/math.lua`

  ### `math.cos`
**Kind:** `fn`

**Arguments:** `x`

Returns the cosine of angle `x` (given in radians).

**Source:** `lua/luajit/math.lua`

  ### `math.cosh`
**Kind:** `fn`

**Arguments:** `x`

Returns the hyperbolic cosine of `x`.

**Source:** `lua/luajit/math.lua`

  ### `math.deg`
**Kind:** `fn`

**Arguments:** `x`

Converts angle `x` from radians to degrees.

**Source:** `lua/luajit/math.lua`

  ### `math.exp`
**Kind:** `fn`

**Arguments:** `x`

Returns the value `e^x` (the exponential function).

**Source:** `lua/luajit/math.lua`

  ### `math.floor`
**Kind:** `fn`

**Arguments:** `x`

Returns the largest integer value less than or equal to `x`.

**Source:** `lua/luajit/math.lua`

  ### `math.fmod`
**Kind:** `fn`

**Arguments:** `x, y`

Returns the remainder of the division of `x` by `y` that rounds the quotient towards zero.

**Source:** `lua/luajit/math.lua`

  ### `math.frexp`
**Kind:** `fn`

**Arguments:** `x`

Returns the mantissa and binary exponent of `x`. The value satisfies `x = m * 2^e`, where `0.5 <= |m| < 1` (or `m = 0`).

**Source:** `lua/luajit/math.lua`

  ### `math.ldexp`
**Kind:** `fn`

**Arguments:** `m, e`

Returns `m * 2^e` (the inverse of `math.frexp`).

**Source:** `lua/luajit/math.lua`

  ### `math.log`
**Kind:** `fn`

**Arguments:** `x, base?`

Returns the logarithm of `x`. If `base` is provided, returns the logarithm in that base; otherwise returns the natural logarithm.

**Source:** `lua/luajit/math.lua`

  ### `math.log10`
**Kind:** `fn`

**Arguments:** `x`

Returns the base-10 logarithm of `x`. Equivalent to `math.log(x, 10)`.

**Source:** `lua/luajit/math.lua`

  ### `math.max`
**Kind:** `fn`

**Arguments:** `x, ...?`

Returns the maximum value among its arguments.

**Source:** `lua/luajit/math.lua`

  ### `math.min`
**Kind:** `fn`

**Arguments:** `x, ...?`

Returns the minimum value among its arguments.

**Source:** `lua/luajit/math.lua`

  ### `math.modf`
**Kind:** `fn`

**Arguments:** `x`

Returns the integral and fractional parts of `x`. Both results have the same sign as `x`.

**Source:** `lua/luajit/math.lua`

  ### `math.pow`
**Kind:** `fn`

**Arguments:** `x, y`

Returns `x` raised to the power `y`. Equivalent to `x^y`.

**Source:** `lua/luajit/math.lua`

  ### `math.rad`
**Kind:** `fn`

**Arguments:** `x`

Converts angle `x` from degrees to radians.

**Source:** `lua/luajit/math.lua`

  ### `math.random`
**Kind:** `fn`

**Arguments:** `m, n`

Returns a pseudo-random number.

* `math.random()` returns a float in the range `[0, 1)`.
      * `math.random(m)` returns an integer in the range `[1, m]`.
      * `math.random(m, n)` returns an integer in the range `[m, n]`.
    
  

**Source:** `lua/luajit/math.lua`

  ### `math.randomseed`
**Kind:** `fn`

**Arguments:** `x`

This function does nothing. All scripts share one pseudo-random number generator, and it is seeded when the script runtime starts.

The first time a script calls this function, a warning is added to the Errors list of the
      script and written to the game console.
    
  

**Source:** `lua/luajit/math.lua`

  ### `math.sin`
**Kind:** `fn`

**Arguments:** `x`

Returns the sine of angle `x` (given in radians).

**Source:** `lua/luajit/math.lua`

  ### `math.sinh`
**Kind:** `fn`

**Arguments:** `x`

Returns the hyperbolic sine of `x`.

**Source:** `lua/luajit/math.lua`

  ### `math.sqrt`
**Kind:** `fn`

**Arguments:** `x`

Returns the square root of `x`. `x` must be non-negative.

**Source:** `lua/luajit/math.lua`

  ### `math.tan`
**Kind:** `fn`

**Arguments:** `x`

Returns the tangent of angle `x` (given in radians).

**Source:** `lua/luajit/math.lua`

  ### `math.tanh`
**Kind:** `fn`

**Arguments:** `x`

Returns the hyperbolic tangent of `x`.

**Source:** `lua/luajit/math.lua`

---


## String

<!-- Source: cs2/api_reference/lua/string.md -->

---
title: String
icon: quote
sidebar_position: 3
---

# String

<Heading level={2} id="string-1">
  string

   _()_
</Heading>

Standard Lua string library. All functions are also available as methods on string values
via the `string` metatable (e.g. `("hello"):upper()`).


  ### `string.byte`
**Kind:** `fn`

**Arguments:** `s, i?, j?`

Returns the numeric byte values of the characters in `s` from position `i` to `j`. `i` defaults to `1`; `j` defaults to `i`. Negative indices count from the end of the string.

**Source:** `lua/luajit/string.lua`

  ### `string.char`
**Kind:** `fn`

**Arguments:** `byte, ...?`

Returns a string formed by the concatenation of characters with the given byte values.

**Source:** `lua/luajit/string.lua`

  ### `string.find`
**Kind:** `fn`

**Arguments:** `s, pattern, init?, plain?`

Searches for the first match of `pattern` in `s`, starting at position `init`. If `plain` is true, the pattern is treated as a plain substring (no magic characters). Returns the start and end positions of the match, plus any captures.

**Source:** `lua/luajit/string.lua`

  ### `string.format`
**Kind:** `fn`

**Arguments:** `s, ...?`

Returns a formatted string, following the same rules as the C `printf` family. Supported conversion specifiers: `%d`, `%i`, `%u`, `%o`, `%x`, `%X`, `%f`, `%e`, `%E`, `%g`, `%G`, `%c`, `%s`, `%q`, `%%`.

**Source:** `lua/luajit/string.lua`

  ### `string.gmatch`
**Kind:** `fn`

**Arguments:** `s, pattern`

Returns an iterator function that, each time it is called, returns the next set of captures from `pattern` over `s`. If the pattern has no captures, the whole match is returned each iteration.

**Source:** `lua/luajit/string.lua`

  ### `string.gsub`
**Kind:** `fn`

**Arguments:** `s, pattern, repl, n?`

Returns a copy of `s` with all matches of `pattern` replaced by `repl`. `repl` can be a string (with `%0`–`%9` capture references), a table (keyed by the first capture), or a function (called with the captures, returning the replacement). `n` limits the number of substitutions.

**Source:** `lua/luajit/string.lua`

  ### `string.len`
**Kind:** `fn`

**Arguments:** `s`

Returns the length of string `s` in bytes. Equivalent to `#s`.

**Source:** `lua/luajit/string.lua`

  ### `string.lower`
**Kind:** `fn`

**Arguments:** `s`

Returns a lowercase copy of `s`.

**Source:** `lua/luajit/string.lua`

  ### `string.match`
**Kind:** `fn`

**Arguments:** `s, pattern, init?`

Searches for the first match of `pattern` in `s`, starting at position `init`. Returns the captures if the pattern has them, or the whole match otherwise.

**Source:** `lua/luajit/string.lua`

  ### `string.rep`
**Kind:** `fn`

**Arguments:** `s, n, sep?`

Returns a string that is the concatenation of `n` copies of `s`, separated by `sep`.

**Source:** `lua/luajit/string.lua`

  ### `string.reverse`
**Kind:** `fn`

**Arguments:** `s`

Returns a string that is the reverse of `s`.

**Source:** `lua/luajit/string.lua`

  ### `string.sub`
**Kind:** `fn`

**Arguments:** `s, i, j?`

Returns the substring of `s` from position `i` to `j` (inclusive). Negative indices count from the end of the string (`-1` is the last character).

**Source:** `lua/luajit/string.lua`

  ### `string.upper`
**Kind:** `fn`

**Arguments:** `s`

Returns an uppercase copy of `s`.

**Source:** `lua/luajit/string.lua`

---


## Table.Clear

<!-- Source: cs2/api_reference/lua/table.clear.md -->

---
title: Table Clearing
icon: eraser
sidebar_position: 6
---

# Table Clearing

## Functions _()_


  ### `table.clear`
**Kind:** `fn`

**Arguments:** `tab`

An extra library function `table.clear()` can be made available via `require("table.clear")`. This clears all keys and values from a table, but preserves the allocated array/hash sizes. Existing references still point to the cleared table, allowing its storage to be reused.

**Source:** `lua/luajit/table.clear.lua`

---


## Table

<!-- Source: cs2/api_reference/lua/table.md -->

---
title: Table
icon: braces
sidebar_position: 4
---

# Table

<Heading level={2} id="table-1">
  table

   _()_
</Heading>

Standard Lua table library, providing utilities for manipulating array-style and general tables.


  ### `table.concat`
**Kind:** `fn`

**Arguments:** `list, sep?, i?, j?`

Returns the elements of `list` joined into a string, with `sep` inserted between each element. Only elements from index `i` to `j` are included. All elements must be strings or numbers.

**Source:** `lua/luajit/table.lua`

  ### `table.foreach`
**Kind:** `fn`

**Arguments:** `list, callback`

Iterates over all key-value pairs in `list` (including non-integer keys) and calls `callback` for each. If `callback` returns a non-nil value, the iteration stops and that value is returned. Deprecated — prefer `pairs` for new code.

**Source:** `lua/luajit/table.lua`

  ### `table.foreachi`
**Kind:** `fn`

**Arguments:** `list, callback`

Iterates over the integer keys of `list` in order and calls `callback` for each. If `callback` returns a non-nil value, the iteration stops and that value is returned. Deprecated — prefer `ipairs` for new code.

**Source:** `lua/luajit/table.lua`

  ### `table.getn`
**Kind:** `fn`

**Arguments:** `list`

Returns the "getn" size of a table, which is the largest integer key `n` such that `t[n]` is not nil. Deprecated — use the `#` length operator for new code.

**Source:** `lua/luajit/table.lua`

  ### `table.insert`
**Kind:** `fn`

**Arguments:** `list, pos, value`

Inserts `value` into `list`. If `pos` is given, inserts before the element at `pos`, shifting up elements `pos..#list`. If only `value` is given, appends it to the end (`list[#list + 1] = value`).

**Source:** `lua/luajit/table.lua`

  ### `table.maxn`
**Kind:** `fn`

**Arguments:** `table`

Returns the largest positive numerical key in `table`, or zero if no such key exists. Unlike `#`, this scans all keys and works correctly with sparse tables.

**Source:** `lua/luajit/table.lua`

  ### `table.move`
**Kind:** `fn`

**Arguments:** `a1, f, e, t, a2?`

Moves elements from table `a1` into table `a2` (or `a1` itself if `a2` is omitted). Copies elements `a1[f]` through `a1[e]` into `a2` starting at index `t`.

**Source:** `lua/luajit/table.lua`

  ### `table.pack`
**Kind:** `fn`

**Arguments:** `...?`

Packs all arguments into a new table and stores the argument count in the field `n`. Equivalent to `{ n = select('#', ...), ... }`.

**Source:** `lua/luajit/table.lua`

  ### `table.remove`
**Kind:** `fn`

**Arguments:** `list, pos?`

Removes and returns the element at position `pos` from `list`, shifting down all subsequent elements. If `pos` is omitted, removes and returns the last element.

**Source:** `lua/luajit/table.lua`

  ### `table.sort`
**Kind:** `fn`

**Arguments:** `list, comp?`

Sorts the elements of `list` in place. `comp` is an optional comparator function; if omitted, the standard `<` operator is used. The sort algorithm is not guaranteed to be stable.

**Source:** `lua/luajit/table.lua`

  ### `table.unpack`
**Kind:** `fn`

**Arguments:** `list, i?, j?`

Returns the elements of `list` as multiple return values, from index `i` to `j`. Equivalent to `return list[i], list[i+1], ..., list[j]`.

**Source:** `lua/luajit/table.lua`

---


## Table.New

<!-- Source: cs2/api_reference/lua/table.new.md -->

---
title: Table Allocation
icon: boxes
sidebar_position: 5
---

# Table Allocation

## Functions _()_


  ### `table.new`
**Kind:** `fn`

**Arguments:** `narray, nhash`

An extra library function `table.new()` can be made available via `require("table.new")`. Returns an empty table with the requested array and hash capacity reserved.

**Source:** `lua/luajit/table.new.lua`

---


## Aabb

<!-- Source: cs2/api_reference/mathlib/aabb.md -->

---
title: Bounding Box
icon: box
---

# Bounding Box

## AABB\_t" />



An axis aligned bounding box. It is stored as a minimum corner and a maximum corner.

Trace shapes and hitbox bounds use this type. Schema fields of type `AABB_t` or `AABBWS_t`
are also read and written as this type.

The `TransformAABB`, `RotateAABB` and `RotateAndScaleAABB` methods of `matrix3x4_t`,
`CTransform` and the quaternion types take a minimum and a maximum corner. You can pass one
`AABB_t` instead of the two corners. In that case, the method also returns an `AABB_t`.

The `+` operator returns the smallest box that contains both boxes. The `==` operator
compares both corners and returns `false` when the other value is not an `AABB_t`.

```cpp lines=false
typedef struct AABB_t {
    Vector m_vMinBounds;
    Vector m_vMaxBounds;
} AABB_t;
```


  ### `AABB_t.m_vMaxBounds`
**Kind:** `prop`

The maximum corner.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.m_vMinBounds`
**Kind:** `prop`

The minimum corner.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t + b`
**Kind:** `meta`

The smallest box that contains both boxes.

**Source:** `mathlib/engine/aabb.lua`


<Heading level={2} id="aabb_t-1">
  AABB_t

  " />
</Heading>

Call `AABB_t()` to create a box with both corners at `(0, 0, 0)`. Call `AABB_t(mins, maxs)`
with two vectors to set the corners. Call `AABB_t(points)` with a Lua array of points to
create the smallest box that contains all of them. If the array is empty, the box is
invalid, the same as `AABB_t.Invalid()`. Call `AABB_t(other)` to make a copy of another box.

Any other arguments throw an error.


  ### `AABB_t()`
**Kind:** `ctor`

A box with both corners at `(0, 0, 0)`.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t(mins, maxs)`
**Kind:** `ctor`

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t(points)`
**Kind:** `ctor`

The smallest box that contains every point in a Lua array.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t(self)`
**Kind:** `ctor`

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.Invalid`
**Kind:** `fn`

Returns a box that contains nothing. The minimum corner is set to `FLT_MAX` and the maximum corner is set to `-FLT_MAX` on every axis. When you add a point to this box, the box becomes that point.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.Copy`
**Kind:** `fn`

**Arguments:** `src, dst`

Copies one box into another box.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.Init`
**Kind:** `fn`

**Arguments:** `box, mins, maxs`

Sets both corners of the box.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.MakeInvalid`
**Kind:** `fn`

**Arguments:** `box`

Changes the box so that it contains nothing. This is the same box as `AABB_t.Invalid()`.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.IsInvalid`
**Kind:** `fn`

**Arguments:** `box`

Returns `true` if the minimum is greater than the maximum on any axis.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.IsEmpty`
**Kind:** `fn`

**Arguments:** `box`

Returns `true` if the size of the box is 0 or negative on any axis. This means the minimum is greater than or equal to the maximum on that axis.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.IsReasonable`
**Kind:** `fn`

**Arguments:** `box`

Returns `true` if both corners pass `Vector.IsReasonable` with its default range of 1000000.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.GetCenter`
**Kind:** `fn`

**Arguments:** `box`

Returns the center of the box. This is the point halfway between the two corners.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.GetSize`
**Kind:** `fn`

**Arguments:** `box`

Returns the size of the box on each axis. This is the maximum corner minus the minimum corner.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.GetBoundingRadius`
**Kind:** `fn`

**Arguments:** `box`

Returns the distance from the center of the box to a corner. A sphere with this radius around the center contains the whole box.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.GetSurfaceArea`
**Kind:** `fn`

**Arguments:** `box`

Returns the total area of the six faces of the box.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.GetVolume`
**Kind:** `fn`

**Arguments:** `box`

Returns the volume of the box.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.Corner`
**Kind:** `fn`

**Arguments:** `box, n`

Returns one of the eight corners of the box. Each bit of `n` chooses the minimum or the maximum value on one axis. Bit 0 chooses the maximum X, bit 1 chooses the maximum Y and bit 2 chooses the maximum Z. For example, 0 is the minimum corner and 7 is the maximum corner.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.LengthOfSmallestDimension`
**Kind:** `fn`

**Arguments:** `box`

Returns the size of the box on its shortest axis.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.LengthOfLargestDimension`
**Kind:** `fn`

**Arguments:** `box`

Returns the size of the box on its longest axis.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.AddPointToBounds`
**Kind:** `fn`

**Arguments:** `box, point`

Grows the box so that it contains a point.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.AddSphereToBounds`
**Kind:** `fn`

**Arguments:** `box, center, radius`

Grows the box so that it contains a sphere.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.AddAABBToBounds`
**Kind:** `fn`

**Arguments:** `box, other`

Grows the box so that it contains another box.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.AddAABBToBounds`
**Kind:** `fn`

**Arguments:** `box, mins, maxs`

Grows the box so that it contains the box given by a minimum and a maximum corner.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.IntersectWith`
**Kind:** `fn`

**Arguments:** `box, other`

Shrinks the box to the part that it shares with another box. If the boxes do not overlap, the result is an invalid box.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.SetToPoint`
**Kind:** `fn`

**Arguments:** `box, point`

Moves both corners of the box to a point, so the box has a size of 0.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.EnsureMinSize`
**Kind:** `fn`

**Arguments:** `box, min_size`

Grows the box equally on both sides of its center so that its size on each axis is at least the given size. An axis that is already large enough does not change.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.Move`
**Kind:** `fn`

**Arguments:** `box, delta`

Moves both corners of the box by an offset.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.Expand`
**Kind:** `fn`

**Arguments:** `box, amount`

Moves both corners of the box outward. The minimum corner moves down by the amount and the maximum corner moves up by the amount. A negative amount shrinks the box.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.GetMinDistToPoint`
**Kind:** `fn`

**Arguments:** `box, point`

Returns the distance from a point to the nearest point of the box. If the point is inside the box, the distance is 0.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.GetMinAxialDistanceToPoint`
**Kind:** `fn`

**Arguments:** `box, point`

Measures how far a point is outside the box on each axis and returns the largest of the three values. If the point is inside the box, the result is 0.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.DistanceTo`
**Kind:** `fn`

**Arguments:** `box, other`

Returns the distance between the nearest points of two boxes. If the boxes overlap, the distance is 0.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.ClosestPoint`
**Kind:** `fn`

**Arguments:** `box, point`

Returns the point of the box that is nearest to the given point. If the point is inside the box, the same point is returned.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.Overlaps`
**Kind:** `fn`

**Arguments:** `box, other`

Returns `true` if the two boxes overlap. Boxes that only touch at a face, an edge or a corner also count as overlapping.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.ContainsPoint`
**Kind:** `fn`

**Arguments:** `box, point`

Returns `true` if the point is inside the box or on its surface.

**Source:** `mathlib/engine/aabb.lua`

  ### `AABB_t.Contains`
**Kind:** `fn`

**Arguments:** `box, other`

Returns `true` if the other box is completely inside this box. Shared faces are allowed.

**Source:** `mathlib/engine/aabb.lua`

---


## Angle

<!-- Source: cs2/api_reference/mathlib/angle.md -->

---
title: Angle
icon: rotate-3d
sidebar_position: 4
---

# Angle

## QAngle" />



An angle with pitch, yaw and roll in degrees. A positive pitch looks down. A positive yaw
turns to the left. The roll tilts the view to the side.

The `+` and `-` operators add and subtract two angles. The `*` and `/` operators take a
number or an angle. With a number, every component is multiplied or divided by it. With an
angle, each component is multiplied or divided by the matching component. The unary `-`
operator returns a negated copy. The `==` operator compares every component and returns
`false` when the other value is not a `QAngle`.

```cpp lines=false
typedef struct QAngle {
    float x, y, z;
} QAngle;
```


  ### `QAngle.x`
**Kind:** `prop`

The pitch. A positive value looks down and a negative value looks up.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.y`
**Kind:** `prop`

The yaw. A positive value turns left and a negative value turns right.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.z`
**Kind:** `prop`

The roll. It tilts the view to the side.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle + b`
**Kind:** `meta`

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle - b`
**Kind:** `meta`

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle * b`
**Kind:** `meta`

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle / b`
**Kind:** `meta`

**Source:** `mathlib/engine/angle.lua`

  ### `-QAngle`
**Kind:** `meta`

**Source:** `mathlib/engine/angle.lua`


<Heading level={2} id="qangle-1">
  QAngle

  " />
</Heading>

Call `QAngle(pitch, yaw, roll)` to create an angle. A component that is not given is set
to 0. Call `QAngle(other)` to make a copy of another angle.

The constant angles below cannot be changed.


  ### `QAngle(pitch, yaw, roll)`
**Kind:** `ctor`

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle(self)`
**Kind:** `ctor`

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.vInvalid`
**Kind:** `prop`

An angle with every component set to `FLT_MAX`, the largest 32 bit float.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.vZero`
**Kind:** `prop`

The angle `(0, 0, 0)`.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.Copy`
**Kind:** `fn`

**Arguments:** `src, dst`

Copies the components of one angle into another angle.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.Pitch`
**Kind:** `fn`

**Arguments:** `ang`

Returns the pitch, which is the X component.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.Yaw`
**Kind:** `fn`

**Arguments:** `ang`

Returns the yaw, which is the Y component.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.Roll`
**Kind:** `fn`

**Arguments:** `ang`

Returns the roll, which is the Z component.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.Init`
**Kind:** `fn`

**Arguments:** `ang, x?, y?, z?`

Sets the components of the angle. A component that is not given is set to 0.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.Normalize`
**Kind:** `fn`

**Arguments:** `ang`

Wraps each component of the angle into the range from -180 to 180. The value -180 is included and the value 180 is not, so 180 becomes -180.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.Normalize`
**Kind:** `fn`

**Arguments:** `ang`

Wraps one value in degrees into the range from -180 to 180 and returns it. The value -180 is included and the value 180 is not, so 180 becomes -180.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.Normalized`
**Kind:** `fn`

**Arguments:** `ang`

Returns a copy of the angle with each component wrapped into the range from -180 to 180. The value -180 is included and the value 180 is not.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.Unpack`
**Kind:** `fn`

**Arguments:** `ang`

Returns the pitch, yaw and roll of the angle.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.GetForward`
**Kind:** `fn`

**Arguments:** `ang`

Returns the forward direction of the angle as a vector with a length of 1. Only the pitch and the yaw are used.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.GetLeftVectors`
**Kind:** `fn`

**Arguments:** `ang`

Returns the forward, left and up directions of the angle. Each vector has a length of 1.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.Length`
**Kind:** `fn`

**Arguments:** `ang`

Returns the length of the angle. This is the square root of the sum of the squares of the pitch, the yaw and the roll.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.LengthSqr`
**Kind:** `fn`

**Arguments:** `ang`

Returns the squared length of the angle. This is the sum of the squares of the pitch, the yaw and the roll.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.Negate`
**Kind:** `fn`

**Arguments:** `ang`

Negates every component of the angle.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.IsValid`
**Kind:** `fn`

**Arguments:** `ang`

Returns `true` if every component is a finite number. NaN and infinity are not finite.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.Invalidate`
**Kind:** `fn`

**Arguments:** `ang`

Sets every component of the angle to NaN.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.NormalizePositive`
**Kind:** `fn`

**Arguments:** `ang`

Wraps each component of the angle into the range from 0 to 360. The value 0 is included and the value 360 is not.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.Clamp`
**Kind:** `fn`

**Arguments:** `ang`

Clamps each component of the angle to the range that the engine uses. The pitch is clamped from -89 to 89, the yaw from -180 to 180 and the roll from -90 to 90. The angle is not wrapped first, so normalize it before you clamp it.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.IsEqual`
**Kind:** `fn`

**Arguments:** `ang, other, tolerance`

Returns `true` if the difference between each component of the two angles is at most `tolerance`. Each difference is wrapped into the range from -180 to 180 first, so 179 and -179 are 2 degrees apart.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.GetRightVectors`
**Kind:** `fn`

**Arguments:** `ang`

Returns the forward, right and up directions of the angle. Each vector has a length of 1. The right direction is the negated left direction from `GetLeftVectors`.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.GetLeftVectorsTranspose`
**Kind:** `fn`

**Arguments:** `ang`

Returns the rows of the rotation matrix of the angle. `GetLeftVectors` returns the columns of the same matrix. This is the transposed forward, left and up.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.GetRightVectorsTranspose`
**Kind:** `fn`

**Arguments:** `ang`

Returns the rows of the rotation matrix of the angle, with the second row negated. This is the transposed forward, right and up.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.RotateVector`
**Kind:** `fn`

**Arguments:** `ang, v`

Rotates a vector by the rotation of the angle. The X component of the vector moves along the forward direction, Y moves along the left direction and Z moves along the up direction.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.ToMatrix`
**Kind:** `fn`

**Arguments:** `ang`

Returns the rotation matrix of the angle. The origin of the matrix is `(0, 0, 0)`.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.ToQuaternion`
**Kind:** `fn`

**Arguments:** `ang`

Converts the angle to a `Quaternion`, which is aligned to 16 bytes.

**Source:** `mathlib/engine/angle.lua`

  ### `QAngle.ToQuaternionStorage`
**Kind:** `fn`

**Arguments:** `ang`

Converts the angle to a `QuaternionStorage`, which is not aligned.

**Source:** `mathlib/engine/angle.lua`

---


## Color

<!-- Source: cs2/api_reference/mathlib/color.md -->

---
title: Color
icon: paint-bucket
---

# Color

<Heading level={2} id="color-1">
  Color

  " />
</Heading>



A colour with red, green, blue and alpha channels. Each channel is a byte from 0 to 255.

`Color(r, g, b)` creates a colour that is fully visible, because an alpha that is not given
is set to 255. A red, green or blue channel that is not given is set to 0.

The `==` operator compares all four channels and returns `false` when the other value is
not a `Color`.


  ### `Color.a`
**Kind:** `prop`

The alpha channel, from 0 to 255. 0 is fully transparent and 255 is fully visible.

**Source:** `mathlib/engine/color.lua`

  ### `Color.b`
**Kind:** `prop`

The blue channel, from 0 to 255.

**Source:** `mathlib/engine/color.lua`

  ### `Color.g`
**Kind:** `prop`

The green channel, from 0 to 255.

**Source:** `mathlib/engine/color.lua`

  ### `Color.r`
**Kind:** `prop`

The red channel, from 0 to 255.

**Source:** `mathlib/engine/color.lua`


<Heading level={2} id="color-2">
  Color

  " />
</Heading>

Call `Color(r, g, b, a)` to create a colour. Call `Color(other)` to make a copy of another
colour.


  ### `Color(r, g, b, a)`
**Kind:** `ctor`

The alpha is 255 if it is not given.

**Source:** `mathlib/engine/color.lua`

  ### `Color(self)`
**Kind:** `ctor`

**Source:** `mathlib/engine/color.lua`

  ### `Color.Unpack`
**Kind:** `fn`

**Arguments:** `col`

Returns the red, green, blue and alpha channels of the colour.

**Source:** `mathlib/engine/color.lua`

  ### `Color.Lerp`
**Kind:** `fn`

**Arguments:** `a, b, t`

Returns a new colour between two colours. This is linear interpolation. Each channel of the result is rounded to the nearest whole number and clamped to the range from 0 to 255.

**Source:** `mathlib/engine/color.lua`

  ### `Color.Hex`
**Kind:** `fn`

**Arguments:** `col`

Returns the colour as a hex string in the format `RRGGBBAA`, with upper case letters. For example, `Color(255, 128, 0)` returns `"FF8000FF"`.

**Source:** `mathlib/engine/color.lua`

---


## Euler

<!-- Source: cs2/api_reference/mathlib/euler.md -->

---
title: Euler Angles
icon: rotate-cw
---

# Euler Angles

## RadianEuler" />



Euler angles in radians. Each component is a rotation around one axis. `x` rotates around
the X axis, which is the roll. `y` rotates around the Y axis, which is the pitch. `z` rotates
around the Z axis, which is the yaw.

A `QAngle` stores the same rotation in degrees and in a different order, which is pitch,
yaw and roll. When you convert to a `QAngle`, both the units and the order change. Animation
and particle schema fields use this type.

The `==` operator compares every component and returns `false` when the other value is not
a `RadianEuler`.

```cpp lines=false
typedef struct RadianEuler {
    float x, y, z;
} RadianEuler;
```


  ### `RadianEuler.x`
**Kind:** `prop`

The rotation around the X axis in radians. This is the roll.

**Source:** `mathlib/engine/euler.lua`

  ### `RadianEuler.y`
**Kind:** `prop`

The rotation around the Y axis in radians. This is the pitch.

**Source:** `mathlib/engine/euler.lua`

  ### `RadianEuler.z`
**Kind:** `prop`

The rotation around the Z axis in radians. This is the yaw.

**Source:** `mathlib/engine/euler.lua`


<Heading level={2} id="radianeuler-1">
  RadianEuler

  " />
</Heading>

Call `RadianEuler()` to create angles with every component set to 0. Call
`RadianEuler(x, y, z)` to set the components. A component that is not given is set to 0.

You can also pass a `QAngle`, a `Quaternion`, a `QuaternionStorage` or a `DegreeEuler` to
convert the same rotation. Pass another `RadianEuler` to make a copy. Any other first
argument throws an error.


  ### `RadianEuler()`
**Kind:** `ctor`

Angles with every component set to 0.

**Source:** `mathlib/engine/euler.lua`

  ### `RadianEuler(x, y, z)`
**Kind:** `ctor`

**Source:** `mathlib/engine/euler.lua`

  ### `RadianEuler(angles)`
**Kind:** `ctor`

**Source:** `mathlib/engine/euler.lua`

  ### `RadianEuler(q)`
**Kind:** `ctor`

**Source:** `mathlib/engine/euler.lua`

  ### `RadianEuler(angles)`
**Kind:** `ctor`

**Source:** `mathlib/engine/euler.lua`

  ### `RadianEuler(self)`
**Kind:** `ctor`

**Source:** `mathlib/engine/euler.lua`

  ### `RadianEuler.Unpack`
**Kind:** `fn`

**Arguments:** `e`

Returns the X, Y and Z components.

**Source:** `mathlib/engine/euler.lua`

  ### `RadianEuler.Copy`
**Kind:** `fn`

**Arguments:** `src, dst`

Copies the components of one value into another value.

**Source:** `mathlib/engine/euler.lua`

  ### `RadianEuler.Init`
**Kind:** `fn`

**Arguments:** `e, x?, y?, z?`

Sets the components. A component that is not given is set to 0.

**Source:** `mathlib/engine/euler.lua`

  ### `RadianEuler.IsValid`
**Kind:** `fn`

**Arguments:** `e`

Returns `true` if every component is a finite number. NaN and infinity are not finite.

**Source:** `mathlib/engine/euler.lua`

  ### `RadianEuler.Invalidate`
**Kind:** `fn`

**Arguments:** `e`

Sets every component to NaN.

**Source:** `mathlib/engine/euler.lua`

  ### `RadianEuler.ToQAngle`
**Kind:** `fn`

**Arguments:** `e`

Converts the angles to a `QAngle` in degrees. The pitch comes from `y`, the yaw comes from `z` and the roll comes from `x`.

**Source:** `mathlib/engine/euler.lua`

  ### `RadianEuler.ToQuaternion`
**Kind:** `fn`

**Arguments:** `e`

Converts the angles to a `Quaternion`.

**Source:** `mathlib/engine/euler.lua`

  ### `RadianEuler.RotateVector`
**Kind:** `fn`

**Arguments:** `e, v`

Rotates a vector by these angles. The result is the same as `QAngle.RotateVector` with the converted angle.

**Source:** `mathlib/engine/euler.lua`

  ### `RadianEuler.Pitch`
**Kind:** `fn`

**Arguments:** `e`

Returns the pitch in radians. This is the `y` component.

**Source:** `mathlib/engine/euler.lua`

  ### `RadianEuler.Yaw`
**Kind:** `fn`

**Arguments:** `e`

Returns the yaw in radians. This is the `z` component.

**Source:** `mathlib/engine/euler.lua`

  ### `RadianEuler.Roll`
**Kind:** `fn`

**Arguments:** `e`

Returns the roll in radians. This is the `x` component.

**Source:** `mathlib/engine/euler.lua`


## DegreeEuler" />



Euler angles in degrees. The components have the same order as in `RadianEuler`. `x`
rotates around the X axis, which is the roll. `y` rotates around the Y axis, which is the
pitch. `z` rotates around the Z axis, which is the yaw.

This means that `QAngle(p, y, r)` and `DegreeEuler(r, p, y)` are the same rotation.

The `==` operator compares every component and returns `false` when the other value is not
a `DegreeEuler`.

```cpp lines=false
typedef struct DegreeEuler {
    float x, y, z;
} DegreeEuler;
```


  ### `DegreeEuler.x`
**Kind:** `prop`

The rotation around the X axis in degrees. This is the roll.

**Source:** `mathlib/engine/euler.lua`

  ### `DegreeEuler.y`
**Kind:** `prop`

The rotation around the Y axis in degrees. This is the pitch.

**Source:** `mathlib/engine/euler.lua`

  ### `DegreeEuler.z`
**Kind:** `prop`

The rotation around the Z axis in degrees. This is the yaw.

**Source:** `mathlib/engine/euler.lua`


<Heading level={2} id="degreeeuler-1">
  DegreeEuler

  " />
</Heading>

Call `DegreeEuler()` to create angles with every component set to 0. Call
`DegreeEuler(x, y, z)` to set the components. A component that is not given is set to 0.

You can also pass a `QAngle`, a `Quaternion`, a `QuaternionStorage` or a `RadianEuler` to
convert the same rotation. Pass another `DegreeEuler` to make a copy. Any other first
argument throws an error.


  ### `DegreeEuler()`
**Kind:** `ctor`

Angles with every component set to 0.

**Source:** `mathlib/engine/euler.lua`

  ### `DegreeEuler(x, y, z)`
**Kind:** `ctor`

**Source:** `mathlib/engine/euler.lua`

  ### `DegreeEuler(angles)`
**Kind:** `ctor`

**Source:** `mathlib/engine/euler.lua`

  ### `DegreeEuler(q)`
**Kind:** `ctor`

**Source:** `mathlib/engine/euler.lua`

  ### `DegreeEuler(angles)`
**Kind:** `ctor`

**Source:** `mathlib/engine/euler.lua`

  ### `DegreeEuler(self)`
**Kind:** `ctor`

**Source:** `mathlib/engine/euler.lua`

  ### `DegreeEuler.Unpack`
**Kind:** `fn`

**Arguments:** `e`

Returns the X, Y and Z components.

**Source:** `mathlib/engine/euler.lua`

  ### `DegreeEuler.Copy`
**Kind:** `fn`

**Arguments:** `src, dst`

Copies the components of one value into another value.

**Source:** `mathlib/engine/euler.lua`

  ### `DegreeEuler.Init`
**Kind:** `fn`

**Arguments:** `e, x?, y?, z?`

Sets the components. A component that is not given is set to 0.

**Source:** `mathlib/engine/euler.lua`

  ### `DegreeEuler.IsValid`
**Kind:** `fn`

**Arguments:** `e`

Returns `true` if every component is a finite number. NaN and infinity are not finite.

**Source:** `mathlib/engine/euler.lua`

  ### `DegreeEuler.Invalidate`
**Kind:** `fn`

**Arguments:** `e`

Sets every component to NaN.

**Source:** `mathlib/engine/euler.lua`

  ### `DegreeEuler.ToQAngle`
**Kind:** `fn`

**Arguments:** `e`

Converts the angles to a `QAngle`. The pitch comes from `y`, the yaw comes from `z` and the roll comes from `x`.

**Source:** `mathlib/engine/euler.lua`

  ### `DegreeEuler.ToQuaternion`
**Kind:** `fn`

**Arguments:** `e`

Converts the angles to a `Quaternion`.

**Source:** `mathlib/engine/euler.lua`

  ### `DegreeEuler.RotateVector`
**Kind:** `fn`

**Arguments:** `e, v`

Rotates a vector by these angles. The result is the same as `QAngle.RotateVector` with the converted angle.

**Source:** `mathlib/engine/euler.lua`

  ### `DegreeEuler.Pitch`
**Kind:** `fn`

**Arguments:** `e`

Returns the pitch in degrees. This is the `y` component.

**Source:** `mathlib/engine/euler.lua`

  ### `DegreeEuler.Yaw`
**Kind:** `fn`

**Arguments:** `e`

Returns the yaw in degrees. This is the `z` component.

**Source:** `mathlib/engine/euler.lua`

  ### `DegreeEuler.Roll`
**Kind:** `fn`

**Arguments:** `e`

Returns the roll in degrees. This is the `x` component.

**Source:** `mathlib/engine/euler.lua`


## RotationVector" />



A rotation stored as one vector. The direction of the vector is the axis of the rotation.
The length of the vector is the angle of the rotation in degrees. This is also called an
exponential map. Angular velocities use this type, for example in
`mathlib.CalcAngularVelocity` and in the quaternion version of `mathlib.SmoothDamp`.

You can call every `Vector` method on a `RotationVector`. `Normalized`, `NormalizedSafe` and
`Scaled` return a `RotationVector`. Other `Vector` methods that create a new vector return a
`Vector`.

The `+`, `-`, `*` and `/` operators take a `RotationVector` or a `Vector` on the right side
and return a `RotationVector`. The `*` and `/` operators also take a number. The unary `-`
operator returns a negated copy, and the `#` operator returns the length. The `==` operator
compares every component and returns `false` when the other value is not a
`RotationVector`.

```cpp lines=false
typedef struct RotationVector {
    float x, y, z;
} RotationVector;
```


  ### `RotationVector + b`
**Kind:** `meta`

**Source:** `mathlib/engine/euler.lua`

  ### `RotationVector - b`
**Kind:** `meta`

**Source:** `mathlib/engine/euler.lua`

  ### `RotationVector * b`
**Kind:** `meta`

**Source:** `mathlib/engine/euler.lua`

  ### `RotationVector / b`
**Kind:** `meta`

**Source:** `mathlib/engine/euler.lua`

  ### `-RotationVector`
**Kind:** `meta`

**Source:** `mathlib/engine/euler.lua`

  ### `#RotationVector`
**Kind:** `meta`

**Source:** `mathlib/engine/euler.lua`


<Heading level={2} id="rotationvector-1">
  RotationVector

  " />
</Heading>

Call `RotationVector()` to create a value with every component set to 0. Call
`RotationVector(x, y, z)` to set the components. A component that is not given is set to 0.
Call `RotationVector(v)` with a `Vector` or a `VectorAligned` to copy its X, Y and Z. Call
`RotationVector(other)` to make a copy of another `RotationVector`. Any other first argument
throws an error.

The constant values below cannot be changed.


  ### `RotationVector()`
**Kind:** `ctor`

**Source:** `mathlib/engine/euler.lua`

  ### `RotationVector(x, y, z)`
**Kind:** `ctor`

**Source:** `mathlib/engine/euler.lua`

  ### `RotationVector(v)`
**Kind:** `ctor`

**Source:** `mathlib/engine/euler.lua`

  ### `RotationVector(self)`
**Kind:** `ctor`

**Source:** `mathlib/engine/euler.lua`

  ### `RotationVector.vOne`
**Kind:** `prop`

The value `(1, 1, 1)`.

**Source:** `mathlib/engine/euler.lua`

  ### `RotationVector.vXAxis`
**Kind:** `prop`

The value `(1, 0, 0)`.

**Source:** `mathlib/engine/euler.lua`

  ### `RotationVector.vYAxis`
**Kind:** `prop`

The value `(0, 1, 0)`.

**Source:** `mathlib/engine/euler.lua`

  ### `RotationVector.vZAxis`
**Kind:** `prop`

The value `(0, 0, 1)`.

**Source:** `mathlib/engine/euler.lua`

  ### `RotationVector.vZero`
**Kind:** `prop`

The value `(0, 0, 0)`.

**Source:** `mathlib/engine/euler.lua`

  ### `RotationVector.FromVector`
**Kind:** `fn`

**Arguments:** `rv, v`

Copies the components of a vector into this value.

**Source:** `mathlib/engine/euler.lua`

  ### `RotationVector.Copy`
**Kind:** `fn`

**Arguments:** `src, dst`

Copies the components of one value into another value.

**Source:** `mathlib/engine/euler.lua`

  ### `RotationVector.Normalized`
**Kind:** `fn`

**Arguments:** `rv`

Returns a normalized copy. If the length is 0, the copy is `(0, 0, 0)`.

**Source:** `mathlib/engine/euler.lua`

  ### `RotationVector.NormalizedSafe`
**Kind:** `fn`

**Arguments:** `rv, fallback`

Returns a normalized copy. If the length is 0, a new `RotationVector` with the components of `fallback` is returned.

**Source:** `mathlib/engine/euler.lua`

  ### `RotationVector.Scaled`
**Kind:** `fn`

**Arguments:** `rv, scalar`

Returns a copy with every component multiplied by a number.

**Source:** `mathlib/engine/euler.lua`

---


## Mathlib

<!-- Source: cs2/api_reference/mathlib/mathlib.md -->

---
title: Math Library
icon: sigma
sidebar_position: 1
---

# Math Library

## mathlib _()_

Functions for angles, vectors, rotations, transforms and geometry. When a function returns
several results, they are returned as separate Lua values in the order shown in its signature.

The math types are `Vector`, `Vector2D`, `Vector4D`, `VectorAligned`, `QAngle`, `RadianEuler`,
`DegreeEuler`, `RotationVector`, `Quaternion`, `matrix3x4_t`, `matrix3x4a_t`, `VMatrix`,
`CTransform`, `AABB_t`, `Rect_t`, `VPlane` and `Color`. They follow the rules below.

**Constructors.** If you call a constructor without arguments, `matrix3x4_t`, `matrix3x4a_t`,
`VMatrix`, `CTransform` and `CTransformUnaligned` are set to the identity. `Color()` is black
with an alpha of 255. All other types are filled with zeros. This means that `Quaternion()` has
four zero components and is not the identity rotation, and `AABB_t()` is a box with zero mins and
maxs. Use `AABB_t.Invalid()` to get an empty box. If you pass another value of the same type, the
constructor copies it. If you pass numbers, they fill the components in order and the other
components are set to zero. `Color(r, g, b)` sets the alpha to 255.

**Changing a value or returning a new one.** A method that is described as working "in place"
changes the value it is called on and returns it. A method with a past participle name returns a
new value and does not change the original. For example, `Normalize` changes the vector and
`Normalized` returns a new vector. The same is true for `Scale` and `Scaled`.

`Copy(src, dst)` copies the first argument into the second argument. This means that
`a:Copy(b)` writes the value of `a` into `b`.

**Views.** Methods with names that start with `As` return a copy of the value. There are three
exceptions. `Vector:AsVector2D`, `Vector4D:AsVector3D` and `Vector4D:AsVector2D` return views
that share memory with the original value. Changes to a view also change the original value.

**Fallback values.** `Vector:NormalizedSafe(fallback)` returns the fallback object itself when the
length of the vector is zero. `VectorAligned:NormalizedSafe` returns a copy of the fallback.

**Units.** `QAngle` and `DegreeEuler` use degrees. `RadianEuler` and `RotationVector` use
radians. Quaternion functions give the unit for each parameter.

**Equality.** The default tolerance of `CTransform:IsEqualTo` is 0.00001. The default tolerance
for the rotation is twice the position tolerance. The default tolerances of
`mathlib.TransformsAreEqual` are 0.01 for the position and 0.1 for the rotation.


  ### `mathlib.AngleNormalize`
**Kind:** `fn`

**Arguments:** `degrees`

Wraps an angle in degrees into the range from -180 up to, but not including, 180.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.AngleNormalizePositive`
**Kind:** `fn`

**Arguments:** `degrees`

Wraps an angle in degrees into the range from 0 up to, but not including, 360.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.AngleMod`
**Kind:** `fn`

**Arguments:** `degrees`

The same function as `AngleNormalizePositive`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.AngleDiff`
**Kind:** `fn`

**Arguments:** `dest, src`

Returns the shortest signed difference in degrees from `src` to `dest`. The result is in the range from -180 up to, but not including, 180.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.UTIL_VecToYaw`
**Kind:** `fn`

**Arguments:** `forward`

Returns the yaw of a forward vector in degrees, in the range from 0 up to, but not including, 360. Returns 0 for a vertical vector.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.InterpolateAngles`
**Kind:** `fn`

**Arguments:** `a, b, t`

Interpolates between two sets of angles. The angles are converted to quaternions first, so the result is correct when the angles cross 180 degrees.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QAngleFromForwardVector`
**Kind:** `fn`

**Arguments:** `forward, pseudo_up?`

Returns the angles that look along a forward vector.

Without `pseudo_up`, the pitch and yaw are in the range from 0 up to, but not including, 360,
      and the roll is zero. With `pseudo_up`, the roll is also calculated, and the result is the same
      as the result of `Vector.ToAngle`.
    
  

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QAngleToVectorsFLU`
**Kind:** `fn`

**Arguments:** `angles`

Returns the forward, left and up vectors of a set of angles. The same function as `QAngle.GetLeftVectors`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.TransformAnglesToLocalSpace`
**Kind:** `fn`

**Arguments:** `angles, parent`

Converts angles in world space to angles that are relative to a parent transform.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.TransformAnglesToWorldSpace`
**Kind:** `fn`

**Arguments:** `angles, parent`

Converts angles that are relative to a parent transform to angles in world space.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.VectorLerp`
**Kind:** `fn`

**Arguments:** `a, b, t`

Linearly interpolates between two vectors. For `VectorAligned` values, use `mathlib.Lerp(t, a, b)`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.InterpolateVectors`
**Kind:** `fn`

**Arguments:** `a, b, t`

The same function as `VectorLerp`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.VectorVectorsFLU`
**Kind:** `fn`

**Arguments:** `forward`

Returns the left and up vectors for a forward direction. The same function as `Vector.GetLeftVectors`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.VectorYawRotate`
**Kind:** `fn`

**Arguments:** `v, yaw`

Rotates a vector around the Z axis. The result has the same type as `v`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.VectorPerpendicularToVector`
**Kind:** `fn`

**Arguments:** `v`

Returns a unit vector that is perpendicular to a unit vector. The result changes smoothly as the input gets close to straight up.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.VectorOrthoNormalToVector`
**Kind:** `fn`

**Arguments:** `up, forward`

Removes the part of `up` that points along `forward`, and normalizes the result. If the two vectors are parallel, the function returns the result of `VectorPerpendicularToVector(forward)`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.CalcClosestPointOnLineSegment`
**Kind:** `fn`

**Arguments:** `p, a, b`

Returns the point on the line segment from `a` to `b` that is closest to `p`. The second result is the position of that point along the segment, clamped to the range from 0 to 1.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.CalcClosestPointToLineT`
**Kind:** `fn`

**Arguments:** `p, a, b`

Returns the position along the line through `a` and `b` of the point that is closest to `p`. The value is 0 at `a` and 1 at `b`, and it is not clamped. The second result is the vector `b - a`. If `a` and `b` are almost at the same position, the position is 0.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.PointWithinViewAngle`
**Kind:** `fn`

**Arguments:** `src, target, look, cos_half_fov`

Returns true if `target` is inside the view cone of a camera.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.Approach`
**Kind:** `fn`

**Arguments:** `target, value, speed`

Moves `value` toward `target` by a distance of no more than `speed`. If `target` is closer than `speed`, the function returns `target`. Numbers move along the number line. Vectors move along the straight line between the two points.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.RemapVal`
**Kind:** `fn`

**Arguments:** `val, a, b, c, d`

Maps `val` from the range `a` to `b` to the range `c` to `d`. The result is not clamped. If `a` equals `b`, the function returns `d` when `val` is greater than or equal to `b`, and `c` otherwise.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.RemapValClamped`
**Kind:** `fn`

**Arguments:** `val, a, b, c, d`

Maps `val` from the range `a` to `b` to the range `c` to `d`. The result is clamped to the range `c` to `d`. If `a` equals `b`, the function returns `d` when `val` is greater than or equal to `b`, and `c` otherwise.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.RemapIntVal`
**Kind:** `fn`

**Arguments:** `val, a, b, c, d`

Maps an integer from the range `a` to `b` to the range `c` to `d`. The distance from `c` is truncated toward zero before it is added to `c`. The result is not clamped.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.RemapIntValClamped`
**Kind:** `fn`

**Arguments:** `val, a, b, c, d`

The same as `RemapIntVal`, but the result is clamped to the range `c` to `d`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.SimpleSpline`
**Kind:** `fn`

**Arguments:** `t`

Returns the Hermite ease curve `t * t * (3 - 2 * t)`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.SimpleSplineRemapVal`
**Kind:** `fn`

**Arguments:** `val, a, b, c, d`

The same as `RemapVal`, but the position in the input range is eased with `SimpleSpline`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.SimpleSplineRemapValClamped`
**Kind:** `fn`

**Arguments:** `val, a, b, c, d`

The same as `SimpleSplineRemapVal`, but the result is clamped to the range `c` to `d`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.FLerp`
**Kind:** `fn`

**Arguments:** `f1, f2, i1, i2, x`

Returns the value at `x` of the line that goes through the points `(i1, f1)` and `(i2, f2)`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.Lerp`
**Kind:** `fn`

**Arguments:** `percent, a, b`

Linear interpolation that returns `a + (b - a) * percent`. The interpolation factor is the first argument. Works with numbers and with all types that support `+`, `-` and multiplication by a number.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.Snap`
**Kind:** `fn`

**Arguments:** `value, step`

Rounds a value to the nearest multiple of `step`. Vectors are rounded one component at a time.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.Sign`
**Kind:** `fn`

**Arguments:** `x`

Returns 1 for zero and positive values, and -1 for negative values.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.WrapInt`
**Kind:** `fn`

**Arguments:** `x, limit`

Wraps an integer into the range from 0 up to, but not including, `limit`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.Clamp`
**Kind:** `fn`

**Arguments:** `value, min, max`

Clamps a number between two numbers. For vectors, each component is clamped between the same components of `min` and `max`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.SmoothDamp`
**Kind:** `fn`

**Arguments:** `current, target, velocity, smooth_time, dt`

Moves a value toward `target` with a critically damped spring.

Numbers are passed by value, so the new velocity is returned as the second result. Save it and
      pass it to the next call. If `smooth_time` is 0 or less, the function returns `target` and the
      old velocity. If `dt` is 0 or less, the function returns `current` and the old velocity.
    
  

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.SmoothDamp`
**Kind:** `fn`

**Arguments:** `current, target, velocity, smooth_time, dt`

The vector version of `SmoothDamp`. The function changes `velocity` in place and returns the new value as a new object.

If `smooth_time` is 0 or less, the function returns a copy of `target`. If `dt` is 0 or less, the
      function returns a copy of `current`. In both cases `velocity` does not change.
    
  

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.SmoothDamp`
**Kind:** `fn`

**Arguments:** `current, target, velocity, smooth_time, dt`

The quaternion version of `SmoothDamp`. The angular velocity is a `RotationVector` in degrees per second, and the function changes it in place. The returned orientation is normalized.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.CalcAngularVelocity`
**Kind:** `fn`

**Arguments:** `q_end, q_begin, dt`

Estimates the angular velocity between two orientations, in degrees per second. Accepts two quaternions or two `QAngle` values.

The estimate is accurate for small changes in rotation. Use it with consecutive orientations,
      because large changes give less accurate results.
    
  

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.CalcAngularVelocityForForwardDirectionRotation`
**Kind:** `fn`

**Arguments:** `v_end, v_start, dt, max_speed?`

Estimates the angular velocity between two directions, in degrees per second. The result can be limited to a maximum speed.

The axis is the cross product of the two normalized directions, and the angle is the arcsine of
      the length of the cross product. Because of this, a turn of more than 90 degrees gives a smaller
      angle, and two opposite directions give zero. Use it with small changes between consecutive
      directions.
    
  

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.Grassia`
**Kind:** `fn`

**Arguments:** `v`

Converts a rotation vector in radians to a unit quaternion. The direction of the vector is the rotation axis, and its length is the rotation angle.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.GrassiaExp`
**Kind:** `fn`

**Arguments:** `v`

The same as `Grassia`, but the result is an aligned `Quaternion`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.Bias`
**Kind:** `fn`

**Arguments:** `x, amount`

Perlin bias. Pushes a value from 0 to 1 toward 0 or toward 1. `x` is clamped to the range from 0 to 1.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.Gain`
**Kind:** `fn`

**Arguments:** `x, amount`

Perlin gain, built from `Bias`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.Bias_Perlin`
**Kind:** `fn`

**Arguments:** `x, amount`

Bias that uses a power curve. `x` and `amount` are clamped to the range from 0 to 1.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.Gain_Perlin`
**Kind:** `fn`

**Arguments:** `x, amount`

Gain built from `Bias_Perlin`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.SmoothCurve`
**Kind:** `fn`

**Arguments:** `x`

Returns the smooth curve `(1 - cos(x * pi)) / 2`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.AngleQuaternion`
**Kind:** `fn`

**Arguments:** `angles`

Returns a quaternion from angles. The same as the `Quaternion` constructor.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.AxisAngleQuaternion`
**Kind:** `fn`

**Arguments:** `axis, degrees`

Returns a quaternion that rotates around a unit axis by an angle in degrees.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.AxisAngleRadiansQuaternion`
**Kind:** `fn`

**Arguments:** `axis, radians`

Returns a quaternion that rotates around a unit axis by an angle in radians.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.BasisToQuaternionFLU`
**Kind:** `fn`

**Arguments:** `forward, left, up`

Returns a quaternion from three orthonormal vectors that point forward, left and up.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionSlerp`
**Kind:** `fn`

**Arguments:** `p, q, t`

An approximation of spherical interpolation along the shorter arc. The result has the same quaternion type as `p`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionBlend`
**Kind:** `fn`

**Arguments:** `p, q, t`

Aligns `q` to `p` with `QuaternionAlign`, interpolates linearly and normalizes the result.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionBlendNoAlign`
**Kind:** `fn`

**Arguments:** `p, q, t`

Interpolates linearly and normalizes the result. Unlike `QuaternionBlend`, this function does not align `q` to `p` first.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionAlign`
**Kind:** `fn`

**Arguments:** `p, q`

Returns `q` or `-q`, whichever is closer to `p`. Both represent the same rotation.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionMult`
**Kind:** `fn`

**Arguments:** `p, q`

Returns the Hamilton product. The same as `p * q`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionDiff`
**Kind:** `fn`

**Arguments:** `q_to, q_from`

Returns the rotation that turns `q_from` into `q_to`. This is `q_to` multiplied by the conjugate of `q_from`, with the conjugate aligned to `q_to`. The result is normalized.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionAngleDiff`
**Kind:** `fn`

**Arguments:** `p, q`

Returns the angle between two orientations in degrees.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionAxisAngle`
**Kind:** `fn`

**Arguments:** `q`

Returns the rotation axis of a quaternion and the rotation angle in degrees.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionLookAt`
**Kind:** `fn`

**Arguments:** `forward, reference_up`

Returns an orientation that looks along `forward`, with its up direction as close as possible to `reference_up`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionFromForwardVector`
**Kind:** `fn`

**Arguments:** `forward`

Returns an orientation that looks along `forward`, with the world Z axis as the up direction. If `forward` is vertical, the X axis is used as the up direction.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionIdentityBlend`
**Kind:** `fn`

**Arguments:** `p, t`

Blends a quaternion toward the identity. When `t` is 0, the result is `p`. When `t` is 1, the result is the identity.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionLn`
**Kind:** `fn`

**Arguments:** `p`

Returns the quaternion logarithm.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionExp`
**Kind:** `fn`

**Arguments:** `p`

Returns the quaternion exponential. This is the inverse of `QuaternionLn`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionAverageExponential`
**Kind:** `fn`

**Arguments:** `list, weights?`

Returns the weighted average of a list of quaternions. The average is calculated in log space.

`list` is a Lua array of quaternions. `weights` is an optional Lua array of numbers, with one
      weight for each quaternion. If `list` is empty, the function throws an error.
    
  

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.BuildRotateBetweenVectorsQuaternion`
**Kind:** `fn`

**Arguments:** `from, to`

Returns the rotation that turns one unit vector into another. If the vectors point in opposite directions, the result is a half turn around an axis that is perpendicular to `from`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.ShortestArc`
**Kind:** `fn`

**Arguments:** `v1, v2`

The same function as `BuildRotateBetweenVectorsQuaternion`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.SplineQuaternions`
**Kind:** `fn`

**Arguments:** `a, b, t`

Interpolates between two quaternions along a smooth rotation path. The result has the same quaternion type as `a`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.Exp`
**Kind:** `fn`

**Arguments:** `v`

The exponential map for an axis and angle in radians. `Exp(v)` returns a rotation around `v` by twice the length of `v`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.Log`
**Kind:** `fn`

**Arguments:** `q`

The inverse of `Exp`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.AngleMatrix`
**Kind:** `fn`

**Arguments:** `angles, position?, scale?`

Returns a rotation matrix from angles. You can also give a position and a scale. A number scales all axes by the same amount. A `Vector` scales each axis by its own component.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.AngleIMatrix`
**Kind:** `fn`

**Arguments:** `angles, position?`

Returns the inverse of the matrix that `AngleMatrix` returns.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixAngles`
**Kind:** `fn`

**Arguments:** `matrix`

Returns the angles, the position and the scale of a matrix. The scale is the length of the X basis column.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionMatrix`
**Kind:** `fn`

**Arguments:** `q, position?, scale?`

Returns a rotation matrix from a quaternion. You can also give a position and a scale. A number scales all axes by the same amount. A `Vector` scales each axis by its own component. The result is always a `matrix3x4_t`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixQuaternion`
**Kind:** `fn`

**Arguments:** `matrix`

Returns the quaternion, the position and the scale of a matrix. The scale is the length of the X basis column.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixPosition`
**Kind:** `fn`

**Arguments:** `matrix`

Returns the origin column of a matrix.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.PositionMatrix`
**Kind:** `fn`

**Arguments:** `position`

Returns a matrix with no rotation and the given origin.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixBuildRotationAboutAxis`
**Kind:** `fn`

**Arguments:** `axis, degrees`

Returns a matrix that rotates around a unit axis by an angle in degrees. Pass the result to `VMatrix(...)` to get a 4x4 matrix.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.ConcatTransforms`
**Kind:** `fn`

**Arguments:** `a, b`

Returns `a * b`. The right operand `b` is applied first.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.ConcatRotations`
**Kind:** `fn`

**Arguments:** `a, b`

Returns `a * b` with the origin set to zero.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixInvert`
**Kind:** `fn`

**Arguments:** `matrix`

Returns the inverse of a matrix that has a rotation, a translation and the same scale on all axes. The scale is taken from the X basis column.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixInvertTR`
**Kind:** `fn`

**Arguments:** `matrix`

Returns the inverse of a matrix that has only a rotation and a translation. The same as `matrix3x4_t.InverseTR`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixInvertGeneral`
**Kind:** `fn`

**Arguments:** `matrix`

Returns the inverse of any 3x4 matrix. If the matrix cannot be inverted, the first result is false and the second result is the identity matrix.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixTranspose`
**Kind:** `fn`

**Arguments:** `matrix`

Returns the transposed rotation with the origin set to zero. The same as `matrix3x4_t.Transposed`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixNormalize`
**Kind:** `fn`

**Arguments:** `matrix`

Returns the scale of each basis column, and then the matrix with basis columns of length 1. The origin does not change.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixOrthogonalize`
**Kind:** `fn`

**Arguments:** `matrix`

Makes the basis columns orthonormal with the Gram-Schmidt process. The origin does not change.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.ComputeMatrixScale`
**Kind:** `fn`

**Arguments:** `matrix`

Returns the lengths of the three basis columns.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.ComputeMatrixMaxScale`
**Kind:** `fn`

**Arguments:** `matrix`

Returns the length of the longest basis column.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.TransformSlerp`
**Kind:** `fn`

**Arguments:** `p, q, t`

Interpolates between two transforms. The orientation is interpolated with `QuaternionSlerp`. The position and the scale are interpolated linearly.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.Slerp`
**Kind:** `fn`

**Arguments:** `p, q, t`

The same function as `TransformSlerp`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.TransformLerp`
**Kind:** `fn`

**Arguments:** `p, q, t`

Interpolates between two transforms. The function aligns the signs of the two quaternions, blends them linearly and normalizes the result. The position and the scale are interpolated linearly.

If the blended quaternion has a length of zero, the identity is used. `t` is not clamped.
    
  

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.TransformDeltaNoScale`
**Kind:** `fn`

**Arguments:** `from, to`

Returns the change from one transform to another, without the scale. The orientation is the orientation of `to` multiplied by the inverse orientation of `from`. The origin is the vector from the origin of `from` to the origin of `to`, rotated by the inverse orientation of `from`. The scale is 1.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.TransformsAreEqual`
**Kind:** `fn`

**Arguments:** `a, b, position_tolerance?, rotation_tolerance?`

Returns true if two transforms are equal within a tolerance. The positions are compared with an absolute tolerance. The scales and orientations are compared with a tolerance on the mantissa, in the same way as `FloatsAreEqual`. A quaternion and its negative are treated as equal.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.AxisAngleTransform`
**Kind:** `fn`

**Arguments:** `axis, degrees`

Returns a transform that rotates around a unit axis by an angle in degrees. The position is the origin and the scale is 1.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.SetupMatrixIdentity`
**Kind:** `fn`

Returns a new identity `VMatrix`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixSetIdentity`
**Kind:** `fn`

Returns a new identity `VMatrix`. The same function as `SetupMatrixIdentity`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixInverseGeneral`
**Kind:** `fn`

**Arguments:** `vm`

Returns the inverse of a 4x4 matrix. If the matrix cannot be inverted, or is too close to that for single precision numbers, the first result is false and the second result is the identity matrix. Use `MatrixInvertGeneral` for 3x4 matrices.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.SetupMatrixScale`
**Kind:** `fn`

**Arguments:** `scale`

Returns a scale matrix.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.SetupMatrixTranslation`
**Kind:** `fn`

**Arguments:** `translation`

Returns a translation matrix.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.SetupMatrixAxisRot`
**Kind:** `fn`

**Arguments:** `axis, degrees`

Returns a matrix that rotates around a unit axis by an angle in degrees.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.SetupMatrixAxisToAxisRot`
**Kind:** `fn`

**Arguments:** `from, to`

Returns a matrix that rotates one unit vector onto another. If the vectors are parallel, the function returns the identity matrix.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.SetupMatrixAngles`
**Kind:** `fn`

**Arguments:** `angles`

Returns a rotation matrix from angles.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.SetupMatrixOrgAngles`
**Kind:** `fn`

**Arguments:** `origin, angles`

Returns a matrix with a rotation from angles and a translation to an origin.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixFromAngles`
**Kind:** `fn`

**Arguments:** `angles`

The same function as `SetupMatrixAngles`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixToAngles`
**Kind:** `fn`

**Arguments:** `vm`

Returns the angles of the upper 3x3 part of a `VMatrix`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixBuildPerspective`
**Kind:** `fn`

**Arguments:** `fov_x, fov_y, z_near, z_far`

Returns a perspective projection matrix that looks along +Z.

After `V3Mul`, points inside the view frustum are mapped to the range from 0 to 1 on each axis.
      The +X and +Y directions of the view space map toward 0. The near plane maps to z = 0 and the far
      plane maps to z = 1.
    
  

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.AlmostEqual`
**Kind:** `fn`

**Arguments:** `a, b, max_ulps?`

Compares two numbers or two vectors by their distance in units in the last place of their 32 bit float representation. Returns false if a value is NaN. Values with different signs are equal only if they are exactly equal.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.FloatsAreEqual`
**Kind:** `fn`

**Arguments:** `a, b, tolerance`

Compares the mantissas of two numbers after their exponents are aligned. The tolerance is looser for numbers close to zero.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.FloatsAreWithinTolerance`
**Kind:** `fn`

**Arguments:** `a, b, tolerance`

Returns true if `|a - b| <= tolerance`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.CloseEnough`
**Kind:** `fn`

**Arguments:** `a, b, epsilon?`

Returns true if `|a - b| <= epsilon`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionsAreEqual`
**Kind:** `fn`

**Arguments:** `a, b, tolerance`

Compares the components of two quaternions with `FloatsAreEqual`. A quaternion and its negative are treated as equal.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatricesAreEqual`
**Kind:** `fn`

**Arguments:** `a, b, tolerance?`

Compares two matrices element by element with an absolute tolerance. For 3x4 matrices, every element is compared. For `VMatrix` values, only the upper 3x3 part is compared.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.RotationsAreEqual`
**Kind:** `fn`

**Arguments:** `a, b, tolerance?`

Compares the upper 3x3 parts of two 3x4 matrices with an absolute tolerance.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.GetRelativeDifferenceSqr`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the squared relative difference `|a - b|² / max(1, |a|², |b|²)`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.GetRelativeDifference`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the square root of `GetRelativeDifferenceSqr`. For matrices, the function returns the largest value of the three basis columns and the origin.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.VectorVectorsFLUPreserveScale`
**Kind:** `fn`

**Arguments:** `forward`

Returns the left and up vectors for a forward direction. Their length is the length of `forward`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.BuildBasisFLU`
**Kind:** `fn`

**Arguments:** `dir`

Returns the normalized direction and orthonormal left and up vectors. The left and up vectors are built from the world X axis, or from the world Y axis if the direction is almost parallel to X.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MakeOrthonormalBasis`
**Kind:** `fn`

**Arguments:** `n`

Returns two unit vectors that are orthonormal to a unit normal. The result changes smoothly for all directions of the normal.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.VectorMatrix`
**Kind:** `fn`

**Arguments:** `forward`

Returns a rotation matrix with `forward` as its X column and the origin at zero. `forward` is not normalized. The Y and Z columns have a length of 1.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionVectorsFLU`
**Kind:** `fn`

**Arguments:** `q`

Returns the forward, left and up vectors of a quaternion.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionScale`
**Kind:** `fn`

**Arguments:** `p, t`

Returns a fractional power of a unit quaternion. The result rotates around the same axis by `t` times the angle.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionMultAligned`
**Kind:** `fn`

**Arguments:** `p, q`

Returns `p * QuaternionAlign(p, q)`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionSM`
**Kind:** `fn`

**Arguments:** `s, p, q`

Returns `(p ^ s) * q`, normalized. `q` is aligned to `p ^ s` before the multiplication.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionMA`
**Kind:** `fn`

**Arguments:** `p, s, q`

Returns `p * (q ^ s)`, normalized. `q ^ s` is aligned to `p` before the multiplication.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.CalcAngleAboutAxis`
**Kind:** `fn`

**Arguments:** `src, dst, axis`

Returns the signed angle in radians from `src` to `dst` around a unit axis.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.CalcRotationBetweenVectors`
**Kind:** `fn`

**Arguments:** `src, dst`

The same function as `BuildRotateBetweenVectorsQuaternion`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.CalcRotationAboutAxis`
**Kind:** `fn`

**Arguments:** `src, dst, axis`

Returns a rotation around `axis` by the angle `CalcAngleAboutAxis(src, dst, axis)`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.QuaternionNormalize`
**Kind:** `fn`

**Arguments:** `q`

Normalizes a quaternion in place and returns its old length. A quaternion with a length of zero becomes the identity, and the function returns 1.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixBuildRotation`
**Kind:** `fn`

**Arguments:** `initial, final`

Returns a matrix that rotates one unit vector onto another.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixBuildRotateZ`
**Kind:** `fn`

**Arguments:** `degrees`

Returns a matrix that rotates around the Z axis by an angle in degrees.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixTranslate`
**Kind:** `fn`

**Arguments:** `vm, translation`

Returns `vm` multiplied by a translation matrix, as a new matrix.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixRotate`
**Kind:** `fn`

**Arguments:** `vm, axis, degrees`

Returns `vm` multiplied by a rotation around `axis`, as a new matrix.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixInverseTranspose`
**Kind:** `fn`

**Arguments:** `m`

Returns the inverse transpose of a matrix. This is the matrix that transforms normals. The result has the same type as `m`.

If the matrix cannot be inverted, the function returns the identity matrix and does not tell you
      that it failed. Use `InverseGeneral` if you need to know whether the inversion worked.
    
  

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixGetRow`
**Kind:** `fn`

**Arguments:** `vm, row`

Returns the first three elements of a row.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixSetRow`
**Kind:** `fn`

**Arguments:** `vm, row, v`

Sets the first three elements of a row.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixGetColumn`
**Kind:** `fn`

**Arguments:** `vm, col`

Returns the first three elements of a column.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixSetColumn`
**Kind:** `fn`

**Arguments:** `vm, col, v`

Sets the first three elements of a column.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.Vector3DMultiplyProjective`
**Kind:** `fn`

**Arguments:** `vm, v`

Transforms a direction by a projective matrix. The direction is rotated and then divided by the projected w. The translation is not applied.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.Vector3DMultiplyPositionProjective`
**Kind:** `fn`

**Arguments:** `vm, v`

Transforms a point by a projective matrix, with the perspective divide. The same as `VMatrix.V3Mul`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.ClampCycle`
**Kind:** `fn`

**Arguments:** `cycle, looping`

If `looping` is true, wraps the cycle into the range from 0 up to, but not including, 1. If `looping` is false, clamps the cycle to the range from 0 to 0.999.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.Dir`
**Kind:** `fn`

**Arguments:** `f`

Returns -1 for negative values, and 1 for zero and positive values.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.SameSign`
**Kind:** `fn`

**Arguments:** `a, b`

Returns true if both values have the same sign. Zero counts as positive.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.PlaneIntersection`
**Kind:** `fn`

**Arguments:** `p1, p2, p3`

Returns the point where three planes meet. If two or more of the planes are parallel, the first result is false and the second result is `nil`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.SetupMatrixReflection`
**Kind:** `fn`

**Arguments:** `plane`

Returns a matrix that reflects points across a plane.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.SetupMatrixProjection`
**Kind:** `fn`

**Arguments:** `origin, plane`

Returns a matrix that projects points from an origin onto a plane. The result is a projective matrix, so apply it with `V3Mul`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixTransformPlane`
**Kind:** `fn`

**Arguments:** `m, plane`

Transforms a plane by a matrix. The same as the `TransformPlane` method of the matrix.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.MatrixITransformPlane`
**Kind:** `fn`

**Arguments:** `m, plane`

Transforms a plane by the inverse of a 3x4 matrix. The same as the `TransformPlaneByInverse` method of the matrix.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.FrustumPlanesFromMatrix`
**Kind:** `fn`

**Arguments:** `clip_to_world`

Returns the six planes of a clip to world matrix as a `Frustum_t`. The plane normals point inward. The planes are in the order right, left, top, bottom, near and far.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.BoxSurfaceArea`
**Kind:** `fn`

**Arguments:** `mins, maxs`

Returns the total area of the six faces of a box.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.ComputeVolume`
**Kind:** `fn`

**Arguments:** `mins, maxs`

Returns the volume of an axis aligned box.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.CalcSqrDistanceToAABB`
**Kind:** `fn`

**Arguments:** `mins, maxs, point`

Returns the squared distance from a point to a box. Returns 0 if the point is inside the box.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.CalcDistanceToAABB`
**Kind:** `fn`

**Arguments:** `mins, maxs, point`

Returns the distance from a point to a box. Returns 0 if the point is inside the box.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.CalcSqrDistanceToAABB2D`
**Kind:** `fn`

**Arguments:** `mins, maxs, point`

Returns the squared distance from a 2D point to a 2D box. Returns 0 if the point is inside the box.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.CalcDistanceToAABB2D`
**Kind:** `fn`

**Arguments:** `mins, maxs, point`

Returns the distance from a 2D point to a 2D box. Returns 0 if the point is inside the box.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.ClosestPointOnAABB`
**Kind:** `fn`

**Arguments:** `mins, maxs, point`

Returns the point in a box that is closest to a given point.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.BoundsFromPointList`
**Kind:** `fn`

**Arguments:** `points`

Returns the bounds of a Lua array of points. If the array is empty, the function returns an empty box, the same as `AABB_t.Invalid()`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.BoundsFromPointList`
**Kind:** `fn`

**Arguments:** `xform, points`

Returns the bounds of a Lua array of points after each point is transformed by `xform`. If the array is empty, the function returns an empty box, the same as `AABB_t.Invalid()`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.GetAabb`
**Kind:** `fn`

**Arguments:** `points`

The same as `BoundsFromPointList(points)`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.GetAabb`
**Kind:** `fn`

**Arguments:** `xform, points`

The same as `BoundsFromPointList(xform, points)`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.TransformAABB`
**Kind:** `fn`

**Arguments:** `xform, bounds`

Transforms a box by a matrix or a transform. The same as the `TransformAABB` method of `xform`.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.BezierEvaluate`
**Kind:** `fn`

**Arguments:** `points, t`

Returns the point at `t` on a Bezier curve of any order. The function uses de Casteljau's algorithm. The control points are a Lua array of any vector type. The order of the curve is the number of control points minus one.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.BezierTangent`
**Kind:** `fn`

**Arguments:** `points, t`

Returns the derivative of the curve at `t`. This is the curve of one order lower through the points `order * (p[i + 1] - p[i])`. If there are fewer than two control points, the function throws an error.

**Source:** `mathlib/engine/mathlib.lua`

  ### `mathlib.GetProjectedAABBBounds`
**Kind:** `fn`

**Arguments:** `proj_from_bounds, bounds`

Returns the bounds of a box after it is projected by a matrix and divided by w. The bounds are in the output coordinates of the matrix. For a usual clip space projection, x and y are in the range from -1 to 1. For `MatrixBuildPerspective`, they are in the range from 0 to 1.

The near plane is at clip space z = 0. Corners behind the near plane are not used, and the edges
      of the box are clipped at the near plane. If the whole box is behind the near plane, the function
      returns `AABB_t.Invalid()`.
    
  

**Source:** `mathlib/engine/mathlib.lua`

---


## Matrix3X4

<!-- Source: cs2/api_reference/mathlib/matrix3x4.md -->

---
title: Matrix 3x4
icon: grid-3x3
---

# Matrix 3x4

## matrix3x4\_t" />



A 3x4 affine matrix stored row by row. The first three columns hold the rotation. They are the
forward, left and up axes. The fourth column holds the origin.

`matrix3x4a_t` is the form aligned to 16 bytes, with the same layout and the same methods. Bone
and attachment transforms use that form.

Read or write elements with `m.m_flMatVal[row][col]`. Indices start at zero. Call
`matrix3x4_t()` or `matrix3x4a_t()` to create a new identity matrix.

```cpp lines=false
typedef struct matrix3x4_t {
    float m_flMatVal[3][4];
} matrix3x4_t;
```

The `==` and `~=` operators compare every element exactly. `tostring` returns all twelve
elements.

The `*` operator with a matrix on the right returns the product, which applies the right matrix
first. With a `Vector` on the right, it transforms the vector as a point, in the same way as
`TransformVector`. The `+` and `-` operators work on each element. The results of the operators
have the type of the left operand.


  ### `matrix3x4_t.m_flMatVal`
**Kind:** `prop`

The elements, stored as 3 rows of 4 numbers. Indices start at zero.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t * b`
**Kind:** `meta`

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t * b`
**Kind:** `meta`

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t + b`
**Kind:** `meta`

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t - b`
**Kind:** `meta`

**Source:** `mathlib/engine/matrix3x4.lua`


<Heading level={2} id="matrix3x4_t-1">
  matrix3x4_t

  " />
</Heading>

The constructor without arguments creates an identity matrix. If a position is not given, the
origin is set to zero. Any other argument type throws an error.


  ### `matrix3x4_t()`
**Kind:** `ctor`

Creates an identity matrix.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t(m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23)`
**Kind:** `ctor`

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t(x_axis, y_axis, z_axis, origin)`
**Kind:** `ctor`

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t(q, position)`
**Kind:** `ctor`

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t(angles, position)`
**Kind:** `ctor`

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t(transform)`
**Kind:** `ctor`

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t(m)`
**Kind:** `ctor`

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.Copy`
**Kind:** `fn`

**Arguments:** `src, dst`

Copies every element of `src` into `dst`.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.Init`
**Kind:** `fn`

**Arguments:** `m, m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23`

Sets all twelve elements in place. Pass the elements row by row.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.InitXYZ`
**Kind:** `fn`

**Arguments:** `m, x_axis, y_axis, z_axis, origin`

Sets the matrix in place from three axis columns and an origin column.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.InitFLU`
**Kind:** `fn`

**Arguments:** `m, forward, left, up, origin`

Sets the matrix in place from the forward, left and up axes and an origin. This is the same as `InitXYZ`, because the forward, left and up axes are the X, Y and Z axes.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.InitFromQAngles`
**Kind:** `fn`

**Arguments:** `m, angles, position?`

Sets the rotation in place from a `QAngle` and sets the origin to `position`.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.InitFromRadianEuler`
**Kind:** `fn`

**Arguments:** `m, angles, position?`

Sets the rotation in place from a `RadianEuler` and sets the origin to `position`.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.InitFromQuaternion`
**Kind:** `fn`

**Arguments:** `m, q, position?`

Sets the rotation in place from a quaternion and sets the origin to `position`. The quaternion must have a length of 1 to give a pure rotation.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.InitFromCTransform`
**Kind:** `fn`

**Arguments:** `m, transform`

Sets the matrix in place from a transform. The rotation part is multiplied by the uniform scale of the transform, and the origin is set to the position of the transform.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.InitFromDiagonal`
**Kind:** `fn`

**Arguments:** `m, diagonal`

Sets the matrix in place to a scale matrix. The diagonal elements are set to the components of `diagonal`, and every other element is set to zero.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.SetToZero`
**Kind:** `fn`

**Arguments:** `m`

Sets every element to zero.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.SetToIdentity`
**Kind:** `fn`

**Arguments:** `m`

Sets the matrix to the identity matrix with a zero origin.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.Invalidate`
**Kind:** `fn`

**Arguments:** `m`

Sets every element to NaN. After this call, `IsValid` returns false.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.IsValid`
**Kind:** `fn`

**Arguments:** `m`

Returns true if no element is NaN or infinite.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.GetOrigin`
**Kind:** `fn`

**Arguments:** `m`

Returns the fourth column, which is the origin.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.SetOrigin`
**Kind:** `fn`

**Arguments:** `m, origin`

Sets the fourth column, which is the origin.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.GetColumn`
**Kind:** `fn`

**Arguments:** `m, column`

Returns a column by its index. Use the values of `MatrixAxisType_t` for the index.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.SetColumn`
**Kind:** `fn`

**Arguments:** `m, vec, column`

Sets a column by its index. Use the values of `MatrixAxisType_t` for the index. The vector comes before the index in the argument list.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.GetForward`
**Kind:** `fn`

**Arguments:** `m`

Returns the first column, which is the forward axis.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.GetLeft`
**Kind:** `fn`

**Arguments:** `m`

Returns the second column, which is the left axis.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.GetUp`
**Kind:** `fn`

**Arguments:** `m`

Returns the third column, which is the up axis.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.GetRow`
**Kind:** `fn`

**Arguments:** `m, row`

Returns the first three elements of a row. The fourth element of the row is left out.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.SetRow`
**Kind:** `fn`

**Arguments:** `m, row, vec`

Sets the first three elements of a row. The fourth element of the row does not change.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.GetBasisVectorsFLU`
**Kind:** `fn`

**Arguments:** `m`

Returns the forward, left and up axes, which are the first three columns.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.ToQuaternion`
**Kind:** `fn`

**Arguments:** `m`

Converts the rotation part to a `Quaternion`. The first three columns must have a length of 1 and be at right angles to each other.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.ToQAngle`
**Kind:** `fn`

**Arguments:** `m`

Converts the rotation part to a `QAngle` in degrees. If the forward axis points almost straight up or down, the roll is set to 0.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.ToCTransform`
**Kind:** `fn`

**Arguments:** `m`

Converts the matrix to a transform. The first three columns are normalized before the rotation is computed. The scale of the transform is the length of the longest of these columns.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.ScaleUpper3x3Matrix`
**Kind:** `fn`

**Arguments:** `m, scale`

Multiplies the first three columns by a number in place. The origin does not change.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.TransformVector`
**Kind:** `fn`

**Arguments:** `m, vec`

Transforms a point. The point is rotated by the first three columns, and then the origin is added.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.RotateVector`
**Kind:** `fn`

**Arguments:** `m, vec`

Rotates a direction by the first three columns. The origin is not added.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.TransformVectorByInverse`
**Kind:** `fn`

**Arguments:** `m, vec`

Applies the inverse transform to a point. The origin is subtracted, and then the point is rotated by the transposed rotation part. The result is correct only if the first three columns have a length of 1 and are at right angles to each other.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.RotateVectorByInverse`
**Kind:** `fn`

**Arguments:** `m, vec`

Rotates a direction by the transposed rotation part, which is the inverse rotation. The result is correct only if the first three columns have a length of 1 and are at right angles to each other.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.RotateExtents`
**Kind:** `fn`

**Arguments:** `m, extents`

Rotates a box that is centered on the origin, given by its half size on each axis. Returns the half size of the smallest axis aligned box that contains the rotated box. Every component of the result is zero or greater.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.TransformAABB`
**Kind:** `fn`

**Arguments:** `m, mins, maxs`

Transforms an axis aligned box by the rotation and the origin. Returns the smallest axis aligned box that contains the transformed box.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.TransformAABB`
**Kind:** `fn`

**Arguments:** `m, bounds`

Transforms a box given as an `AABB_t` and returns the result as an `AABB_t`. It works in the same way as the version that takes `mins` and `maxs`.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.TransformAABBByInverse`
**Kind:** `fn`

**Arguments:** `m, mins, maxs`

Transforms an axis aligned box by the inverse transform. Returns the smallest axis aligned box that contains the transformed box. The result is correct only if the first three columns have a length of 1 and are at right angles to each other.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.TransformAABBByInverse`
**Kind:** `fn`

**Arguments:** `m, bounds`

Transforms a box given as an `AABB_t` by the inverse transform and returns the result as an `AABB_t`. It works in the same way as the version that takes `mins` and `maxs`.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.RotateAABB`
**Kind:** `fn`

**Arguments:** `m, mins, maxs`

Rotates an axis aligned box by the rotation part. The origin is not added. Returns the smallest axis aligned box that contains the rotated box.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.RotateAABB`
**Kind:** `fn`

**Arguments:** `m, bounds`

Rotates a box given as an `AABB_t` and returns the result as an `AABB_t`. It works in the same way as the version that takes `mins` and `maxs`.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.RotateAABBByInverse`
**Kind:** `fn`

**Arguments:** `m, mins, maxs`

Rotates an axis aligned box by the transposed rotation part, which is the inverse rotation. The origin is not used. Returns the smallest axis aligned box that contains the rotated box.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.RotateAABBByInverse`
**Kind:** `fn`

**Arguments:** `m, bounds`

Rotates a box given as an `AABB_t` by the inverse rotation and returns the result as an `AABB_t`. It works in the same way as the version that takes `mins` and `maxs`.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.GetOrthogonalityError`
**Kind:** `fn`

**Arguments:** `m`

Returns the sum of the absolute dot products between each pair of the first three columns. The result is 0 if the columns are at right angles to each other.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.GetDeterminant`
**Kind:** `fn`

**Arguments:** `m`

Returns the determinant of the 3x3 rotation part.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.GetTrace`
**Kind:** `fn`

**Arguments:** `m`

Returns the trace of the 3x3 rotation part, which is the sum of its diagonal elements.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.Transposed`
**Kind:** `fn`

**Arguments:** `m`

Returns a new matrix with the 3x3 rotation part transposed and the origin set to zero. The result has the same type as `m`.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.InverseTR`
**Kind:** `fn`

**Arguments:** `m`

Returns the inverse of a matrix that contains only a rotation and a translation. The rotation part is transposed, and the origin is rotated by the transposed part and negated. The result is wrong if the matrix contains scale. The result has the same type as `m`.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.IsEqualTo`
**Kind:** `fn`

**Arguments:** `a, b, tolerance?`

Returns true if every element of `a` differs from the same element of `b` by at most `tolerance`.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.TransformPlane`
**Kind:** `fn`

**Arguments:** `m, plane`

Transforms a plane by the rotation and the origin. This works in the same way as `MatrixTransformPlane` in the engine.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4_t.TransformPlaneByInverse`
**Kind:** `fn`

**Arguments:** `m, plane`

Transforms a plane by the inverse of the rotation and the origin. This works in the same way as `MatrixITransformPlane` in the engine.

**Source:** `mathlib/engine/matrix3x4.lua`


## matrix3x4a\_t" />



The form of `matrix3x4_t` that is aligned to 16 bytes. It has the same layout, constructors and
methods.

The `==` and `~=` operators compare every element exactly. `tostring` returns all twelve
elements.


  ### `matrix3x4a_t.m_flMatVal`
**Kind:** `prop`

The elements, stored as 3 rows of 4 numbers. Indices start at zero.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4a_t * b`
**Kind:** `meta`

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4a_t * b`
**Kind:** `meta`

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4a_t + b`
**Kind:** `meta`

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4a_t - b`
**Kind:** `meta`

**Source:** `mathlib/engine/matrix3x4.lua`


<Heading level={2} id="matrix3x4a_t-1">
  matrix3x4a_t

  " />
</Heading>




  ### `matrix3x4a_t()`
**Kind:** `ctor`

Creates an identity matrix.

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4a_t(m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23)`
**Kind:** `ctor`

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4a_t(x_axis, y_axis, z_axis, origin)`
**Kind:** `ctor`

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4a_t(q, position)`
**Kind:** `ctor`

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4a_t(angles, position)`
**Kind:** `ctor`

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4a_t(transform)`
**Kind:** `ctor`

**Source:** `mathlib/engine/matrix3x4.lua`

  ### `matrix3x4a_t(m)`
**Kind:** `ctor`

**Source:** `mathlib/engine/matrix3x4.lua`


## MatrixAxisType\_t _()_

Column indices for `GetColumn` and `SetColumn`.

---


## Quaternion

<!-- Source: cs2/api_reference/mathlib/quaternion.md -->

---
title: Quaternion
icon: orbit
---

# Quaternion

<Heading level={2} id="quaternion-1">
  Quaternion

  " />
</Heading>



A quaternion with the imaginary part in `x`, `y` and `z` and the real part in `w`. Use unit
quaternions, which have a length of 1, for rotations.

`Quaternion` is aligned to 16 bytes and is the form used by `CTransform`. `QuaternionStorage`
is the unaligned form with the same layout and the same methods. The constructor of each type
accepts the other type, so you can convert between them.

```cpp lines=false
typedef struct __declspec(align(16)) Quaternion {
    float x, y, z, w;
} Quaternion;
```


  ### `Quaternion.w`
**Kind:** `prop`

The real part.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.x`
**Kind:** `prop`

The x component of the imaginary part.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.y`
**Kind:** `prop`

The y component of the imaginary part.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.z`
**Kind:** `prop`

The z component of the imaginary part.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion + b`
**Kind:** `meta`

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion - b`
**Kind:** `meta`

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion * b`
**Kind:** `meta`

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion * b`
**Kind:** `meta`

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion * b`
**Kind:** `meta`

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion * b`
**Kind:** `meta`

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion / b`
**Kind:** `meta`

**Source:** `mathlib/engine/quaternion.lua`

  ### `-Quaternion`
**Kind:** `meta`

**Source:** `mathlib/engine/quaternion.lua`


<Heading level={2} id="quaternion-2">
  Quaternion

  " />
</Heading>

The constructor without arguments sets every component to zero. Use `qIdentity` or
`Quaternion(0, 0, 0, 1)` for the identity rotation.

When you pass numbers, missing components are set to 0. A `QAngle`, `RadianEuler` or
`DegreeEuler` is converted to the same rotation. Any other argument type throws an error.


  ### `Quaternion()`
**Kind:** `ctor`

Sets every component to zero.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion(x, y, z, w)`
**Kind:** `ctor`

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion(imaginary, real)`
**Kind:** `ctor`

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion(angles)`
**Kind:** `ctor`

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion(angles)`
**Kind:** `ctor`

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion(q)`
**Kind:** `ctor`

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.qIdentity`
**Kind:** `prop`

The identity rotation `(0, 0, 0, 1)`. It is read only.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.qZero`
**Kind:** `prop`

A quaternion with every component set to zero. It is read only.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.FromAxisAngle`
**Kind:** `fn`

**Arguments:** `axis, degrees`

Creates a quaternion that rotates by the given number of degrees around an axis. The axis must have a length of 1, because it is not normalized.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.FromQAngle`
**Kind:** `fn`

**Arguments:** `angles`

Creates a quaternion from a `QAngle`. This is the same as calling the constructor with a `QAngle`.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.Unpack`
**Kind:** `fn`

**Arguments:** `q`

Returns the `x`, `y`, `z` and `w` components.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.Copy`
**Kind:** `fn`

**Arguments:** `src, dst`

Copies the components of `src` into `dst`.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.Init`
**Kind:** `fn`

**Arguments:** `q, x?, y?, z?, w?`

Sets the components in place. Pass four numbers, or pass a vector for the imaginary part and a number for the real part. When you pass numbers, missing numbers are set to 0.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.IsValid`
**Kind:** `fn`

**Arguments:** `q`

Returns true if none of the four components is NaN or infinite.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.Invalidate`
**Kind:** `fn`

**Arguments:** `q`

Sets every component to NaN. After this call, `IsValid` returns false.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.ImaginaryPart`
**Kind:** `fn`

**Arguments:** `q`

Returns the imaginary part `x`, `y` and `z` as a new `Vector`.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.RealPart`
**Kind:** `fn`

**Arguments:** `q`

Returns the real part `w`.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.Length`
**Kind:** `fn`

**Arguments:** `q`

Returns the length of the quaternion, using all four components.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.LengthSqr`
**Kind:** `fn`

**Arguments:** `q`

Returns the squared length of the quaternion, using all four components.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.Dot`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the dot product of the two quaternions, using all four components.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.Normalize`
**Kind:** `fn`

**Arguments:** `q`

Scales the quaternion in place so that its length is 1, and returns the length it had before. If the length is zero, the quaternion is set to the identity rotation and the function returns 0.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.Normalized`
**Kind:** `fn`

**Arguments:** `q`

Returns a copy of the quaternion with a length of 1. If the length is zero, the copy is the identity rotation.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.IsNormalized`
**Kind:** `fn`

**Arguments:** `q, tolerance?`

Returns true if the length differs from 1 by less than `tolerance`.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.Conjugate`
**Kind:** `fn`

**Arguments:** `q`

Returns the conjugate, which is the quaternion with the imaginary part negated. For a unit quaternion, the conjugate is also the inverse.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.Inverse`
**Kind:** `fn`

**Arguments:** `q`

Returns the inverse, which is the conjugate divided by the squared length. This works for a quaternion of any length. If the length is zero, the function returns the identity rotation.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.RotateVector`
**Kind:** `fn`

**Arguments:** `q, vec`

Rotates a vector by the quaternion. The quaternion must have a length of 1.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.RotateVector`
**Kind:** `fn`

**Arguments:** `q, vec`

Rotates a `VectorAligned` by the quaternion. The quaternion must have a length of 1. The `w` component of the result is 0.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.RotateVectorByInverse`
**Kind:** `fn`

**Arguments:** `q, vec`

Rotates a vector by the inverse of the rotation. The quaternion must have a length of 1.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.RotateVectorByInverse`
**Kind:** `fn`

**Arguments:** `q, vec`

Rotates a `VectorAligned` by the inverse of the rotation. The quaternion must have a length of 1. The `w` component of the result is 0.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.RotateExtents`
**Kind:** `fn`

**Arguments:** `q, extents`

Rotates a box that is centered on the origin, given by its half size on each axis. Returns the half size of the smallest axis aligned box that contains the rotated box. Every component of the result is zero or greater. The quaternion is normalized before it is used.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.RotateExtents`
**Kind:** `fn`

**Arguments:** `q, extents`

Rotates a box that is centered on the origin, given by its half size on each axis. Returns the half size of the smallest axis aligned box that contains the rotated box. Every component of the result is zero or greater, and the `w` component is 0. The quaternion is normalized before it is used.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.RotateAABB`
**Kind:** `fn`

**Arguments:** `q, mins, maxs`

Rotates an axis aligned box around the origin. Returns the smallest axis aligned box that contains the rotated box. The quaternion must have a length of 1.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.RotateAABB`
**Kind:** `fn`

**Arguments:** `q, bounds`

Rotates a box given as an `AABB_t` and returns the result as an `AABB_t`. It works in the same way as the version that takes `mins` and `maxs`.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.RotateAABBByInverse`
**Kind:** `fn`

**Arguments:** `q, mins, maxs`

Rotates an axis aligned box around the origin by the inverse of the rotation. Returns the smallest axis aligned box that contains the rotated box.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.RotateAABBByInverse`
**Kind:** `fn`

**Arguments:** `q, bounds`

Rotates a box given as an `AABB_t` by the inverse of the rotation and returns the result as an `AABB_t`. It works in the same way as the version that takes `mins` and `maxs`.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.GetForward`
**Kind:** `fn`

**Arguments:** `q`

Returns the positive X axis rotated by the quaternion. This is the forward direction.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.GetLeft`
**Kind:** `fn`

**Arguments:** `q`

Returns the positive Y axis rotated by the quaternion. This is the left direction.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.GetUp`
**Kind:** `fn`

**Arguments:** `q`

Returns the positive Z axis rotated by the quaternion. This is the up direction.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.GetAxis`
**Kind:** `fn`

**Arguments:** `q`

Returns the rotation axis with a length of 1. If the imaginary part is zero, for example for the identity rotation, the function returns `Vector(0, 0, 1)`.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.GetAngleDegrees`
**Kind:** `fn`

**Arguments:** `q`

Returns the rotation angle around the axis from `GetAxis`, in degrees. The result is greater than -180 and at most 180.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.GetAxisAngle`
**Kind:** `fn`

**Arguments:** `q`

Returns the rotation axis and the rotation angle in radians. The axis is the same as the result of `GetAxis`. The angle is between 0 and `2 * math.pi`.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.SetAxisAngle`
**Kind:** `fn`

**Arguments:** `q, axis, radians`

Sets the quaternion in place to a rotation by an angle in radians around an axis. The axis must have a length of 1, because it is not normalized.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.ToMatrix`
**Kind:** `fn`

**Arguments:** `q, origin?`

Converts the rotation to an aligned 3x4 matrix. The fourth column of the matrix is set to `origin`.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.ToQAngle`
**Kind:** `fn`

**Arguments:** `q`

Converts the rotation to a `QAngle` with the pitch, yaw and roll in degrees.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.ToRadianEuler`
**Kind:** `fn`

**Arguments:** `q`

Converts the rotation to a `RadianEuler` in radians. `x` is the roll, `y` is the pitch and `z` is the yaw.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.ToDegreeEuler`
**Kind:** `fn`

**Arguments:** `q`

Converts the rotation to a `DegreeEuler` in degrees. `x` is the roll, `y` is the pitch and `z` is the yaw.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.ToRotationVector`
**Kind:** `fn`

**Arguments:** `q`

Converts the rotation to a rotation vector, which is the rotation axis multiplied by the rotation angle in degrees. The axis comes from `GetAxis` and the angle comes from `GetAngleDegrees`. This form is also called the exponential map.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.ToQuaternion`
**Kind:** `fn`

**Arguments:** `q`

Returns a copy as an aligned `Quaternion`.

**Source:** `mathlib/engine/quaternion.lua`

  ### `Quaternion.ToQuaternionStorage`
**Kind:** `fn`

**Arguments:** `q`

Returns a copy as an unaligned `QuaternionStorage`.

**Source:** `mathlib/engine/quaternion.lua`


## QuaternionStorage" />



The unaligned form of `Quaternion` with the same layout. Schema fields and `CTransformUnaligned`
use this form.

It has every method of `Quaternion`, and the methods work in the same way. Methods and
operators that return a quaternion return a `QuaternionStorage`, except `ToQuaternion`.
`ToMatrix` returns an unaligned `matrix3x4_t`.


  ### `QuaternionStorage.w`
**Kind:** `prop`

The real part.

**Source:** `mathlib/engine/quaternion.lua`

  ### `QuaternionStorage.x`
**Kind:** `prop`

The x component of the imaginary part.

**Source:** `mathlib/engine/quaternion.lua`

  ### `QuaternionStorage.y`
**Kind:** `prop`

The y component of the imaginary part.

**Source:** `mathlib/engine/quaternion.lua`

  ### `QuaternionStorage.z`
**Kind:** `prop`

The z component of the imaginary part.

**Source:** `mathlib/engine/quaternion.lua`

  ### `QuaternionStorage + b`
**Kind:** `meta`

**Source:** `mathlib/engine/quaternion.lua`

  ### `QuaternionStorage - b`
**Kind:** `meta`

**Source:** `mathlib/engine/quaternion.lua`

  ### `QuaternionStorage * b`
**Kind:** `meta`

**Source:** `mathlib/engine/quaternion.lua`

  ### `QuaternionStorage * b`
**Kind:** `meta`

**Source:** `mathlib/engine/quaternion.lua`

  ### `QuaternionStorage * b`
**Kind:** `meta`

**Source:** `mathlib/engine/quaternion.lua`

  ### `QuaternionStorage * b`
**Kind:** `meta`

**Source:** `mathlib/engine/quaternion.lua`

  ### `QuaternionStorage / b`
**Kind:** `meta`

**Source:** `mathlib/engine/quaternion.lua`

  ### `-QuaternionStorage`
**Kind:** `meta`

**Source:** `mathlib/engine/quaternion.lua`


<Heading level={2} id="quaternionstorage-1">
  QuaternionStorage

  " />
</Heading>




  ### `QuaternionStorage()`
**Kind:** `ctor`

Sets every component to zero.

**Source:** `mathlib/engine/quaternion.lua`

  ### `QuaternionStorage(x, y, z, w)`
**Kind:** `ctor`

**Source:** `mathlib/engine/quaternion.lua`

  ### `QuaternionStorage(imaginary, real)`
**Kind:** `ctor`

**Source:** `mathlib/engine/quaternion.lua`

  ### `QuaternionStorage(angles)`
**Kind:** `ctor`

**Source:** `mathlib/engine/quaternion.lua`

  ### `QuaternionStorage(angles)`
**Kind:** `ctor`

**Source:** `mathlib/engine/quaternion.lua`

  ### `QuaternionStorage(q)`
**Kind:** `ctor`

**Source:** `mathlib/engine/quaternion.lua`

  ### `QuaternionStorage.qIdentity`
**Kind:** `prop`

The identity rotation `(0, 0, 0, 1)`. It is read only.

**Source:** `mathlib/engine/quaternion.lua`

  ### `QuaternionStorage.qZero`
**Kind:** `prop`

A quaternion with every component set to zero. It is read only.

**Source:** `mathlib/engine/quaternion.lua`

---


## Rect

<!-- Source: cs2/api_reference/mathlib/rect.md -->

---
title: Rectangle
icon: rectangle-horizontal
---

# Rectangle

## Rect\_t" />



A rectangle of whole numbers, stored as a position and a size. The position `x`, `y` is the
top left cell. The last column and the last row are part of the rectangle, so `MaximumX` is
`x + width - 1` and `MaximumY` is `y + height - 1`.

Only engine structures use this type. The render functions use floats.

The `==` operator compares all four fields and returns `false` when the other value is not
a `Rect_t`.

```cpp lines=false
typedef struct Rect_t {
    int x, y;
    int width, height;
} Rect_t;
```


  ### `Rect_t.height`
**Kind:** `prop`

**Source:** `mathlib/engine/rect.lua`

  ### `Rect_t.width`
**Kind:** `prop`

**Source:** `mathlib/engine/rect.lua`

  ### `Rect_t.x`
**Kind:** `prop`

The left column.

**Source:** `mathlib/engine/rect.lua`

  ### `Rect_t.y`
**Kind:** `prop`

The top row.

**Source:** `mathlib/engine/rect.lua`


<Heading level={2} id="rect_t-1">
  Rect_t

  " />
</Heading>

Call `Rect_t()` to create a rectangle with every field set to 0. Call
`Rect_t(x, y, width, height)` to set the fields. A field that is not given is set to 0. Call
`Rect_t(other)` to make a copy of another rectangle. Any other first argument throws an
error.


  ### `Rect_t()`
**Kind:** `ctor`

A rectangle with every field set to 0.

**Source:** `mathlib/engine/rect.lua`

  ### `Rect_t(x, y, width, height)`
**Kind:** `ctor`

**Source:** `mathlib/engine/rect.lua`

  ### `Rect_t(self)`
**Kind:** `ctor`

**Source:** `mathlib/engine/rect.lua`

  ### `Rect_t.Clear`
**Kind:** `fn`

**Arguments:** `rect`

Sets every field of the rectangle to 0.

**Source:** `mathlib/engine/rect.lua`

  ### `Rect_t.Copy`
**Kind:** `fn`

**Arguments:** `src, dst`

Copies one rectangle into another rectangle.

**Source:** `mathlib/engine/rect.lua`

  ### `Rect_t.Unpack`
**Kind:** `fn`

**Arguments:** `rect`

Returns the X, Y, width and height of the rectangle.

**Source:** `mathlib/engine/rect.lua`

  ### `Rect_t.Size`
**Kind:** `fn`

**Arguments:** `rect`

Returns the area of the rectangle, which is `width * height`.

**Source:** `mathlib/engine/rect.lua`

  ### `Rect_t.IsEmpty`
**Kind:** `fn`

**Arguments:** `rect`

Returns `true` if the width or the height is 0 or negative.

**Source:** `mathlib/engine/rect.lua`

  ### `Rect_t.MaximumX`
**Kind:** `fn`

**Arguments:** `rect`

Returns the last column of the rectangle, which is `x + width - 1`.

**Source:** `mathlib/engine/rect.lua`

  ### `Rect_t.MaximumY`
**Kind:** `fn`

**Arguments:** `rect`

Returns the last row of the rectangle, which is `y + height - 1`.

**Source:** `mathlib/engine/rect.lua`

  ### `Rect_t.Intersects`
**Kind:** `fn`

**Arguments:** `rect, other`

Returns `true` if the two rectangles share at least one cell. Rectangles that only touch at an edge do not share a cell.

**Source:** `mathlib/engine/rect.lua`

  ### `Rect_t.Contains`
**Kind:** `fn`

**Arguments:** `rect, other`

Returns `true` if the other rectangle is completely inside this rectangle.

**Source:** `mathlib/engine/rect.lua`

  ### `Rect_t.ContainsPoint`
**Kind:** `fn`

**Arguments:** `rect, px, py`

Returns `true` if the cell at the given column and row is inside the rectangle.

**Source:** `mathlib/engine/rect.lua`

  ### `Rect_t.SetFromMaxMins`
**Kind:** `fn`

**Arguments:** `rect, min_x, min_y, max_x, max_y`

Sets the rectangle from its first and last cells. Both cells are part of the rectangle, so the width is `max_x - min_x + 1` and the height is `max_y - min_y + 1`.

**Source:** `mathlib/engine/rect.lua`

  ### `Rect_t.IntersectWith`
**Kind:** `fn`

**Arguments:** `rect, other`

Shrinks the rectangle to the cells that it shares with another rectangle. If the rectangles do not share a cell, the result has a width or a height of 0 or less.

**Source:** `mathlib/engine/rect.lua`

  ### `Rect_t.Union`
**Kind:** `fn`

**Arguments:** `rect, other`

Grows the rectangle to the smallest rectangle that contains both rectangles.

**Source:** `mathlib/engine/rect.lua`

---


## Transform

<!-- Source: cs2/api_reference/mathlib/transform.md -->

---
title: Transform
icon: axis-3d
---

# Transform

## CTransform" />



A position, a rotation stored as a quaternion and a uniform scale, in 32 bytes aligned to 16
bytes. The scale is stored in the `w` component of `m_vPositionWScale`. Use `GetOrigin` and
`GetScale` to read the position and the scale.

Trace results, `CGameSceneNode.m_nodeToWorld` and the fields of `CMsgTransform` messages use
this type.

```cpp lines=false
typedef struct __declspec(align(16)) CTransform {
    VectorAligned m_vPositionWScale;
    Quaternion m_orientation;
} CTransform;
```

The `==` and `~=` operators compare every component exactly. `tostring` returns the position,
the orientation and the scale. Call `CTransform()` to create a new identity transform.

The `*` operator with a transform on the right combines the two transforms. The right transform
is applied first. If the orientation of the right transform is closer to the negated orientation
of the left transform, it is negated before the multiplication. The orientation of the result is
normalized. If its length is zero, it is set to the identity rotation. The scale of the result is
the product of the two scales. With a vector on the right, the `*` operator transforms the vector
as a point, in the same way as `TransformVector`.


  ### `CTransform.m_orientation`
**Kind:** `prop`

The rotation.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.m_vPositionWScale`
**Kind:** `prop`

The position in `x`, `y` and `z`, and the uniform scale in `w`.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform * b`
**Kind:** `meta`

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform * b`
**Kind:** `meta`

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform * b`
**Kind:** `meta`

**Source:** `mathlib/engine/transform.lua`


<Heading level={2} id="ctransform-1">
  CTransform

  " />
</Heading>

The constructor without arguments creates an identity transform. When you pass a position, a
`QAngle` orientation is converted to a quaternion, and the scale is 1 if it is not given. When
you pass a matrix, the scale is the length of the longest of its first three columns. Any other
argument type throws an error.


  ### `CTransform()`
**Kind:** `ctor`

Creates an identity transform.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform(position, orientation, scale)`
**Kind:** `ctor`

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform(m)`
**Kind:** `ctor`

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform(transform)`
**Kind:** `ctor`

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.Copy`
**Kind:** `fn`

**Arguments:** `src, dst`

Copies the position, the scale and the orientation of `src` into `dst`.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.IsValid`
**Kind:** `fn`

**Arguments:** `t`

Returns true if no component of the position or the orientation is NaN or infinite. The scale is not checked. Call `GetScale` if you need to check the scale.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.IsEqualTo`
**Kind:** `fn`

**Arguments:** `a, b, tolerance?, rotation_tolerance?`

Returns true if the two transforms are equal within the given tolerances.

Each position component may differ by at most `tolerance`. The scale and the orientation
      components are compared relative to their size. The scale uses `tolerance` and the orientation
      uses `rotation_tolerance`. If both values are smaller than about `3.8e-6`, they always count as
      equal. An orientation also matches its negated quaternion, because both give the same rotation.
    
  

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.SetToIdentity`
**Kind:** `fn`

**Arguments:** `t`

Sets the transform to the identity. The position is zero, the rotation is the identity rotation and the scale is 1.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.InitFromQAngles`
**Kind:** `fn`

**Arguments:** `t, angles, position?, scale?`

Sets the transform in place from a `QAngle`, a position and a scale.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.InitFromQuaternion`
**Kind:** `fn`

**Arguments:** `t, q, position?, scale?`

Sets the transform in place from a quaternion, a position and a scale.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.InitFromMatrix`
**Kind:** `fn`

**Arguments:** `t, m`

Sets the transform in place from a matrix. The first three columns are normalized before the rotation is computed. The scale is the length of the longest of these columns.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.InitFromRotationAboutAxis`
**Kind:** `fn`

**Arguments:** `t, axis, degrees, position?, scale?`

Sets the transform in place to a rotation by a number of degrees around an axis, with a position and a scale. The axis must have a length of 1.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.InitFLU`
**Kind:** `fn`

**Arguments:** `t, forward, left, up, origin`

Sets the transform in place from the forward, left and up axes and a position. The scale is set to 1. The axes must have a length of 1 and be at right angles to each other.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.InitFromForwardVector`
**Kind:** `fn`

**Arguments:** `t, forward, origin, scale?`

Sets the transform in place so that its forward axis points along `forward`. The up axis is chosen to be as close as possible to the world up axis `Vector(0, 0, 1)`. If `forward` points straight up, `Vector(-1, 0, 0)` is used in place of the world up axis. If it points straight down, `Vector(1, 0, 0)` is used.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.GetOrigin`
**Kind:** `fn`

**Arguments:** `t`

Returns a copy of the position as a `Vector`. The scale is not included.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.SetOrigin`
**Kind:** `fn`

**Arguments:** `t, origin`

Sets the position. The scale does not change.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.ShiftOrigin`
**Kind:** `fn`

**Arguments:** `t, offset`

Adds an offset to the position. The scale does not change.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.GetScale`
**Kind:** `fn`

**Arguments:** `t`

Returns the uniform scale, which is stored in the `w` component of `m_vPositionWScale`.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.SetScale`
**Kind:** `fn`

**Arguments:** `t, scale`

Sets the uniform scale.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.SetPosition`
**Kind:** `fn`

**Arguments:** `t, position`

Sets the position. The scale does not change. This is the same function as `SetOrigin`.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.SetPositionAndScale`
**Kind:** `fn`

**Arguments:** `t, position, scale?`

Sets the position and the scale together. If you pass only a `VectorAligned`, its `w` component is used as the scale. If you pass only a `Vector`, an error is thrown.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.GetPositionAndScale`
**Kind:** `fn`

**Arguments:** `t`

Returns a reference to `m_vPositionWScale`, which holds the position and the scale. Changes to the returned value change this transform. Keep the transform alive while you use the reference.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.GetOrientation`
**Kind:** `fn`

**Arguments:** `t`

Returns a reference to the orientation. Changes to the returned value change this transform. Keep the transform alive while you use the reference.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.SetOrientation`
**Kind:** `fn`

**Arguments:** `t, q`

Sets the orientation.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.SetAngles`
**Kind:** `fn`

**Arguments:** `t, angles`

Sets the orientation from a `QAngle`.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.ToQuaternion`
**Kind:** `fn`

**Arguments:** `t`

Returns a copy of the orientation as a `Quaternion`.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.ToQAngle`
**Kind:** `fn`

**Arguments:** `t`

Returns the orientation as a `QAngle` in degrees.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.ToMatrix`
**Kind:** `fn`

**Arguments:** `t`

Converts the transform to a matrix. The rotation part is multiplied by the scale, and the fourth column is the position.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.Unscaled`
**Kind:** `fn`

**Arguments:** `t`

Returns a copy of the transform with the scale set to 1.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.GetForward`
**Kind:** `fn`

**Arguments:** `t`

Returns the positive X axis rotated by the orientation. This is the forward direction. The scale is not applied.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.GetLeft`
**Kind:** `fn`

**Arguments:** `t`

Returns the positive Y axis rotated by the orientation. This is the left direction. The scale is not applied.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.GetUp`
**Kind:** `fn`

**Arguments:** `t`

Returns the positive Z axis rotated by the orientation. This is the up direction. The scale is not applied.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.GetBasisVectorsFLU`
**Kind:** `fn`

**Arguments:** `t`

Returns the forward, left and up axes rotated by the orientation. The scale is not applied.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.TransformVector`
**Kind:** `fn`

**Arguments:** `t, vec`

Transforms a point from local space into the parent space of the transform. The point is rotated, multiplied by the scale, and then the position is added.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.TransformVector`
**Kind:** `fn`

**Arguments:** `t, vec`

Transforms a `VectorAligned` point from local space into the parent space of the transform. The point is rotated, multiplied by the scale, and then the position is added. The `w` component of the result is 0.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.RotateVector`
**Kind:** `fn`

**Arguments:** `t, vec`

Rotates a direction by the orientation. The scale is not applied.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.RotateVector`
**Kind:** `fn`

**Arguments:** `t, vec`

Rotates a `VectorAligned` direction by the orientation. The scale is not applied. The `w` component of the result is 0.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.RotateAndScaleVector`
**Kind:** `fn`

**Arguments:** `t, vec`

Rotates a direction by the orientation and multiplies it by the scale. The position is not added.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.RotateAndScaleVector`
**Kind:** `fn`

**Arguments:** `t, vec`

Rotates a `VectorAligned` direction by the orientation and multiplies it by the scale. The position is not added. The `w` component of the result is 0.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.TransformVectorByInverse`
**Kind:** `fn`

**Arguments:** `t, vec`

Transforms a point from the parent space into the local space of the transform. The position is subtracted, the result is rotated by the inverse orientation, and then it is divided by the scale. If the scale is zero, the result is a zero vector.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.TransformVectorByInverse`
**Kind:** `fn`

**Arguments:** `t, vec`

Transforms a `VectorAligned` point from the parent space into the local space of the transform. The position is subtracted, the result is rotated by the inverse orientation, and then it is divided by the scale. If the scale is zero, the result is a zero vector. The `w` component of the result is 0.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.RotateVectorByInverse`
**Kind:** `fn`

**Arguments:** `t, vec`

Rotates a direction by the inverse orientation. The scale is not applied.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.RotateVectorByInverse`
**Kind:** `fn`

**Arguments:** `t, vec`

Rotates a `VectorAligned` direction by the inverse orientation. The scale is not applied. The `w` component of the result is 0.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.RotateExtents`
**Kind:** `fn`

**Arguments:** `t, extents`

Rotates a box that is centered on the origin, given by its half size on each axis. Returns the half size of the smallest axis aligned box that contains the rotated box. Every component of the result is zero or greater. The scale is not applied.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.TransformAABB`
**Kind:** `fn`

**Arguments:** `t, mins, maxs`

Transforms an axis aligned box. The box is multiplied by the scale and rotated, and then the position is added. Returns the smallest axis aligned box that contains the result.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.TransformAABB`
**Kind:** `fn`

**Arguments:** `t, bounds`

Transforms a box given as an `AABB_t` and returns the result as an `AABB_t`. It works in the same way as the version that takes `mins` and `maxs`.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.TransformAABBByInverse`
**Kind:** `fn`

**Arguments:** `t, mins, maxs`

Transforms an axis aligned box by the inverse transform. The position is subtracted, the box is divided by the scale, and then it is rotated by the inverse orientation. Returns the smallest axis aligned box that contains the result. If the scale is zero, the result is a box of zero size at the origin.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.TransformAABBByInverse`
**Kind:** `fn`

**Arguments:** `t, bounds`

Transforms a box given as an `AABB_t` by the inverse transform and returns the result as an `AABB_t`. It works in the same way as the version that takes `mins` and `maxs`.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.RotateAABB`
**Kind:** `fn`

**Arguments:** `t, mins, maxs`

Rotates an axis aligned box by the orientation. The scale and the position are not applied. Returns the smallest axis aligned box that contains the rotated box.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.RotateAABB`
**Kind:** `fn`

**Arguments:** `t, bounds`

Rotates a box given as an `AABB_t` and returns the result as an `AABB_t`. It works in the same way as the version that takes `mins` and `maxs`.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.RotateAABBByInverse`
**Kind:** `fn`

**Arguments:** `t, mins, maxs`

Rotates an axis aligned box by the inverse orientation. The scale and the position are not applied. Returns the smallest axis aligned box that contains the rotated box.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.RotateAABBByInverse`
**Kind:** `fn`

**Arguments:** `t, bounds`

Rotates a box given as an `AABB_t` by the inverse orientation and returns the result as an `AABB_t`. It works in the same way as the version that takes `mins` and `maxs`.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.RotateAndScaleAABB`
**Kind:** `fn`

**Arguments:** `t, mins, maxs`

Multiplies an axis aligned box by the scale and rotates it by the orientation. The position is not added. Returns the smallest axis aligned box that contains the result.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.RotateAndScaleAABB`
**Kind:** `fn`

**Arguments:** `t, bounds`

Scales and rotates a box given as an `AABB_t` and returns the result as an `AABB_t`. It works in the same way as the version that takes `mins` and `maxs`.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.RotateAndScaleAABBByInverse`
**Kind:** `fn`

**Arguments:** `t, mins, maxs`

Divides an axis aligned box by the scale and rotates it by the inverse orientation. The position is not used. Returns the smallest axis aligned box that contains the result. If the scale is zero, the result is a box of zero size at the origin.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.RotateAndScaleAABBByInverse`
**Kind:** `fn`

**Arguments:** `t, bounds`

Divides a box given as an `AABB_t` by the scale, rotates it by the inverse orientation and returns the result as an `AABB_t`. It works in the same way as the version that takes `mins` and `maxs`.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.Inverse`
**Kind:** `fn`

**Arguments:** `t`

Returns the inverse transform, including the uniform scale. The orientation of the result is the normalized conjugate of the orientation. The scale of the result is 1 divided by the scale.

The result is an exact inverse only if the orientation has a length of 1 and the scale is not
      zero. If the scale is zero, the scale of the result is 8388608.
    
  

**Source:** `mathlib/engine/transform.lua`

  ### `CTransform.InverseTR`
**Kind:** `fn`

**Arguments:** `t`

The same function as `Inverse`. It also inverts the uniform scale.

**Source:** `mathlib/engine/transform.lua`


## CTransformUnaligned" />



An unaligned transform. It stores the position, the uniform scale and the `QuaternionStorage`
orientation in separate fields.

The `==` and `~=` operators compare every component exactly. `tostring` returns the position,
the orientation and the scale.


  ### `CTransformUnaligned.m_flScale`
**Kind:** `prop`

The uniform scale.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransformUnaligned.m_orientation`
**Kind:** `prop`

The rotation.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransformUnaligned.m_vPosition`
**Kind:** `prop`

The position.

**Source:** `mathlib/engine/transform.lua`


<Heading level={2} id="ctransformunaligned-1">
  CTransformUnaligned

  " />
</Heading>


  ### `CTransformUnaligned()`
**Kind:** `ctor`

Creates an identity transform.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransformUnaligned(position, orientation, scale)`
**Kind:** `ctor`

**Source:** `mathlib/engine/transform.lua`

  ### `CTransformUnaligned(transform)`
**Kind:** `ctor`

**Source:** `mathlib/engine/transform.lua`

  ### `CTransformUnaligned.AsTransform`
**Kind:** `fn`

**Arguments:** `t`

Returns a copy as an aligned `CTransform`.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransformUnaligned.IsValid`
**Kind:** `fn`

**Arguments:** `t`

Returns true if no component of the position, the scale or the orientation is NaN or infinite.

**Source:** `mathlib/engine/transform.lua`

  ### `CTransformUnaligned.Copy`
**Kind:** `fn`

**Arguments:** `src, dst`

Copies the position, the scale and the orientation of `src` into `dst`.

**Source:** `mathlib/engine/transform.lua`

---


## Vector

<!-- Source: cs2/api_reference/mathlib/vector.md -->

---
title: Vector
icon: move-3d
sidebar_position: 2
---

# Vector

<Heading level={2} id="vector-1">
  Vector

  " />
</Heading>



A vector with three components. Use it for positions, directions and offsets in the world.

The components are stored as 32 bit floats. In Source 2, `x` points forward, `y` points left
and `z` points up.

The `+` and `-` operators add and subtract two vectors. The `*` and `/` operators take a
number or a vector. With a number, every component is multiplied or divided by it. With a
vector, each component is multiplied or divided by the matching component. The unary `-`
operator returns a negated copy, and the `#` operator returns the length. The `==` operator
compares every component and returns `false` when the other value is not a `Vector`.


  ### `Vector.x`
**Kind:** `prop`

The X component, which points forward.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.y`
**Kind:** `prop`

The Y component, which points left.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.z`
**Kind:** `prop`

The Z component, which points up.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector / b`
**Kind:** `meta`

**Source:** `mathlib/engine/vector.lua`

  ### `Vector * b`
**Kind:** `meta`

**Source:** `mathlib/engine/vector.lua`

  ### `-Vector`
**Kind:** `meta`

**Source:** `mathlib/engine/vector.lua`

  ### `Vector + b`
**Kind:** `meta`

**Source:** `mathlib/engine/vector.lua`

  ### `#Vector`
**Kind:** `meta`

**Source:** `mathlib/engine/vector.lua`

  ### `Vector - b`
**Kind:** `meta`

**Source:** `mathlib/engine/vector.lua`


<Heading level={2} id="vector-2">
  Vector

  " />
</Heading>

Call `Vector(x, y, z)` to create a vector. A component that is not given is set to 0. Call
`Vector(other)` to make a copy of another vector.

The constant vectors below cannot be changed.


  ### `Vector(x, y, z)`
**Kind:** `ctor`

**Source:** `mathlib/engine/vector.lua`

  ### `Vector(self)`
**Kind:** `ctor`

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.vForward`
**Kind:** `prop`

The vector `(1, 0, 0)`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.vInvalid`
**Kind:** `prop`

A vector with every component set to `FLT_MAX`, the largest 32 bit float.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.vLeft`
**Kind:** `prop`

The vector `(0, 1, 0)`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.vOne`
**Kind:** `prop`

The vector `(1, 1, 1)`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.vUp`
**Kind:** `prop`

The vector `(0, 0, 1)`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.vZero`
**Kind:** `prop`

The vector `(0, 0, 0)`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.ToAngle`
**Kind:** `fn`

**Arguments:** `fwd, pseudo_up?`

Converts a forward direction into the angle that points along it.

Without `pseudo_up`, the roll of the angle is always 0. If the direction points straight up,
      the pitch is -90. If it points straight down, the pitch is 90. In both cases the yaw is 0.

      With `pseudo_up`, the roll is chosen so that the up direction of the angle is as close as
      possible to `pseudo_up`.
    
  

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.Copy`
**Kind:** `fn`

**Arguments:** `src, dst`

Copies the components of one vector into another vector.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.Cross`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the cross product of two vectors.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.DistTo`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the distance between two points.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.DistTo2D`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the distance between two points. The Z component is ignored.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.DistTo2DSqr`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the squared distance between two points. The Z component is ignored.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.DistToSqr`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the squared distance between two points.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.Dot`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the dot product of two vectors.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.Init`
**Kind:** `fn`

**Arguments:** `vec, x?, y?, z?`

Sets the components of the vector. A component that is not given is set to 0.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.Length`
**Kind:** `fn`

**Arguments:** `vec`

Returns the length of the vector.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.Length2D`
**Kind:** `fn`

**Arguments:** `vec`

Returns the length of the vector. The Z component is ignored.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.Length2DSqr`
**Kind:** `fn`

**Arguments:** `vec`

Returns the squared length of the vector. The Z component is ignored.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.LengthSqr`
**Kind:** `fn`

**Arguments:** `vec`

Returns the squared length of the vector.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.Lerp`
**Kind:** `fn`

**Arguments:** `a, b, t`

Returns a new vector between two vectors. This is linear interpolation.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.Negate`
**Kind:** `fn`

**Arguments:** `vec`

Negates every component of the vector.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.NormalizeInPlaceSafe`
**Kind:** `fn`

**Arguments:** `vec, fallback`

Normalizes the vector so that its length is 1. If the length is 0, the components of `fallback` are copied into the vector.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.NormalizeInPlace`
**Kind:** `fn`

**Arguments:** `vec`

Normalizes the vector so that its length is 1. If the length is 0, the vector stays at `(0, 0, 0)`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.NormalizedSafe`
**Kind:** `fn`

**Arguments:** `vec, fallback`

Returns a normalized copy of the vector.

If the length of the vector is 0, this function returns the `fallback` object itself. It
      does not make a copy, so changes to the returned vector also change `fallback`. Use
      `Vector(result)` to make a separate copy.
    
  

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.Normalized`
**Kind:** `fn`

**Arguments:** `vec`

Returns a normalized copy of the vector. If the length is 0, the copy is `(0, 0, 0)`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.Scale`
**Kind:** `fn`

**Arguments:** `vec, scalar`

Multiplies every component of the vector by a number.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.Scaled`
**Kind:** `fn`

**Arguments:** `vec, scalar`

Returns a copy of the vector with every component multiplied by a number.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.To`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the normalized direction from `a` to `b`. If the two points are the same, the result is `(0, 0, 0)`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.Unpack`
**Kind:** `fn`

**Arguments:** `vec`

Returns the X, Y and Z components of the vector.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.RayDist`
**Kind:** `fn`

**Arguments:** `vec, start, dir`

Returns the length of the cross product of `dir` and `vec - start`.

When `dir` has a length of 1, this is the shortest distance from the point to the infinite
      line that goes through `start` in the direction of `dir`. Points behind `start` are measured
      to the same line.
    
  

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.ClosestLinePoint`
**Kind:** `fn`

**Arguments:** `vec, a, b`

Returns the point on the segment from `a` to `b` that is closest to the given point. The result is always on the segment, between `a` and `b`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.InverseLerp`
**Kind:** `fn`

**Arguments:** `vec, a, b`

Projects the point onto the line through `a` and `b` and returns how far along the line it is. The factor is 0 at `a` and 1 at `b`. It is not clamped, so it can be below 0 or above 1.

If the squared distance between `a` and `b` is below 0.00001, the factor is 0.
    
  

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.To2DDirection`
**Kind:** `fn`

**Arguments:** `vec`

Returns the yaw of the direction in degrees, in the range from -180 to 180. A yaw of 0 points along +X, and 90 points along +Y. The value is calculated with `atan2(y, x)`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.InitFLU`
**Kind:** `fn`

**Arguments:** `vec, f?, l?, u?`

Sets the components from forward, left and up values. A value that is not given is set to 0. In Source 2, forward is X, left is Y and up is Z, so this is the same as `Init`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.FLU`
**Kind:** `fn`

**Arguments:** `f?, l?, u?`

Creates a vector from forward, left and up values. A value that is not given is set to 0.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.Zero`
**Kind:** `fn`

**Arguments:** `vec`

Sets every component of the vector to 0.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.Invalidate`
**Kind:** `fn`

**Arguments:** `vec`

Sets every component of the vector to NaN.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.IsValid`
**Kind:** `fn`

**Arguments:** `vec`

Returns `true` if every component is a finite number. NaN and infinity are not finite.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.IsZero`
**Kind:** `fn`

**Arguments:** `vec, tolerance?`

Returns `true` if every component is greater than `-tolerance` and less than `tolerance`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.EqualsZero`
**Kind:** `fn`

**Arguments:** `vec`

Returns `true` if every component is exactly 0.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.IsReasonable`
**Kind:** `fn`

**Arguments:** `vec, range?`

Returns `true` if the length of the vector is finite and less than `range`. Use it to check that a world position has a sensible value.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.IsLengthGreaterThan`
**Kind:** `fn`

**Arguments:** `vec, val`

Returns `true` if the length of the vector is greater than `val`. The squared values are compared, so no square root is calculated.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.IsLengthLessThan`
**Kind:** `fn`

**Arguments:** `vec, val`

Returns `true` if the length of the vector is less than `val`. The squared values are compared, so no square root is calculated.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.Min`
**Kind:** `fn`

**Arguments:** `a, b`

Returns a vector that has the smaller value of each component of `a` and `b`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.Max`
**Kind:** `fn`

**Arguments:** `a, b`

Returns a vector that has the larger value of each component of `a` and `b`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.MulAdd`
**Kind:** `fn`

**Arguments:** `vec, a, b, scalar`

Sets the vector to `a + b * scalar`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.ProjectOnto`
**Kind:** `fn`

**Arguments:** `vec, onto`

Returns the part of the vector that is parallel to `onto`. This is the projection of the vector onto `onto`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.Flatten`
**Kind:** `fn`

**Arguments:** `vec, up`

Returns a copy of the vector with the part along `up` removed. `up` is normalized first.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.LargestComponent`
**Kind:** `fn`

**Arguments:** `vec`

Returns the index of the component with the largest absolute value. The index is 0 for X, 1 for Y and 2 for Z.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.SmallestComponent`
**Kind:** `fn`

**Arguments:** `vec`

Returns the index of the component with the smallest absolute value. The index is 0 for X, 1 for Y and 2 for Z.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.LargestComponentValue`
**Kind:** `fn`

**Arguments:** `vec`

Returns the largest absolute value of the three components.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.SmallestComponentValue`
**Kind:** `fn`

**Arguments:** `vec`

Returns the smallest absolute value of the three components.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.AsVector2D`
**Kind:** `fn`

**Arguments:** `vec`

Returns a `Vector2D` that shares the memory of the X and Y components of the vector. Changes to the `Vector2D` also change the vector. Keep the vector alive for as long as you use the `Vector2D`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.GetLeftVectors`
**Kind:** `fn`

**Arguments:** `fwd`

Returns the left and up directions for a forward direction. Both results are normalized.

If the forward direction points almost straight up or down, the left direction is
      `(0, 1, 0)`.
    
  

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.GetRightVectors`
**Kind:** `fn`

**Arguments:** `fwd`

Returns the right and up directions for a forward direction. Both results are normalized. The right direction is the negated left direction from `GetLeftVectors`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.Get2DDirection`
**Kind:** `fn`

**Arguments:** `vec, other`

Returns the yaw in degrees of the direction from `other` to this vector, in the range from -180 to 180. The Z component is ignored.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.GetVectorDirection2D`
**Kind:** `fn`

**Arguments:** `vec, b`

Returns the normalized direction from `b` to this vector, with the Z component set to 0. If the two points have the same X and Y, the result is `(0, 0, 0)`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.IsIntersect`
**Kind:** `fn`

**Arguments:** `vec, v, tolerance, ignore_z`

Returns `true` if the difference between each compared component of the two points is less than `tolerance`.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.OffsetX`
**Kind:** `fn`

**Arguments:** `vec, d`

Returns a copy of the vector with `d` subtracted from the X component.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.OffsetY`
**Kind:** `fn`

**Arguments:** `vec, d`

Returns a copy of the vector with `d` subtracted from the Y component.

**Source:** `mathlib/engine/vector.lua`

  ### `Vector.OffsetZ`
**Kind:** `fn`

**Arguments:** `vec, d`

Returns a copy of the vector with `d` subtracted from the Z component.

**Source:** `mathlib/engine/vector.lua`

---


## Vector2

<!-- Source: cs2/api_reference/mathlib/vector2.md -->

---
title: Vector 2D
icon: move
sidebar_position: 3
---

# Vector 2D

## Vector2D" />



A vector with two components, stored as `cdata<struct Vector2D>`.

The `+` and `-` operators add and subtract two vectors. The `*` and `/` operators take a
number or a vector. With a number, every component is multiplied or divided by it. With a
vector, each component is multiplied or divided by the matching component. The unary `-`
operator returns a negated copy, and the `#` operator returns the length. The `==` operator
compares both components and returns `false` when the other value is not a `Vector2D`.


  ### `Vector2D.x`
**Kind:** `prop`

The X component.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.y`
**Kind:** `prop`

The Y component.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D + b`
**Kind:** `meta`

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D - b`
**Kind:** `meta`

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D * b`
**Kind:** `meta`

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D / b`
**Kind:** `meta`

**Source:** `mathlib/engine/vector2.lua`

  ### `-Vector2D`
**Kind:** `meta`

**Source:** `mathlib/engine/vector2.lua`

  ### `#Vector2D`
**Kind:** `meta`

**Source:** `mathlib/engine/vector2.lua`


<Heading level={2} id="vector2d-1">
  Vector2D

  " />
</Heading>

Call `Vector2D(x, y)` to create a vector. A component that is not given is set to 0. Call
`Vector2D(other)` to make a copy of another vector.

The constant vectors below cannot be changed.


  ### `Vector2D(x, y)`
**Kind:** `ctor`

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D(self)`
**Kind:** `ctor`

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.vOne`
**Kind:** `prop`

The vector `(1, 1)`.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.vZero`
**Kind:** `prop`

The vector `(0, 0)`.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.Unpack`
**Kind:** `fn`

**Arguments:** `vec`

Returns the X and Y components of the vector.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.Copy`
**Kind:** `fn`

**Arguments:** `src, dst`

Copies the components of one vector into another vector.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.Init`
**Kind:** `fn`

**Arguments:** `vec, x?, y?`

Sets the components of the vector. A component that is not given is set to 0.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.AsVector`
**Kind:** `fn`

**Arguments:** `vec, z`

Returns a new `Vector` with the X and Y components of this vector and the given Z component.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.Cross`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the 2D cross product of two vectors. This is the Z component of the 3D cross product, `a.x * b.y - a.y * b.x`.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.DistTo`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the distance between two points.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.DistToSqr`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the squared distance between two points.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.Dot`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the dot product of two vectors.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.Length`
**Kind:** `fn`

**Arguments:** `vec`

Returns the length of the vector.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.LengthSqr`
**Kind:** `fn`

**Arguments:** `vec`

Returns the squared length of the vector.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.Lerp`
**Kind:** `fn`

**Arguments:** `a, b, t`

Returns a new vector between two vectors. This is linear interpolation.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.NormalizeInPlace`
**Kind:** `fn`

**Arguments:** `vec`

Normalizes the vector so that its length is 1. If the length is 0, the vector stays at `(0, 0)`.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.Normalized`
**Kind:** `fn`

**Arguments:** `vec`

Returns a normalized copy of the vector. If the length is 0, the copy is `(0, 0)`.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.Scale`
**Kind:** `fn`

**Arguments:** `vec, s`

Multiplies both components of the vector by a number.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.Scaled`
**Kind:** `fn`

**Arguments:** `vec, s`

Returns a copy of the vector with both components multiplied by a number.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.To2DDirection`
**Kind:** `fn`

**Arguments:** `vec`

Returns the direction of the vector in degrees, in the range from -180 to 180. An angle of 0 points along +X, and 90 points along +Y. The value is calculated with `atan2(y, x)`.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.Zero`
**Kind:** `fn`

**Arguments:** `vec`

Sets both components of the vector to 0.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.IsZero`
**Kind:** `fn`

**Arguments:** `vec, tolerance?`

Returns `true` if both components are greater than `-tolerance` and less than `tolerance`.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.IsValid`
**Kind:** `fn`

**Arguments:** `vec`

Returns `true` if both components are finite numbers. NaN and infinity are not finite.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.Negate`
**Kind:** `fn`

**Arguments:** `vec`

Negates both components of the vector.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.MulAdd`
**Kind:** `fn`

**Arguments:** `vec, a, b, scalar`

Sets the vector to `a + b * scalar`.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.Min`
**Kind:** `fn`

**Arguments:** `a, b`

Returns a vector that has the smaller value of each component of `a` and `b`.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.Max`
**Kind:** `fn`

**Arguments:** `a, b`

Returns a vector that has the larger value of each component of `a` and `b`.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.LargestComponentValue`
**Kind:** `fn`

**Arguments:** `vec`

Returns the component with the larger absolute value. The sign of the component is kept. If both have the same absolute value, Y is returned.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.IsLengthGreaterThan`
**Kind:** `fn`

**Arguments:** `vec, val`

Returns `true` if the length of the vector is greater than `val`. The squared values are compared, so no square root is calculated.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.IsLengthLessThan`
**Kind:** `fn`

**Arguments:** `vec, val`

Returns `true` if the length of the vector is less than `val`. The squared values are compared, so no square root is calculated.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.NormalizeInPlaceSafe`
**Kind:** `fn`

**Arguments:** `vec, fallback`

Normalizes the vector so that its length is 1. If the length is 0, the components of `fallback` are copied into the vector.

**Source:** `mathlib/engine/vector2.lua`

  ### `Vector2D.NormalizedSafe`
**Kind:** `fn`

**Arguments:** `vec, fallback`

Returns a normalized copy of the vector. If the length is 0, a copy of `fallback` is returned.

**Source:** `mathlib/engine/vector2.lua`

---


## Vector4D

<!-- Source: cs2/api_reference/mathlib/vector4d.md -->

---
title: Vector 4D
icon: move-diagonal
---

# Vector 4D

## Vector4D" />



A vector with four float components. It is used by convars of type `CONVAR_TYPE_VECTOR4D`
and by homogeneous coordinates, for example in `VMatrix.V4Mul`.

The `+` and `-` operators add and subtract two vectors. The `*` and `/` operators take a
number or a vector. With a number, every component is multiplied or divided by it. With a
vector, each component is multiplied or divided by the matching component. The unary `-`
operator returns a negated copy, and the `#` operator returns the length. The `==` operator
compares every component and returns `false` when the other value is not a `Vector4D`.

```cpp lines=false
typedef struct Vector4D {
    float x, y, z, w;
} Vector4D;
```


  ### `Vector4D.w`
**Kind:** `prop`

The W component.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.x`
**Kind:** `prop`

The X component.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.y`
**Kind:** `prop`

The Y component.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.z`
**Kind:** `prop`

The Z component.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D + b`
**Kind:** `meta`

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D - b`
**Kind:** `meta`

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D * b`
**Kind:** `meta`

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D / b`
**Kind:** `meta`

**Source:** `mathlib/engine/vector4d.lua`

  ### `-Vector4D`
**Kind:** `meta`

**Source:** `mathlib/engine/vector4d.lua`

  ### `#Vector4D`
**Kind:** `meta`

**Source:** `mathlib/engine/vector4d.lua`


<Heading level={2} id="vector4d-1">
  Vector4D

  " />
</Heading>

Call `Vector4D()` to create a vector with every component set to 0. Call
`Vector4D(x, y, z, w)` to set the components. A component that is not given is set to 0.
Call `Vector4D(xyz, w)` with a `Vector` or `VectorAligned` to take X, Y and Z from it. If `w`
is not given, it is set to 0. Call `Vector4D(other)` to make a copy of another `Vector4D`.

Any other first argument throws an error.


  ### `Vector4D()`
**Kind:** `ctor`

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D(x, y, z, w)`
**Kind:** `ctor`

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D(xyz, w)`
**Kind:** `ctor`

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D(self)`
**Kind:** `ctor`

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.Unpack`
**Kind:** `fn`

**Arguments:** `v`

Returns the X, Y, Z and W components of the vector.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.Copy`
**Kind:** `fn`

**Arguments:** `src, dst`

Copies the components of one vector into another vector.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.Init`
**Kind:** `fn`

**Arguments:** `v, x?, y?, z?, w?`

Sets the components of the vector from four numbers. A component that is not given is set to 0.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.Init`
**Kind:** `fn`

**Arguments:** `v, xyz, w?`

Sets the components of the vector from a 3D vector and a W value.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.Zero`
**Kind:** `fn`

**Arguments:** `v`

Sets every component of the vector to 0.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.Negate`
**Kind:** `fn`

**Arguments:** `v`

Negates every component of the vector.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.MulAdd`
**Kind:** `fn`

**Arguments:** `v, a, b, scalar`

Sets the vector to `a + b * scalar`.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.Dot`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the dot product of two vectors. All four components are used.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.Length`
**Kind:** `fn`

**Arguments:** `v`

Returns the length of the vector. All four components are used.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.LengthSqr`
**Kind:** `fn`

**Arguments:** `v`

Returns the squared length of the vector. All four components are used.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.DistTo`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the distance between two points. All four components are used.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.DistToSqr`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the squared distance between two points. All four components are used.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.IsValid`
**Kind:** `fn`

**Arguments:** `v`

Returns `true` if every component is a finite number. NaN and infinity are not finite.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.IsZero`
**Kind:** `fn`

**Arguments:** `v, tolerance?`

Returns `true` if every component is greater than `-tolerance` and less than `tolerance`.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.AsVector3D`
**Kind:** `fn`

**Arguments:** `v`

Returns a `Vector` that shares the memory of the X, Y and Z components of this vector. Changes to the `Vector` also change this vector. Keep this vector alive for as long as you use the `Vector`.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.AsVector2D`
**Kind:** `fn`

**Arguments:** `v`

Returns a `Vector2D` that shares the memory of the X and Y components of this vector. Changes to the `Vector2D` also change this vector. Keep this vector alive for as long as you use the `Vector2D`.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.NormalizeInPlace`
**Kind:** `fn`

**Arguments:** `v`

Normalizes the vector so that its length is 1. If the length is 0, the vector stays at `(0, 0, 0, 0)`.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.Normalized`
**Kind:** `fn`

**Arguments:** `v`

Returns a normalized copy of the vector. If the length is 0, the copy is `(0, 0, 0, 0)`.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.Lerp`
**Kind:** `fn`

**Arguments:** `a, b, t`

Returns a new vector between two vectors. This is linear interpolation.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.Min`
**Kind:** `fn`

**Arguments:** `a, b`

Returns a vector that has the smaller value of each component of `a` and `b`.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.Max`
**Kind:** `fn`

**Arguments:** `a, b`

Returns a vector that has the larger value of each component of `a` and `b`.

**Source:** `mathlib/engine/vector4d.lua`

  ### `Vector4D.Abs`
**Kind:** `fn`

**Arguments:** `v`

Returns a copy of the vector with the absolute value of each component.

**Source:** `mathlib/engine/vector4d.lua`

---


## Vectoraligned

<!-- Source: cs2/api_reference/mathlib/vectoraligned.md -->

---
title: Aligned Vector
icon: scaling
---

# Aligned Vector

## VectorAligned" />



A 3D vector that is aligned to 16 bytes. It has a fourth component `w`. In a plain vector,
`w` is padding and is usually 0. Inside a `CTransform`, `w` holds the uniform scale. The
arithmetic operators treat `w` as a real component and include it in the result.

The `+`, `-`, `*` and `/` operators take a `VectorAligned` or a `Vector` on the right side.
A `Vector` is treated as having `w = 0`. The `*` and `/` operators also take a number, which
multiplies or divides all four components. The unary `-` operator negates all four
components. The `#` operator returns the length of X, Y and Z.

When a `Vector` is on the left side of an operator, the `Vector` operator is used, so the
result is a `Vector`.

The `==` operator compares only X, Y and Z, and it returns `false` when the other value is
not a `VectorAligned`. Use `IsEqual4D` to compare all four components.

```cpp lines=false
typedef struct __declspec(align(16)) VectorAligned {
    float x, y, z, w;
} VectorAligned;
```


  ### `VectorAligned.w`
**Kind:** `prop`

The fourth component. It is 0 for a plain vector and holds the scale inside a `CTransform`.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.x`
**Kind:** `prop`

The X component.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.y`
**Kind:** `prop`

The Y component.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.z`
**Kind:** `prop`

The Z component.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned + b`
**Kind:** `meta`

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned - b`
**Kind:** `meta`

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned * b`
**Kind:** `meta`

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned / b`
**Kind:** `meta`

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `-VectorAligned`
**Kind:** `meta`

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `#VectorAligned`
**Kind:** `meta`

**Source:** `mathlib/engine/vectoraligned.lua`


<Heading level={2} id="vectoraligned-1">
  VectorAligned

  " />
</Heading>

Call `VectorAligned()` to create a vector with every component set to 0. Call
`VectorAligned(x, y, z, w)` to set the components. A component that is not given is set to
0\. Call `VectorAligned(vec)` with a `Vector` to copy X, Y and Z and set `w` to 0. Call
`VectorAligned(other)` to make a copy of another `VectorAligned`.

Any other first argument throws an error.

The constant vectors below cannot be changed. Their `w` component is 0.


  ### `VectorAligned()`
**Kind:** `ctor`

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned(x, y, z, w)`
**Kind:** `ctor`

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned(vec)`
**Kind:** `ctor`

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned(self)`
**Kind:** `ctor`

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.vBackward`
**Kind:** `prop`

The vector `(-1, 0, 0)`.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.vDown`
**Kind:** `prop`

The vector `(0, 0, -1)`.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.vForward`
**Kind:** `prop`

The vector `(1, 0, 0)`.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.vLeft`
**Kind:** `prop`

The vector `(0, 1, 0)`.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.vNegativeOne`
**Kind:** `prop`

The vector `(-1, -1, -1)`.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.vOne`
**Kind:** `prop`

The vector `(1, 1, 1)`.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.vRight`
**Kind:** `prop`

The vector `(0, -1, 0)`.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.vUp`
**Kind:** `prop`

The vector `(0, 0, 1)`.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.vZero`
**Kind:** `prop`

The vector `(0, 0, 0)`.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.Unpack`
**Kind:** `fn`

**Arguments:** `vec`

Returns the X, Y, Z and W components of the vector.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.Copy`
**Kind:** `fn`

**Arguments:** `src, dst`

Copies all four components of one vector into another vector.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.Init`
**Kind:** `fn`

**Arguments:** `vec, x?, y?, z?`

Sets the X, Y and Z components of the vector and sets `w` to 0. A component that is not given is set to 0.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.InitZero`
**Kind:** `fn`

**Arguments:** `vec`

Sets all four components of the vector to 0.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.IsValid`
**Kind:** `fn`

**Arguments:** `vec`

Returns `true` if X, Y and Z are finite numbers. NaN and infinity are not finite. The `w` component is not checked.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.Length`
**Kind:** `fn`

**Arguments:** `vec`

Returns the length of X, Y and Z.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.Length2D`
**Kind:** `fn`

**Arguments:** `vec`

Returns the length of X and Y.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.Length4D`
**Kind:** `fn`

**Arguments:** `vec`

Returns the length of all four components.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.LengthSqr`
**Kind:** `fn`

**Arguments:** `vec`

Returns the squared length of X, Y and Z.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.LengthSqr2D`
**Kind:** `fn`

**Arguments:** `vec`

Returns the squared length of X and Y.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.NormalizeInPlace`
**Kind:** `fn`

**Arguments:** `vec`

Normalizes X, Y and Z so that their length is 1. The `w` component does not change. If the length is 0, X, Y and Z stay at 0.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.NormalizeInPlaceSafe`
**Kind:** `fn`

**Arguments:** `vec, fallback`

Normalizes X, Y and Z so that their length is 1. The `w` component does not change. If the length is 0, X, Y and Z are copied from `fallback`.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.Normalized`
**Kind:** `fn`

**Arguments:** `vec`

Returns a copy of the vector with X, Y and Z normalized. The `w` component does not change. If the length is 0, X, Y and Z of the copy are 0.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.NormalizedSafe`
**Kind:** `fn`

**Arguments:** `vec, fallback`

Returns a copy of the vector with X, Y and Z normalized. The `w` component does not change. If the length of X, Y and Z is 0, a copy of all four components of `fallback` is returned.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.NormalizeInPlace2D`
**Kind:** `fn`

**Arguments:** `vec`

Normalizes X and Y so that their length is 1. The Z and W components do not change. If the length is 0, X and Y stay at 0.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.NormalizeInPlaceSafe2D`
**Kind:** `fn`

**Arguments:** `vec, fallback`

Normalizes X and Y so that their length is 1. The Z and W components do not change. If the length is 0, X and Y are copied from `fallback`.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.Normalized2D`
**Kind:** `fn`

**Arguments:** `vec`

Returns a copy of the vector with X and Y normalized. The Z and W components do not change. If the length is 0, X and Y of the copy are 0.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.NormalizedSafe2D`
**Kind:** `fn`

**Arguments:** `vec, fallback`

Returns a copy of the vector with X and Y normalized. The Z and W components do not change. If the length of X and Y is 0, a copy of all four components of `fallback` is returned.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.Dot`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the dot product of X, Y and Z of two vectors.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.Dot2D`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the dot product of X and Y of two vectors.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.Cross`
**Kind:** `fn`

**Arguments:** `a, b`

Returns the cross product of X, Y and Z of two vectors. The `w` component of the result is 0.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.Abs`
**Kind:** `fn`

**Arguments:** `vec`

Returns a copy of the vector with the absolute value of all four components.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.Min`
**Kind:** `fn`

**Arguments:** `a, b`

Returns a vector that has the smaller value of each of the four components of `a` and `b`. A `Vector` is treated as having `w = 0`.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.Max`
**Kind:** `fn`

**Arguments:** `a, b`

Returns a vector that has the larger value of each of the four components of `a` and `b`. A `Vector` is treated as having `w = 0`.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.AsVector`
**Kind:** `fn`

**Arguments:** `vec`

Returns a new `Vector` with the X, Y and Z components of this vector.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.AsVector3D`
**Kind:** `fn`

**Arguments:** `vec`

Returns a new `Vector` with the X, Y and Z components of this vector. This is the same function as `AsVector`.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.LargestComponentValue`
**Kind:** `fn`

**Arguments:** `vec`

Returns the largest absolute value of X, Y and Z. The `w` component is ignored.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.SmallestComponentValue`
**Kind:** `fn`

**Arguments:** `vec`

Returns the smallest absolute value of X, Y and Z. The `w` component is ignored.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.IsZero`
**Kind:** `fn`

**Arguments:** `vec, tolerance?`

Returns `true` if X, Y and Z are each greater than `-tolerance` and less than `tolerance`.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.IsZero2D`
**Kind:** `fn`

**Arguments:** `vec, tolerance?`

Returns `true` if X and Y are each greater than `-tolerance` and less than `tolerance`.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.IsZero4D`
**Kind:** `fn`

**Arguments:** `vec, tolerance?`

Returns `true` if all four components are each greater than `-tolerance` and less than `tolerance`.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.EqualsZero`
**Kind:** `fn`

**Arguments:** `vec`

Returns `true` if X, Y and Z are exactly 0.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.EqualsZero2D`
**Kind:** `fn`

**Arguments:** `vec`

Returns `true` if X and Y are exactly 0.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.EqualsZero4D`
**Kind:** `fn`

**Arguments:** `vec`

Returns `true` if all four components are exactly 0.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.IsEqual2D`
**Kind:** `fn`

**Arguments:** `a, b`

Returns `true` if X and Y of the two vectors are exactly equal.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.IsEqual4D`
**Kind:** `fn`

**Arguments:** `a, b`

Returns `true` if all four components of the two vectors are exactly equal.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.IsApproximatelyEqual`
**Kind:** `fn`

**Arguments:** `a, b, tolerance?`

Returns `true` if the difference between X, Y and Z of the two vectors is less than `tolerance` for each component.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.IsApproximatelyEqual2D`
**Kind:** `fn`

**Arguments:** `a, b, tolerance?`

Returns `true` if the difference between X and Y of the two vectors is less than `tolerance` for each component.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.IsApproximatelyEqual4D`
**Kind:** `fn`

**Arguments:** `a, b, tolerance?`

Returns `true` if the difference between all four components of the two vectors is less than `tolerance` for each component.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.IsApproximatelyZero`
**Kind:** `fn`

**Arguments:** `vec, tolerance?`

Returns `true` if X, Y and Z are each greater than `-tolerance` and less than `tolerance`. This is the same as `IsZero` with a smaller default tolerance.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.IsApproximatelyZero2D`
**Kind:** `fn`

**Arguments:** `vec, tolerance?`

Returns `true` if X and Y are each greater than `-tolerance` and less than `tolerance`. This is the same as `IsZero2D` with a smaller default tolerance.

**Source:** `mathlib/engine/vectoraligned.lua`

  ### `VectorAligned.IsApproximatelyZero4D`
**Kind:** `fn`

**Arguments:** `vec, tolerance?`

Returns `true` if all four components are each greater than `-tolerance` and less than `tolerance`. This is the same as `IsZero4D` with a smaller default tolerance.

**Source:** `mathlib/engine/vectoraligned.lua`

---


## Vmatrix

<!-- Source: cs2/api_reference/mathlib/vmatrix.md -->

---
title: Matrix 4x4
icon: table
---

# Matrix 4x4

## VMatrix" />



A 4x4 matrix of floats stored row by row. The projection matrix and the view projection matrix
of the view setup use this type. The first three rows have the same layout as a `matrix3x4_t`.

Read or write elements with `vm.m[row][col]`. Indices start at zero.

```cpp lines=false
typedef struct VMatrix {
    float m[4][4];
} VMatrix;
```

The `==` and `~=` operators compare every element exactly. `tostring` returns all sixteen
elements.

The `*` operator with a `Vector` on the right works in the same way as `VMul4x3`. With a
`VPlane` on the right, it works in the same way as `TransformPlane`. With a matrix on the right,
it works in the same way as `MatrixMul`. The `+`, `-` and unary `-` operators work on each
element.


  ### `VMatrix.m`
**Kind:** `prop`

The elements, stored as 4 rows of 4 numbers. Indices start at zero.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix * b`
**Kind:** `meta`

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix * b`
**Kind:** `meta`

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix * b`
**Kind:** `meta`

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix + b`
**Kind:** `meta`

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix - b`
**Kind:** `meta`

**Source:** `mathlib/engine/vmatrix.lua`

  ### `-VMatrix`
**Kind:** `meta`

**Source:** `mathlib/engine/vmatrix.lua`


<Heading level={2} id="vmatrix-1">
  VMatrix

  " />
</Heading>

The constructor without arguments creates an identity matrix. A `matrix3x4_t` is copied into
the first three rows, and the last row is set to `(0, 0, 0, 1)`. Any other argument type throws
an error.


  ### `VMatrix()`
**Kind:** `ctor`

Creates an identity matrix.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix(m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23, m30, m31, m32, m33)`
**Kind:** `ctor`

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix(forward, left, up)`
**Kind:** `ctor`

Sets the first three columns to the axes. The translation is zero.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix(matrix)`
**Kind:** `ctor`

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix(self)`
**Kind:** `ctor`

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.Init`
**Kind:** `fn`

**Arguments:** `vm, m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23, m30, m31, m32, m33`

Sets all 16 elements in place. Pass the elements row by row.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.Init`
**Kind:** `fn`

**Arguments:** `vm, matrix`

Sets the matrix in place from a 3x4 matrix. The first three rows are copied, and the last row is set to `(0, 0, 0, 1)`.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.InitFLU`
**Kind:** `fn`

**Arguments:** `vm, forward, left, up, origin`

Sets the matrix in place from the forward, left and up axes and an origin. The last row is set to `(0, 0, 0, 1)`.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.Identity`
**Kind:** `fn`

**Arguments:** `vm`

Sets the matrix to the identity matrix in place.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.Copy`
**Kind:** `fn`

**Arguments:** `src, dst`

Copies all 16 elements of `src` into `dst`.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.SetupMatrixOrgAngles`
**Kind:** `fn`

**Arguments:** `vm, origin, angles`

Sets the matrix in place to a rotation from `angles` and a translation from `origin`. The last row is set to `(0, 0, 0, 1)`.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.GetForward`
**Kind:** `fn`

**Arguments:** `vm`

Returns the first column of the first three rows, which is the forward axis.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.GetLeft`
**Kind:** `fn`

**Arguments:** `vm`

Returns the second column of the first three rows, which is the left axis.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.GetUp`
**Kind:** `fn`

**Arguments:** `vm`

Returns the third column of the first three rows, which is the up axis.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.GetTranslation`
**Kind:** `fn`

**Arguments:** `vm`

Returns the fourth column of the first three rows, which is the translation.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.GetOrigin`
**Kind:** `fn`

**Arguments:** `vm`

Returns the translation. This is the same function as `GetTranslation`, with the name that `matrix3x4_t` uses.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.GetBasisVectors`
**Kind:** `fn`

**Arguments:** `vm`

Returns the forward, left and up axes, which are the first three columns of the first three rows.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.GetScale`
**Kind:** `fn`

**Arguments:** `vm`

Returns the lengths of the forward, left and up axes in `x`, `y` and `z`.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.SetForward`
**Kind:** `fn`

**Arguments:** `vm, forward`

Sets the first column of the first three rows, which is the forward axis.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.SetLeft`
**Kind:** `fn`

**Arguments:** `vm, left`

Sets the second column of the first three rows, which is the left axis.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.SetUp`
**Kind:** `fn`

**Arguments:** `vm, up`

Sets the third column of the first three rows, which is the up axis.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.SetBasisVectors`
**Kind:** `fn`

**Arguments:** `vm, forward, left, up`

Sets the first three columns of the first three rows to the forward, left and up axes.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.SetTranslation`
**Kind:** `fn`

**Arguments:** `vm, translation`

Sets the fourth column of the first three rows, which is the translation.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.PreTranslate`
**Kind:** `fn`

**Arguments:** `vm, translation`

Applies a translation in the input space of the matrix. The translation column is set to `vm:VMul4x3(translation)`.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.PostTranslate`
**Kind:** `fn`

**Arguments:** `vm, translation`

Applies a translation in the output space of the matrix. The offset is added to the translation column.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.Scale`
**Kind:** `fn`

**Arguments:** `vm, scale`

Returns a new matrix with the first three columns scaled. The first column of every row is multiplied by `scale.x`, the second column by `scale.y` and the third column by `scale.z`. The first three elements of the fourth column do not change.

This function is meant for affine matrices. The bottom right element of the result is always
      set to 1.
    
  

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.ScaleUpper3x3Matrix`
**Kind:** `fn`

**Arguments:** `vm, scale`

Multiplies the first three columns of the first three rows by a number in place.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.NormalizeBasisVectors`
**Kind:** `fn`

**Arguments:** `vm`

Returns a copy in which the forward, left and up axes have a length of 1. An axis with a length of zero does not change.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.As3x4`
**Kind:** `fn`

**Arguments:** `vm`

Returns a reference to the first three rows as a `matrix3x4_t`. Changes to the returned value change the original matrix. Keep the original matrix alive while you use the reference. Use `Set3x4` to get a separate copy.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.CopyFrom3x4`
**Kind:** `fn`

**Arguments:** `vm, matrix`

Copies a 3x4 matrix into the first three rows in place. The last row is set to `(0, 0, 0, 1)`.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.Set3x4`
**Kind:** `fn`

**Arguments:** `vm`

Copies the first three rows into a new `matrix3x4_t` and returns it.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.IsRotationMatrix`
**Kind:** `fn`

**Arguments:** `vm`

Returns true if the first three elements of each of the first three rows form vectors with a length of 1 that are at right angles to each other. The allowed difference is 0.01.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.IsIdentity`
**Kind:** `fn`

**Arguments:** `vm`

Returns true if every element is exactly equal to the same element of the identity matrix.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.IsValid`
**Kind:** `fn`

**Arguments:** `vm`

Returns true if no element is NaN or infinite.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.V3Mul`
**Kind:** `fn`

**Arguments:** `vm, v`

Multiplies a point by the full 4x4 matrix, with the `w` component of the point set to 1. Then the result is divided by its `w` component. This is the perspective divide.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.V4Mul`
**Kind:** `fn`

**Arguments:** `vm, v`

Multiplies a vector with four components by the full 4x4 matrix.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.ApplyRotation`
**Kind:** `fn`

**Arguments:** `vm, v`

Multiplies a direction by the 3x3 rotation part. The translation is not added. This is the same function as `VMul3x3`.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.VMul3x3`
**Kind:** `fn`

**Arguments:** `vm, v`

Multiplies a direction by the 3x3 rotation part. The translation is not added.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.VMul3x3Transpose`
**Kind:** `fn`

**Arguments:** `vm, v`

Multiplies a direction by the transposed 3x3 rotation part. For a pure rotation matrix, this is the inverse rotation.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.VMul4x3`
**Kind:** `fn`

**Arguments:** `vm, v`

Multiplies a point by the first three rows, and adds the translation. The last row is not used. This is the same as the `*` operator with a `Vector`.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.VMul4x3Transpose`
**Kind:** `fn`

**Arguments:** `vm, v`

Applies the inverse of a matrix that contains only a rotation and a translation. The translation is subtracted, and then the point is multiplied by the transposed 3x3 rotation part.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.MatrixMul`
**Kind:** `fn`

**Arguments:** `vm, other`

Returns the product `vm * other`. When the result is applied to a point, `other` is applied first. This is the same as the `*` operator with a matrix.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.TransformPlane`
**Kind:** `fn`

**Arguments:** `vm, plane`

Transforms a plane by the rotation and the translation. This is the same as the `*` operator with a `VPlane`.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.InverseGeneral`
**Kind:** `fn`

**Arguments:** `vm`

Returns the inverse of any matrix. The first return value is true on success.

If the matrix cannot be inverted, or is too close to a matrix that cannot be inverted, the
      function returns false and an identity matrix. This happens when no pivot element larger than
      `1e-6` can be found.
    
  

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.InverseTR`
**Kind:** `fn`

**Arguments:** `vm`

Returns the inverse of a matrix that contains only a rotation and a translation. This is faster than `InverseGeneral`. The result is wrong if the matrix contains scale or projection.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.Transpose`
**Kind:** `fn`

**Arguments:** `vm`

Returns the transposed matrix.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.Transpose3x3`
**Kind:** `fn`

**Arguments:** `vm`

Returns a copy with only the 3x3 rotation part transposed. The other elements do not change.

**Source:** `mathlib/engine/vmatrix.lua`

  ### `VMatrix.GetIdentityMatrix`
**Kind:** `fn`

Returns a new identity matrix. Each call returns a new copy.

**Source:** `mathlib/engine/vmatrix.lua`

---


## Vplane

<!-- Source: cs2/api_reference/mathlib/vplane.md -->

---
title: Plane
icon: square
---

# Plane

## VPlane" />



A plane, stored as a normal with a length of 1 and a distance along that normal. A point
`p` is on the plane when `p:Dot(m_Normal) == m_Dist`. The side that the normal points to is
the front of the plane.

The `SECONDARYVIEWMODE_PLANAR_REFLECTION` render attribute uses a plane. Both matrix types
can transform a plane with `TransformPlane`. `mathlib` can build reflection and projection
matrices from planes.

The `==` operator compares the normal and the distance and returns `false` when the other
value is not a `VPlane`.

```cpp lines=false
typedef struct VPlane {
    Vector m_Normal;
    float m_Dist;
} VPlane;
```


  ### `VPlane.m_Dist`
**Kind:** `prop`

The distance from the origin to the plane, measured along the normal.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.m_Normal`
**Kind:** `prop`

The normal of the plane. It should have a length of 1.

**Source:** `mathlib/engine/vplane.lua`


<Heading level={2} id="vplane-1">
  VPlane

  " />
</Heading>

Call `VPlane()` to create a plane with every field set to 0. Call `VPlane(normal, dist)` to
set the normal and the distance. The normal is stored as given. Call `VPlane(normal, point)`
to create a plane through a point. The normal is normalized first. Call `VPlane(a, b, c)` to
create a plane through three points in clockwise order. Call `VPlane(other)` to make a copy
of another plane.

Any other arguments throw an error.


  ### `VPlane()`
**Kind:** `ctor`

A plane with every field set to 0.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane(normal, dist)`
**Kind:** `ctor`

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane(normal, point)`
**Kind:** `ctor`

The normal is normalized.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane(a, b, c)`
**Kind:** `ctor`

A plane through three points in clockwise order.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane(self)`
**Kind:** `ctor`

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.Init`
**Kind:** `fn`

**Arguments:** `plane, normal, dist`

Sets the normal and the distance of the plane. The normal is stored as given.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.InitFromPointAndNormal`
**Kind:** `fn`

**Arguments:** `plane, point, normal`

Sets the plane so that it goes through a point. The normal is normalized first.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.InitFromClockwisePoints`
**Kind:** `fn`

**Arguments:** `plane, a, b, c`

Sets the plane so that it goes through three points. The normal is the normalized cross product of `c - a` and `b - a`.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.Copy`
**Kind:** `fn`

**Arguments:** `src, dst`

Copies one plane into another plane.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.DistTo`
**Kind:** `fn`

**Arguments:** `plane, point`

Returns the signed distance from a point to the plane. The distance is positive in front of the plane and negative behind it.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.IntersectLineSegment`
**Kind:** `fn`

**Arguments:** `plane, start, stop`

Finds the point where a segment crosses the plane. If the segment crosses the plane, this function returns `true` and the point. Otherwise it returns `false` and `nil`.

If the segment is parallel to the plane and lies on the plane, this function returns `true`
      and a copy of `start`.
    
  

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.GetPointSide`
**Kind:** `fn`

**Arguments:** `plane, point, epsilon?`

Returns the side of the plane that a point is on, as a `SideType` value. A point is on the plane when its distance to the plane is less than `epsilon`.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.GetPointSideExact`
**Kind:** `fn`

**Arguments:** `plane, point`

Returns `SIDE_FRONT` if the point is in front of the plane. Otherwise it returns `SIDE_BACK`. A point that is exactly on the plane returns `SIDE_BACK`. This function never returns `SIDE_ON`.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.BoxOnPlaneSide`
**Kind:** `fn`

**Arguments:** `plane, mins, maxs`

Returns the side of the plane that a box is on, as a `SideType` value. If the box is completely in front of the plane, it returns `SIDE_FRONT`. If it is completely behind the plane, it returns `SIDE_BACK`. If the plane cuts through the box, it returns `SIDE_ON`.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.BoxOnPlaneSide`
**Kind:** `fn`

**Arguments:** `plane, bounds`

Returns the side of the plane that a box is on. This is the same as the version with `mins` and `maxs`, with the box given as one `AABB_t`.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.PointInFront`
**Kind:** `fn`

**Arguments:** `plane, point`

Returns `true` if the point is in front of the plane. A point that is exactly on the plane returns `false`.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.ReflectPoint`
**Kind:** `fn`

**Arguments:** `plane, point`

Moves a point to its mirror position on the other side of the plane. The point that you pass is changed.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.ReflectDirection`
**Kind:** `fn`

**Arguments:** `plane, dir`

Mirrors a direction across the plane. The direction that you pass is changed.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.ConvertToPlaneEquation`
**Kind:** `fn`

**Arguments:** `plane`

Returns the plane as the equation `Ax + By + Cz + D = 0`. The X, Y and Z components of the result are the normal, and the W component is `-m_Dist`.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.Flip`
**Kind:** `fn`

**Arguments:** `plane`

Returns a new plane in the same position that faces the other way. Both the normal and the distance are negated.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.GetPointOnPlane`
**Kind:** `fn`

**Arguments:** `plane`

Returns the point of the plane that is nearest to the origin. This is `m_Normal * m_Dist`.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.SnapPointToPlane`
**Kind:** `fn`

**Arguments:** `plane, point`

Moves a point along the normal until it is on the plane, and returns the new point.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.CalculateClosestPointOnPlane`
**Kind:** `fn`

**Arguments:** `plane, point`

Returns the point of the plane that is nearest to the given point.

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.IntersectWithPlane`
**Kind:** `fn`

**Arguments:** `plane, other`

Finds the line where two planes meet. This function returns `true` and two points on that line. If the planes are parallel, it returns `false` and `nil` for both points.

The second point is the first point plus the cross product of the two normals.
    
  

**Source:** `mathlib/engine/vplane.lua`

  ### `VPlane.IsZero`
**Kind:** `fn`

**Arguments:** `plane`

Returns `true` if the distance is exactly 0 and each component of the normal is between -1e-9 and 1e-9.

**Source:** `mathlib/engine/vplane.lua`


## SideType _()_

The side of a plane that a point or a box is on.



## Frustum\_t" />



A view frustum with up to eight planes. The normals of the planes point into the frustum.
The planes are stored in two `fourplanes_t` groups, with four planes in each group. Each
plane uses one lane of the SIMD arrays in its group.

The `m_frustumStruct` field of the view setup has this type.
`mathlib.FrustumPlanesFromMatrix` builds a frustum from a clip to world matrix.

A lane that is not used has a normal of `(0, 0, 0)` and a distance of `-FLT_MAX`, so it never
culls anything.

```cpp lines=false
typedef struct __declspec(align(16)) fourplanes_t {
    float nX[4], nY[4], nZ[4], dist[4];
    float xSign[4], ySign[4], zSign[4];
    float nXAbs[4], nYAbs[4], nZAbs[4];
} fourplanes_t;

typedef struct __declspec(align(16)) Frustum_t {
    fourplanes_t m_Planes[2];
} Frustum_t;
```


  ### `Frustum_t.m_Planes`
**Kind:** `prop`

An array of 2 groups. The first index is 0.

**Source:** `mathlib/engine/vplane.lua`


## fourplanes\_t _()_


  ### `fourplanes_t.dist`
**Kind:** `prop`

The distance for each of the 4 lanes.

**Source:** `mathlib/engine/vplane.lua`

  ### `fourplanes_t.nX`
**Kind:** `prop`

The X component of the normal for each of the 4 lanes.

**Source:** `mathlib/engine/vplane.lua`

  ### `fourplanes_t.nXAbs`
**Kind:** `prop`

The absolute value of the X component of the normal for each of the 4 lanes.

**Source:** `mathlib/engine/vplane.lua`

  ### `fourplanes_t.nY`
**Kind:** `prop`

The Y component of the normal for each of the 4 lanes.

**Source:** `mathlib/engine/vplane.lua`

  ### `fourplanes_t.nYAbs`
**Kind:** `prop`

The absolute value of the Y component of the normal for each of the 4 lanes.

**Source:** `mathlib/engine/vplane.lua`

  ### `fourplanes_t.nZ`
**Kind:** `prop`

The Z component of the normal for each of the 4 lanes.

**Source:** `mathlib/engine/vplane.lua`

  ### `fourplanes_t.nZAbs`
**Kind:** `prop`

The absolute value of the Z component of the normal for each of the 4 lanes.

**Source:** `mathlib/engine/vplane.lua`

  ### `fourplanes_t.xSign`
**Kind:** `prop`

For each of the 4 lanes, all bits are set when the X component of the normal is negative.

**Source:** `mathlib/engine/vplane.lua`

  ### `fourplanes_t.ySign`
**Kind:** `prop`

The same as `xSign`, for the Y component.

**Source:** `mathlib/engine/vplane.lua`

  ### `fourplanes_t.zSign`
**Kind:** `prop`

The same as `xSign`, for the Z component.

**Source:** `mathlib/engine/vplane.lua`


<Heading level={2} id="frustum_t-1">
  Frustum_t

  " />
</Heading>

Call `Frustum_t()` to create a frustum with no planes, which culls nothing. Call
`Frustum_t(planes)` with a Lua array of up to eight `VPlane` values to store them in order.
Call `Frustum_t(other)` to make a copy of another frustum. Any other argument throws an
error.


  ### `Frustum_t()`
**Kind:** `ctor`

A frustum with no planes, which culls nothing.

**Source:** `mathlib/engine/vplane.lua`

  ### `Frustum_t(planes)`
**Kind:** `ctor`

A frustum with up to eight planes.

**Source:** `mathlib/engine/vplane.lua`

  ### `Frustum_t(self)`
**Kind:** `ctor`

**Source:** `mathlib/engine/vplane.lua`

  ### `Frustum_t.SetPlane`
**Kind:** `fn`

**Arguments:** `frustum, index, plane`

Stores a plane at an index and updates its sign and absolute value lanes. Indexes 0 to 3 are in the first group, and indexes 4 to 7 are in the second group.

**Source:** `mathlib/engine/vplane.lua`

  ### `Frustum_t.GetPlane`
**Kind:** `fn`

**Arguments:** `frustum, index`

Returns the plane at an index.

**Source:** `mathlib/engine/vplane.lua`

  ### `Frustum_t.SetPlanes`
**Kind:** `fn`

**Arguments:** `frustum, planes`

Removes every plane, then stores the planes of a Lua array in order, starting at index 0.

**Source:** `mathlib/engine/vplane.lua`

  ### `Frustum_t.GetPlanes`
**Kind:** `fn`

**Arguments:** `frustum, count?`

Returns the first `count` planes as a Lua array.

**Source:** `mathlib/engine/vplane.lua`

  ### `Frustum_t.Clear`
**Kind:** `fn`

**Arguments:** `frustum`

Removes every plane, so the frustum culls nothing.

**Source:** `mathlib/engine/vplane.lua`

  ### `Frustum_t.CullPoint`
**Kind:** `fn`

**Arguments:** `frustum, point`

Returns `true` if the point is behind any plane of the frustum. A point that is exactly on a plane is not culled.

**Source:** `mathlib/engine/vplane.lua`

  ### `Frustum_t.CullBox`
**Kind:** `fn`

**Arguments:** `frustum, mins, maxs`

Returns `true` if the box is completely behind any plane of the frustum.

**Source:** `mathlib/engine/vplane.lua`

  ### `Frustum_t.CullBox`
**Kind:** `fn`

**Arguments:** `frustum, bounds`

Returns `true` if the box is completely behind any plane of the frustum. This is the same as the version with `mins` and `maxs`, with the box given as one `AABB_t`.

**Source:** `mathlib/engine/vplane.lua`

---


## Http

<!-- Source: cs2/api_reference/networking/http.md -->

---
title: HTTP
icon: router
---

# HTTP

<Heading level={2} id="http-1">
  http

   _()_
</Heading>

Sends HTTP requests. Every request runs in the background, and the response is passed to
`callback` when it arrives.

Add `http` to the `requirements` in `extension.json` before you call `require("http")`.

You can leave out `options`. So `http.Get(url, callback)` and `http.Get(url, options, callback)`
both work. A value in the place of `options` that is not a table, such as `false`, counts as no
options.

A callback runs only if the script is still loaded when the response arrives. When another
script is turned on or off, or a developer script is saved, all scripts are reloaded and the
callbacks that are waiting are dropped. In that case the `shutdown` callback of the script
receives `"refresh"` or `"reload"`, and the script can send the request again.

A script can have at most 64 requests that have not finished. Past that, a new request throws
an error and is not sent.

A header name that is not a string throws an error. A header name or value that contains CR, LF
or NUL also throws an error. An option key that is not listed in `http.Options` is ignored, and
the request is still sent. In a developer script, an unknown key also adds a warning to the
errors of the script, once for each key.

Redirects are followed. Only the `http` and `https` schemes are allowed. Certificates are always
checked.

Market scripts cannot send requests to the computer of the user or to a private network. Such
requests fail with the `"blocked"` error.


  ### `http.Get`
**Kind:** `fn`

**Arguments:** `url, options?, callback`

Sends a `GET` request.

**Source:** `networking/framework/http.lua`

  ### `http.Head`
**Kind:** `fn`

**Arguments:** `url, options?, callback`

Sends a `HEAD` request. The server returns only the headers, so the body of the response is empty.

**Source:** `networking/framework/http.lua`

  ### `http.Post`
**Kind:** `fn`

**Arguments:** `url, options?, callback`

Sends a `POST` request.

**Source:** `networking/framework/http.lua`

  ### `http.Put`
**Kind:** `fn`

**Arguments:** `url, options?, callback`

Sends a `PUT` request.

**Source:** `networking/framework/http.lua`

  ### `http.Delete`
**Kind:** `fn`

**Arguments:** `url, options?, callback`

Sends a `DELETE` request.

**Source:** `networking/framework/http.lua`

  ### `http.Patch`
**Kind:** `fn`

**Arguments:** `url, options?, callback`

Sends a `PATCH` request.

**Source:** `networking/framework/http.lua`

  ### `http.Cookies`
**Kind:** `fn`

Creates a new cookie container. Pass it in `options.cookie` of a request.

**Source:** `networking/framework/http.lua`

  ### `http.Error`
**Kind:** `type`

The reason why no response arrived.

**Source:** `networking/framework/http.lua`

  ### `http.Callback`
**Kind:** `type`

**Source:** `networking/framework/http.lua`


## http.Response _()_

The response to a request. `status` is `0` only when `error` is set. An HTTP error status such
as `404` is still a response, so `error` is `nil` for it.


  ### `http.Response.body`
**Kind:** `prop`

The body of the response.

**Source:** `networking/framework/http.lua`

  ### `http.Response.error`
**Kind:** `prop`

The reason why no response arrived. It is `nil` when a response arrived.

**Source:** `networking/framework/http.lua`

  ### `http.Response.status`
**Kind:** `prop`

The HTTP status code. It is `0` when no response arrived.

**Source:** `networking/framework/http.lua`


## http.Authorization _()_


  ### `http.Authorization.password`
**Kind:** `prop`

**Source:** `networking/framework/http.lua`

  ### `http.Authorization.username`
**Kind:** `prop`

**Source:** `networking/framework/http.lua`


## http.Options _()_


  ### `http.Options.authorization`
**Kind:** `prop`

The user name and password for HTTP authentication.

**Source:** `networking/framework/http.lua`

  ### `http.Options.cookie`
**Kind:** `prop`

A cookie container. Its cookies are sent with the request, and the cookies from the response are added to it.

**Source:** `networking/framework/http.lua`

  ### `http.Options.headers`
**Kind:** `prop`

The request headers, with header names as keys.

**Source:** `networking/framework/http.lua`

  ### `http.Options.json`
**Kind:** `prop`

A JSON string to send as the body. It sets the `Content-Type: application/json` header. When `json` is set, `params` is ignored.

**Source:** `networking/framework/http.lua`

  ### `http.Options.params`
**Kind:** `prop`

The parameters, as a table of names and values or as a string that is already encoded. For `POST`, `PUT` and `PATCH` they are sent in the body as form data. For other methods they are added to the query string of the URL. A string that contains CR, LF or NUL throws an error.

**Source:** `networking/framework/http.lua`

  ### `http.Options.timeout`
**Kind:** `prop`

The time limit for the whole request, in seconds. The default is `30`. The value is clamped to the range from `1` to `300`.

**Source:** `networking/framework/http.lua`


## http.Cookie _()_


  ### `http.Cookie.expire`
**Kind:** `prop`

The time the cookie expires, as a Unix timestamp in seconds.

**Source:** `networking/framework/http.lua`

  ### `http.Cookie.host`
**Kind:** `prop`

**Source:** `networking/framework/http.lua`

  ### `http.Cookie.name`
**Kind:** `prop`

**Source:** `networking/framework/http.lua`

  ### `http.Cookie.path`
**Kind:** `prop`

The path of the cookie. The default is `"/"`.

**Source:** `networking/framework/http.lua`

  ### `http.Cookie.secure`
**Kind:** `prop`

`true` to send the cookie only over HTTPS.

**Source:** `networking/framework/http.lua`

  ### `http.Cookie.subdomains`
**Kind:** `prop`

`true` to also send the cookie to subdomains of `host`.

**Source:** `networking/framework/http.lua`

  ### `http.Cookie.value`
**Kind:** `prop`

**Source:** `networking/framework/http.lua`


## http.Cookies _()_

A container for cookies. It holds cookies to send with requests, and it collects the cookies
that servers return. Use the same container for several requests to keep the cookies between
them. Create a container with `http.Cookies`.


  ### `http.Cookies:Set`
**Kind:** `method`

**Arguments:** `cookie`

Adds a cookie to the container. If `expire` is `0`, the cookie is a session cookie.

Throws an error if a string field contains CR, LF, NUL or TAB.
    
  

**Source:** `networking/framework/http.lua`

---


## Netchan

<!-- Source: cs2/api_reference/networking/netchan.md -->

---
title: Network Channel
icon: arrow-down-up
---

# Network Channel

## netchan _()_

Statistics for the current network connection to the game server. All fields are read-only.

Each field is read from the engine's net channel at the moment you access it. When the game is
not connected to a server, every field is **nil**.


  ### `netchan.address`
**Kind:** `prop`

The address of the server.

**Source:** `networking/game/netchan.lua`

  ### `netchan.avg_data`
**Kind:** `prop`

The average amount of data in bytes per second, for each direction.

**Source:** `networking/game/netchan.lua`

  ### `netchan.avg_latency`
**Kind:** `prop`

The average network ping, in seconds.

**Source:** `networking/game/netchan.lua`

  ### `netchan.avg_packets`
**Kind:** `prop`

The average number of packets per second, for each direction.

**Source:** `networking/game/netchan.lua`

  ### `netchan.data_rate`
**Kind:** `prop`

The data rate set for the connection, in bytes per second.

**Source:** `networking/game/netchan.lua`

  ### `netchan.dropped`
**Kind:** `prop`

The fraction of packets that were dropped, for each direction.

**Source:** `networking/game/netchan.lua`

  ### `netchan.engine_latency`
**Kind:** `prop`

The outgoing ping that the engine measures, in seconds.

**Source:** `networking/game/netchan.lua`

  ### `netchan.is_localhost`
**Kind:** `prop`

Whether the server runs on the local computer.

**Source:** `networking/game/netchan.lua`

  ### `netchan.is_loopback`
**Kind:** `prop`

Whether the connection is a loopback connection.

**Source:** `networking/game/netchan.lua`

  ### `netchan.is_playback`
**Kind:** `prop`

Whether a demo is playing. This is `false` for a live connection.

**Source:** `networking/game/netchan.lua`

  ### `netchan.is_timing_out`
**Kind:** `prop`

Whether the connection is timing out at the moment.

**Source:** `networking/game/netchan.lua`

  ### `netchan.late`
**Kind:** `prop`

The fraction of packets that arrived too late, for each direction.

**Source:** `networking/game/netchan.lua`

  ### `netchan.name`
**Kind:** `prop`

The name of the connection.

**Source:** `networking/game/netchan.lua`

  ### `netchan.peak_jitter`
**Kind:** `prop`

The highest delivery jitter in milliseconds, for each direction.

**Source:** `networking/game/netchan.lua`

  ### `netchan.sequence_nr`
**Kind:** `prop`

The current packet sequence number, for each direction.

**Source:** `networking/game/netchan.lua`

  ### `netchan.time`
**Kind:** `prop`

The current time of the net channel, in seconds.

**Source:** `networking/game/netchan.lua`

  ### `netchan.time_connected`
**Kind:** `prop`

The number of seconds since the connection was made.

**Source:** `networking/game/netchan.lua`

  ### `netchan.time_since_last_received`
**Kind:** `prop`

The number of seconds since the last packet was received.

**Source:** `networking/game/netchan.lua`

  ### `netchan.time_until_timeout`
**Kind:** `prop`

The number of seconds that are left before the connection times out.

**Source:** `networking/game/netchan.lua`

  ### `netchan.timeout`
**Kind:** `prop`

The connection timeout in seconds.

**Source:** `networking/game/netchan.lua`

  ### `netchan.total_data`
**Kind:** `prop`

The total number of bytes sent or received, for each direction.

**Source:** `networking/game/netchan.lua`

  ### `netchan.total_packets`
**Kind:** `prop`

The total number of packets sent or received, for each direction.

**Source:** `networking/game/netchan.lua`

  ### `netchan.RemoteFrametime`
**Kind:** `fn`

Returns the frame timing of the server, in seconds. The three values are the frame time, the standard deviation of the frame time, and the standard deviation of the frame start time.

When the game is not connected to a server, all three values are **nil**.
    
  

**Source:** `networking/game/netchan.lua`

  ### `netchan.GetInstance`
**Kind:** `fn`

Returns a pointer to the net channel.

<Warning>
        Use this pointer only with FFI. The pointer is owned by the connection. Do not keep it after the
        game disconnects from the server.
      </Warning>
    
  

**Source:** `networking/game/netchan.lua`

  ### `netchan.FLOW.Value`
**Kind:** `type`

A table with one value for each direction of traffic. Use `netchan.FLOW` as the index. `[0]` is the value for outgoing traffic and `[1]` is the value for incoming traffic.

**Source:** `networking/game/netchan.lua`

  ### `netchan.FLOW`
**Kind:** `type`

The direction of traffic. Use it as the index of a `netchan.FLOW.Value`.


  

**Source:** `networking/game/netchan.lua`

---


## Protobuf

<!-- Source: cs2/api_reference/networking/protobuf.md -->

---
title: Protocol Buffers
icon: code
---

# Protocol Buffers

## protobuf.MessageWrapper _()_

A protobuf message, such as the message that a `usermsg_*` event passes. Read its fields by
name. A field that the message does not declare reads as **nil**. A repeated field reads as a
table, and a field that holds a message reads as another `protobuf.MessageWrapper`.

`pairs` goes over every field that the message declares. `tostring` returns
`protobuf.Message<Name>`, where `Name` is the name of the message type.

## protobuf _()_

Shows the game's own user messages on the local client. The messages are not sent to the
server or to other players.


  ### `protobuf.TextMsg`
**Kind:** `fn`

**Arguments:** `dest, msg, param1?, param2?, param3?, param4?`

Shows a `TextMsg` message on the local client, in the same way as the game shows its own status messages.

`msg` is plain text or a localization token that starts with `#`. The token can contain the
      placeholders `%s1`, `%s2`, `%s3` and `%s4`. They are replaced by `param1`, `param2`, `param3`
      and `param4`. Parameters that are **nil** are skipped, so the parameters after them move up by
      one place.
    
  

**Source:** `networking/game/protobuf.lua`

  ### `protobuf.SayText`
**Kind:** `fn`

**Arguments:** `chat, entity?, text`

Shows a `SayText` chat line on the local client, as if the given player sent it.

**Source:** `networking/game/protobuf.lua`

  ### `protobuf.SayText2`
**Kind:** `fn`

**Arguments:** `chat, entity, messagename, param1?, param2?, param3?, param4?`

Shows a `SayText2` chat line on the local client. This is the localized and formatted form of a chat message.

`messagename` is a localization token, for example `Cstrike_Chat_All`. The placeholders `%s1`,
      `%s2`, `%s3` and `%s4` in the token are replaced by `param1`, `param2`, `param3` and `param4`.
    
  

**Source:** `networking/game/protobuf.lua`

  ### `protobuf.HudPrintDest`
**Kind:** `type`

The place where `protobuf.TextMsg` shows a message. The values are the game's own `HUD_PRINT*` constants.


  

**Source:** `networking/game/protobuf.lua`

---


## Websocket

<!-- Source: cs2/api_reference/networking/websocket.md -->

---
title: WebSocket
icon: cable
---

# WebSocket

<Heading level={2} id="websocket-1">
  websocket

   _()_
</Heading>

Opens WebSocket connections. A connection stays open, and it passes what happens to the
callbacks that you give to `websocket.Connect`.

Add `websocket` to the `requirements` in `extension.json` before you call
`require("websocket")`.

You can leave out `options`. So `websocket.Connect(url, callbacks)` and
`websocket.Connect(url, options, callbacks)` both work. A value in the place of `options` that is
not a table, such as `false`, counts as no options.

Market scripts can connect only to `wss://` URLs on public addresses. Developer scripts can also
use `ws://` URLs and connect to the computer of the user or to a private network. For a `wss://`
URL, the certificate of the server must be valid for its host name.

Callbacks run only while the script is loaded. All scripts are reloaded together. So when
another script is turned on or off, or a developer script is saved, every open socket is closed
with code `1001`, and none of its callbacks run again. In that case the `shutdown` callback of
the script receives `"refresh"` or `"reload"`, and the script can connect again when it runs.

A socket stays open as long as the script is loaded, even if no variable holds its
`websocket.Socket` any more.

A script can have at most 16 sockets that are not closed. Past that, `websocket.Connect` throws
an error. A message can be at most 64 MB. At most 64 callbacks of one socket run in one frame,
and the rest run in the next frames. If 4096 messages or 64 MB of messages are waiting for the
script, the socket is closed with the `"overflow"` error.

A socket does not reconnect by itself, and it does not send pings to the server. To find a
connection that stopped working without a close, send a message on a timer and wait for a reply.

An option key or a callback key that is not listed below is ignored. In a developer script, an
unknown key also adds a warning to the errors of the script, once for each key.


  ### `websocket.Connect`
**Kind:** `fn`

**Arguments:** `url, options?, callbacks`

Starts to connect and returns at once. Everything that happens after that is passed to `callbacks`. This includes a failure to connect.

Throws an error in these cases. A callback is not a function. A header name is not a string, or
      it is not a valid HTTP token. A header value contains CR, LF or NUL. A subprotocol is not a valid
      HTTP token. A header is `Host`, `Upgrade`, `Connection` or starts with `Sec-WebSocket-`. The
      handshake sets these headers itself.
    
  

**Source:** `networking/framework/websocket.lua`

  ### `websocket.Error`
**Kind:** `type`

The reason why a connection failed or was lost.

**Source:** `networking/framework/websocket.lua`

  ### `websocket.State`
**Kind:** `type`

**Source:** `networking/framework/websocket.lua`


## websocket.Options _()_


  ### `websocket.Options.headers`
**Kind:** `prop`

Extra headers to send with the handshake request.

**Source:** `networking/framework/websocket.lua`

  ### `websocket.Options.protocols`
**Kind:** `prop`

One or more subprotocols to offer to the server.

**Source:** `networking/framework/websocket.lua`


## websocket.Callbacks _()_

All callbacks are optional. Each callback receives the socket as its first argument.

`close` is always the last callback of a socket, and it is called exactly once, whether the
connection opened or not. When the connection failed or was lost, `error` is called just
before `close`.


  ### `websocket.Callbacks.close`
**Kind:** `prop`

Called when the connection is closed. `code` is `1006` when the server did not send a close frame.

**Source:** `networking/framework/websocket.lua`

  ### `websocket.Callbacks.error`
**Kind:** `prop`

Called when the connection failed or was lost.

**Source:** `networking/framework/websocket.lua`

  ### `websocket.Callbacks.message`
**Kind:** `prop`

Called when a message arrives. `is_binary` is `true` for a binary message.

**Source:** `networking/framework/websocket.lua`

  ### `websocket.Callbacks.open`
**Kind:** `prop`

Called when the connection is open. `protocol` is the subprotocol that the server chose, or `""` if it chose none.

**Source:** `networking/framework/websocket.lua`


## websocket.Socket _()_

A connection that `websocket.Connect` opened.


  ### `websocket.Socket:Send`
**Kind:** `method`

**Arguments:** `data, binary?`

Sends a message. Returns `true` if the message was queued to be sent.

Returns `false` and sends nothing in these cases. The socket is not open, which includes the
      time before `open` is called. A text message is not valid UTF-8. More than 64 MB is still
      waiting to be sent.
    
  

**Source:** `networking/framework/websocket.lua`

  ### `websocket.Socket:Close`
**Kind:** `method`

**Arguments:** `code?, reason?`

Closes the connection. The `close` callback is called when the closing handshake is done. If the socket is still connecting, `open` is not called. If the socket is already closing or closed, this function does nothing.

Throws an error if `code` is not `1000` and not in the range from `3000` to `4999`. Also throws
      an error if `reason` is longer than 123 bytes. If `reason` is not valid UTF-8, it is not sent.
    
  

**Source:** `networking/framework/websocket.lua`

  ### `websocket.Socket:State`
**Kind:** `method`

Returns the state of the connection.

**Source:** `networking/framework/websocket.lua`

---


## Esp

<!-- Source: cs2/api_reference/rendering/esp.md -->

---
title: ESP
icon: view
---

# ESP

<Heading level={2} id="esp-1">
  esp

   _()_
</Heading>

Adds custom elements to the built-in ESP. Your script can draw extra information next to the
ESP of each entity, and the user can turn each element on or off in the menu.


  ### `esp.AddItem`
**Kind:** `fn`

**Arguments:** `category, name`

Adds a simple ESP element to the given category. The element appears in the ESP options, and the user can turn it on or off. The element does not draw anything by itself. Your script does the drawing.

**Source:** `rendering/game/esp.lua`

  ### `esp.AddText`
**Kind:** `fn`

**Arguments:** `category, name, preview, callback, position?`

Adds a custom text element. `callback` is called for each entity and returns the text to draw.

**Source:** `rendering/game/esp.lua`

  ### `esp.AddBar`
**Kind:** `fn`

**Arguments:** `category, name, callback, position?`

Adds a custom bar element, similar to the built-in health bar. `callback` is called for each entity and returns the values of the bar.

**Source:** `rendering/game/esp.lua`

  ### `esp.AddText.Callback`
**Kind:** `type`

The callback of a custom text element. It is called for each entity and returns the text to draw for that entity.

Return an empty string to hide the text for that entity. The callback receives **nil** when
      there is no entity. Return an empty string in that case.

      If the callback throws an error or returns a value that is not a string, nothing is drawn and
      the error is shown as a script error.
    
  

**Source:** `rendering/game/esp.lua`

  ### `esp.AddBar.Callback`
**Kind:** `type`

The callback of a custom bar element. It is called for each entity. The values are converted to integers.

Return the current value, and optionally the maximum value. If you return a maximum, the bar is
      filled to the current value as a percentage of the maximum, from 0 to 100, and the bar shows the
      current value. If you do not return a maximum, the current value is used directly as the fill.

      Return a negative current value to hide the bar. A maximum of 0 or less also hides the bar. The
      callback receives **nil** when there is no entity. Return -1 in that case.

      If the callback throws an error or returns a value that is not a number, nothing is drawn and
      the error is shown as a script error.
    
  

**Source:** `rendering/game/esp.lua`

  ### `esp.Category`
**Kind:** `type`

The ESP categories that a custom element can be added to. The first two are the player ESP lists. The others are the world ESP lists. Each world list draws its own kind of entity.


  

**Source:** `rendering/game/esp.lua`

  ### `esp.Position`
**Kind:** `type`

The side of the ESP box where a custom text or bar element is placed at first. This value is only the default. If the user saved a position for the element, the saved position is used.



    
      The world ESP categories always place their elements at the bottom. They accept any value, but
      they always use `kBottom`.
    
  

**Source:** `rendering/game/esp.lua`


## esp.Element _()_

A handle to a custom ESP element that was created by `esp.AddItem`, `esp.AddText` or
`esp.AddBar`. Reading or writing a field that does not exist throws an error.


  ### `esp.Element.name`
**Kind:** `prop`

The name of the element in the menu. Read-only.

**Source:** `rendering/game/esp.lua`

  ### `esp.Element.value`
**Kind:** `prop`

Whether the user has turned the element on. You can also set this field.

**Source:** `rendering/game/esp.lua`

  ### `esp.Element:AddGroup`
**Kind:** `method`

Returns the options group of the colour picker of the element. You can add more controls to this group.

**Source:** `rendering/game/esp.lua`

---


## Indicators

<!-- Source: cs2/api_reference/rendering/indicators.md -->

---
title: Indicators
icon: gauge
---

# Indicators

<Heading level={2} id="indicators-1">
  indicators

   _()_
</Heading>

Adds indicators of your script to the indicator stack on the screen. The cheat shows its own
status lines in the same stack. Script indicators use the style of the stack, and they fade,
resize and stack in the same way as the built-in indicators.

Script indicators have no menu option. The script decides when they are shown. When the script
is unloaded, all indicators that it created are removed.

The built-in indicators are not available here, but their menu switches are normal options,
for example `ui.Lookup("Common", "Visuals", "World", "Miscellaneous", "Windows", "Indicators", "Double Tap")`.


  ### `indicators.position`
**Kind:** `prop`

The position of the top of the indicator stack on the screen. Read only.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Create`
**Kind:** `fn`

**Arguments:** `id`

Creates an indicator for the calling script and returns its handle. If the script already has an indicator with this id, that indicator is returned. Throws an error if the id is empty, or if the script already has 64 indicators.

A new indicator is hidden and empty. Add elements with `AddText`, `AddLines`, `AddSpinner` and
      `AddIcon`. The elements are drawn from left to right in the order that they were added. Then set
      `visible` to `true` in any event to show the indicator.
    
  

**Source:** `rendering/framework/indicators.lua`


## indicators.Item _()_

A handle to one indicator that a script created. Read and assign its fields directly. Reading
an unknown field returns `nil`. Assigning an unknown field or a read only field throws an
error.

Two handles are equal with `==` when they point to the same indicator. An indicator exists until
the script is unloaded, so keep the handle if you need it later.


  ### `indicators.Item.anim`
**Kind:** `prop`

The progress of the fade animation, from `0` to `1`. You can use it to match your own drawing to the indicator. Read only.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Item.color`
**Kind:** `prop`

The colour of the accent line. Assign `nil` to use the default colour of the style.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Item.id`
**Kind:** `prop`

The identifier. Read only.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Item.size`
**Kind:** `prop`

The current size of the indicator window, with the animation applied. Read only.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Item.visible`
**Kind:** `prop`

Whether the indicator is shown. It fades in and out with the animation of the stack.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Item:AddText`
**Kind:** `method`

**Arguments:** `text?`

Adds a text element and returns its handle.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Item:AddLines`
**Kind:** `method`

**Arguments:** `count`

Adds a block of text lines on top of each other and returns its handle. The block takes the space of one element. The number of lines cannot be changed later, so hide a line instead of removing it. Throws an error if `count` is not between 1 and 8.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Item:AddSpinner`
**Kind:** `method`

**Arguments:** `alt_style?`

Adds a round progress spinner and returns its handle.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Item:AddIcon`
**Kind:** `method`

**Arguments:** `source, scale?, rotation?, invert?`

Adds an icon element and returns its handle. The source can be a weapon id as an integer, a Panorama image path as a string, such as `"panorama/images/icons/ui/bullet.vsvg_c"`, or an `Image` object that was loaded with the `render` API.

A weapon id or a Panorama path works only if the game resources for it are available.
    
  

**Source:** `rendering/framework/indicators.lua`


## indicators.Text _()_

A text element of an indicator.


  ### `indicators.Text.color`
**Kind:** `prop`

The colour of the text. Assign `nil` to use the default colour of the style.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Text.text`
**Kind:** `prop`

The text that is shown. Assigning it also makes the element visible.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Text.visible`
**Kind:** `prop`

Whether the element is drawn.

**Source:** `rendering/framework/indicators.lua`


## indicators.Lines _()_

A block of text lines on top of each other. Use `lines[1]` to `lines[count]` to get each line as
an `indicators.Text` handle. An index outside this range throws an error.


  ### `indicators.Lines[integer]`
**Kind:** `prop`

The handle of a line. The first line has the index 1.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Lines.count`
**Kind:** `prop`

The number of lines. Read only.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Lines.visible`
**Kind:** `prop`

Whether the whole block is drawn.

**Source:** `rendering/framework/indicators.lua`


## indicators.Spinner _()_

A round progress spinner element.


  ### `indicators.Spinner.color`
**Kind:** `prop`

The colour of the spinner. Assign `nil` to use the default colour of the style.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Spinner.percent`
**Kind:** `prop`

The progress, from `0` to `1`. A value outside this range is changed to the nearest end of the range.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Spinner.visible`
**Kind:** `prop`

Whether the element is drawn.

**Source:** `rendering/framework/indicators.lua`


## indicators.Icon _()_

An icon element.


  ### `indicators.Icon.color`
**Kind:** `prop`

The tint colour. Assign `nil` to use the default colour of the style.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Icon.invert`
**Kind:** `prop`

Whether the icon is flipped vertically.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Icon.rotation`
**Kind:** `prop`

The rotation in degrees.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Icon.scale`
**Kind:** `prop`

The height, relative to the base icon size of the indicator.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Icon.source`
**Kind:** `prop`

The source of the icon. It is a weapon id, a Panorama path or an `Image` object.

**Source:** `rendering/framework/indicators.lua`

  ### `indicators.Icon.visible`
**Kind:** `prop`

Whether the element is drawn.

**Source:** `rendering/framework/indicators.lua`

---


## Panorama

<!-- Source: cs2/api_reference/rendering/panorama.md -->

---
title: Panorama
icon: panels-top-left
---

# Panorama

## panorama.Object _()_

A handle to a Panorama value. Panorama runs JavaScript in V8, and the value can be an object,
an array or a function.

Index the handle to read a property. If the property is an object or a function, you get
another `panorama.Object`. For any other value, you get a plain Lua value. Assign to a property
of the handle to set it. Call the handle to run it as a JavaScript function.

The object of a panel, as returned by `panorama.ids.<Id>`, is also a `panorama.Object`. Its type
is the type of the panel, for example `panorama.panels.CSGOHud`.

Call a method with a dot, as in `handle.method(...)`. If you use a colon, the handle itself is
passed as the first argument. A JavaScript array is also a handle. Its first index is `0`, and
`#` returns its `length`.

A handle works while the context that it was read through exists. For a value read through
`panorama`, this is the own context of the script, which exists until the script unloads. For
a panel from `panorama.ids`, this is the context of the scripts of that panel. This means that
the handle of a HUD panel stops working when the HUD is created again. Using a handle after
that throws an error. Read `panorama.ids.<Id>` again to get a new handle. It returns **nil**
while the panel does not exist.

`tostring` returns `panorama.Object<paneltype>` for the object of a panel that exists, and
`panorama.Object` for any other value.


  ### `panorama.Object(...)`
**Kind:** `ctor`

**Source:** `rendering/engine/panorama.lua`

  ### `panorama.Object[string]`
**Kind:** `prop`

**Source:** `rendering/engine/panorama.lua`

  ### `#panorama.Object`
**Kind:** `meta`

**Source:** `rendering/engine/panorama.lua`


<Heading level={2} id="panorama-1">
  panorama

   _()_
</Heading>



Gives access to the Panorama UI of the game. Panorama runs JavaScript in V8.

Every script gets its own Panorama context. The context is created the first time that the
script uses this namespace. It is a JavaScript context of the same kind as the contexts of the
panels of the game. It holds the API objects of the game, `$`, and the standard globals of
JavaScript. It does not depend on the HUD or the main menu, so it works in the same way in the
menu, on the loading screen and in a match.

When the script unloads, the context is deleted. Everything that was registered from it is
deleted with it. This includes the handlers for unhandled events, the panels from
`panorama.CreatePanel`, and the functions that were defined in the context.

**`panorama.<Name>`** reads a global of the context of the script. This can be an API object,
such as `panorama.GameStateAPI` or `panorama["$"]`, a JavaScript global such as `panorama.JSON`,
or a name that the script defined. The API objects are listed on the API objects page under
Panorama in Dumped Data. A name that does not exist returns **nil**. Assign
`panorama.<Name> = value` to define a global in the context, for code that you compile with
`panorama.Compile`.

**`panorama.ids.<Id>`** returns the object of the panel with this id. Its type is the type of
the panel. The ids that the game had when the docs were made are listed on the Panel ids page
under Panorama in Dumped Data. The id of any other panel that exists also works, but its object
has no specific type. If several panels have the same id, the panel that owns a JavaScript
context is returned. Every lookup goes through all panels of the game, so keep the handle while
it works. Do not look it up again in every frame.

The functions below are names that Lua defines on `panorama`. If a JavaScript global has the
same name, you can still get it with `panorama.Compile("return Name")`.

An id is the name of one panel. A panel type is a class of panels. Valve uses some type names
also as ids. For example, `panorama.ids.CSGOHud` is the plain `Panel` at the root of the HUD,
and the panel of type `CSGOHud` is its child `panorama.ids.Hud`. `MainMenu` and `LoadingScreen`
work in the same way. The Panel ids page under Panorama in Dumped Data has a note for every id
of this kind. While the game runs, read `.paneltype` to get the type of a panel.

Values are converted to JavaScript when Lua calls a JavaScript function or assigns a property,
and when a Lua function that JavaScript called returns. This table shows the conversion.

| Lua                        | JavaScript                                                                                                                                                                                 |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| nil                        | `null`                                                                                                                                                                                     |
| boolean, number, string    | the same value                                                                                                                                                                             |
| table with the keys 1 to n | a new `Array`, with the first index `0`                                                                                                                                                    |
| any other table            | a new plain object with the string keys and number keys of the table. Keys of other types are left out. A table that holds itself, or that is passed twice in one call, becomes one object |
| function                   | a JavaScript function that calls the Lua function                                                                                                                                          |
| `panorama.Object`          | the JavaScript value of the handle                                                                                                                                                         |
| anything else              | `null`                                                                                                                                                                                     |

Values are converted back to Lua in this way. `null` and `undefined` become **nil**. Booleans,
numbers and strings stay the same. A `BigInt` becomes a string with its decimal value. Objects,
arrays and functions become `panorama.Object` handles. Any other value, such as a symbol,
becomes **nil**.

If JavaScript throws an exception while Lua reads, assigns, calls or compiles, Lua throws an
error with the message of the exception. The message starts with the script and the line where
the exception came from.

When you give a Lua function to JavaScript, these rules apply.

The JavaScript arguments are converted to Lua. The first return value of the Lua function is
returned to JavaScript. If the function returns nothing, JavaScript gets `undefined`.

The function runs as the script that gave it. If it throws an error, the error is listed under
the errors of that script, and JavaScript gets it as an `Error` that it can catch.

The function works until its script unloads. This includes the `shutdown` listener of the
script. After that, calling the function throws an error. All scripts are reloaded together,
so the function also stops working when you turn another script on or off, or when you save a
developer script.

Each time the function is passed to JavaScript, a new JavaScript function is made. Two of them
are not equal with `===`.

Calling the function with `new` throws a `TypeError`.

Lua and JavaScript can call each other up to 32 levels deep. A table can be nested up to 32
levels deep. If you go deeper, an error is thrown on the side that made the call.

`$.RegisterEventHandler` throws an error in the context of a script, because the game would keep
the handler on the target panel after the context of the script is deleted. Use
`panorama.RegisterForUnhandledEvent` instead, or run the registration in the own context of the
panel with `panorama.Compile(source, panel)`.

```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/rendering/panorama:0:0dd1741c4c39"
local hud = panorama.ids.Hud
if hud then
  local radar = hud.FindChildTraverse("HudRadar")
  if radar then radar.visible = false end

  local label = panorama.CreatePanel("Label", hud, "MyLabel") --[[@as panorama.panels.Label]]
  label.SetLocString("#SFUI_BuyMenu_Cannot_Afford")
end
```

```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/rendering/panorama:1:1941d88fb604"
local make_counter = panorama.Compile([[
  var kills = 0;
  return {
    kill: function (name) { kills++; return 'You killed ' + name + '! (' + kills + ')'; },
    reset: function () { kills = 0; },
  };
]])

local counter = make_counter()
print(counter.kill("XYZ")) -- You killed XYZ! (1)
counter.reset()
```


  ### `panorama.ids`
**Kind:** `prop`

The panels by id.

**Source:** `rendering/engine/panorama.lua`

  ### `panorama.Compile`
**Kind:** `fn`

` and the line in `source`." args="source, panel?" sig="[{"s":"panorama.Compile"},{"s":"("},{"s":"source"},{"s":": "},{"s":"string","k":"prim","h":"/cs2/api_reference/lua/base#string"},{"s":"[, "},{"s":"panel"},{"s":": "},{"s":"("},{"s":"panorama.Object","k":"class","h":"/cs2/api_reference/rendering/panorama#panoramaobject"},{"s":"|"},{"s":"panorama.panels.Panel","k":"class","h":"/cs2/dumped_data/panorama/panels/Panel#panoramapanelspanel"},{"s":")"},{"s":"]"},{"s":")"},{"s":": "},{"s":"panorama.Object","k":"class","h":"/cs2/api_reference/rendering/panorama#panoramaobject"}]" ret="[[{"s":"panorama.Object","k":"class","h":"/cs2/api_reference/rendering/panorama#panoramaobject"}]]" params="[{"n":"source","t":[{"s":"string","k":"prim","h":"/cs2/api_reference/lua/base#string"}],"d":"The JavaScript code of the function body."},{"n":"panel","t":[{"s":"("},{"s":"panorama.Object","k":"class","h":"/cs2/api_reference/rendering/panorama#panoramaobject"},{"s":"|"},{"s":"panorama.panels.Panel","k":"class","h":"/cs2/dumped_data/panorama/panels/Panel#panoramapanelspanel"},{"s":")"}],"d":"The panel whose context compiles the body. The default is the own context of the script.","o":true}]" source="rendering/engine/panorama.lua" sourceHref="https://gitlab.com/neverlose/LuaLS-CS2/-/edit/master/library/api/rendering/engine/panorama.lua#L171">
    
      Without `panel`, the body runs in the own context of the script. With `panel`, the body runs in
      the context of the scripts of that panel. There, the globals that those scripts defined are
      visible, and `$.GetContextPanel()` returns the panel that owns the context. The function then
      works while that context exists. Anything that the function registers there belongs to that
      context. It is not deleted when the script unloads.
    

    
      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/rendering/panorama:2:44cf56051d89"
      local hud = panorama.ids.Hud
      if hud then
        local owner = panorama.Compile("return $.GetContextPanel().id", hud)
        print(owner()) -- the id of the panel that owns the HUD's context
      end
      ```
    
  

  ### `panorama.RegisterForUnhandledEvent`
**Kind:** `fn`

**Arguments:** `event, handler`

Calls `handler` each time the event `event` is dispatched and no panel handles it. This works in the same way as `$.RegisterForUnhandledEvent`. `handler` gets the arguments of the event. The registration belongs to the context of the script and ends when the script unloads.

Throws an error if `event` is not an event that the game registered.
    

    
      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/rendering/panorama:3:181f1f359506"
      panorama.RegisterForUnhandledEvent("CSGOShowMainMenu", function()
        print("main menu shown")
      end)
      ```
    
  

**Source:** `rendering/engine/panorama.lua`

  ### `panorama.UnregisterForUnhandledEvent`
**Kind:** `fn`

**Arguments:** `event, id`

Removes a handler that was registered with `panorama.RegisterForUnhandledEvent`.

**Source:** `rendering/engine/panorama.lua`

  ### `panorama.DispatchEvent`
**Kind:** `fn`

**Arguments:** `event, ...?`

Dispatches the event `event` with `...` as its arguments. This works in the same way as `$.DispatchEvent`. Every argument is passed as text, and the event converts it to the argument types that it uses.

If you pass one more argument after the arguments of the event, it is the target of the event.
      The target is a panel or the id of a panel. Without a target, the event starts at the context
      of the script, so only the handlers for unhandled events get it.
    

    
      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/rendering/panorama:4:e0e7a742c85c"
      panorama.DispatchEvent("PlaySoundEffect", "UIPanorama.store_item_activated")
      ```
    
  

**Source:** `rendering/engine/panorama.lua`

  ### `panorama.CreatePanel`
**Kind:** `fn`

**Arguments:** `type, parent, id?, properties?`

Creates a panel of type `type` as a child of `parent` and returns the object of the panel. This works in the same way as `$.CreatePanel`. The panel belongs to the script. It is deleted when the script unloads, in any place where it was put.

`properties` sets attributes and panel events in the same way as layout XML, for example
      `{ class = "MyClass", text = "#SFUI_BuyMenu_Cannot_Afford" }`. A panel event, such as
      `onactivate` or `onmouseover`, can be a string of JavaScript or a Lua function. A string runs in
      the context of the scripts of the panel. A Lua function runs as the script when the event
      happens. A Lua function is set on the new panel with `SetPanelEvent`, so you can also call
      `panel.SetPanelEvent("onactivate", fn)` later to get the same result.
    

    
      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/rendering/panorama:5:4439d00947f8"
      local menu = panorama.ids.MainMenu
      if menu then
        panorama.CreatePanel("TextButton", menu, "lua_button", {
          text = "Click me",
          style = "horizontal-align: center; vertical-align: center; width: 240px; height: 60px;",
          onactivate = function()
            panorama.DispatchEvent("PlaySoundEffect", "UIPanorama.store_item_activated")
          end,
        })
      end
      ```
    
  

**Source:** `rendering/engine/panorama.lua`

---


## Particles

<!-- Source: cs2/api_reference/rendering/particles.md -->

---
title: Particles
icon: sparkle
---

# Particles

## particles.Create.Options _()_

Options for `particles.Create`. If a field has a value that is not in its enum, an error is
thrown.


  ### `particles.Create.Options.attachment`
**Kind:** `prop`

How control point 0 is placed. Defaults to `WORLDORIGIN`. The values `POINT`, `POINT_FOLLOW`, `CUSTOMORIGIN` and `CUSTOMORIGIN_FOLLOW` throw an error here. To use them, create the effect with another value, and then bind the point with `SetEntity`.

**Source:** `rendering/game/particles.lua`

  ### `particles.Create.Options.check_fow`
**Kind:** `prop`

Whether the effect is hidden in fog of war, in the same way as the effects of the game. Defaults to `true`.

**Source:** `rendering/game/particles.lua`

  ### `particles.Create.Options.create_pvs`
**Kind:** `prop`

Whether the scene objects of the effect take part in visibility culling. Defaults to `true`.

**Source:** `rendering/game/particles.lua`

  ### `particles.Create.Options.entity`
**Kind:** `prop`

The entity that the effect is attached to. It is required for every `attachment` value except `WORLDORIGIN`. An invalid entity throws an error.

**Source:** `rendering/game/particles.lua`

  ### `particles.Create.Options.lifespan`
**Kind:** `prop`

The number of seconds until the effect is destroyed. The end cap of the effect is played when it is destroyed. The default is `0`, which means that the effect never expires. A negative value throws an error.

**Source:** `rendering/game/particles.lua`


## particles.Particle _()_

A particle effect that was created by this script.

The effect is not destroyed when the last reference to this object is gone. The effect runs
until you call `Destroy`, until its `lifespan` ends, until the script unloads, or until the map
ends. After that, every method returns `false` or `nil`.


  ### `particles.Particle:IsValid`
**Kind:** `method`

Returns whether the effect is still alive and owned by this object.

**Source:** `rendering/game/particles.lua`

  ### `particles.Particle:GetName`
**Kind:** `method`

Returns the path of the resource that the effect was created from. Returns `nil` after the effect is gone.

**Source:** `rendering/game/particles.lua`

  ### `particles.Particle:SetControlPoint`
**Kind:** `method`

**Arguments:** `point, position`

Moves a control point. The orientation of the point is reset to the identity rotation.

**Source:** `rendering/game/particles.lua`

  ### `particles.Particle:SetTransform`
**Kind:** `method`

**Arguments:** `point, transform, interpolation?`

Sets the position and the orientation of a control point.

**Source:** `rendering/game/particles.lua`

  ### `particles.Particle:SetEntity`
**Kind:** `method`

**Arguments:** `point, entity, attachment?, attachment_name?`

Binds a control point to an entity. An invalid entity throws an error.

**Source:** `rendering/game/particles.lua`

  ### `particles.Particle:SetPath`
**Kind:** `method`

**Arguments:** `point, points`

Sets the shape of a rope or trail effect from a list of points. This works for effects that read a particle snapshot from a control point. For example, the effect `"particles/entity/spectator_utility_trail.vpcf"` reads it from control point `0`.

Each call replaces the points of the previous call. The effect decides when it reads the
      points. Most effects, including the spectator trail, copy the points only when particles
      spawn. A later call does not move particles that are already alive. Point `i` gets a creation
      time of `(i - 1) / (#points - 1)`.

      The effect gets one particle for each point. The spectator trail scales the width of each
      particle by its index along the path. Because of this, a path with only two points draws
      nothing that you can see. Split a straight line into 16 to 24 points.

      The spectator trail draws a beam once. Control point `3` holds the particle life in seconds,
      the width and the alpha. Control point `16` holds the colour. The alpha of a particle grows
      with its age. Its width grows during the first half of its life and shrinks to zero during the
      second half. Give the effect a `lifespan` that is a little longer than the particle life.
    

    
      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/rendering/particles:0:bdd7e82b4f31"
      local from, to = Vector(0, 0, 64), Vector(256, 0, 64)
      local points = {}
      for i = 0, 23 do
          local t = i / 23
          points[#points + 1] = from + (to - from) * t
      end
      local beam = particles.Create("particles/entity/spectator_utility_trail.vpcf", { lifespan = 2.3 })
      if beam then
          beam:SetPath(0, points)
          beam:SetControlPoint(16, Vector(255, 0, 0)) -- color
          beam:SetControlPoint(3, Vector(2, 0.4, 1))  -- particle life, width, alpha
      end
      ```
    
  

**Source:** `rendering/game/particles.lua`

  ### `particles.Particle:Destroy`
**Kind:** `method`

**Arguments:** `play_endcap?`

Destroys the effect.

**Source:** `rendering/game/particles.lua`


<Heading level={2} id="particles-1">
  particles

   _()_
</Heading>

Creates and controls particle effects. The game simulates and renders the effects.

A script can have at most 256 effects alive at the same time. Creating one more throws an
error.


  ### `particles.Create`
**Kind:** `fn`

**Arguments:** `path, options?`

Creates a particle effect from a compiled `.vpcf` resource.

Returns `nil` when the client is not in a game or when the resource cannot be found. Throws an
      error if another script registered the name with `particles.Register`.
    

    
      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/rendering/particles:1:0ee9dbd59084"
      local fx = particles.Create("particles/entity/spectator_utility_trail.vpcf", { lifespan = 3 })
      if fx then fx:SetControlPoint(0, Vector(0, 0, 64)) end
      ```
    
  

**Source:** `rendering/game/particles.lua`

  ### `particles.Register`
**Kind:** `fn`

**Arguments:** `name, bytes`

Loads a compiled particle system from memory, so that `particles.Create(name)` can use it. Use this function to add an effect that the game does not have, such as a rope that stays in the world.

`bytes` must be a compiled CS2 particle system, which is the content of a `.vpcf_c` file. Bad
      data can crash the game.

      The name must not be used by any resource of the game. A name that a script registered before
      it unloaded can be registered again. A name that is already registered throws an error.

      The registration belongs to this script. Only this script can create effects from the name.
      When the script unloads, those effects are removed without their end cap and the resource is
      freed.

      A script can register at most 32 particle systems. Each one can be at most 4 MB. Going over
      either limit throws an error.
    

    
      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/rendering/particles:2:b3060b0a4590"
      local ok, err = particles.Register("particles/my_script/rope.vpcf", require("rope_bytes"))
      if not ok then print(err) return end

      local rope = particles.Create("particles/my_script/rope.vpcf")
      if rope then rope:SetPath(0, { Vector(0, 0, 64), Vector(256, 0, 64) }) end
      ```
    
  

**Source:** `rendering/game/particles.lua`

  ### `particles.Attachment`
**Kind:** `type`

How an effect or a control point follows its entity.


  

**Source:** `rendering/game/particles.lua`

---


## Projection

<!-- Source: cs2/api_reference/rendering/projection.md -->

---
title: Projection
icon: scan
---

# Projection

<Heading level={2} id="projection-1">
  projection

   _()_
</Heading>

Converts positions in the world to positions on the screen. The functions use the current
projection of the engine and the current render size.

Pixel coordinates start at the top left corner of the screen, and y increases downward. The
functions that take world positions use the first split screen slot, which is slot `0`.

All functions except `Matrix` return new values that you can keep and change. If a function
can return **nil**, it returns **nil** when the engine data that it needs is not available.

Call these functions from a render callback, when the render size is valid. Functions that take
world positions also need an active camera.


  ### `projection.Ellipse`
**Kind:** `prop`

**Source:** `rendering/engine/projection.lua`

  ### `projection.Rect`
**Kind:** `prop`

**Source:** `rendering/engine/projection.lua`

  ### `projection.Matrix`
**Kind:** `fn`

**Arguments:** `slot?`

Returns a reference to the projection matrix of the engine for a split screen slot. You can change the matrix through the reference. When the engine updates the matrix, the values that you read change too. Throws an error if the slot is out of range. Returns **nil** if the projection matrices are not available.

<Warning>
        If you write to the reference, you change the projection matrix of the engine, and this changes
        the rendering. Use the reference only while the engine memory is valid. Use `VMatrix(matrix)`
        to make a separate copy.
      </Warning>
    
  

**Source:** `rendering/engine/projection.lua`

  ### `projection.WorldToClip`
**Kind:** `fn`

**Arguments:** `pos`

Converts a world position to homogeneous clip coordinates. The result is not divided by w.

**Source:** `rendering/engine/projection.lua`

  ### `projection.ClipToScreen`
**Kind:** `fn`

**Arguments:** `clip`

Converts homogeneous clip coordinates to pixels. x and y are divided by w. If w is less than `0.0001`, `0.0001` is used in its place.

z is ignored. Points outside the viewport and points with a w of zero or less are also
      converted. This function uses only the current render size. It returns **nil** only if the
      render size is not available.
    
  

**Source:** `rendering/engine/projection.lua`

  ### `projection.WorldToScreen`
**Kind:** `fn`

**Arguments:** `pos`

Converts a world position to pixels. Returns **nil** if clip w is `0.0001` or less.

Points outside the viewport are also converted, and clip z is not checked. To find out whether
      the point is on the screen, compare the result with `engine.GetScreenSize`.
    
  

**Source:** `rendering/engine/projection.lua`

  ### `projection.WorldToScreenRounded`
**Kind:** `fn`

**Arguments:** `pos`

The same as `WorldToScreen`, but each pixel coordinate is rounded down. Negative values are also rounded down.

**Source:** `rendering/engine/projection.lua`

  ### `projection.Ellipsoid`
**Kind:** `fn`

**Arguments:** `center, radii`

Returns the bounds on the screen of the outline of an ellipsoid. The axes of the ellipsoid are aligned with the axes of the world.

Returns **nil** if the outline is not finite or is not in front of the camera. This includes
      shapes that cross the plane of the camera. The bounds can go past the edges of the viewport.
    
  

**Source:** `rendering/engine/projection.lua`

  ### `projection.EllipsoidClipped`
**Kind:** `fn`

**Arguments:** `center, radii`

The same as `Ellipsoid`, with one difference. If the outline is not finite or is not in front of the camera, this function returns the bounds of the part of the ellipsoid where clip w is greater than `0.0001`. It returns **nil** only if no such part exists. The bounds are not limited to the viewport.

**Source:** `rendering/engine/projection.lua`

  ### `projection.Sphere`
**Kind:** `fn`

**Arguments:** `center, radius`

The same as `Ellipsoid`, with the same radius on all three world axes.

**Source:** `rendering/engine/projection.lua`

  ### `projection.EllipsoidEllipse`
**Kind:** `fn`

**Arguments:** `center, radii`

Returns the full ellipse that an ellipsoid makes on the screen. The axes of the ellipsoid are aligned with the axes of the world. This function returns **nil** in the same cases as `Ellipsoid`. Because of perspective, the center of the ellipse can be at a different place from `WorldToScreen(center)`.

**Source:** `rendering/engine/projection.lua`

  ### `projection.SphereEllipse`
**Kind:** `fn`

**Arguments:** `center, radius`

The same as `EllipsoidEllipse`, with the same radius on all three world axes.

**Source:** `rendering/engine/projection.lua`

  ### `projection.Camera`
**Kind:** `fn`

Returns the position and orientation of the camera, calculated from the current projection matrix. The returned transform has a scale of `1`. Returns **nil** if the length of a camera axis in the matrix is `0.0001` or less.

**Source:** `rendering/engine/projection.lua`

  ### `projection.ClipLineToScreen`
**Kind:** `fn`

**Arguments:** `start, finish`

Cuts a line segment in world space to the part where clip z is `0` or greater. Then converts its end points to pixels and returns them in their original order. Returns **nil** if clip z is negative at both end points.

The conversion divides by w. If w is less than `0.0001`, `0.0001` is used in its place. The end
      points that are returned can be outside the viewport.
    
  

**Source:** `rendering/engine/projection.lua`


## ProjectionRect" />



A rectangle on the screen, with floating point bounds in pixels.


  ### `ProjectionRect.m_vMaxBounds`
**Kind:** `prop`

The maximum x and y.

**Source:** `rendering/engine/projection.lua`

  ### `ProjectionRect.m_vMinBounds`
**Kind:** `prop`

The minimum x and y.

**Source:** `rendering/engine/projection.lua`


## ProjectionEllipse" />



The ellipse that the outline of a shape makes on the screen, in pixels. The radii are the
lengths of the half axes.


  ### `ProjectionEllipse.center`
**Kind:** `prop`

The center on the screen.

**Source:** `rendering/engine/projection.lua`

  ### `ProjectionEllipse.radius_major`
**Kind:** `prop`

The length of the long half axis.

**Source:** `rendering/engine/projection.lua`

  ### `ProjectionEllipse.radius_minor`
**Kind:** `prop`

The length of the short half axis.

**Source:** `rendering/engine/projection.lua`

  ### `ProjectionEllipse.rotation`
**Kind:** `prop`

The rotation of the long axis in radians, in screen coordinates.

**Source:** `rendering/engine/projection.lua`


<Heading level={2} id="projectionrect-1">
  projection.Rect

  " />
</Heading>


  ### `projection.Rect()`
**Kind:** `ctor`

All bounds are zero.

**Source:** `rendering/engine/projection.lua`

  ### `projection.Rect(min_bounds, max_bounds)`
**Kind:** `ctor`

**Source:** `rendering/engine/projection.lua`

  ### `projection.Rect(rect)`
**Kind:** `ctor`

A separate copy.

**Source:** `rendering/engine/projection.lua`


<Heading level={2} id="projectionellipse-1">
  projection.Ellipse

  " />
</Heading>


  ### `projection.Ellipse()`
**Kind:** `ctor`

All values are zero.

**Source:** `rendering/engine/projection.lua`

  ### `projection.Ellipse(center, radius_major, radius_minor, rotation)`
**Kind:** `ctor`

**Source:** `rendering/engine/projection.lua`

  ### `projection.Ellipse(ellipse)`
**Kind:** `ctor`

A separate copy.

**Source:** `rendering/engine/projection.lua`

---


## Render

<!-- Source: cs2/api_reference/rendering/render.md -->

---
title: Render
icon: brush
sidebar_position: 1
---

# Render

<Heading level={2} id="render-1">
  render

   _()_
</Heading>

Draws shapes, images and text on the screen.

Call the drawing functions only inside the `render` event. If you call one anywhere else, it
throws an error. Coordinates are in screen pixels. The point `Vector2D(0, 0)` is the top left
corner of the screen.


  ### `render.DrawLine`
**Kind:** `fn`

**Arguments:** `startpos, endpos, color, thickness?`

Draws a straight line between two points.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawBezier`
**Kind:** `fn`

**Arguments:** `a, b, c, d, color, thickness, segments?`

Draws a cubic Bézier curve through four control points. The curve starts at `a` and ends at `d`.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawRect`
**Kind:** `fn`

**Arguments:** `position, size, color, rounding?`

Draws a filled rectangle from `position` to `position + size`.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawRectOutline`
**Kind:** `fn`

**Arguments:** `position, size, color, thickness?, rounding`

Draws the outline of a rectangle from `position` to `position + size`.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawRectShadow`
**Kind:** `fn`

**Arguments:** `position, size, color, rounding?, shadow_size?, offset?`

Draws a soft shadow around a rectangle. The rectangle itself is not drawn.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawRectGradient`
**Kind:** `fn`

**Arguments:** `position, size, top_left, top_right, bottom_left, bottom_right, rounding?`

Draws a filled rectangle with a different colour at each corner. The colours blend smoothly across the rectangle.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawRectGlow`
**Kind:** `fn`

**Arguments:** `position, size, color, rounding?`

Draws a glow along the border of a rectangle.

This function uses one radius for all four corners. If you pass an array for `rounding`, only
      its first value is used.
    
  

**Source:** `rendering/framework/render.lua`

  ### `render.DrawWindow`
**Kind:** `fn`

**Arguments:** `position, size, rounding?, shadow?, alpha?`

Draws a transparent window background with a frosted look and a shadow. It looks the same as the windows of the cheat menu.

This function uses one radius for all four corners. If you pass an array for `rounding`, only
      its first value is used.
    
  

**Source:** `rendering/framework/render.lua`

  ### `render.DrawWindowShadow`
**Kind:** `fn`

**Arguments:** `position, size, rounding?`

Draws the shadow that the windows of the menu use, for a rectangle with the given position and size. The colour, size and offset of the shadow come from the current menu style.

This function uses one radius for all four corners. If you pass an array for `rounding`, only
      its first value is used.
    
  

**Source:** `rendering/framework/render.lua`

  ### `render.DrawCircle`
**Kind:** `fn`

**Arguments:** `center, color, radius, start_degrees?, percentage?`

Draws a filled circle. Pass `percentage` to draw only a part of the circle, like a slice of a pie.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawCircleOutline`
**Kind:** `fn`

**Arguments:** `center, color, radius, start_degrees?, percentage?, thickness?`

Draws the outline of a circle. Pass `percentage` to draw only a part of the circle, like a slice of a pie.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawCircleGradient`
**Kind:** `fn`

**Arguments:** `center, outer, inner, radius, gradient?, start_degrees?, percentage?`

Draws a filled circle whose colour changes from `outer` at the edge to `inner` at the center.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawEllipse`
**Kind:** `fn`

**Arguments:** `center, color, radius_major, radius_minor, rotation?`

Draws a filled ellipse.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawEllipseOutline`
**Kind:** `fn`

**Arguments:** `center, color, radius_major, radius_minor, rotation?, thickness?`

Draws the outline of an ellipse.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawEllipseGradient`
**Kind:** `fn`

**Arguments:** `center, outer, inner, radius_major, radius_minor, rotation?, gradient?`

Draws a filled ellipse whose colour changes from `outer` at the edge to `inner` at the center.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawCircleBlur`
**Kind:** `fn`

**Arguments:** `center, alpha, radius, start_degrees?, percentage?`

Blurs everything behind a circular area of the screen.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawCircleShadow`
**Kind:** `fn`

**Arguments:** `center, color, radius, size, start_degrees?, percentage?, offset?`

Draws a soft shadow around a circle. The circle itself is not drawn.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawTriangle`
**Kind:** `fn`

**Arguments:** `p1, p2, p3, color`

Draws a filled triangle.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawTriangleOutline`
**Kind:** `fn`

**Arguments:** `p1, p2, p3, color, thickness?`

Draws the outline of a triangle.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawPoly`
**Kind:** `fn`

**Arguments:** `points, color`

Draws a filled convex polygon. List the corners in order around the edge of the polygon.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawPolyline`
**Kind:** `fn`

**Arguments:** `points, color, closed, thickness`

Draws lines that connect the given points one after another.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawPolyGradient`
**Kind:** `fn`

**Arguments:** `center?, points, outer, inner, offset?`

Draws a filled convex polygon whose colour changes from `outer` at the edges to `inner` at `center`. A polygon with fewer than three points is not drawn.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawPolyShadow`
**Kind:** `fn`

**Arguments:** `points, color, thickness, offset?`

Draws a soft glow around a convex polygon. The polygon itself is not drawn. Use `render.DrawPoly` if you also want to fill it.

**Source:** `rendering/framework/render.lua`

  ### `render.DrawImage`
**Kind:** `fn`

**Arguments:** `image, position, size?, color?, mode?, rounding?`

Draws an image. An image that is not loaded yet, or an empty image such as the avatar of a user who has no avatar, draws nothing.

If `size` is `nil`, or both of its values are `0` or less, the image is drawn at its own size.
      If only one value is `0` or less, that value is calculated from the other one, so the image
      keeps its proportions.
    
  

**Source:** `rendering/framework/render.lua`

  ### `render.LoadImage`
**Kind:** `fn`

**Arguments:** `bytes, size?`

Loads an image from the bytes of an image file. The format, such as PNG, JPG, GIF or SVG, is detected from the data.

Throws an error if the data is shorter than three bytes, or if a value of `size` is not above
      `0` and at most `16384`. A GIF is always loaded at its own size.
    
  

**Source:** `rendering/framework/render.lua`

  ### `render.LoadImageRGBA`
**Kind:** `fn`

**Arguments:** `bytes, size`

Loads an image from raw RGBA pixel data that is not compressed.

Each value of `size` must be above `0` and at most `16384`. The data must contain at least
      `size.x * size.y * 4` bytes. Otherwise this function throws an error.
    
  

**Source:** `rendering/framework/render.lua`

  ### `render.LoadImageFromFile`
**Kind:** `fn`

**Arguments:** `file, size?`

Loads an image from a file in the folder of the script. Throws an error if the file does not exist or is empty.

**Source:** `rendering/framework/render.lua`

  ### `render.CreateShader`
**Kind:** `fn`

**Arguments:** `source, callback?`

Compiles a pixel shader from HLSL source with shader model 5.0.

This function returns at once, and the shader compiles in the background. When it finishes,
      `callback` is called and the shader can be drawn. Shaders compile in the order they were
      requested, so a shader can wait for others to finish first. If you release the shader, or the
      script unloads, before its compile starts, the compile is cancelled.

      The source must define `float4 main(PSInput i) : SV_Target`. It returns **linear** RGB with
      **straight** alpha. A `Color` is in sRGB, so convert it with `nl_srgb_to_linear` first. The
      result is clamped to the range from `0` to `1`, and a pixel with a NaN value is not drawn.

      The following names are already declared for the shader. Do not declare them yourself.

      ```hlsl
      struct PSInput { float4 pos : SV_POSITION; float4 col : COLOR0; float2 uv : TEXCOORD0; };
      // uv runs 0..1 across the drawn rectangle; pos is the pixel on screen.
      float2 nl_resolution;  // screen size in pixels
      float2 nl_mouse;       // cursor position
      float2 nl_pos;         // rectangle position and size, as passed to render.DrawShader
      float2 nl_size;
      float4 nl_rounding;    // corner radii in pixels: top-left, top-right, bottom-right, bottom-left
      float  nl_time;        // seconds since the shader was created
      float  nl_delta;       // seconds since the last frame
      uint   nl_frame;       // frame counter
      float4 nl_params[16];  // the `params` numbers, four to an element: params[1] is nl_params[0].x
      Texture2D nl_image;    // the `image` option
      Texture2D nl_backdrop; // a blurred copy of what is under the draw
      SamplerState nl_linear_clamp, nl_linear_wrap;
      float3 nl_srgb_to_linear(float3 c);  float3 nl_linear_to_srgb(float3 c);
      float4 nl_vertex_color(PSInput i);   // the `color` option, linear
      float2 nl_local_px(PSInput i);       // position inside the rectangle in pixels
      float2 nl_screen_uv(PSInput i);      // position on screen, 0..1
      float4 nl_sample_image(PSInput i);   // nl_image at uv
      float4 nl_sample_backdrop(PSInput i); // nl_backdrop under this pixel
      ```

      The shader can use only these resources. A global variable must be `static`, because the
      script has no constant buffer of its own. Pass values to the shader through `params` instead.
      `#include` is not available.

      Names that start with `nl_`, and the name `PSInput`, belong to the code around the source. If
      the source declares one of them, the compile fails.

      The compile also fails if the shader has more than 8192 instructions, if nested
      `[unroll(n)]` loops unroll to more than 1024 iterations in total, or if the source is larger
      than 1 MB after its macros are expanded. The error message then gives the reason.

      Throws an error if the source is empty or larger than 64 KB, if the script already has 32
      shaders, or if 256 shaders exist across all scripts.
    

    
      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/rendering/render:0:fc6d1ac9bf26"
      local plasma = render.CreateShader([[
      float4 main(PSInput i) : SV_Target
      {
          float2 p = nl_local_px(i) / 40.0;
          float v = sin(p.x + nl_time) + sin(p.y + nl_time * 1.3) + sin(p.x + p.y + nl_time * 0.7);
          float3 c = 0.5 + 0.5 * cos(v + float3(0.0, 2.0, 4.0));
          return float4(c, nl_params[0].x);
      }
      ]], function(ok, err) if not ok then print(err) end end)

      events.render:Set(function()
          render.DrawShader(plasma, Vector2D(100, 100), Vector2D(300, 150), { params = { 0.9 }, rounding = 12 })
      end)
      ```
    
  

**Source:** `rendering/framework/render.lua`

  ### `render.DrawShader`
**Kind:** `fn`

**Arguments:** `shader, position, size, options?`

Draws a rectangle in which every pixel is calculated by `shader`. A shader that is still compiling, or that failed, draws nothing. Throws an error if the shader was released or belongs to another script.

Rotation from `render.PushRotation` and clip rectangles apply in the same way as for other
      drawing functions.

      Inside a `render.PushTransform` block, the rectangle is drawn on the plane. Then `uv`,
      `nl_local_px` and the rounding follow the plane, and `nl_pos` and `nl_size` are the rectangle
      on the plane as you passed it. `nl_mouse`, `nl_screen_uv` and `nl_backdrop` stay in screen
      space. The backdrop under such a rectangle is not blurred, so do not read it there.

      A script can draw at most 512 shaders in each frame. At most 16 of them can read
      `nl_backdrop`, because each of these blurs the area under it first. A draw over either limit
      throws an error.
    
  

**Source:** `rendering/framework/render.lua`

  ### `render.PushSmoothing`
**Kind:** `fn`

**Arguments:** `smoothing`

Sets the amount of smoothing, also called antialiasing, for everything drawn until the matching `render.PopSmoothing` call.

If a smoothing, blur, clip rectangle or clip shader is still pushed when your `render` callback
      returns, it is removed for you and a script error is shown.
    
  

**Source:** `rendering/framework/render.lua`

  ### `render.PopSmoothing`
**Kind:** `fn`

Removes the smoothing set by `render.PushSmoothing`.

**Source:** `rendering/framework/render.lua`

  ### `render.PushBlur`
**Kind:** `fn`

**Arguments:** `strength?, is_static?`

Starts a blur area. Everything drawn until the matching `render.PopBlur` call is drawn over a blurred copy of the background.

**Source:** `rendering/framework/render.lua`

  ### `render.PopBlur`
**Kind:** `fn`

Ends the blur area started by `render.PushBlur`. If no blur area is started, this function does nothing.

**Source:** `rendering/framework/render.lua`

  ### `render.PushClipRect`
**Kind:** `fn`

**Arguments:** `position, size, intersect?`

Starts a clip rectangle. Everything drawn until the matching `render.PopClipRect` call is cut to this rectangle.

**Source:** `rendering/framework/render.lua`

  ### `render.PopClipRect`
**Kind:** `fn`

Ends the clip rectangle started by `render.PushClipRect`.

**Source:** `rendering/framework/render.lua`

  ### `render.PushShadow`
**Kind:** `fn`

Starts recording shapes for a shadow. The matching `render.PopShadow` call draws a shadow for the recorded shapes.

**Source:** `rendering/framework/render.lua`

  ### `render.PopShadow`
**Kind:** `fn`

**Arguments:** `color, size, offset?`

Ends the recording started by `render.PushShadow` and draws a soft shadow for the recorded shapes.

**Source:** `rendering/framework/render.lua`

  ### `render.PushClipShader`
**Kind:** `fn`

Starts a clip shader area. The shapes drawn until the matching `render.PopClipShaderRect` or `render.PopClipShaderCircle` call are cut to the shape given to that call.

If the area is still open when your `render` callback returns, it is closed for you and
      everything drawn inside it is thrown away.
    
  

**Source:** `rendering/framework/render.lua`

  ### `render.PopClipShaderRect`
**Kind:** `fn`

**Arguments:** `position, size, color?, rounding?`

Ends the clip shader area started by `render.PushClipShader` and cuts the shapes drawn inside it to a rectangle.

**Source:** `rendering/framework/render.lua`

  ### `render.PopClipShaderCircle`
**Kind:** `fn`

**Arguments:** `center, radius, start_degrees?, percentage?`

Ends the clip shader area started by `render.PushClipShader` and cuts the shapes drawn inside it to a circle or to a part of a circle.

**Source:** `rendering/framework/render.lua`

  ### `render.PushComplexShadow`
**Kind:** `fn`

Starts recording shapes for a detailed shadow. The matching `render.PopComplexShadow` call draws a high quality shadow that follows the exact outline of the recorded shapes.

Recordings cannot be nested. If you start one inside another, its shapes become part of the
      outer recording and do not get a shadow of their own.
    
  

**Source:** `rendering/framework/render.lua`

  ### `render.PopComplexShadow`
**Kind:** `fn`

**Arguments:** `color, only_shadow?, bottom_right_or_top_left?`

Ends the recording started by `render.PushComplexShadow` and draws the shadow.

**Source:** `rendering/framework/render.lua`

  ### `render.PushRotation`
**Kind:** `fn`

**Arguments:** `ang, center?`

Starts a rotation. Everything drawn until the matching `render.PopRotation` call is rotated by `ang`.

Only one rotation can be active at a time. If you call this function again before
      `render.PopRotation`, the new rotation replaces the old one.
    
  

**Source:** `rendering/framework/render.lua`

  ### `render.PopRotation`
**Kind:** `fn`

Applies and ends the rotation started by `render.PushRotation`. Throws an error if `render.PushRotation` was not called earlier in the same frame.

**Source:** `rendering/framework/render.lua`

  ### `render.LoadFont`
**Kind:** `fn`

**Arguments:** `builtin`

Loads one of the ESP fonts. The size, effect and hinting of these fonts come from the ESP settings. If you pass them, they are ignored.

**Source:** `rendering/framework/render.lua`

  ### `render.LoadFont`
**Kind:** `fn`

**Arguments:** `name, size, effect?, hinting?`

Loads a font at a given size. Pass the name of a font family, or the path of a font file in the folder of the script. Font files can be `.ttf`, `.otf` or `.ttc` files.

A name can be the name of a font installed on the system or of a font that comes with the
      cheat. An unknown name throws an error. In the first moments after the game starts, the list of
      installed fonts is still being read. During this time any name is accepted, and the font stays
      not `loaded` until the list is ready. If the name is unknown, a script error is shown at that
      point and the `failed` field of the font becomes `true`.

      A name that contains `/` or `\`, or that ends in `.ttf`, `.otf` or `.ttc`, is a path. The path
      is relative to the folder of the script, in the same way as for `render.LoadImageFromFile`. You
      can also pass a path that `cloud.ReadResource` returned for a file of the script. An absolute
      path works only when it points inside the folder of the script. Any other path throws the same
      error as a missing file.

      The file is read and checked when you call this function. Throws an error if the file is not a
      usable scalable font, or if it is larger than 32 MB. A `.ttc` file loads its first face. Each
      file is read only once. Loading it again at any size uses the same data until the script
      reloads, so you can call this function from a `render` callback without a large cost.

      Throws an error if `size` is not above `0` and at most `512`, or if `effect` or `hinting` is not
      a value of its enum.
    
  

**Source:** `rendering/framework/render.lua`

  ### `render.DrawText`
**Kind:** `fn`

**Arguments:** `position, color, font, flags?, ...?`

Draws text. The values after `flags` are converted to strings with `tostring` and joined to make the text. If the font is not loaded yet, nothing is drawn.

`flags` is a string of letters. Use `c` to center the text on `position`, `r` to align the
      right edge of the text to `position`, and `d` to apply DPI scaling. If you pass both `c` and
      `r`, the text is centered.

      Text is placed on whole pixels. A font with `render.FontHinting.Msdf` keeps the exact position,
      so its text can move smoothly.
    
  

**Source:** `rendering/framework/render.lua`

  ### `render.DrawTextEx`
**Kind:** `fn`

**Arguments:** `position, color, font, size, flags?, ...?`

Draws text at `size` pixels instead of the size of the font. In all other ways it works like `render.DrawText`.

With a `render.FontHinting.Msdf` font, the text stays sharp at any size and the space between
      the letters stays exact. So you can animate the size without loading more fonts. Other fonts
      are stretched and become blurry at sizes other than their own size. An outline or drop shadow
      is scaled with the text.

      Throws an error if `size` is not above `0` and at most `512`.
    

    
      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/rendering/render:1:8bcdbd9fb8f1"
      local font = render.LoadFont("Calibri", 24, render.FontEffect.kOutline, render.FontHinting.Msdf)

      events.render:Set(function()
          render.DrawTextEx(Vector2D(200, 200), Color(255, 255, 255), font, 48, "c", "Hello")
      end)
      ```
    
  

**Source:** `rendering/framework/render.lua`

  ### `render.MeasureText`
**Kind:** `fn`

**Arguments:** `font, flags?, ...?`

Returns the size in pixels that the text would have if it was drawn with `render.DrawText`. Takes the same `flags` and values as `render.DrawText`. Only the `d` flag changes the result.

Returns `Vector2D(0, 0)` if the text is empty or the font is not loaded yet.
    
  

**Source:** `rendering/framework/render.lua`

  ### `render.MeasureTextEx`
**Kind:** `fn`

**Arguments:** `font, size, flags?, ...?`

Returns the size in pixels that the text would have if it was drawn with `render.DrawTextEx` at `size`. Returns `Vector2D(0, 0)` if the text is empty or the font is not loaded yet.

Throws an error if `size` is not above `0` and at most `512`.
    
  

**Source:** `rendering/framework/render.lua`

  ### `render.LoadImageFromFile`
**Kind:** `fn`

**Arguments:** `file, size?`

Loads an image from a file. This version adds the formats and the search order of the game to the loader of the framework. The file is searched for in this order.

1. A path that ends with `.vsvg_c` or `.vtex_c` is loaded as a compiled game texture.
      2. Other paths are first searched for in the files of the running script.
      3. Then the path is searched for in the working directory of the game.
      4. Then the path is searched for in the search paths of the engine file system.

      For steps 2 to 4, the path must be relative. An absolute path, a path with a drive letter, or a
      path that uses `..` to leave the working directory throws an error. If the file is not found,
      an error is thrown.

      The resources that are available depend on the game content that is installed. Load images
      outside the render event.
    
  

**Source:** `rendering/game/render.lua`

  ### `render.PushTransform`
**Kind:** `fn`

**Arguments:** `transform`

Starts drawing on a plane in the world. Everything that is drawn before `render.PopTransform` is placed on the plane of `transform` instead of on the screen. The screen position `(x, y)` of a shape is used as a position on the plane. The shape is placed at `origin + left * x - up * y` and then projected to the screen.

* The forward axis of the transform is the normal of the plane. It points at the viewer. Text
        drawn at `(0, 0)` with the `"c"` flag is centred on the origin and reads from left to right
        when you look at the plane from the front.
      * The scale of the transform is the number of world units for each pixel on the plane. For
        example, with a scale of `0.1`, a font with a size of 12 pixels is about 1.2 units tall.
      * Only one transform can be active at a time. Calling this function again before
        `render.PopTransform` throws an error. Use `render.PushRotation` inside the block to rotate
        shapes on the plane.

      Only the corners of each shape are projected on the CPU. Circles, ellipses, rounded
      rectangles, gradients, images and `render.DrawShader` find the position of each pixel on the
      plane in the shader. Because of this, they keep their shape in perspective. For example, a
      circle that you see at an angle looks like an ellipse. The glyphs of text and the colours of
      simple shapes are interpolated across their triangles instead. At the size of a glyph, you
      cannot see the difference.

      Screen space effects do not follow the plane. Blur, shadows, glows, clip shaders, clip
      rectangles and the `nl_backdrop` of a shader all stay in screen space. Do not use them inside
      the block. A clip rectangle that was pushed before the block still clips the block on the
      screen. Text that is more than 8192 pixels away from the origin of the plane is not drawn.

      Call this function only from the `render` event, and call `render.PopTransform` in the same
      event. If the block is still open when the callback returns, the block is closed and a script
      error is shown. Its shapes are drawn without projection for that frame.
    

    
      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/rendering/render:2:6bd7517e5b6d"
      local font = render.LoadFont("Verdana", 24)

      events.render:Set(function()
          local origin = Vector(0, 0, 64)
          render.PushTransform(CTransform(origin, QAngle(0, 90, 0), 0.1))
          render.DrawText(Vector2D(0, 0), Color(255, 255, 255), font, "c", "hello")
          render.PopTransform()
      end)
      ```
    
  

**Source:** `rendering/game/render.lua`

  ### `render.PopTransform`
**Kind:** `fn`

Ends the block that `render.PushTransform` started, and projects it to the screen. Calling this function without an open block throws an error.

If any vertex of the block is behind the camera, the whole block is dropped. A triangle that
      reaches behind the eye cannot be projected.
    
  

**Source:** `rendering/game/render.lua`

  ### `render.Rounding`
**Kind:** `type`

The rounding of the corners of a rectangle. Pass one number to round all four corners by the same radius. Pass an array of up to four numbers to give each corner its own radius.

**Source:** `rendering/framework/render.lua`

  ### `render.ImageMode`
**Kind:** `type`

How an image fills the rectangle it is drawn in. Used by `render.DrawImage` and `group:AddImage`.


  

**Source:** `rendering/framework/render.lua`

  ### `render.EspFont`
**Kind:** `type`

The fonts that the ESP uses.


  

**Source:** `rendering/framework/render.lua`

  ### `render.FontEffect`
**Kind:** `type`

An outline or shadow that is added to every character of a font.


  

**Source:** `rendering/framework/render.lua`

  ### `render.FontHinting`
**Kind:** `type`

The hinting of a font. Hinting decides how the outlines of the characters are fitted to the pixel grid, and so it also decides the width of the text.



    
      All values except `DirectWriteAliased` draw text with smooth grayscale edges from FreeType.

      The `DirectWrite` values follow the hinting instructions of the font in the same way as
      DirectWrite. The font and the size still decide whether the outlines are drawn natural,
      symmetric or without hinting.

      The `AutoHint` values ignore the hinting instructions of the font and use the FreeType
      autohinter.
    
  

**Source:** `rendering/framework/render.lua`


## Image _()_

An image that you can draw with `render.DrawImage` or show in the menu.

Images are decoded in the background after the load function returns. Until `loaded` is
`true`, `size` can be `0x0` or the size that you asked for. An SVG image and an image made from
an RGBA buffer have their size at once.

An empty image, such as the avatar of a user who has no avatar, has the size `0x0`, its `loaded`
field is `false` and its `failed` field is `true`.


  ### `Image.failed`
**Kind:** `prop`

Whether the image will never load. This is `true` if the data could not be decoded or if there is no image at all. This field is read only.

**Source:** `rendering/framework/render.lua`

  ### `Image.loaded`
**Kind:** `prop`

Whether the image is decoded and can be drawn. Until it is `true`, `render.DrawImage` draws nothing. This field is read only.

**Source:** `rendering/framework/render.lua`

  ### `Image.size`
**Kind:** `prop`

The width and height of the image in pixels. Check `loaded` before you use this value.

**Source:** `rendering/framework/render.lua`


## Shader _()_

A pixel shader that the script writes in HLSL. Create it with `render.CreateShader` and draw it
with `render.DrawShader`.

The shader is compiled in the background, so it is not ready at once. Until it is ready,
drawing it draws nothing.

A script can keep at most 32 shaders. A shader that failed to compile still counts toward
this limit until you release it. All shaders of a script are released when the script unloads.


  ### `Shader:IsValid`
**Kind:** `method`

Returns `true` if the shader still exists. Returns `false` after `Release` is called.

**Source:** `rendering/framework/render.lua`

  ### `Shader:IsReady`
**Kind:** `method`

Returns `true` if the shader is compiled and can be drawn.

**Source:** `rendering/framework/render.lua`

  ### `Shader:GetError`
**Kind:** `method`

Returns the error message if the shader failed. Line numbers in the message are counted from the first line of the source that the script passed.

If the shader compiled but the graphics device could not create it, the message says so.
      Returns `nil` while the shader compiles and after it is created successfully.
    
  

**Source:** `rendering/framework/render.lua`

  ### `Shader:Release`
**Kind:** `method`

Releases the shader now, before the script unloads. If you draw the shader after this, it throws an error. Calling this function on a shader that is already released does nothing.

**Source:** `rendering/framework/render.lua`


## render.ShaderOptions _()_

Options for `render.DrawShader`. If a field has the wrong type, `render.DrawShader` throws an
error.


  ### `render.ShaderOptions.blur`
**Kind:** `prop`

How strongly `nl_backdrop` is blurred. `0` gives an almost sharp copy. Must be `0` or more. Defaults to `1`.

**Source:** `rendering/framework/render.lua`

  ### `render.ShaderOptions.color`
**Kind:** `prop`

The vertex colour that the shader reads with `nl_vertex_color`. Defaults to opaque white.

**Source:** `rendering/framework/render.lua`

  ### `render.ShaderOptions.image`
**Kind:** `prop`

The image that the shader reads as `nl_image`.

**Source:** `rendering/framework/render.lua`

  ### `render.ShaderOptions.params`
**Kind:** `prop`

Up to 64 numbers that the shader reads as `nl_params`. Missing numbers are `0`. More than 64 numbers, or a NaN or infinite number, throw an error.

**Source:** `rendering/framework/render.lua`

  ### `render.ShaderOptions.rounding`
**Kind:** `prop`

The corner radius in pixels. The output of the shader is cut to these corners. Each radius is limited to half of the shorter side of the rectangle. A NaN or infinite number throws an error.

**Source:** `rendering/framework/render.lua`


## Font _()_

A font that you can use with `render.DrawText` and `render.MeasureText`.

Fonts load in the background. While a font is not loaded, `render.DrawText` draws nothing with
it and `render.MeasureText` returns `Vector2D(0, 0)`. Check `loaded` before you draw. The
`loaded` field is `false` both while the font is loading and after it failed. Use `failed` to
tell these two cases apart, for example to switch to another font.

A font family name can match several font files. The `file` and `face` fields tell you which
file and which face were chosen.

Only `size` can be changed. Assigning to any other field does nothing.


  ### `Font.face`
**Kind:** `prop`

The index of the face inside the font file. A `.ttc` file holds several faces, and the named styles of a variable font also use this index. This field is read only.

**Source:** `rendering/framework/render.lua`

  ### `Font.failed`
**Kind:** `prop`

Whether the font cannot load at its current size. This is `true` if the font name is not installed or if the size could not be loaded. Always `false` for a font from `render.EspFont`. A font file is checked when you load it, so a font loaded from a file does not fail later because the file cannot be read. This field is read only.

**Source:** `rendering/framework/render.lua`

  ### `Font.file`
**Kind:** `prop`

The file name of the font file that was chosen, without its folder. For a font loaded from a file, this is the file name from the path that you passed. Empty for a font from `render.EspFont` and for a font that is not loaded yet. This field is read only.

**Source:** `rendering/framework/render.lua`

  ### `Font.loaded`
**Kind:** `prop`

Whether the font is ready to draw. A new size is ready one frame or more after it is set. This field is read only.

**Source:** `rendering/framework/render.lua`

  ### `Font.size`
**Kind:** `prop`

The font size in pixels. You can assign a new size from above `0` to `512`. Other values throw an error. Assigning a new size loads the font again and resets `failed`. For a font from `render.EspFont`, an assigned size is ignored.

**Source:** `rendering/framework/render.lua`

---


## Ui

<!-- Source: cs2/api_reference/rendering/ui.md -->

---
title: User Interface
icon: layout-grid
sidebar_position: 2
---

# User Interface

## ui _()_

Creates and reads menu elements. Scripts can add their own tabs, groups and controls to the
menu, read and change the values of these controls, and find the built-in options of the cheat
to react to them.

The paths of built-in options can change between updates. Check that the result of `ui.Lookup`
is not `nil` before you use its fields.


  ### `ui.Lookup`
**Kind:** `fn`

**Arguments:** `root, ...?`

Finds a menu element by its path. Returns `nil` if no element matches the path. Throws an error if it is called without arguments, or if a segment is not a string.

A path has one segment for each menu level, from the root down to the element. The segments are
      the category, the tab, the group, any nested groups, and then the control. Write each segment as
      it is shown in the English menu. Case and colour codes are ignored, so `"Hit Chance"` and
      `"hit chance"` find the same control.

      To get the exact path of a control, turn on developer mode, right click the control and choose
      Copy Path. The generated menu reference in `menu/*.lua` lists every built-in path, with the type
      of the control and the values that it accepts.

      The type of the return value depends on where the path ends.

      | The path ends at                     | Returns                                                                                      |
      | ------------------------------------ | -------------------------------------------------------------------------------------------- |
      | a control                            | `ui.Variable`. If the control has a colour picker, the picker is returned as a second value. |
      | a group                              | `ui.Group`                                                                                   |
      | a tab                                | `ui.Tab`                                                                                     |
      | only `"Settings"` or `"ESP Options"` | `ui.Group`                                                                                   |
      | only `"Aimbot"` or `"Common"`        | `nil`                                                                                        |

      Some groups have weapon presets or map presets. By default, a path finds the element in the
      global preset. To find the element in a specific preset, add the name of the preset right after
      the tab, for example `ui.Lookup("Aimbot", "Rage", "AWP", "Selection", "Hit Chance")`.
    

    
      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/rendering/ui:0:bf2b205c07db"
      local hit_chance = ui.Lookup("Aimbot", "Rage", "Selection", "Hit Chance") --[[@as ui.Variable?]]
      local double_tap = ui.Lookup("Common", "Visuals", "World", "Miscellaneous", "Windows", "Indicators", "Double Tap") --[[@as ui.Variable?]]
      local menu_scale = ui.Lookup("Settings", "Menu Scale") --[[@as ui.Variable?]]

      if hit_chance then print("hit chance: ", hit_chance.value) end
      if double_tap and double_tap.value then print("double tap indicator is on") end
      if menu_scale then print("menu scale: ", menu_scale.value) end
      ```
    
  

**Source:** `rendering/framework/ui.lua`

  ### `ui.AddTab`
**Kind:** `fn`

**Arguments:** `name`

Returns the tab with the given name in the sidebar entry of your script. If the tab does not exist, it is created.

**Source:** `rendering/framework/ui.lua`

  ### `ui.GetAlpha`
**Kind:** `fn`

Returns the current opacity of the menu, from `0` to `1`.

**Source:** `rendering/framework/ui.lua`

  ### `ui.GetScale`
**Kind:** `fn`

**Arguments:** `type?`

Returns the DPI scale factor of the given part of the interface.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Scaled`
**Kind:** `fn`

**Arguments:** `value, type?`

Returns `value` multiplied by the DPI scale factor of the given part of the interface.

**Source:** `rendering/framework/ui.lua`

  ### `ui.GetPosition`
**Kind:** `fn`

Returns the position of the top left corner of the main menu window. Returns `nil` if the menu window has not been created yet.

**Source:** `rendering/framework/ui.lua`

  ### `ui.GetSize`
**Kind:** `fn`

Returns the size of the main menu window. Returns `nil` if the menu window has not been created yet.

**Source:** `rendering/framework/ui.lua`

  ### `ui.GetCursorPosition`
**Kind:** `fn`

Returns the current position of the mouse cursor on the screen.

**Source:** `rendering/framework/ui.lua`

  ### `ui.GetStyleColor`
**Kind:** `fn`

**Arguments:** `name`

Returns one colour of the current menu theme. Returns `nil` if the name is not a known colour name.

**Source:** `rendering/framework/ui.lua`

  ### `ui.GetStyle`
**Kind:** `fn`

Returns all colours of the current menu theme in a table. The keys are the colour names and the values are `Color` objects.

**Source:** `rendering/framework/ui.lua`

  ### `ui.SetSidebarEntry`
**Kind:** `fn`

**Arguments:** `name?, icon?`

Sets the name, the icon, or both, of the sidebar entry of your script. If the entry does not exist yet, it is created.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Localize`
**Kind:** `fn`

**Arguments:** `lang, name, value`

Adds a translation of a menu string for one language. When the user selects that language, the menu shows the translation. Throws an error if the language is not known.

`name` is a string that your script uses as menu text, such as the name of a control.
    
  

**Source:** `rendering/framework/ui.lua`

  ### `ui.GetUnitSystem`
**Kind:** `fn`

Returns the measurement system that the user selected in the menu, either `"imperial"` or `"metric"`.

**Source:** `rendering/framework/ui.lua`

  ### `ui.ConvertDistance`
**Kind:** `fn`

**Arguments:** `distance`

Converts a distance in game units to the measurement system of the user. One game unit is one inch.

Returns three values. The first value is the converted distance. The second value is the unit
      suffix, `"m"` for metres or `"ft"` for feet. The third value is the measurement system, either
      `"imperial"` or `"metric"`.
    
  

**Source:** `rendering/framework/ui.lua`

  ### `ui.GetActiveBinds`
**Kind:** `fn`

**Arguments:** `include_hidden?`

Returns the key binds that are active now.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Lookup.Root`
**Kind:** `type`

The first segment of a path for `ui.Lookup`.

**Source:** `rendering/framework/ui.lua`

  ### `ui.StyleName`
**Kind:** `type`

The names of the colours in the menu theme.

**Source:** `rendering/framework/ui.lua`

  ### `ui.UnitSystem`
**Kind:** `type`

**Source:** `rendering/framework/ui.lua`

  ### `ui.ColorKey`
**Kind:** `type`

Selects a colour in a set or in a category, either by its position starting at 1 or by its name.

**Source:** `rendering/framework/ui.lua`

  ### `ui.ScaleType`
**Kind:** `type`

The parts of the interface that each have their own DPI scale.


  

**Source:** `rendering/framework/ui.lua`

  ### `ui.GroupColumn`
**Kind:** `type`

The column that a group is placed in.


  

**Source:** `rendering/framework/ui.lua`


## ui.Element _()_

The fields and methods that all menu element handles share. Tabs, groups and controls are all
elements.

Read a field to get its value, and assign the field to change it. Reading or assigning a field
that the element does not have throws an error. Assigning a read only field also throws an
error.

Two handles are equal with `==` when they point to the same element. For example, two calls to
`ui.Lookup` with the same path return equal handles.

You can read the fields of any element. You can assign `name`, `visible`, `disabled` and `icon`
only on elements that your script created. Assigning them on other elements throws an error.


  ### `ui.Element.disabled`
**Kind:** `prop`

Whether the element is disabled. A disabled element is grey and the user cannot change it.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Element.icon`
**Kind:** `prop`

The name of the icon of the element.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Element.id`
**Kind:** `prop`

A number that identifies the element. It is unique in the current session and is never used again for another element in that session. Read only. The element gets a new number after a reload or a restart, so do not save it.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Element.name`
**Kind:** `prop`

The name of the element, as it is shown in the English menu.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Element.visible`
**Kind:** `prop`

Whether the element is shown.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Element:Export`
**Kind:** `method`

Returns the current state of the element as a JSON string.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Element:Import`
**Kind:** `method`

**Arguments:** `str`

Sets the state of the element from a string that `Export` returned. Throws an error if the string is not valid JSON.

**Source:** `rendering/framework/ui.lua`


## ui.Tab _()_



A tab in the sidebar of the menu. A tab holds groups.


  ### `ui.Tab:AddGroup`
**Kind:** `method`

**Arguments:** `name, column?`

Returns the group with the given name in this tab. If the group does not exist, it is created.

**Source:** `rendering/framework/ui.lua`


## ui.Group _()_



A group in a tab. A group holds controls.


  ### `ui.Group:Entity`
**Kind:** `method`

**Arguments:** `name, default_color?`

Adds an entity selector with a colour picker. This is a menu control, and it is not related to game entities. Use `Chip` on the returned control to add items that the user can select.

The main switch and each chip have their own colour, like the world entities in the ESP
      settings.
    
  

**Source:** `rendering/framework/ui.lua`

  ### `ui.Group:AddToggle`
**Kind:** `method`

**Arguments:** `name, default_value?`

Adds a checkbox.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Group:AddCombo`
**Kind:** `method`

**Arguments:** `name, items`

Adds a dropdown where the user selects one item. The first item is selected at the start.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Group:AddMultiCombo`
**Kind:** `method`

**Arguments:** `name, items`

Adds a dropdown where the user can select several items. No item is selected at the start. Throws an error if there are more than 32 items.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Group:AddList`
**Kind:** `method`

**Arguments:** `name, items`

Adds a list where the user selects one item. The first item is selected at the start.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Group:AddMultiList`
**Kind:** `method`

**Arguments:** `name, items`

Adds a list where the user can select several items. No item is selected at the start. Throws an error if there are more than 32 items.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Group:AddTextBox`
**Kind:** `method`

**Arguments:** `name, default_value?`

Adds a text input.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Group:AddLabel`
**Kind:** `method`

**Arguments:** `name`

Adds a line of text that the user cannot change.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Group:AddButton`
**Kind:** `method`

**Arguments:** `name`

Adds a button. Use `AddCallback` on the returned control to run code when the button is clicked.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Group:AddHotkey`
**Kind:** `method`

**Arguments:** `name, default_key?`

Adds a hotkey control.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Group:AddColorPicker`
**Kind:** `method`

**Arguments:** `name, default_value?`

Adds a colour picker with one colour.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Group:AddColorSet`
**Kind:** `method`

**Arguments:** `name, colors`

Adds a colour set. A colour set is an ordered list of two or more colours, and each colour has a fixed role. For example, the first colour can be the start of a gradient and the second colour can be the end. Throws an error if there are fewer than two colours.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Group:AddColorModes`
**Kind:** `method`

**Arguments:** `name, categories`

Adds a mode picker. A mode picker has two or more categories, and each category has its own colours. The user selects one category at a time. Throws an error if there are fewer than two categories.

With up to three categories, the categories are shown as tabs. With more than three categories,
      they are shown in a dropdown.
    
  

**Source:** `rendering/framework/ui.lua`

  ### `ui.Group:AddImage`
**Kind:** `method`

**Arguments:** `image, size?, rounding?, mode?, color?`

Adds an image to the menu.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Group:AddSlider`
**Kind:** `method`

**Arguments:** `name, min, max, default?, scale?, format?`

Adds a slider. A slider stores an integer. `min` and `max` are rounded to the nearest integer. Throws an error if `min` or `max` is not a finite number, or if `max` is less than `min`.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Group.AddSlider.Format`
**Kind:** `type`

Sets how the value of a slider is shown.

Pass a string to show it as a suffix after the number. The string is cut to five bytes, without
      splitting a character. It is not a printf format.

      Pass a function to build the whole text from the stored integer value. If the range of the
      slider is wider than 65536 steps, the function is not used and an error message is printed.
    
  

**Source:** `rendering/framework/ui.lua`


## ui.Variable _()_



A control in a group. Read `v.value` to get the value, and assign `v.value = x` to change it.
The type of `value` depends on the type of the control.

| Control      | `value`                                                                                                              |
| ------------ | -------------------------------------------------------------------------------------------------------------------- |
| toggle       | `boolean`                                                                                                            |
| slider       | `integer`, the stored value before the scale is applied                                                              |
| combo        | `string`, the name of the selected item                                                                              |
| list         | `integer`, the position of the selected item, starting at 1                                                          |
| multicombo   | `string[]`, the names of the selected items                                                                          |
| multilist    | `integer[]`, the positions of the selected items, starting at 1                                                      |
| textbox      | `string`                                                                                                             |
| label        | `string`, the text of the label. Assigning it has no effect.                                                         |
| button       | `true` in the frame when the button is clicked. Assigning any value clicks the button.                               |
| color picker | `Color`. See `ui.ColorPicker`.                                                                                       |
| color set    | A view of the colours. See `ui.ColorSet`.                                                                            |
| color modes  | A view of the categories. See `ui.ColorModes`.                                                                       |
| skyboxes     | `string`, the material file name without the extension, such as `"sky_de_dust2"`. `"Default"` is the sky of the map. |

When you assign a combo, the name is compared without case, and a name that is not in the list
is ignored. To assign a multicombo or a multilist, pass a list of item names or positions, or a
table with item names as keys and `true` as values.

A skybox selector lists its names in `items`. Until the game materials are scanned, the list
holds only `"Default"`, and reading a selected skybox returns `nil`. After the scan, assigning a
name that is not in the list is ignored. Assigning a value that is not a string throws an error.

Some fields exist only for some control types. These are `values`, `raw_values`,
`effective_values`, `is_down`, `items`, `min`, `max`, `category` and `categories`. Reading them
on other control types throws an error, in the same way as unknown fields.


  ### `ui.Variable.default`
**Kind:** `prop`

The default value, which the control is reset to. You can assign it only on controls that your script created. Assigning it also sets the value, unless the config of the user already has a saved value for this control. In that case, the saved value is kept. An active override is not changed.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable.effective_value`
**Kind:** `prop`

The value that features use. This is `raw_value` if an override is set, and `value` if not. Read only. It is the same as `variable:Get()`.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable.effective_values`
**Kind:** `prop`

Multi select controls only. The override if there is one, and the selected items if not. Read only.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable.is_down`
**Kind:** `prop`

Hotkeys only. Whether the hotkey is active now. Read only.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable.items`
**Kind:** `prop`

Combos, lists and skybox selectors only. The names of the items. Assign a table to replace the items of a combo or a list that your script created. The items of a skybox selector are read only.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable.max`
**Kind:** `prop`

Sliders only. The largest value. You can assign it only on sliders that your script created. The new value must be a finite number that is greater than `min`, or an error is thrown. If the range becomes wider than 65536 steps, the format function is no longer used.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable.min`
**Kind:** `prop`

Sliders only. The smallest value. You can assign it only on sliders that your script created. The new value must be a finite number that is less than `max`, or an error is thrown. If the range becomes wider than 65536 steps, the format function is no longer used.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable.parent`
**Kind:** `prop`

The group that holds this control. Read only.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable.raw_value`
**Kind:** `prop`

The override. Reading it returns the active override, or `nil` if there is none. Assigning it sets an override that features use instead of `value`, and the config of the user is not changed. Assign `nil` to remove the override.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable.raw_values`
**Kind:** `prop`

Multi select controls only. The override as a table in the same form, or `nil` if there is no override. Assigning it works like assigning `raw_value`.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable.tooltip`
**Kind:** `prop`

The tooltip text. You can assign it only on controls that your script created.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable.type`
**Kind:** `prop`

The name of the control type, such as `toggle`, `slider` or `combo`. Read only.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable.value`
**Kind:** `prop`

The value in the menu. Reading it ignores an active override.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable.values`
**Kind:** `prop`

Multi select controls only. The selected items as a table, with item names as keys and `true` as values. Assigning it works like assigning `value`.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable:Get`
**Kind:** `method`

**Arguments:** `...?`

Returns the value that features use. This is the `raw_value` override if one is set, and `value` if not. Without arguments, it is the same as reading `effective_value`. For colour sets and colour modes, it returns a view of the colours.

For multi select controls, you can pass the name of an item, or its position starting at 1. The
      function then returns whether that item is selected, instead of the whole selection.
    
  

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable:GetValue`
**Kind:** `method`

**Arguments:** `...?`

Returns the value in the menu and ignores an active override. This is the method form of reading `value`. Multi select controls accept the same arguments as `Get`.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable:SetValue`
**Kind:** `method`

**Arguments:** `value`

Sets the value in the menu. This also removes an active override. This is the method form of assigning `value`.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable:GetRawValue`
**Kind:** `method`

**Arguments:** `...?`

Returns the active override, or **nil** if there is none. This is the method form of reading `raw_value`. Multi select controls accept the same arguments as `Get`.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable:SetRawValue`
**Kind:** `method`

**Arguments:** `value`

Sets an override that features use instead of the menu value. The config of the user is not changed. Pass **nil** to remove the override. This is the method form of assigning `raw_value`.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable:Chip`
**Kind:** `method`

**Arguments:** `name, default_value?`

Adds a chip to an entity selector that your script created. The user can select the chip, and the chip has its own colour. Throws an error if the control is not an entity selector, or if another script created it.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable:AddGroup`
**Kind:** `method`

Returns a group that is attached to this control, so that you can add controls inside it. If the control already has such a group, that group is returned. Throws an error if the control is itself inside a group that is attached to another control.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable:AddCallback`
**Kind:** `method`

**Arguments:** `fn, call_now?`

Adds a function that is called with the control when its value changes. You can add several different functions. Adding the same function again has no effect.

Pass `true` for `call_now` to also call the function once right away. If the function throws
      an error, the error is printed and is not passed to the caller.
    
  

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable:RemoveCallback`
**Kind:** `method`

**Arguments:** `fn`

Removes a function that was added with `AddCallback`. Pass the same function value that you added.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable:AddColorPicker`
**Kind:** `method`

**Arguments:** `default_value?`

Adds a colour picker with one colour next to this control, for example next to a toggle, and returns the picker. Throws an error if your script did not create the control, or if the control does not support colour pickers.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable:AddColorSet`
**Kind:** `method`

**Arguments:** `colors`

Adds a colour set with two or more colours, each with a fixed role, next to this control, and returns the picker. Throws an error if there are fewer than two colours, if your script did not create the control, or if the control does not support colour pickers.

**Source:** `rendering/framework/ui.lua`

  ### `ui.Variable:AddColorModes`
**Kind:** `method`

**Arguments:** `categories`

Adds a mode picker with two or more categories next to this control, and returns the picker. Throws an error if there are fewer than two categories, if your script did not create the control, or if the control does not support colour pickers.

**Source:** `rendering/framework/ui.lua`


## ui.ColorPicker _()_



A picker with exactly one colour. `value`, `raw_value` and `default` are plain `Color` values.


  ### `ui.ColorPicker.default`
**Kind:** `prop`

The default colour.

**Source:** `rendering/framework/ui.lua`

  ### `ui.ColorPicker.raw_value`
**Kind:** `prop`

The override colour, or `nil` if there is none. Assign `nil` to remove the override.

**Source:** `rendering/framework/ui.lua`

  ### `ui.ColorPicker.value`
**Kind:** `prop`

The colour.

**Source:** `rendering/framework/ui.lua`


## ui.ColorsNode _()_

A live view of the colours of a colour set. Index it with a `ui.ColorKey` to read a `Color` or
to assign one. An assignment changes the control right away. The view supports `#`, `pairs` and
`ipairs`. With `pairs`, the keys are the colour names, or the positions for colours that have no
name.

When you get the view through `raw_value`, reading a colour that has no override returns `nil`.
Assigning `nil` to a colour removes the override of that colour only. In other views, assigning
`nil` throws an error.

The view always shows the current state of the control, so you can keep it for as long as you
want.


  ### `ui.ColorsNode[string|integer]`
**Kind:** `prop`

**Source:** `rendering/framework/ui.lua`

  ### `#ui.ColorsNode`
**Kind:** `meta`

**Source:** `rendering/framework/ui.lua`


## ui.ColorSet _()_



A picker with an ordered set of two or more colours, each with a fixed role. It has no single
value. Use `value[color]` to read or assign one colour.


  ### `ui.ColorSet.default`
**Kind:** `prop`

The default colours, one for each colour. You can assign them only on pickers that your script created. If the config of the user has a saved value, the saved value is kept.

**Source:** `rendering/framework/ui.lua`

  ### `ui.ColorSet.raw_value`
**Kind:** `prop`

The overrides, one for each colour. You can override each colour separately. Assign `nil` to the field itself to remove all overrides.

**Source:** `rendering/framework/ui.lua`

  ### `ui.ColorSet.value`
**Kind:** `prop`

The colours. `v.value['start'] = Color(...)` changes one colour.

**Source:** `rendering/framework/ui.lua`


## ui.ColorCategoryNode _()_

A live view of the categories of a mode picker. Index it with the position of a category,
starting at 1, or with its name, to get the colours of that category. The view supports `#`,
`pairs` and `ipairs` over the categories.


  ### `ui.ColorCategoryNode[string|integer]`
**Kind:** `prop`

**Source:** `rendering/framework/ui.lua`

  ### `#ui.ColorCategoryNode`
**Kind:** `meta`

**Source:** `rendering/framework/ui.lua`


## ui.ColorModes _()_



A picker with two or more categories, also called modes. The user switches between them, and
features check `category` to know which mode is selected. Each category has its own colours,
which you address with `value[category][color]`.


  ### `ui.ColorModes.categories`
**Kind:** `prop`

The names of the categories. Read only.

**Source:** `rendering/framework/ui.lua`

  ### `ui.ColorModes.category`
**Kind:** `prop`

The selected category. Reading it returns the position of the category, starting at 1. Assign a position or a category name. Throws an error if the category does not exist.

**Source:** `rendering/framework/ui.lua`

  ### `ui.ColorModes.default`
**Kind:** `prop`

The default colours, addressed in the same way.

**Source:** `rendering/framework/ui.lua`

  ### `ui.ColorModes.raw_value`
**Kind:** `prop`

The overrides, addressed in the same way. Assign `nil` to the field itself to remove all overrides.

**Source:** `rendering/framework/ui.lua`

  ### `ui.ColorModes.value`
**Kind:** `prop`

The colours of all categories. `v.value['Gradient']['start'] = Color(...)` changes one colour.

**Source:** `rendering/framework/ui.lua`

  ### `ui.ColorModes.Mode`
**Kind:** `type`

One mode of a picker that `AddColorModes` created. A mode is a category with its own colours.

**Source:** `rendering/framework/ui.lua`


## ui.ActiveBind _()_

One key bind in the key bind list on the screen.


  ### `ui.ActiveBind.is_active`
**Kind:** `prop`

Whether the bind is active now.

**Source:** `rendering/framework/ui.lua`

  ### `ui.ActiveBind.mode`
**Kind:** `prop`

How the bind is activated. `0` is toggle and `1` is hold.

**Source:** `rendering/framework/ui.lua`

  ### `ui.ActiveBind.name`
**Kind:** `prop`

The name of the option that the bind controls, as it is shown in the list.

**Source:** `rendering/framework/ui.lua`

  ### `ui.ActiveBind.reference`
**Kind:** `prop`

The menu control that the bind belongs to.

**Source:** `rendering/framework/ui.lua`

  ### `ui.ActiveBind.should_show`
**Kind:** `prop`

Whether the bind should be shown in the list.

**Source:** `rendering/framework/ui.lua`

  ### `ui.ActiveBind.value`
**Kind:** `prop`

The current value of the option that the bind controls.

**Source:** `rendering/framework/ui.lua`

---


## View

<!-- Source: cs2/api_reference/rendering/view.md -->

---
title: View
icon: camera
---

# View

## view\.ViewContext _()_

The camera of the current frame. It is passed to the `override_view` event. Read and change
`origin`, `angles` and `fov` inside that callback. Do not keep the object after the callback
returns.

Reading or writing a field that does not exist throws an error.


  ### `view.ViewContext.angles`
**Kind:** `prop`

The view angles of the camera.

**Source:** `rendering/game/view.lua`

  ### `view.ViewContext.fov`
**Kind:** `prop`

The field of view of the camera in degrees. Setting it to `0` throws an error.

**Source:** `rendering/game/view.lua`

  ### `view.ViewContext.origin`
**Kind:** `prop`

The position of the camera in world space.

**Source:** `rendering/game/view.lua`


## view\.ViewmodelContext _()_

The root of the first person viewmodel. It is passed to the `override_viewmodel` event.

When the event fires, the bob and sway of the game and the viewmodel options of the cheat are
already applied. After the event, the game still adds aim punch, the offsets for ladders and for
lowered weapons, the blend of the iron sights, and the `viewmodel_offset_*` console variables.

During the callback, `arms` and `weapon` still hold the final transform and the attachments of
the last frame. If you change `angles`, they are normalized after the callback. Do not keep the
object after the callback returns.


  ### `view.ViewmodelContext.angles`
**Kind:** `prop`

The angles of the viewmodel root.

**Source:** `rendering/game/view.lua`

  ### `view.ViewmodelContext.arms`
**Kind:** `prop`

The HUD arms that are being set up. Read-only.

**Source:** `rendering/game/view.lua`

  ### `view.ViewmodelContext.origin`
**Kind:** `prop`

The position of the viewmodel root in world space.

**Source:** `rendering/game/view.lua`

  ### `view.ViewmodelContext.weapon`
**Kind:** `prop`

The HUD weapon that is attached to `arms`. Read-only.

**Source:** `rendering/game/view.lua`

---


## Cloud

<!-- Source: cs2/api_reference/runtime/cloud.md -->

---
title: Cloud
icon: cloud
---

# Cloud

<Heading level={2} id="cloud-1">
  cloud

   _()_
</Heading>

Gets information about the user account, the running script, and the loaded config. Also
gives access to the files that are shipped with the script.


  ### `cloud.GetUser`
**Kind:** `fn`

Returns information about the user who is logged in. Other code can fake this information, so do not use it for security checks.

**Source:** `runtime/framework/cloud.lua`

  ### `cloud.GetUsername`
**Kind:** `fn`

Returns the login name of the user who is logged in. Other code can fake this name, so do not use it for security checks.

**Source:** `runtime/framework/cloud.lua`

  ### `cloud.GetScript`
**Kind:** `fn`

Returns information about the script that is running.

**Source:** `runtime/framework/cloud.lua`

  ### `cloud.GetConfig`
**Kind:** `fn`

Returns information about the config that is loaded.

Throws an error if no config is loaded. No config is loaded when the client starts, until the
      first config is applied. The error is also thrown if the loaded config was removed. Wrap the
      call in `pcall` if your code can run at these times.
    
  

**Source:** `runtime/framework/cloud.lua`

  ### `cloud.ReadResource`
**Kind:** `fn`

**Arguments:** `name`

Returns the full path on disk to a file or folder that is shipped with the running script, such as an image or a sound. Returns an empty string if the script has no file or folder with that name.

`name` is relative to the root folder of the script. For a developer script, the path points
      into the folder of the script. For a market script, the file is first extracted to a temporary
      folder. If `name` is a folder, every file in it is extracted. So the folder on disk has the
      same files for both kinds of script.

      `filesystem.List`, `filesystem.Read` and `filesystem.Info` accept the returned path. They also
      accept other paths in the files of the script that are built from it, such as its parent
      folder. This also works for market scripts.
    
  

**Source:** `runtime/framework/cloud.lua`


## cloud.User _()_

Information about the user who is logged in.


  ### `cloud.User.avatar`
**Kind:** `prop`

The avatar image.

**Source:** `runtime/framework/cloud.lua`

  ### `cloud.User.license`
**Kind:** `prop`

The license code of the account.

**Source:** `runtime/framework/cloud.lua`

  ### `cloud.User.name`
**Kind:** `prop`

The login name.

**Source:** `runtime/framework/cloud.lua`


## cloud.Item _()_

Information about a script or a config.


  ### `cloud.Item.author`
**Kind:** `prop`

The author of the item.

**Source:** `runtime/framework/cloud.lua`

  ### `cloud.Item.id`
**Kind:** `prop`

The ID of the item in the cloud. It stays the same across sessions. A developer script is not in the cloud, so its ID is valid only for the current session. Do not save the ID of a developer script.

**Source:** `runtime/framework/cloud.lua`

  ### `cloud.Item.is_market`
**Kind:** `prop`

`true` if the item comes from the market.

**Source:** `runtime/framework/cloud.lua`

  ### `cloud.Item.modify_time`
**Kind:** `prop`

The time the item was last modified, as a Unix timestamp in seconds.

**Source:** `runtime/framework/cloud.lua`

  ### `cloud.Item.name`
**Kind:** `prop`

The name of the item.

**Source:** `runtime/framework/cloud.lua`

---


## Events

<!-- Source: cs2/api_reference/runtime/events.md -->

---
title: Events
icon: paperclip
sidebar_position: 1
---

# Events

<Heading level={2} id="events-1">
  events

   _()_
</Heading>

Runs your functions when something happens in the cheat or in the game. Index `events` with
the name of an event to get an `events.Event` object for it. Call `events.<name>:Set(fn)` to
add a callback, and call `events.<name>:Unset(fn)` to remove it.

Every name returns an event object. So a misspelled name does not throw an error, but its
callback never runs.

There are three kinds of events.

**Cheat events** come from the cheat itself. Some are about the cheat, such as `render`,
`shutdown`, `config_state` and `mouse_input`. Others come from the game loop, such as
`createmove`, `pre_movement`, `override_view` and `entity_created`. Each event passes its own
arguments. They are listed on [Cheat Events](/cs2/api_reference/events/cheat).

**Game events** are the game events of the engine, under their engine names, such as
`player_death`, `round_start` and `bomb_planted`. The callback receives one table with the
fields of the event. The fields are listed on [Game Events](/cs2/api_reference/events/game).
A player field such as `userid` holds the player entity, or **nil** if the entity is not
available. `userid_raw` always holds the raw value from the engine.

**Custom events** use any other name. Scripts in the same runtime use them to send data to each
other. `events.<name>:Fire(...)` passes its arguments to every callback that is set on that
name.

The example adds a menu toggle that turns a `render` callback on and off. The callback is a
named function that is defined once, because `Unset` removes only the same function that `Set`
added.

```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/runtime/events:0:1b4a03b2654b"
local font = render.LoadFont(render.EspFont.kRegular)
local watermark = ui.AddTab("Lua"):AddGroup("Example"):AddToggle("Watermark")

local function draw_watermark()
    render.DrawText(Vector2D(16, 16), Color(255, 255, 255), font, nil, "neverlose")
end

watermark:AddCallback(function(toggle)
    if toggle:Get() then
        events.render:Set(draw_watermark)
    else
        events.render:Unset(draw_watermark)
    end
end, true) -- `true` also runs it once now, for the toggle's current value
```


  ### `events[string]`
**Kind:** `prop`

**Source:** `runtime/framework/events.lua`

  ### `events.Event`
**Kind:** `type`

` returns for one event name. `T` is the type of the callback of that event. The [Cheat Events](/cs2/api_reference/events/cheat) and [Game Events](/cs2/api_reference/events/game) pages list it for each event. The callback of a custom event receives the values that are passed to `Fire`." ret="[[{"s":"events.Event.Traits","k":"generic","h":"/cs2/api_reference/runtime/events#eventseventtraits"},{"s":"\u003c"},{"s":"T","k":"generic"},{"s":"\u003e"},{"s":"|"},{"s":"fun","k":"prim"},{"s":"("},{"s":"callback"},{"s":":"},{"s":" "},{"s":"T","k":"generic"},{"s":","},{"s":" "},{"s":"enabled"},{"s":"?"},{"s":":"},{"s":" "},{"s":"boolean","k":"prim","h":"/cs2/api_reference/lua/base#boolean"},{"s":")"}]]" source="runtime/framework/events.lua" sourceHref="https://gitlab.com/neverlose/LuaLS-CS2/-/edit/master/library/api/runtime/framework/events.lua#L78">
    
      The object does not store anything. Each time you index `events`, you get a new object, and
      two objects for the same name are equal. Callbacks belong to the script that set them, and
      they are removed when the script is unloaded. If one callback throws an error, the error is
      printed and the other callbacks still run.

      `event:Set(callback)` starts calling `callback` each time the event fires.

      `event:Unset(callback)` stops calling it. You must pass the same function value that you
      passed to `Set`. A new function with the same code is a different function.

      `event:Fire(...)` fires a custom event.

      You can also call the object directly. `event(callback)` and `event(callback, true)` set the
      callback, and `event(callback, false)` removes it.
    
  


## events.Event.Traits _()_

The methods of every `events.Event`.


  ### `events.Event.Traits:Fire`
**Kind:** `method`

**Arguments:** `...?`

Fires a custom event. Every callback that scripts in the same runtime set on this name is called with `...`. Returns `true` when the event was fired.

If the name belongs to an event that the cheat fires itself, the function does nothing and
      returns `false`. These names include `render`, `mouse_input`, `shutdown`, `config_state` and
      the game events. Use a name of your own for custom events.

      The first time this happens for a name, a warning is also added to the errors of the script.
    
  

**Source:** `runtime/framework/events.lua`

  ### `events.Event.Traits:Set`
**Kind:** `method`

**Arguments:** `callback`

Adds a callback that runs each time this event fires. If this script already set the same callback, nothing happens.

**Source:** `runtime/framework/events.lua`

  ### `events.Event.Traits:Unset`
**Kind:** `method`

**Arguments:** `callback`

Removes a callback that this script set for this event. Pass the same function that you passed to `Set`.

**Source:** `runtime/framework/events.lua`

---


## Hwmon

<!-- Source: cs2/api_reference/runtime/hwmon.md -->

---
title: Hardware Monitor
icon: chart-line
---

# Hardware Monitor

## hwmon _()_

Read only values about the frame rate and the hardware of the computer.

The frame rate values are measured over the last second. If a value is not available, it is
`0`.

Load and usage values are fractions. `0` means idle and `1` means 100 percent.


  ### `hwmon.average_fps`
**Kind:** `prop`

The average frame rate in the last second.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.cpu_base_frequency`
**Kind:** `prop`

The rated maximum frequency of the CPU, in MHz.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.cpu_cores`
**Kind:** `prop`

Information for each logical processor, sorted by `processor_id` and then by `core_id`.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.cpu_cores_count`
**Kind:** `prop`

The number of entries in `cpu_cores`.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.cpu_count`
**Kind:** `prop`

The number of logical processors.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.cpu_frequency`
**Kind:** `prop`

The average current frequency of all logical processors, in MHz.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.cpu_load`
**Kind:** `prop`

The average load of all logical processors, as a fraction. The value can be above `1` when the CPU runs faster than its base frequency.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.cpu_max_frequency`
**Kind:** `prop`

The highest current frequency of a single logical processor, in MHz.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.cpu_max_load`
**Kind:** `prop`

The highest load of a single logical processor, as a fraction.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.frame_smoothness`
**Kind:** `prop`

How even the frame times are, from 0 to 1. A higher value means more even frame times.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.gpu_count`
**Kind:** `prop`

The number of graphics adapters.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.gpu_list`
**Kind:** `prop`

Information for each graphics adapter.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.max_fps`
**Kind:** `prop`

The highest frame rate in the last second.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.memory_load`
**Kind:** `prop`

The part of the physical memory that is in use, as a fraction.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.memory_total`
**Kind:** `prop`

The total physical memory, in megabytes.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.memory_usage`
**Kind:** `prop`

The physical memory that is in use, in megabytes.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.min_fps`
**Kind:** `prop`

The lowest frame rate in the last second.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.one_percent_low_fps`
**Kind:** `prop`

The 1 percent low frame rate. This is the frame rate of the slowest 1 percent of frames in the last second.

**Source:** `runtime/framework/hwmon.lua`


## hwmon.CPUCoreInfo _()_

Information about one logical processor.


  ### `hwmon.CPUCoreInfo.core_id`
**Kind:** `prop`

The index of the logical processor in its processor group.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.CPUCoreInfo.frequency_mhz`
**Kind:** `prop`

The current frequency of this logical processor, in MHz.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.CPUCoreInfo.processor_id`
**Kind:** `prop`

The index of the processor group. On most computers this is `0`.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.CPUCoreInfo.usage`
**Kind:** `prop`

The current load of this logical processor, as a fraction.

**Source:** `runtime/framework/hwmon.lua`


## hwmon.GPUAdapterInfo _()_

Information about one graphics adapter.


  ### `hwmon.GPUAdapterInfo.adapter_luid`
**Kind:** `prop`

The locally unique identifier of the adapter. It is valid until Windows restarts.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.GPUAdapterInfo.adapter_name`
**Kind:** `prop`

The name of the adapter, such as the model of the graphics card.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.GPUAdapterInfo.dedicated_vram_mb`
**Kind:** `prop`

The dedicated video memory of the adapter, in megabytes. It includes system memory that is reserved only for the adapter.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.GPUAdapterInfo.device_id`
**Kind:** `prop`

The PCI device ID.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.GPUAdapterInfo.gpu_usage_percent`
**Kind:** `prop`

The current load of the adapter, as a fraction from 0 to 1.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.GPUAdapterInfo.shared_memory_mb`
**Kind:** `prop`

The system memory that the adapter can share with the CPU, in megabytes.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.GPUAdapterInfo.unique_id`
**Kind:** `prop`

An identifier for the adapter made from its device ID and vendor ID, such as `"0x00002684_0x000010DE"`.

**Source:** `runtime/framework/hwmon.lua`

  ### `hwmon.GPUAdapterInfo.vendor_id`
**Kind:** `prop`

The PCI vendor ID.

**Source:** `runtime/framework/hwmon.lua`


## LUID" />


  ### `LUID.HighPart`
**Kind:** `prop`

**Source:** `runtime/framework/hwmon.lua`

  ### `LUID.LowPart`
**Kind:** `prop`

**Source:** `runtime/framework/hwmon.lua`


## PLUID" />

---


## Log

<!-- Source: cs2/api_reference/runtime/log.md -->

---
title: Logging
icon: terminal
---

# Logging

## log _()_

Writes text to the game console and to the event log on the screen.

Every function here converts its arguments to strings and joins them without a separator,
in the same way as `print`.


  ### `log.Print`
**Kind:** `fn`

**Arguments:** `...?`

The same function as `print`. Writes a line to the game console with the `neverlose ·` prefix.

When the Lua Output option of Log Events is enabled in the menu, the line is also added to
      the event log on the screen.
    
  

**Source:** `runtime/framework/log.lua`

  ### `log.Console`
**Kind:** `fn`

**Arguments:** `...?`

Writes one line to the game console exactly as given, without the `neverlose ·` prefix. The line is always written to the console, whatever the Lua Output setting is. If the text is empty, nothing is written.

Colour codes change the colour of the text that comes after them. Use `\aRRGGBBAA` with eight
      hex digits for a custom colour. The alpha digits are ignored, so the text is always fully
      visible. Use `\aDEFAULT` for the default console colour and `\aACCENT` for the menu accent
      colour.
    

    
      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/runtime/log:0:4ce450f79019"
      log.Console("\a7FDBFFFFmy script\aDEFAULT loaded")
      ```
    
  

**Source:** `runtime/framework/log.lua`

  ### `log.Event`
**Kind:** `fn`

**Arguments:** `icon?, color?, ...?`

Adds an entry to the event log on the screen. Entries are shown only while the Event Logs window is enabled in the menu. This function does not write to the game console. Use `log.Console` to write to the console. If the text is empty, no entry is added.

If the text is the same as the text of the last entry, the last entry stays on the screen
      for longer and a counter such as `[x2]` is added to it.

      Put `^` before and after the text that you want to be bold. Bold text is drawn in the colour
      given by `color`. Use `\n` to start a new line.
    

    
      ```lua luaintelligence=true semantic="lua:cs2:cs2/api_reference/runtime/log:1:2ee2a095e30b"
      log.Event("bell", Color(120, 200, 255), "Loaded ^", cloud.GetScript().name, "^")
      ```
    
  

**Source:** `runtime/framework/log.lua`

---


## Time

<!-- Source: cs2/api_reference/runtime/time.md -->

---
title: Time
icon: clock
sidebar_position: 2
---

# Time

<Heading level={2} id="time-1">
  time

   _()_
</Heading>

Gets the current time and runs functions after a delay. Scripts do not have the `os` library,
so use these functions to get the time.


  ### `time.GetEpochMilliseconds`
**Kind:** `fn`

Returns the number of milliseconds since 1 January 1970, UTC.

**Source:** `runtime/framework/time.lua`

  ### `time.GetUnixTime`
**Kind:** `fn`

Returns the number of seconds since 1 January 1970, UTC.

**Source:** `runtime/framework/time.lua`

  ### `time.GetLocalTime`
**Kind:** `fn`

Returns the current local time of day as four numbers. These are the hour from 0 to 23, the minute, the second, and the millisecond from 0 to 999.

**Source:** `runtime/framework/time.lua`

  ### `time.Format`
**Kind:** `fn`

**Arguments:** `format, t?`

Formats a time as local time with a C `strftime` format string, for example `"%H:%M:%S"`.

Returns **nil** if the format uses a code that the Windows runtime does not know, such as `%P`,
      `%l` or `%-d`. Also returns **nil** if the time cannot be converted to local time, for example
      a negative timestamp.
    
  

**Source:** `runtime/framework/time.lua`

  ### `time.Schedule`
**Kind:** `fn`

**Arguments:** `delay, callback, ...?`

Runs a function once after a delay.

The delay is rounded down to whole milliseconds. A negative delay is treated as `0`. If
      `delay` is NaN, the function throws an error. A script can have at most 16384 scheduled calls
      that have not run yet. Past that, this function throws an error.

      Scheduled calls that have not run yet are dropped when the script shuts down.
    
  

**Source:** `runtime/framework/time.lua`

---


## Utils

<!-- Source: cs2/api_reference/runtime/utils.md -->

---
title: Utilities
icon: clipboard
---

# Utilities

## utils _()_

Read `utils.clipboard` to get the text on the clipboard. Assign a string to
`utils.clipboard` to replace the text on the clipboard.


  ### `utils.clipboard`
**Kind:** `prop`

The text on the clipboard, in UTF-8. It is an empty string if the clipboard has no text.

**Source:** `runtime/framework/utils.lua`

---


## Advanced Luajit

<!-- Source: cs2/api_reference/tutorials/advanced-luajit.md -->

---
title: Advanced LuaJIT
description: Non-obvious LuaJIT 2.1 language tricks - syntax sugar, the and/or ternary, varargs, control flow, patterns, metamethods, and performance.
icon: wand-sparkles
sidebar_position: 3
---

# Advanced LuaJIT

A collection of LuaJIT 2.1 language features and idioms for the Neverlose CS2 runtime, from call syntax to tables and module patterns. If you're new to Lua, read the [Lua Crash Course](/cs2/api_reference/tutorials/lua-crash-course) first.

<Info>
  Targets **LuaJIT 2.1 with Lua 5.2 compatibility enabled**, including table `__len` and `rawlen`. The CS2 sandbox exposes the [documented libraries](/cs2/api_reference/extensions/globals-and-modules); an upstream LuaJIT library is not automatically available to extensions.
</Info>

## String literals: quotes and long brackets

`"double"` and `'single'` quotes are **completely identical** in Lua - same escapes, same type. Pick whichever lets you avoid escaping the quote *inside* the string:

```lua
local a = "it's fine"          -- ' needs no escaping inside "..."
local b = 'say "hi"'           -- " needs no escaping inside '...'
local c = "she said \"hi\""    -- escaping only when you mix
```

Combined with the **no-parentheses call sugar** (a call with a single string literal drops its parentheses), this is why you constantly see:

```lua
local ffi = require "ffi"      -- same as require("ffi")
local ffi = require 'ffi'      -- identical - quote style is your choice
require "bit"                  -- works for any single-string-argument call
```

<Note>
  The sugar applies to **one string (or one table) literal only**. `require "a", "b"` is a syntax error - anything with multiple arguments, or a variable instead of a literal, still needs parentheses: `require(name)`. The FFI examples require `"ffi"` in [extension.json](/cs2/api_reference/extensions/requirements).
</Note>

For multi-line strings or content full of quotes, use **long brackets** `[[ ... ]]`. They take no escapes at all, and `[==[ ... ]==]` (any number of `=`) lets you embed `]]`:

```lua
local sql = [[
  SELECT * FROM users
  WHERE name = "O'Brien"      -- both quote types, zero escaping
]]

local nested = [==[ contains a ]] sequence ]==]
```

<Tip>
  A long bracket that starts with an immediate newline drops that first newline - so `[[\nfoo]]` is just `"foo"`. Handy for keeping the opening `[[` on its own line without a leading blank.
</Tip>

## Syntax sugar worth abusing

<Steps>
  <Step title="String / table call without parentheses" icon="quote">
    `f "x"` is exactly `f("x")`, and `f {a=1}` is `f({a=1})`. Chain them to build a DSL:

    ```lua
    local function setting(name)
      return function(value) print(name, value) end
    end
    setting "enabled" (true) -- setting("enabled")(true)
    ```

    This works because `setting("enabled")` returns *another function*.
  </Step>

  <Step title="Methods on literals" icon="at-sign">
    Any string literal is a `string` object - call library methods directly (parentheses required around the literal):

    ```lua
    local hex = ("%02X"):format(255)   -- "FF"
    local pad = ("-"):rep(20)
    ```
  </Step>

  <Step title="Bare do...end blocks" icon="brackets">
    A standalone `do ... end` scopes locals and groups post-construction setup:

    ```lua
    local item = {} do
      item.color = "red"   -- configure, then let the temporaries die
    end
    ```
  </Step>

  <Step title="Multiple assignment & swap" icon="arrow-right-left">
    The right-hand side is fully evaluated before assignment, so swaps need no temp:

    ```lua
    local a, b = 10, 20
    a, b = b, a
    local point = {1, 2, 3}
    local x, y, z = unpack(point)
    ```
  </Step>
</Steps>

## The `and` / `or` ternary - and its trap

Lua's idiomatic ternary is `cond and a or b`. It's elegant until `a` itself is falsy:

<Tabs>
  <Tab title="The pitfall" icon="triangle-alert">
    ```lua
    local opts = { saveable = false }
    local flag = true
    -- Looks like a default of `true`, but it's broken:
    local saveable = opts.saveable or true
    -- If opts.saveable == false  ->  false or true  ->  true  (wrong!)

    -- Same bug with the ternary when the "true" branch is false:
    local x = flag and false or "fallback"   -- never yields false
    ```
  </Tab>

  <Tab title="The fix" icon="check">
    ```lua
    local opts = { saveable = false }
    -- Explicit nil check instead of `or`:
    local saveable = opts.saveable
    if saveable == nil then saveable = true end

    -- Strict boolean coercion:
    local value = true
    local b = (value == true) and true or false
    ```
  </Tab>
</Tabs>

## Truthiness and numbers

* **Only `nil` and `false` are falsy** - `0`, `""`, and `0/0` (NaN) are all truthy.
* **NaN is never equal to itself**: `0/0 ~= 0/0` is `true`. Use it to detect NaN: `local function isnan(x) return x ~= x end`.
* **`1/0` is `inf`**, `-1/0` is `-inf`, and `math.huge` is `inf`.
* **Hex float literals** exist: `0x1.8p1` is `3.0`, `0xFFp-4` is `15.9375` (`p` is the binary exponent).
* **`tonumber` takes a base** 2-36: `tonumber("ff", 16)` -> `255`, `tonumber("777", 8)` -> `511`.

```lua
-- # on a table with holes returns SOME border, not "the length":
print(#({1, 2, nil, 4}))   -- may be 2 OR 4 - undefined
-- Reliable emptiness check, regardless of key types:
local t = {}
if next(t) == nil then --[[ empty ]] end
```

<Warning>
  **`t[1]` and `t[1.0]` are the same key**. Use a string such as `"1"` when you need a distinct key.
</Warning>

## Control flow

### `goto` and `::continue::`

LuaJIT 2.1 (and Lua 5.2+) has `goto`. The canonical use is a `continue`:

```lua
local items = {{enabled = true}, {enabled = false}, {enabled = true}}
for i = 1, #items do
  local it = items[i]
  if not it.enabled then goto continue end
  print("Enabled item", i)
  ::continue::
end
```

<Note>
  `goto` can only jump to a label in the same or an enclosing block, and cannot jump *into* a local's scope. Beyond `continue`, it's handy for compact state machines without a dispatch table.
</Note>

### `repeat ... until true` - continue without goto

Before `goto` existed, the idiom for `continue` was a one-shot `repeat` loop where `break` jumps to the end. It still works (even on stock Lua 5.1) and is common in older code:

```lua
for i = 1, 5 do repeat
  if i % 2 == 0 then break end     -- `break` here behaves like `continue`
  print(i)                        -- 1, 3, 5
until true end                     -- the outer loop is what actually iterates
```

### Proper tail calls

`return f(x)` is a **tail call** - Lua reuses the stack frame, so deep/mutual recursion never overflows:

```lua
local function loop(i, acc)
  if i == 0 then return acc end
  return loop(i - 1, acc + i)   -- no stack growth, even for millions of iterations
end
```

## Varargs and multiple returns

<Tabs items={["Count, don't trust #", "Pack a multireturn", "Forward only the rest"]}>
  <Tab title="Count, don't trust #">
    ```lua
    -- # is unreliable with holes; select('#', ...) counts trailing nils too.
    local function f(...)
      local n = select('#', ...)   -- true argument count
      for i = 1, n do
        local v = select(i, ...)   -- nth value (and everything after)
        print(i, v)
      end
    end
    ```
  </Tab>

  <Tab title="Pack a multireturn">
    ```lua
    -- {f()} captures every return value into an array.
    local function dimensions() return 320, 180 end
    local args = { dimensions() }
    -- Decide scalar vs list by how many came back:
    local value = #args > 1 and args or args[1]
    ```
  </Tab>

  <Tab title="Forward only the rest">
    ```lua
    -- select(n, ...) returns the nth arg AND all following ones.
    local function tail(_, ...) return ... end
    print(tail(1, 2, 3, 4))   -- 2  3  4
    ```
  </Tab>
</Tabs>

<Warning>
  `{...}` silently drops trailing `nil`s from the length operator. If `nil` is a meaningful value, store the count explicitly: `local t = { n = select('#', ...), ... }` and iterate `1, t.n`.
</Warning>

### Expression adjustment rules

A call expands to all its return values **only when it's the last item** in a list; anywhere else it's truncated to one. Wrapping it in parentheses forces a single value:

```lua
local function f() return 1, 2 end
local function g() return 3, 4 end
local function h() return 5, 6 end
print(f(), 10)       -- f() truncated to 1 value
print(10, f())       -- f() expanded to all values
local t = { g(), h() }   -- g() -> 1 value, h() -> all of them
local x = (f())          -- parentheses FORCE exactly one value
```

<Tip>
  `assert` returns its arguments on success, so it chains around calls that signal failure with `nil, err`:

  ```lua
  local text = "320"
  local width = assert(tonumber(text), "width must be numeric")
  print(width) -- 320; invalid numeric text would raise the supplied error.
  ```
</Tip>

## Strings and patterns

Lua patterns are not regex, but they have a few power features people miss:

<AccordionGroup>
  <Accordion title="Byte escapes and multi-return string.byte" icon="binary">
    ```lua
    local space   = "\x20"            -- hex escape
    local hair    = "\xE2\x80\x8A"    -- raw UTF-8 bytes (U+200A HAIR SPACE)

    -- string.byte returns a range of bytes at once:
    local r, g, b = ("\xFF\x80\x00"):byte(1, 3)   -- 255  128  0
    ```
  </Accordion>

  <Accordion title="Captures with gmatch / gsub" icon="regex">
    ```lua
    -- Parse "key:value;" pairs:
    for k, v in ("a:1;b:2;"):gmatch("([^:]+):([^;]+);") do
      print(k, v)   -- a 1 / b 2
    end

    -- gsub with a table replacement (looked up per match):
    local out = ("hi {name}"):gsub("{(%w+)}", { name = "world" })
    print(out)   -- hi world   (table lookup also works as the replacement)
    ```
  </Accordion>

  <Accordion title="%b and %f - balanced match and frontier" icon="braces">
    ```lua
    -- %bxy matches a balanced pair, e.g. %b() for nested parentheses:
    print(("f(a, g(b))"):match("%b()"))   -- (a, g(b))

    -- %f[set] is a frontier: a zero-width boundary into `set`.
    -- Match whole words starting with an uppercase letter:
    for w in ("Hello big World"):gmatch("%f[%a]%u%a*") do print(w) end  -- Hello / World
    ```
  </Accordion>
</AccordionGroup>

## The `bit` library

LuaJIT's `bit` library does fast 32-bit integer math (results are signed 32-bit).

```lua
-- FNV-1a hash, with the FNV prime multiply decomposed into shifts:
local function fnv1a(str)
  local h = 0x811c9dc5
  for i = 1, #str do
    h = bit.bxor(h, str:byte(i))
    h = h + bit.lshift(h,1) + bit.lshift(h,4)
          + bit.lshift(h,7) + bit.lshift(h,8) + bit.lshift(h,24)  -- = 0x01000193
  end
  return h % 0x100000000   -- wrap to unsigned 32-bit
end
```

<Tip>
  Beyond the basics, the `bit` library also gives you `bit.tohex`, `bit.tobit` (normalize to int32), `bit.ror` / `bit.rol` (rotates), and `bit.bswap` (byte-swap, e.g. endianness flips). These are JIT-friendly and far faster than arithmetic equivalents.
</Tip>

## Standard library extras

These standard-library features are available in the CS2 runtime:

```lua
print(string.rep("ab", 3, "-")) -- "ab-ab-ab" (separator)
local t = {"one", "two", "three"}
print(table.concat(t, ", ", 2, 3)) -- "two, three" (slice)

local function add(a, b) return a + b end
local ok, total = xpcall(add, tostring, 2, 3) -- Forwards arguments to add.
print(ok, total) -- true, 5

local function last(...) return select(-1, ...) end
print(last("one", "two", "three")) -- "three"
```

## Building strings and serializing values

Collect text fragments in a table, then join them once with `table.concat`. For binary serialization, the host-provided [`msgpack`](/cs2/api_reference/data/msgpack) library round-trips Lua values:

```lua
local parts = {"hp=", tostring(100), "\n"}
local text = table.concat(parts) -- "hp=100\n"

local blob = msgpack.pack({1, 2, name = "x"})
local obj = msgpack.unpack(blob)
print(obj.name) -- "x"
```

<Note>
  The current Neverlose runtime does not expose `string.buffer`. Use [`json`](/cs2/api_reference/data/json) when the serialized data needs to be human-readable.
</Note>

## Scoping and module patterns

<Steps>
  <Step title="IIFE module with private state" icon="box">
    Wrap a unit in an immediately-invoked function and expose only what you assign outward:

    ```lua
    local M do (function()
      local secret = 42          -- private
      M = { get = function() return secret end }
    end)() end
    ```
  </Step>

  <Step title="Forward-declare for mutual recursion" icon="arrow-left-right">
    Declare locals up front so two functions can reference each other:

    ```lua
    local even, odd
    function even(n) return n == 0 or odd(n - 1) end
    function odd(n)  return n ~= 0 and even(n - 1) end
    ```
  </Step>

  <Step title="Upvalue captured before assignment" icon="link">
    A closure can reference a local that's assigned *later* - by call time it's set:

    ```lua
    local self
    local mt = { describe = function() return self.name end }
    self = setmetatable({ name = "obj" }, { __index = mt })
    ```
  </Step>

  <Step title="local function for self-recursion" icon="repeat">
    `local function f` declares the name *before* the body, so `f` can call itself.
    `local f = function() f() end` would capture a **global** `f`, not the local.
  </Step>

  <Step title="Each loop iteration gets a fresh local" icon="repeat-2">
    Unlike old JavaScript `var`, every iteration of a `for` loop binds a **new** variable, so closures capture distinct values:

    ```lua
    local fns = {}
    for i = 1, 3 do fns[i] = function() return i end end
    print(fns[1](), fns[2](), fns[3]())   -- 1  2  3   (not 3 3 3)
    ```
  </Step>
</Steps>

## Metamethods beyond OOP

Metamethods aren't just for classes. Any operator can be repurposed.

<Tabs>
  <Tab title="Operator overloading" icon="plus">
    Overload an arithmetic operator to mean something else entirely. A library can use `^` (`__pow`) as a hidden "unlock" call:

    ```lua
    local lib = setmetatable({ SIGNED = 0x1337 }, {
      __pow = function(self, token)
        assert(token == self.SIGNED, "bad token")
        return self        -- hand back the "real" library
      end,
    })

    lib = lib ^ lib.SIGNED   -- looks like math, is actually a handshake
    ```
  </Tab>

  <Tab title="__index as a function" icon="function-square">
    `__index` can be a **function**, turning every missing-key access into a computed lookup (virtual dispatch):

    ```lua
    local proxy = setmetatable({}, {
      __index = function(t, key)
        return function(...) print("called", key, ...) end
      end,
    })

    proxy.anything("x")   -- prints: called  anything  x
    ```
  </Tab>

  <Tab title="__call" icon="phone">
    `__call` makes a table behave like a function - useful for callable config objects:

    ```lua
    local counter = setmetatable({ n = 0 }, {
      __call = function(self) self.n = self.n + 1; return self.n end,
    })

    print(counter(), counter())   -- 1  2
    ```
  </Tab>
</Tabs>

<Tip>
  Use `rawget` / `rawset` / `rawequal` / `rawlen` to bypass metamethods when you need the *real* table state - essential inside an `__index`/`__newindex` to avoid infinite recursion.
</Tip>

### Weak tables (`__mode`)

A table with `__mode` lets the GC collect its entries. Great for caches keyed by objects without leaking them:

```lua
local cache = setmetatable({}, { __mode = "k" })  -- "k", "v", or "kv"
local object = {}
cache[object] = "computed result"
object = nil -- The key can now be collected, removing its cache entry.
```

### Locking and read-only tables

`__metatable` hides the real metatable, and an erroring `__newindex` makes a table tamper-proof - a common way to ship a "protected" module:

```lua
local real_impl = { version = 1 }
local api = setmetatable({}, {
  __metatable = false,   -- getmetatable() returns false; setmetatable() errors
  __index     = real_impl,
  __newindex  = function(_, k) error("read-only: " .. tostring(k), 2) end,
})
```

<Note>
  The `2` passed to `error` blames the **caller's** line, not the line inside `__newindex` - so the error points at the offending write.
</Note>

## Performance notes (LuaJIT-specific)

<Columns cols={2}>
  <Card title="Preallocate tables" icon="database">
    `require("table.new")(narr, nhash)` allocates a table with capacity up front; `require("table.clear")(t)` empties one for reuse without churning the GC.
  </Card>

  <Card title="Cache globals to locals" icon="gauge">
    Hoist hot globals/fields into locals (`local sin = math.sin`) - local access is a register, global access is a hash lookup.
  </Card>

  <Card title="Build strings with concat" icon="layers">
    `table.concat(parts)` beats repeated `..` in loops - it avoids creating an intermediate string per concatenation.
  </Card>

  <Card title="Know what is NYI" icon="triangle-alert">
    Some operations abort trace compilation and fall back to the interpreter. Keep hot loops free of NYI calls so the compiler can stay on-trace.
  </Card>
</Columns>

## Further reading

<Columns cols={2}>
  <Card title="Lua Crash Course" icon="graduation-cap" href="/cs2/api_reference/tutorials/lua-crash-course">
    The language fundamentals, if you want a refresher first.
  </Card>

  <Card title="FFI Crash Course" icon="cpu" href="/cs2/api_reference/tutorials/ffi-crash-course">
    Reach into memory and call native functions from Lua.
  </Card>

  <Card title="Lua 5.1 Reference" icon="book-open" href="https://www.lua.org/manual/5.1/" arrow>
    The base language manual LuaJIT implements.
  </Card>

  <Card title="LuaJIT Extensions" icon="external-link" href="https://luajit.org/extensions.html" arrow>
    `bit`, `table.new`/`table.clear`, and the 5.2 features LuaJIT backports.
  </Card>
</Columns>

---


## Ffi Crash Course

<!-- Source: cs2/api_reference/tutorials/ffi-crash-course.md -->

---
title: FFI Crash Course
description: An in-depth introduction to FFI - pointers, memory, type punning, signatures, interfaces, and casting - for Lua developers.
icon: cpu
sidebar_position: 2
---

# FFI Crash Course

An introduction to FFI for Lua developers: allocate C data, work with pointers, and understand native function bindings in the current CS2 API.

<Info>
  The first sections use **illustrative C fragments**, not complete programs. The Lua examples assume the FFI setup below; run each example separately unless a section says to continue the preceding example.
</Info>

## What is FFI?

FFI - the **Foreign Function Interface** - is an interface between different languages that lets them interact. It's generally used to create *bindings* to C/C++ libraries from higher-level languages like Python, Ruby, and - most importantly for us - Lua. Bindings are just functions in the high-level language that reach down into the low-level libraries and call their functions.

In practice, FFI is most commonly used to provide an interface into the game process's memory. This lets you interact directly with the game - such as calling its functions - without relying on a middleman like the Lua API. Because FFI bridges Lua and C, using it requires some knowledge of low-level memory and how languages like C interact with it.

## ctype and cdata

API declaration labels distinguish the C type object you call from the C values it creates:

| Label   | Meaning                                                                                         | Projection example                                   |
| ------- | ----------------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| `ctype` | A C type object. Call it as a constructor to create a value.                                    | `projection.Rect`                                    |
| `cdata` | A C value, such as a struct, scalar, or pointer. Its type determines its fields and operations. | The `ProjectionRect` returned by `projection.Rect()` |

`namespace projection` groups the projection helpers. Within that namespace,
`ctype projection.Rect` documents the constructor, while `cdata ProjectionRect`
documents the values returned by it. A ctype can also expose helper functions and constants.

`projection.Rect` is already available in the CS2 API:

```lua
local Rect = projection.Rect -- ctype: the constructor
local rect = Rect()          -- cdata: a rectangle value
print(rect.m_vMinBounds.x)
```

`ffi.typeof` returns a ctype object; calling it or passing it to `ffi.new` creates
cdata values. Technically, a ctype is itself a special kind of cdata, so Lua's
`type()` returns `"cdata"` for both the constructor and its values. These labels
describe their roles, not two distinct results of `type()`.
See the [LuaJIT FFI glossary](https://luajit.org/ext_ffi_api.html).

Neither label establishes ownership or lifetime. A cdata value may be an independent
copy, or a pointer/reference to storage owned by the game. Follow each API's copy
and borrowed-reference documentation before retaining or modifying a value.

## Memory and how languages interact with it

In low-level languages like C, memory can be interacted with directly. C gives us **pointers** for this - essentially memory addresses that ideally point to a specific variable. That variable can be of any type (integer, float, string, or even another pointer) and contain any value.

### Safe and unsafe pointers

A pointer stores an address. A **null pointer** represents no object, but a non-null address can still be invalid or refer to an object that has already been freed. Before dereferencing, you need to know which allocation owns the bytes, their type, and how long they remain alive.

<Tabs>
  <Tab title="Safe pointer" icon="shield-check">
    ```c
    float floating_point = 100.f;             // A normal variable.
    float* floating_pointer = &floating_point; // & takes its address.
    ```

    `floating_pointer` holds the address of `floating_point` and remains valid while that variable is alive.
  </Tab>

  <Tab title="Unsafe pointer" icon="triangle-alert">
    ```c
    char* our_string = (char*)0x1234;
    char first_character = *our_string;
    ```

    We assign an arbitrary address instead of one we retrieved from something known to exist. We can't guarantee `0x1234` points to a real string - it almost certainly doesn't.
  </Tab>
</Tabs>

<Warning>
  Dereferencing the invented address above is undefined behavior and can crash the application. The `&` operator takes an object's address; it does not extend that object's lifetime.
</Warning>

### Reading memory and dereferencing

**Dereferencing** is the act of reading the memory at the address stored in a pointer. In C you use the `*` operator before the variable name:

```c
float original_value = *floating_pointer;
```

If `floating_pointer` held the address of `floating_point`, then `original_value` now contains `100.f`.

### Why unsafe pointers?

FFI can allocate its own objects with `ffi.new`, or interpret an address owned by native code. For native addresses, the caller must establish the layout and lifetime. The CS2 API provides [`memory.FindPattern`](/cs2/api_reference/engine/memory#memory-FindPattern) for scanning and [`memory.CreateInterface`](/cs2/api_reference/engine/memory#memory-CreateInterface) for interface lookup. Prefer typed APIs such as [`entity`](/cs2/api_reference/engine/entity) when they already expose the operation you need.

## Type punning

Type punning is the act of bypassing the type system to reinterpret the same bytes as a different type. Suppose a region of memory at `0x405060` holds the start of an XML string. We can read its first 4 bytes as a 4-byte integer instead:

```c
int* int_pointer = (int*)0x405060;
int first_4_bytes = *int_pointer;

// The same, tied into one line:
int first_4_bytes = *(int*)0x405060;
```

The bracketed type before the address is a **cast** - telling the language how to interpret that address.

<Tip>
  **Memory is just a massive sequence of bytes**, and it can be represented however you want. This flexibility is exactly where a lot of the unsafety comes from.
</Tip>

## Pointer arithmetic

Pointer arithmetic works like normal arithmetic (`+` and `-` only - no multiply/divide), but it accounts for the **size of the pointer's type**. Every type has a size, so adding `1` to a pointer advances it by one element, not one byte.

```c
int our_array[10] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 0};
int* our_array_ptr = our_array; // The array converts to a pointer here.
int our_2nd_value = *(our_array + 1);
```

<Note>
  On CS2's Windows x64 platform, `int` is 4 bytes. If `our_array` starts at `0x14001f068`, then `our_array + 1` is `0x14001f06C`: the address advances by the element size.
</Note>

## Getting practical with FFI

Lua isn't really intended to be a low-level language, so it lacks some operators C provides. The following sections show how to achieve what we need anyway.

### Getting started

Declare `ffi` in your [extension manifest](/cs2/api_reference/extensions/requirements):

```json extension.json
{
  "language": "lua",
  "requirements": ["ffi"]
}
```

Then load it in `main.lua`:

```lua
local ffi = require("ffi")
```

The local `ffi` variable is available in that Lua file. Add the same `require` line to other files or standalone examples that use it.

<Tip>
  The [`FFI reference`](/cs2/api_reference/lua/ffi) describes the exposed library. Declaring `ffi` also enables `memory.CreateInterface`, `memory.FindPattern`, `memory.BindVirtual`, `memory.VirtualThunk`, and `memory.BindPattern`. Those functions are `nil` without the requirement.
</Tip>

### Dereferencing in Lua

Lua has no dereference operator like C's `*`. FFI pointer indexing reads an element at an offset: `[0]` reads the pointed-to element, and `[1]` reads the next one.

```lua
local values = ffi.new("int[3]", {10, 20, 30})
local pointer = ffi.cast("int*", values)
print(pointer[0], pointer[1]) -- 10, 20
assert(pointer[2] == values[2]) -- Keep the owning array alive during use.
```

An array owns its elements; a pointer stores an address. Casting the array to a pointer does not transfer ownership or keep the array alive.

### Signatures and signature scanning

[`memory.FindPattern(module, pattern[, find_all])`](/cs2/api_reference/engine/memory#memory-FindPattern) scans a loaded module for a byte pattern. It normally returns the first matching address as `userdata`. The pattern uses **space-separated hex bytes**, and `?` marks a **wildcard** byte:

```lua
-- Format example only; this is not a verified function signature.
local pattern = "48 8B ? ?"
```

Wildcards can cover changing operands such as relative addresses. A match only identifies bytes: it does not prove that the address is a function, establish its prototype, or resolve a relative instruction operand.

The helper below accepts a module and pattern you have verified against the current game build. It performs no scan until you call it with those inputs:

```lua
local function find_unique(module, verified_pattern)
  assert(memory.FindPattern, "Declare ffi in extension.json")
  local matches = {memory.FindPattern(module, verified_pattern, true)}
  assert(#matches == 1, "Expected exactly one pattern match")
  return matches[1]
end
```

<Tip>
  With `find_all = true`, the scanner returns every match in ascending address order as separate return values. No match produces no return values, so the table above is empty. Full scans cost more than first-match scans. Revalidate patterns, layouts, and function prototypes after game updates.
</Tip>

### The interface system

Source2 modules export interfaces identified by version strings. [`memory.CreateInterface(module, interface)`](/cs2/api_reference/engine/memory#memory-CreateInterface) returns an interface pointer or `nil`. This example checks an interface name used by the current API sources without calling any virtual functions:

```lua
assert(memory.CreateInterface, "Declare ffi in extension.json")
local instance = memory.CreateInterface("engine2.dll", "Source2EngineToClient001")
if instance ~= nil then
  print("Engine interface available")
else
  print("Engine interface unavailable in this build")
end
```

<Info>
  Check an interface result before casting. A null FFI pointer is truthy in Lua: `ffi.cast("void*", nil) or error(...)` does not detect it. Compare a typed pointer with `nil` to test for null. A non-null result still does not validate its type or lifetime.
</Info>

### Casting

Casting via `ffi.cast` tells Lua how to interpret a value; it does not validate the address. This example casts an allocation owned by the script, writes through the pointer, then reads the owning array:

```lua
local values = ffi.new("double[2]", {1.5, 2.5})
local pointer = ffi.cast("double*", values)
pointer[1] = 3.5
print(values[0], values[1]) -- 1.5, 3.5
```

<AccordionGroup>
  <Accordion title="Worked example: wrapping a live entity pointer" icon="code" defaultOpen>
    Use a pointer returned by a live entity rather than guessing which native function returns one. Run this while connected to a game; the lookup can return `nil` before a pawn exists.

    ```lua
    local pawn = entity.GetLocalPlayerPawn()
    if pawn then
      local instance = pawn:GetInstance()
      local wrapped = entity.WrapInstance(instance)
      if wrapped then print("Resolved entity:", wrapped:GetClassName()) end
    end
    ```
  </Accordion>
</AccordionGroup>

### Binding native functions

A virtual table stores native function pointers. Calling one requires the current slot, complete C prototype, and a live compatible instance. The following helpers reduce repeated binding code:

| Helper                                                                                                         | Instance handling                                                            |
| -------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| [`memory.BindVirtual(module, interface, index, ...)`](/cs2/api_reference/engine/memory#memory-BindVirtual)     | Resolves the interface and binds it to the returned function.                |
| [`memory.VirtualThunk(index, ...)`](/cs2/api_reference/engine/memory#memory-VirtualThunk)                      | The returned function takes the instance as its first argument on each call. |
| [`memory.BindPattern(module, interface, signature, ...)`](/cs2/api_reference/engine/memory#memory-BindPattern) | Resolves the interface and function pattern, then binds the instance.        |

The trailing arguments describe the function type as accepted by `ffi.typeof`. These helpers cannot discover the correct slot or prototype for you. Use a current typed API such as [`engine.GetScreenSize`](/cs2/api_reference/engine/engine#engine-GetScreenSize) when it already covers the task.

## Calling WinAPI

`ffi.C` is the **default C library namespace**. On Windows it includes the default C runtime and kernel32 exports. Use `ffi.load` for other libraries, such as user32. `ffi.cdef` declares a prototype; it does not load a DLL or make every Windows function available through `ffi.C`.

<Steps>
  <Step title="Declare the function" icon="file-code">
    Use `ffi.cdef` once to tell FFI the function's C prototype. Match the WinAPI signature exactly - argument types and return type.

    ```lua
    local ffi = require("ffi")

    ffi.cdef[[
        short GetAsyncKeyState(int vKey);
        unsigned long GetTickCount(void);
    ]]
    local user32 = ffi.load("user32")
    ```
  </Step>

  <Step title="Call through the library namespace" icon="terminal">
    Continue the preceding code. `GetAsyncKeyState` belongs to user32; `GetTickCount` is available through `ffi.C`.

    ```lua
    -- VK_LBUTTON = 0x01. The high bit is set while the key is down.
    if bit.band(user32.GetAsyncKeyState(0x01), 0x8000) ~= 0 then
      print("left mouse is held")
    end

    local uptime_ms = ffi.C.GetTickCount()
    ```
  </Step>
</Steps>

<Warning>
  A `cdef` prototype is a **promise**, not a check. Incorrect argument or return types can corrupt memory or crash the game. Verify the declaration against the Windows API reference.
</Warning>

### Strings and wide strings

WinAPI comes in ANSI (`...A`) and wide/UTF-16 (`...W`) variants. Lua strings map directly to `const char*` for the `A` variants; for `W` variants you must build a wide buffer yourself.

<Tabs items={["ANSI variant", "Wide variant"]}>
  <Tab title="ANSI variant">
    ```lua
    local user32 = ffi.load("user32")
    ffi.cdef[[
        int MessageBoxA(void* hWnd, const char* text, const char* caption, unsigned int type);
    ]]

    -- 0x40 = MB_ICONINFORMATION. Lua strings pass straight through as const char*.
    user32.MessageBoxA(nil, "Hello from FFI!", "Neverlose", 0x40)
    ```
  </Tab>

  <Tab title="Wide variant">
    ```lua
    local user32 = ffi.load("user32")
    ffi.cdef[[
        int MultiByteToWideChar(unsigned int cp, unsigned long flags,
            const char* str, int cb, wchar_t* wide, int cch);
        int MessageBoxW(void* hWnd, const wchar_t* text, const wchar_t* caption, unsigned int type);
    ]]

    -- Convert a UTF-8 Lua string into a wchar_t buffer (CP_UTF8 = 65001).
    local function to_wide(s)
      local len = ffi.C.MultiByteToWideChar(65001, 0, s, -1, nil, 0)
      assert(len > 0, "UTF-8 conversion failed")
      local buf = ffi.new("wchar_t[?]", len)
      assert(ffi.C.MultiByteToWideChar(65001, 0, s, -1, buf, len) > 0,
        "UTF-8 conversion failed")
      return buf
    end

    user32.MessageBoxW(nil, to_wide("Wide text"), to_wide("Neverlose"), 0x40)
    ```
  </Tab>
</Tabs>

<Tip>
  Use `ffi.new` to allocate C buffers (`ffi.new("char[?]", size)`), `ffi.string(ptr[, len])` to read a C string back into Lua, and `ffi.sizeof(type)` when an API needs a struct size. These are the building blocks for any output-parameter WinAPI call.
</Tip>

### Output parameters

Many WinAPI functions return data through pointer arguments. Allocate a one-element array with `ffi.new`, pass it, then read index `[0]`:

```lua
ffi.cdef[[
    int QueryPerformanceCounter(int64_t* lpPerformanceCount);
]]

local counter = ffi.new("int64_t[1]")  -- A pointer to one int64_t.
assert(ffi.C.QueryPerformanceCounter(counter) ~= 0, "Counter unavailable")
print("perf counter:", counter[0])
```

<Note>
  For wall-clock time and scheduling, use [`time.GetEpochMilliseconds`](/cs2/api_reference/runtime/time#time-GetEpochMilliseconds), [`time.GetUnixTime`](/cs2/api_reference/runtime/time#time-GetUnixTime), and [`time.Schedule`](/cs2/api_reference/runtime/time#time-Schedule). The clock functions return boxed `int64_t` values; scheduling delays use seconds. Epoch time can jump when the system clock changes, so it is not a monotonic performance counter.
</Note>

## Power features

`ffi` is more than `ffi.cast` - these are the parts that turn it into a real metaprogramming tool. The subsections run from everyday building blocks to the rarer, more advanced corners.

### Richer C declarations

`ffi.cdef` understands far more than plain structs.

<Tabs>
  <Tab title="enum" icon="list-ordered">
    ```lua
    ffi.cdef[[ enum { DOCS_READY = 1, DOCS_VISIBLE = 2 }; ]]

    -- Enum constants are available on ffi.C:
    local flags = 3
    print(bit.band(flags, ffi.C.DOCS_READY) ~= 0) -- true
    ```
  </Tab>

  <Tab title="union" icon="layers">
    ```lua
    -- A union reinterprets the same bytes - type punning without pointers:
    ffi.cdef[[ typedef union { float f; uint32_t i; } fi_t; ]]

    local u = ffi.new("fi_t"); u.f = 1.0
    print(bit.tohex(u.i))   -- 3f800000  (the IEEE-754 bits of 1.0)
    ```
  </Tab>

  <Tab title="bitfields" icon="binary">
    ```lua
    -- Pack flags into exact bit widths:
    ffi.cdef[[ struct Bits { unsigned int a:1, b:7, c:24; }; ]]

    local b = ffi.new("struct Bits")
    b.a = 1; b.b = 100
    ```
  </Tab>

  <Tab title="packed / aligned" icon="box">
    ```lua
    -- Disable padding, or force alignment, with GCC attributes:
    ffi.cdef[[
      struct __attribute__((packed)) Raw { uint8_t tag; uint32_t value; };
      struct __attribute__((aligned(16))) Vec4 { float x, y, z, w; };
    ]]
    ```
  </Tab>

  <Tab title="variadic" icon="ellipsis">
    ```lua
    ffi.cdef[[ int printf(const char *fmt, ...); ]]

    -- Pass explicit cdata for the variadic args (float is promoted to double):
    ffi.C.printf("hp=%d\n", ffi.new("int", 100))
    ```
  </Tab>
</Tabs>

### Constructors and initialization

A ctype from `ffi.typeof` is **callable** - use it as a fast constructor, and initialize structs/arrays many ways:

```lua
local Pt = ffi.typeof("struct { double x, y; }")

local a = Pt(1, 2)                       -- positional
local b = Pt{ x = 1, y = 2 }            -- by field name
local arr = ffi.new("int[3]", {1, 2, 3}) -- array from a table
ffi.new("int[10]")                       -- zero-filled
```

### 64-bit integers

Numbers above 2^53 lose precision as Lua doubles. Use the `LL` / `ULL` suffixes (or `int64_t` cdata) to keep full width - handy for addresses, handles, and timestamps:

```lua
local a = 0x140000000LL    -- signed 64-bit literal
local b = 5ULL             -- unsigned 64-bit literal
print(a + 1)               -- 5368709121LL  (exact, boxed as cdata)
```

<Warning>
  `tonumber(int64)` collapses back to a double and **loses precision above 2^53**. Keep large addresses in `uintptr_t`/`int64_t` cdata and only convert at the very end.
</Warning>

### Calling conventions

In a function pointer type you can specify `__cdecl`, `__stdcall`, `__fastcall`, or `__thiscall`. On **x64 they're all ignored** (there's a single ABI); they only matter for 32-bit targets.

### Memory, copying, and introspection

<Tabs items={["allocate", "copy & fill", "introspect", "type checks"]}>
  <Tab title="allocate">
    ```lua
    -- "?" is a runtime length; the third arg initializes the array.
    local n = 4
    local buf = ffi.new("unsigned char[?]", n, {10, 20, 30, 40})
    print(buf[0], buf[n - 1]) -- 10, 40
    ```
  </Tab>

  <Tab title="copy & fill">
    ```lua
    local source = ffi.new("uint8_t[3]", {10, 0, 20})
    local destination = ffi.new("char[16]")
    ffi.fill(destination, ffi.sizeof(destination), 0)
    ffi.copy(destination, source, ffi.sizeof(source))
    local bytes = ffi.string(destination, 3) -- Keeps the embedded zero byte.
    ffi.copy(destination, "hello") -- Also copies the terminating zero byte.
    print(#bytes, ffi.string(destination)) -- 3, "hello"
    ```
  </Tab>

  <Tab title="introspect">
    ```lua
    local Pair = ffi.typeof("struct { int a, b; }")
    print(ffi.sizeof(Pair))        -- 8 bytes on Windows x64.
    print(ffi.offsetof(Pair, "b")) -- 4-byte field offset.
    print(ffi.alignof(Pair))       -- 4-byte alignment.
    ```
  </Tab>

  <Tab title="type checks">
    ```lua
    local value = ffi.new("int", 42)
    print(ffi.istype("int", value)) -- true
    print(ffi.typeof(value))        -- The ctype of this value.
    local e = ffi.errno()           -- C runtime errno, not Win32 GetLastError.
    ```
  </Tab>
</Tabs>

### Platform introspection

Branch on architecture instead of hardcoding - offsets and signatures differ between x86 and x64:

```lua
print(ffi.os)           -- "Windows" for CS2.
print(ffi.arch)         -- "x64" for CS2.
print(ffi.abi("64bit")) -- true; other queries include "le", "win", and "gc64".
```

### `ffi.gc` - finalizers

Attach a finalizer that runs when its cdata is garbage-collected. This example uses a Lua-owned array and a bookkeeping callback:

```lua
local buffer = ffi.gc(ffi.new("uint8_t[16]"), function()
  print("Buffer collected")
end)
buffer[0] = 42
print(buffer[0])

-- Remove the callback when ownership or cleanup policy changes.
ffi.gc(buffer, nil)
```

For a native resource, the finalizer must call its documented release function exactly once. Garbage collection timing is unspecified; release scarce resources explicitly when their work is finished.

### `ffi.metatype` - methods and operators on cdata

Attach a metatable to a C type and its values become first-class Lua objects: indexable, callable, comparable, with a custom constructor via `__new`.

```lua
local ffi = require("ffi")
ffi.cdef[[ typedef struct { uintptr_t v; } addr_t; ]]

local addr
addr = ffi.metatype("addr_t", {
  __new      = function(ct, p) return ffi.new(ct, ffi.cast("uintptr_t", p)) end,
  __lt       = function(a, b) return a.v < b.v end,           -- enables <  and >
  __tostring = function(self) return ("addr<0x%x>"):format(tonumber(self.v)) end,
  __index    = {
    to_int = function(self)      return tonumber(self.v) end,
    at     = function(self, off) return addr(self.v + off) end,
  },
})

local p = addr(0x1000)
print(p:at(0x10))    -- addr<0x1010>
print(addr(1) < addr(2))   -- true
```

<Note>
  A single metatable can mix a **table** `__index` with **function** metamethods. Calling `addr(x)` dispatches to `__new`. Forward-declare `addr` so the `at` closure captures the local constructor. The small numeric addresses above are values only; the example never dereferences them. Its `tonumber` conversions are suitable only for values exactly representable as Lua numbers.
</Note>

### Metatypes: more metamethods

Beyond `__index` / `__new`, a metatype accepts the full arithmetic and comparison set - `__add`, `__sub`, `__eq`, `__len`, `__concat`, `__call` - plus `__gc`, a finalizer that runs for **every** instance of that type (an alternative to per-object `ffi.gc`):

```lua
local Record = ffi.metatype("struct { int id; }", {
  __gc = function(self) print("Collected record", self.id) end,
  __eq = function(a, b) return a.id == b.id end,
})
local first, second = Record(7), Record(7)
print(first == second) -- true
```

### `ffi.typeof` templates with `$`

`$` is a placeholder in a C declaration, substituted by a ctype **or a number** argument. This is how you build pointer/array/struct types programmatically without string concatenation:

```lua
local Pair = ffi.typeof("struct { int a, b; }")

local PairPtr = ffi.typeof("$*",   Pair)   -- pointer to Pair
local PairVLA = ffi.typeof("$[?]", Pair)   -- variable-length array of Pair

local n = 8
local Fixed = ffi.typeof("int[$]", n)      -- int[8] - length from a Lua number

-- Nesting structs by reference (great for big Win32 / PE headers):
local Header = ffi.typeof("struct { $ dir[16]; }", PairPtr)
```

<Tip>
  **Cache ctypes** - parsing declaration strings repeatedly adds work and can create distinct anonymous types. Build each type once outside hot callbacks and reuse its constructor. Use `pcall(ffi.typeof, ...)` when probing whether a named type already exists.
</Tip>

### Callbacks - Lua functions as C function pointers

`ffi.cast` can turn a Lua function into a C function pointer. This example owns the callback and invokes it directly, so no native system retains it:

```lua
local cb = ffi.cast("int(*)(int)", function(value)
  return value * 2
end)

print(cb(21)) -- 42
cb:set(function(value) return value * 3 end)
print(cb(21)) -- 63
cb:free() -- No code may call this pointer after it is freed.
```

<Warning>
  Callback slots are limited and must be released explicitly with `cb:free()`. Retain the callback handle so you can release or replace it. Before freeing a callback registered with native code, unregister it and ensure no call is still in progress. C-to-Lua callbacks also have JIT restrictions; consult the FFI reference before using them in a compiled loop.
</Warning>

### Reusable runtime ctypes

Named C structures cannot be redefined. When you need a private layout, create an anonymous ctype once and reuse it:

```lua
local Record = ffi.typeof("struct { int value; }")
local first = Record(10)
local second = Record(20)
print(first.value + second.value) -- 30
```

### Pitfall: cdata lifetime

`ffi.cast` and pointer arithmetic **do not keep the source allocation alive**. If nothing else references it, the GC frees it and your pointer dangles:

```lua
-- BUG: the array may be collected while `p` still points into it.
local p = ffi.cast("int*", ffi.new("int[10]"))

-- FIX: keep the owner alive for as long as the pointer is used.
local arr = ffi.new("int[10]")
local p   = ffi.cast("int*", arr)   -- `arr` anchors the memory
```

## Further reading

<Columns cols={2}>
  <Card title="Lua Crash Course" icon="graduation-cap" href="/cs2/api_reference/tutorials/lua-crash-course">
    New to Lua? Start with the language essentials before diving into FFI.
  </Card>

  <Card title="Advanced LuaJIT" icon="wand-sparkles" href="/cs2/api_reference/tutorials/advanced-luajit">
    Non-obvious LuaJIT 2.1 language tricks beyond FFI.
  </Card>

  <Card title="API Reference" icon="code" href="/cs2/api_reference/introduction/overview">
    Explore the full Lua API: entity, render, trace, ui, utils, and more.
  </Card>

  <Card title="Win32 API on MSDN" icon="external-link" href="https://learn.microsoft.com/en-us/windows/win32/api/" arrow>
    Official Microsoft reference for every WinAPI function - verify signatures before declaring them with `ffi.cdef`.
  </Card>

  <Card title="LuaJIT FFI docs" icon="book-open" href="https://luajit.org/ext_ffi.html" arrow>
    The complete FFI reference: `ffi.cdef`, `ffi.cast`, `ffi.new`, `ffi.string`, and more.
  </Card>
</Columns>

<Note>
  This guide is adapted from the community FFI crash course originally written by **owen** and **sapphyrus**, and expanded & improved by **Salvatore**. FFI is powerful but unforgiving - incorrect casts or addresses crash the game, so test incrementally.
</Note>

---


## Lua Crash Course

<!-- Source: cs2/api_reference/tutorials/lua-crash-course.md -->

---
title: Lua Crash Course
description: Learn the essentials of Lua - variables, control flow, functions, tables, metatables, classes, and modules - in one page.
icon: graduation-cap
sidebar_position: 1
---

# Lua Crash Course

A fast, example-driven tour of Lua in the Neverlose CS2 runtime. Each section introduces small examples; where code continues an earlier example or demonstrates an error, the text calls that out.

<Note>
  CS2 uses LuaJIT 2.1 with Lua 5.2 compatibility enabled. Its sandbox exposes the [documented environment](/cs2/api_reference/lua/base), including `print` for output to the game console. Master the basics here, then explore the [API Reference](/cs2/api_reference/introduction/overview).
</Note>

## Comments

```lua
-- Two dashes start a one-line comment.

--[[
     Adding two ['s and ]'s makes it a
     multi-line comment.
--]]
```

## Variables and flow control

Numbers, strings, and booleans behave much like in Python. Variables are **global by default** - use `local` to scope them.

```lua
num = 42  -- Numbers can be integer or floating point.

s = 'walternate'           -- Immutable strings, single quotes.
t = "double-quotes are also fine"
u = [[ Double brackets
       start and end
       multi-line strings.]]
t = nil  -- Undefines t; Lua has garbage collection.
```

<Warning>
  Variables are **global by default**. Forgetting `local` is the most common source of bugs and name collisions in Lua scripts.

  ```lua
  thisIsGlobal = 5        -- Leaks into the global scope.
  local line = "hello"   -- Properly scoped to the block.
  ```
</Warning>

### Conditionals

```lua
local num, s, line = 42, "walternate", "reader"
if num > 40 then
  print('over 40')
elseif s ~= 'walternate' then  -- ~= is "not equals".
  print('not over 40')
else
  print('Winter is coming, ' .. line)  -- .. concatenates.
end
```

<Tip>
  Only `nil` and `false` are falsy - **`0` and `''` are truthy!** This trips up developers coming from C or JavaScript.

  ```lua
  local aBoolValue = false
  if not aBoolValue then print('it was false') end

  -- 'and' / 'or' short-circuit, mimicking a ? b : c:
  ans = aBoolValue and 'yes' or 'no'  --> 'no'
  ```
</Tip>

### Loops

<Tabs items={["while", "numeric-for", "repeat"]}>
  <Tab title="while">
    ```lua
    local num = 42
    while num < 50 do
      num = num + 1  -- No ++ or += operators.
    end
    ```
  </Tab>

  <Tab title="numeric-for">
    ```lua
    -- Range is begin, end[, step] and includes both ends.
    karlSum = 0
    for i = 1, 100 do
      karlSum = karlSum + i
    end

    -- Count down with a negative step:
    local fredSum = 0
    for j = 100, 1, -1 do fredSum = fredSum + j end
    ```
  </Tab>

  <Tab title="repeat">
    ```lua
    local num = 3
    repeat
      print('the way of the future')
      num = num - 1
    until num == 0
    ```
  </Tab>
</Tabs>

## Functions

Functions are first-class values - assign them, pass them around, and return them. Closures capture surrounding locals.

<Steps>
  <Step title="Define a function" icon="function-square">
    ```lua
    function fib(n)
      if n < 2 then return 1 end
      return fib(n - 2) + fib(n - 1)
    end
    ```
  </Step>

  <Step title="Return closures" icon="package">
    The returned function remembers `x` even after `adder` returns.

    ```lua
    function adder(x)
      return function (y) return x + y end
    end
    a1 = adder(9)
    print(a1(16))  --> 25
    ```
  </Step>

  <Step title="Work with multiple values" icon="list">
    Assignments and returns balance lists. Extra receivers become `nil`; extra senders are discarded.

    ```lua
    x, y, z = 1, 2, 3, 4  -- 4 is thrown away.

    function bar(a, b, c)
      print(a, b, c)
      return 4, 8, 15, 16, 23, 42
    end
    x, y = bar('zaphod')  --> prints "zaphod  nil  nil"
    ```
  </Step>
</Steps>

<Info>
  A call with a **single string or table argument** can drop the parentheses:

  ```lua
  local function h(options) print(options.key1) end
  print 'hello'              -- Same as print('hello').
  h{ key1 = 'Sonmi~451' }   -- Same as h({ key1 = ... }).
  ```
</Info>

## Tables

Tables are Lua's **only** compound data structure - associative arrays that work as both dictionaries and lists.

<Tabs>
  <Tab title="As dictionaries" icon="book-marked">
    ```lua
    -- String keys can use dot notation:
    t = { key1 = 'value1', key2 = false }
    print(t.key1)   -- 'value1'
    t.newKey = {}   -- Add a key/value pair.
    t.key2 = nil    -- Remove a key.

    -- Any non-nil value can be a key:
    u = { ['@!#'] = 'qbert', [6.28] = 'tau' }
    print(u[6.28])  -- 'tau'

    for key, val in pairs(u) do  -- Iterate every pair.
      print(key, val)
    end
    ```
  </Tab>

  <Tab title="As lists" icon="list-ordered">
    ```lua
    -- List literals implicitly set up integer keys:
    v = { 'value1', 'value2', 1.21, 'gigawatts' }

    for i = 1, #v do  -- #v is the length of the list.
      print(v[i])     -- Indices start at 1, not 0!
    end
    ```
  </Tab>
</Tabs>

<Warning>
  Lua indices **start at `1`**, and `#t` only gives a meaningful length for tables with consecutive integer keys. A "list" is not a real type - it's just a table used a certain way.
</Warning>

## Metatables and metamethods

A metatable gives a table operator-overloading and custom lookup behavior.

```lua
local f1 = { a = 1, b = 2 }
local f2 = { a = 1, b = 3 }
metafraction = {}
function metafraction.__add(f1, f2)
  local sum = {}
  sum.b = f1.b * f2.b
  sum.a = f1.a * f2.b + f2.a * f1.b
  return sum
end

setmetatable(f1, metafraction)
setmetatable(f2, metafraction)

s = f1 + f2  -- Calls __add(f1, f2) from f1's metatable.
```

The `__index` metamethod overloads failed lookups, enabling prototype-style defaults:

```lua
defaultFavs = { animal = 'gru', food = 'donuts' }
myFavs = { food = 'pizza' }
setmetatable(myFavs, { __index = defaultFavs })

eatenBy = myFavs.animal  -- 'gru' - found via the metatable.
```

<AccordionGroup>
  <Accordion title="Common metamethods reference" icon="table" description="The keys Lua recognizes on a metatable">
    | Metamethod            | Triggered by              |
    | --------------------- | ------------------------- |
    | `__add(a, b)`         | `a + b`                   |
    | `__sub(a, b)`         | `a - b`                   |
    | `__mul(a, b)`         | `a * b`                   |
    | `__div(a, b)`         | `a / b`                   |
    | `__mod(a, b)`         | `a % b`                   |
    | `__pow(a, b)`         | `a ^ b`                   |
    | `__unm(a)`            | `-a`                      |
    | `__concat(a, b)`      | `a .. b`                  |
    | `__len(a)`            | `#a`                      |
    | `__eq(a, b)`          | `a == b`                  |
    | `__lt(a, b)`          | `a < b`                   |
    | `__le(a, b)`          | `a <= b`                  |
    | `__index(a, b)`       | `a.b` (function or table) |
    | `__newindex(a, b, c)` | `a.b = c`                 |
    | `__call(a, ...)`      | `a(...)`                  |
  </Accordion>
</AccordionGroup>

## Classes and inheritance

Lua has no built-in classes - you build them from tables and metatables. The `:` syntax adds an implicit `self` argument.

```lua
Dog = {}

function Dog:new()
  local newObj = { sound = 'woof' }
  self.__index = self
  return setmetatable(newObj, self)
end

function Dog:makeSound()
  print('I say ' .. self.sound)
end

mrDog = Dog:new()
mrDog:makeSound()  -- 'I say woof'
```

<Info>
  `function Dog:new()` is shorthand for `function Dog.new(self)`. The colon just injects the first `self` argument - that's the whole trick behind Lua "methods".
</Info>

Continue the `Dog` example to build inheritance by chaining `__index` through another class:

```lua
LoudDog = Dog:new()  -- LoudDog inherits Dog's methods.

function LoudDog:makeSound()
  local s = self.sound .. ' '
  print(s .. s .. s)
end

seymour = LoudDog:new()
seymour:makeSound()  -- 'woof woof woof'
```

## Modules

Use `require` to load a module. It runs the file once, caches the return value, and keeps the file's locals private.

<Tabs items={["mod.lua", "main.lua"]}>
  <Tab title="mod.lua">
    ```lua
    local M = {}

    local function sayMyName()  -- Private to this file.
      print('Hrunkner')
    end

    function M.sayHello()       -- Exported.
      print('Why hello there')
      sayMyName()
    end

    return M
    ```
  </Tab>

  <Tab title="main.lua">
    ```lua
    local mod = require('mod')  -- Runs mod.lua once, then caches.

    mod.sayHello()   -- 'Why hello there' then 'Hrunkner'
    -- mod.sayMyName() would error: sayMyName is private to mod.lua.
    ```
  </Tab>
</Tabs>

<Tip>
  Use `require("lib/helper")` for `lib/helper.lua` inside the same extension. Paths are relative to the extension root, and modules are cached while the extension is loaded. Reload the extension to rerun its files. See [Requirements](/cs2/api_reference/extensions/requirements) for external dependencies and built-in modules.
</Tip>

## Further reading

<Columns cols={2}>
  <Card title="Advanced LuaJIT" icon="wand-sparkles" href="/cs2/api_reference/tutorials/advanced-luajit">
    Non-obvious LuaJIT 2.1 tricks once you know the basics.
  </Card>

  <Card title="FFI Crash Course" icon="cpu" href="/cs2/api_reference/tutorials/ffi-crash-course">
    Go lower-level: read memory and call native functions from Lua.
  </Card>

  <Card title="API Reference" icon="code" href="/cs2/api_reference/introduction/overview">
    Explore the Neverlose Lua API: entity, render, trace, ui, utils, and more.
  </Card>
</Columns>

---
