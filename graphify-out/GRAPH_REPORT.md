# Graph Report - wallbang-rust  (2026-11-25)

## Corpus Check
- 79 files · ~156,950 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 13 file(s) not represented in the graph (top: .cso 4, .hlsl 4, .glb 2)

## Summary
- 4329 nodes · 5687 edges · 187 communities (171 shown, 16 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 51 edges (avg confidence: 0.81)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Game Events
- mathlib _()_
- api.rs
- entity _()_
- simulation.Movement _()_
- Vector
- simulation.Player _()_
- VectorAligned" />
- offsets.rs
- CTransform" />
- Quaternion
- matrix3x4\_t" />
- Render
- render/src/hooks.rs
- VMatrix" />
- simulation.Weapon _()_
- trace.Trace _()_
- menu.rs
- map.rs
- model.rs
- Vector2D" />
- src/preview.rs
- GameState
- AABB\_t" />
- QAngle" />
- UserCmd _()_
- DrawList
- hwmon _()_
- Vector4D" />
- panorama.rs
- find_process.rs
- input.InputPollContext _()_
- Power features
- HTTP
- Ui
- indicators.Item _()_
- Cheat Events
- Math
- Projection
- settings.rs
- signatures.rs
- UserCmd
- Functions _()_
- ragebot.Shot _()_
- netchan _()_
- FFI
- VPlane" />
- modules.rs
- schema.Type _()_
- ui.Variable _()_
- app/src/lib.rs
- native_preview.rs
- input.rs
- neverlose-api.md
- Console
- Schema Values
- pawn.rs
- loadlib.rs
- Rect\_t" />
- simulation.Grenade _()_
- AgentData
- diag.rs
- storage.rs
- font.rs
- Advanced LuaJIT
- fourplanes\_t _()_
- ui _()_
- dx11.rs
- ModelRenderer
- DegreeEuler" />
- motion.Status _()_
- QuaternionStorage" />
- RadianEuler" />
- RotationVector" />
- Dx11
- registry.rs
- VisualSettings
- ui.rs
- usercmd.rs
- metatable _()_
- particles.Particle _()_
- core/src/hooks.rs
- memory.rs
- state.rs
- Engine
- Wallbang Rust — рабочий роадмап
- cmd.rs
- QAngle
- Vec3
- Pawn
- Cloud
- WebSocket
- globals _()_
- ui.Group _()_
- hk_present
- resources/src/lib.rs
- matrix3x4a\_t" />
- initialize
- read_ptr
- Settings
- bit _()_
- User Command
- String
- filesystem _()_
- Random
- User Interface
- ui_state.rs
- mod.rs
- Environment
- ESP
- ScriptPerformance _()_
- Table
- bone.rs
- esp.rs
- .default
- MiscSettings
- CloseHandle
- Reading API Pages
- CTransformUnaligned" />
- JSON
- panorama.Object _()_
- sounds.Sound _()_
- sigscan/src/main.rs
- examples/preview.rs
- Storage
- ordering
- Lua Crash Course
- view\.ViewmodelContext _()_
- Color
- hitbox.rs
- History
- CS2 Scripting
- Input
- JIT
- AGENTS.md
- ChamsCfg
- Localization
- Hitbox _()_
- UserCmd.InputHistoryEntryHandle _()_
- UserCmd.SubtickMoveStepHandle _()_
- .render
- hk_wndproc
- Getting Started
- Anatomy
- Lifecycle and Events
- Requirements
- Events
- protobuf _()_
- ui.Element _()_
- wallbang_config
- mutation.rs
- multipoint.rs
- cleanup.rs
- msgpack _()_
- input.ButtonHandle _()_
- Memory
- Time
- ui.ActiveBind _()_
- ui.ColorModes _()_
- UserCmd.InputHistory _()_
- movement.rs
- Globals and Modules
- Font _()_
- Nav
- render.ShaderOptions _()_
- simulation.GrenadeDamage _()_
- simulation.GrenadePoint _()_
- TickAndInterpolationAmount\_t" />
- UserCmd.InButtonHandle _()_
- TrackProperty
- VTableHook
- .camera
- hitchance.rs
- Plane
- Lua Intelligence
- Shader _()_
- hide_shots.rs
- Utilities
- Table Clearing
- Image _()_
- ui.ColorSet _()_
- shaders.rs
- Euler Angles
- Math Library
- Matrix 4x4
- MENU_OPEN
- sigscan

## God Nodes (most connected - your core abstractions)
1. `Game Events` - 275 edges
2. `mathlib _()_` - 162 edges
3. `DrawList` - 89 edges
4. `entity _()_` - 85 edges
5. `simulation.Movement _()_` - 74 edges
6. `Vector` - 74 edges
7. `simulation.Player _()_` - 70 edges
8. `VectorAligned" />` - 66 edges
9. `Quaternion` - 64 edges
10. `CTransform" />` - 64 edges

## Surprising Connections (you probably didn't know these)
- `on_create_move()` --calls--> `key_down()`  [INFERRED]
  feature/src/combat/cmd.rs → core/src/input.rs
- `predicted_velocity()` --references--> `Vec3`  [EXTRACTED]
  feature/src/movement.rs → core/src/math.rs
- `select()` --calls--> `angle_diff()`  [INFERRED]
  feature/src/combat/target.rs → core/src/math.rs
- `select()` --calls--> `angle_to()`  [INFERRED]
  feature/src/combat/target.rs → core/src/math.rs
- `draw_esp()` --calls--> `w2s()`  [INFERRED]
  feature/src/esp.rs → core/src/math.rs

## Import Cycles
- 2-file cycle: `core/src/modules.rs -> core/src/pattern.rs -> core/src/modules.rs`
- 2-file cycle: `render/src/draw.rs -> render/src/dx11.rs -> render/src/draw.rs`
- 2-file cycle: `lua/src/api.rs -> lua/src/runtime.rs -> lua/src/api.rs`
- 3-file cycle: `lua/src/api.rs -> lua/src/lib.rs -> lua/src/runtime.rs -> lua/src/api.rs`
- 3-file cycle: `lua/src/api.rs -> lua/src/runtime.rs -> lua/src/bindings.rs -> lua/src/api.rs`
- 4-file cycle: `lua/src/api.rs -> lua/src/lib.rs -> lua/src/runtime.rs -> lua/src/bindings.rs -> lua/src/api.rs`

## Communities (187 total, 16 thin omitted)

### Community 0 - "Game Events"
Cohesion: 0.01
Nodes (275): `achievement_earned`, `achievement_earned_local`, `achievement_event`, `achievement_info_loaded`, `achievement_write_failed`, `add_bullet_hit_marker`, `add_player_sonar_icon`, `ammo_pickup` (+267 more)

### Community 1 - "mathlib _()_"
Cohesion: 0.01
Nodes (162): mathlib _()_, `mathlib.AlmostEqual`, `mathlib.AngleDiff`, `mathlib.AngleIMatrix`, `mathlib.AngleMatrix`, `mathlib.AngleMod`, `mathlib.AngleNormalize`, `mathlib.AngleNormalizePositive` (+154 more)

### Community 2 - "api.rs"
Cohesion: 0.05
Nodes (53): args_to_string(), Database, database_path(), encoded_len(), event_set(), event_unset(), json_keys(), json_to_lua() (+45 more)

### Community 3 - "entity _()_"
Cohesion: 0.02
Nodes (85): entity _()_, `entity.FirstByClassName`, `entity.FirstByDesignerName`, `Entity:GetAbsAngles`, `Entity:GetAbsVelocity`, `Entity:GetAbsViewOffset`, `Entity:GetActiveWeapon`, `Entity:GetBombDamage` (+77 more)

### Community 4 - "simulation.Movement _()_"
Cohesion: 0.03
Nodes (74): simulation.Movement _()_, `simulation.Movement:CanJump`, `simulation.Movement:CanMove`, `simulation.Movement:GetMaxSpeed`, `simulation.Movement.host_timescale`, `simulation.Movement.m_bDesiresDuck`, `simulation.Movement.m_bDucked`, `simulation.Movement.m_bDucking` (+66 more)

### Community 5 - "Vector"
Cohesion: 0.03
Nodes (74): Vector2, Vector, `-Vector`, `#Vector`, `Vector.AsVector2D`, `Vector / b`, `Vector * b`, `Vector + b` (+66 more)

### Community 6 - "simulation.Player _()_"
Cohesion: 0.03
Nodes (70): simulation.Player _()_, `simulation.Player:Clone`, `simulation.Player.controller`, `simulation.Player.curtime`, `simulation.Player.frametime`, `simulation.Player:GetEyePosition`, `simulation.Player:GetShape`, `simulation.Player:GetSimpleFilter` (+62 more)

### Community 7 - "VectorAligned" />"
Cohesion: 0.03
Nodes (66): VectorAligned" />, `-VectorAligned`, `#VectorAligned`, `VectorAligned()`, `VectorAligned.Abs`, `VectorAligned.AsVector`, `VectorAligned.AsVector3D`, `VectorAligned + b` (+58 more)

### Community 8 - "offsets.rs"
Cohesion: 0.03
Nodes (61): BASE_BUTTON_STATE, BASE_CLIENT_TICK, BASE_FORWARD_MOVE, BASE_HAS_BITS, BASE_MOUSE_DX, BASE_MOUSE_DY, BASE_RANDOM_SEED, BASE_SIDE_MOVE (+53 more)

### Community 9 - "CTransform" />"
Cohesion: 0.03
Nodes (64): CTransform" />, `CTransform()`, `CTransform * b`, `CTransform * b`, `CTransform * b`, `CTransform.Copy`, `CTransform.GetBasisVectorsFLU`, `CTransform.GetForward` (+56 more)

### Community 10 - "Quaternion"
Cohesion: 0.03
Nodes (63): Quaternion, `-Quaternion`, `Quaternion()`, `Quaternion(angles)`, `Quaternion(angles)`, `Quaternion + b`, `Quaternion - b`, `Quaternion * b` (+55 more)

### Community 11 - "matrix3x4\_t" />"
Cohesion: 0.03
Nodes (61): matrix3x4\_t" />, `matrix3x4_t()`, `matrix3x4_t(angles, position)`, `matrix3x4_t * b`, `matrix3x4_t * b`, `matrix3x4_t + b`, `matrix3x4_t - b`, `matrix3x4_t.Copy` (+53 more)

### Community 12 - "Render"
Cohesion: 0.03
Nodes (60): Render, `render.CreateShader`, `render.DrawBezier`, `render.DrawCircle`, `render.DrawCircleBlur`, `render.DrawCircleGradient`, `render.DrawCircleOutline`, `render.DrawCircleShadow` (+52 more)

### Community 13 - "render/src/hooks.rs"
Cohesion: 0.05
Nodes (22): DRAW, DX, ESP_OVERLAY, ESP_US, FRAME, HOOK_US, LAST_REFRESH_MS, MENU (+14 more)

### Community 14 - "VMatrix" />"
Cohesion: 0.04
Nodes (56): VMatrix" />, `-VMatrix`, `VMatrix()`, `VMatrix.ApplyRotation`, `VMatrix.As3x4`, `VMatrix * b`, `VMatrix * b`, `VMatrix * b` (+48 more)

### Community 15 - "simulation.Weapon _()_"
Cohesion: 0.04
Nodes (48): simulation.Weapon _()_, `simulation.Weapon:CanAttack`, `simulation.Weapon:GetInaccuracy`, `simulation.Weapon:GetMaxSpeed`, `simulation.Weapon:GetPlayerShootPosition`, `simulation.Weapon:GetSpread`, `simulation.Weapon:GetThrowStrength`, `simulation.Weapon:HasSecondaryAttack` (+40 more)

### Community 16 - "trace.Trace _()_"
Cohesion: 0.04
Nodes (48): Trace, `trace.AABB`, `trace.BuiltInCollisionGroup_t`, `trace.Bullet`, `trace.Capsule`, `trace.CONTENTS`, `trace.ConvexHull`, `trace.Filter` (+40 more)

### Community 17 - "menu.rs"
Cohesion: 0.04
Nodes (43): BAR_POSITIONS, BOX_STYLES, CLANTAG_STYLES, CONFIG_TAB, COUNTS, DEATH_EFFECTS, DEFAULT_H, DEFAULT_W (+35 more)

### Community 18 - "map.rs"
Cohesion: 0.06
Nodes (19): DOS_MAGIC, find_export_rva(), IMAGE_BASE_OFFSET_IN_NT_HEADERS64, map_image(), MapError, Import, InvalidPe, Reloc (+11 more)

### Community 19 - "model.rs"
Cohesion: 0.05
Nodes (36): BONE_COUNT, CAMERA_FOV, CHEST, ELBOW_L, ELBOW_R, FIT_MARGIN, FOOT_L, FOOT_R (+28 more)

### Community 20 - "Vector2D" />"
Cohesion: 0.05
Nodes (43): Vector2D" />, `-Vector2D`, `#Vector2D`, `Vector2D.AsVector`, `Vector2D + b`, `Vector2D - b`, `Vector2D * b`, `Vector2D / b` (+35 more)

### Community 21 - "src/preview.rs"
Cohesion: 0.07
Nodes (32): set_request(), BONES, CHEST, clear_model_request(), draw_esp_overlay(), ELBOW_L, ELBOW_R, EspPreviewState (+24 more)

### Community 22 - "GameState"
Cohesion: 0.11
Nodes (5): state(), STATE, GameState, Menu, Style

### Community 23 - "AABB\_t" />"
Cohesion: 0.05
Nodes (41): AABB\_t" />, `AABB_t()`, `AABB_t.AddAABBToBounds`, `AABB_t.AddAABBToBounds`, `AABB_t.AddPointToBounds`, `AABB_t.AddSphereToBounds`, `AABB_t + b`, `AABB_t.ClosestPoint` (+33 more)

### Community 24 - "QAngle" />"
Cohesion: 0.05
Nodes (41): Angle, Color, QAngle" />, `-QAngle`, `QAngle + b`, `QAngle - b`, `QAngle * b`, `QAngle / b` (+33 more)

### Community 25 - "UserCmd _()_"
Cohesion: 0.05
Nodes (41): UserCmd _()_, `UserCmd.attack1_start_history_index`, `UserCmd.attack2_start_history_index`, `UserCmd.client_tick`, `UserCmd:Clone`, `UserCmd:Finalized`, `UserCmd.forwardmove`, `UserCmd.has_new_snapshot` (+33 more)

### Community 26 - "DrawList"
Cohesion: 0.11
Nodes (4): Color, DrawList, get(), RARITY_COLORS

### Community 27 - "hwmon _()_"
Cohesion: 0.05
Nodes (39): Hardware Monitor, hwmon _()_, `hwmon.average_fps`, `hwmon.cpu_base_frequency`, `hwmon.cpu_cores`, `hwmon.cpu_cores_count`, `hwmon.cpu_count`, `hwmon.cpu_frequency` (+31 more)

### Community 28 - "Vector4D" />"
Cohesion: 0.05
Nodes (39): Vector4D" />, `-Vector4D`, `#Vector4D`, `Vector4D()`, `Vector4D.Abs`, `Vector4D.AsVector2D`, `Vector4D.AsVector3D`, `Vector4D + b` (+31 more)

### Community 29 - "panorama.rs"
Cohesion: 0.09
Nodes (14): GetUiEngineFn, hud_panel(), menu_panel(), PanelMode, Hud, MainMenu, Panorama, resolve_panel() (+6 more)

### Community 30 - "find_process.rs"
Cohesion: 0.08
Nodes (13): launch_cs2(), module_in_snapshot(), name_eq(), pid_in_snapshot(), POLL_INTERVAL_MS, process_handle(), wait_for_module(), wait_for_process() (+5 more)

### Community 31 - "input.InputPollContext _()_"
Cohesion: 0.06
Nodes (36): input.InputPollContext _()_, `input.InputPollContext.aim_punch`, `input.InputPollContext.eye_position`, `input.InputPollContext.frame_start`, `input.InputPollContext:GetViewAngles`, `input.InputPollContext.in_attack`, `input.InputPollContext.in_attack2`, `input.InputPollContext.in_back` (+28 more)

### Community 32 - "Power features"
Cohesion: 0.06
Nodes (35): 64-bit integers, Binding native functions, Callbacks - Lua functions as C function pointers, Calling conventions, Calling WinAPI, Casting, Constructors and initialization, ctype and cdata (+27 more)

### Community 33 - "HTTP"
Cohesion: 0.06
Nodes (35): HTTP, http.Authorization _()_, `http.Authorization.password`, `http.Authorization.username`, `http.Callback`, http.Cookie _()_, `http.Cookie.expire`, `http.Cookie.host` (+27 more)

### Community 35 - "indicators.Item _()_"
Cohesion: 0.06
Nodes (33): Indicators, `indicators.Create`, indicators.Icon _()_, `indicators.Icon.color`, `indicators.Icon.invert`, `indicators.Icon.rotation`, `indicators.Icon.scale`, `indicators.Icon.source` (+25 more)

### Community 36 - "Cheat Events"
Cohesion: 0.06
Nodes (32): `aim_ack`, `aim_fire`, Cheat Events, `config_state`, `console_input`, `createmove`, `display_mode_changed`, `entity_created` (+24 more)

### Community 37 - "Math"
Cohesion: 0.06
Nodes (32): Math, `math.abs`, `math.acos`, `math.asin`, `math.atan`, `math.atan2`, `math.ceil`, `math.cos` (+24 more)

### Community 38 - "Projection"
Cohesion: 0.07
Nodes (30): Projection, `projection.Camera`, `projection.ClipLineToScreen`, `projection.ClipToScreen`, `projection.Ellipse`, `projection.Ellipse()`, `projection.Ellipse(center, radius_major, radius_minor, rotation)`, `projection.Ellipse(ellipse)` (+22 more)

### Community 39 - "settings.rs"
Cohesion: 0.12
Nodes (15): AntiAimSettings, AutosSettings, CombatSettings, DuckPeekSettings, EMPTY_GROUP, EMPTY_LEGIT_GROUP, KnifeSettings, LegitGroup (+7 more)

### Community 40 - "signatures.rs"
Cohesion: 0.07
Nodes (27): CREATE_MOVE_STEAL_BYTES, IDA_PRO_PATH, SIG_CACHE_PARTICLE, SIG_CMD_SERIALIZE, SIG_CMD_SET_MESSAGE_DATA, SIG_CMD_WRITE_MESSAGE, SIG_CREATE_MOVE, SIG_CREATE_MOVE_PROLOGUE (+19 more)

### Community 41 - "UserCmd"
Cohesion: 0.17
Nodes (5): read_f32(), read_i32(), write(), set_bits(), UserCmd

### Community 42 - "Functions _()_"
Cohesion: 0.07
Nodes (29): `assert`, `collectgarbage`, `defer`, `error`, Functions _()_, `getmetatable`, `inext`, `ipairs` (+21 more)

### Community 43 - "ragebot.Shot _()_"
Cohesion: 0.07
Nodes (29): Editor Setup, Ragebot, `ragebot.GetQuickPeekState`, `ragebot.IsMouseOverrideActive`, `ragebot.QuickPeekPhase`, ragebot.QuickPeekState _()_, `ragebot.QuickPeekState.interpolated_position`, `ragebot.QuickPeekState.phase` (+21 more)

### Community 44 - "netchan _()_"
Cohesion: 0.07
Nodes (29): netchan _()_, `netchan.address`, `netchan.avg_data`, `netchan.avg_latency`, `netchan.avg_packets`, `netchan.data_rate`, `netchan.dropped`, `netchan.engine_latency` (+21 more)

### Community 45 - "FFI"
Cohesion: 0.07
Nodes (28): FFI, `ffi.abi`, `ffi.alignof`, `ffi.arch`, `ffi.C`, `ffi.cast`, ffi.cb\* _()_, `ffi.cb*:free` (+20 more)

### Community 46 - "VPlane" />"
Cohesion: 0.07
Nodes (28): VPlane" />, `VPlane()`, `VPlane(a, b, c)`, `VPlane.BoxOnPlaneSide`, `VPlane.BoxOnPlaneSide`, `VPlane.CalculateClosestPointOnPlane`, `VPlane.ConvertToPlaneEquation`, `VPlane.Copy` (+20 more)

### Community 47 - "modules.rs"
Cohesion: 0.13
Nodes (16): build_number(), create_interface(), CreateInterfaceFn, csgo_input(), GetModuleHandleA(), GetProcAddress(), GetSystemMetrics(), input_system() (+8 more)

### Community 48 - "schema.Type _()_"
Cohesion: 0.07
Nodes (27): schema _()_, schema.Scope _()_, `schema.Scope[string]`, `schema[string]`, Schema System, schema.Type _()_, `#schema.Type`, `schema.Type:Atomic` (+19 more)

### Community 49 - "ui.Variable _()_"
Cohesion: 0.07
Nodes (27): ui.Variable _()_, `ui.Variable:AddCallback`, `ui.Variable:AddColorModes`, `ui.Variable:AddColorPicker`, `ui.Variable:AddColorSet`, `ui.Variable:AddGroup`, `ui.Variable:Chip`, `ui.Variable.default` (+19 more)

### Community 50 - "app/src/lib.rs"
Cohesion: 0.12
Nodes (17): CREATE_MOVE, CREATE_MOVE_TRAMPOLINE, CreateMoveFn, deferred_init(), DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DllMain(), FRAME_STAGE (+9 more)

### Community 51 - "native_preview.rs"
Cohesion: 0.13
Nodes (19): is_menu_open(), available(), build_script(), CAPTURE_FRAME, CAPTURE_GRACE_FRAMES, CAPTURE_LAST_OK, Driver, DRIVER (+11 more)

### Community 52 - "input.rs"
Cohesion: 0.14
Nodes (17): cursor_position(), CursorController, GetAsyncKeyState(), GetCursorPos(), Input, INPUT_MOUSE, key_down(), MOUSE (+9 more)

### Community 53 - "neverlose-api.md"
Cohesion: 0.08
Nodes (24): 1. Get the definitions, 2. Add `.luarc.json` to your extension, 3. Set up your editor, 4. Check that it works, Aabb, Aligned Vector, Contents, Database (+16 more)

### Community 54 - "Console"
Cohesion: 0.08
Nodes (25): Console, console.Command _()_, `console.Command(...)`, console.CommandLine _()_, `console.CommandLine(...)`, `console.CommandLine.is_valid`, `console.CommandLine.name`, `console.ConVarFlags_t` (+17 more)

### Community 55 - "Schema Values"
Cohesion: 0.08
Nodes (24): Cheat, Schema Values, value.AtomicCollection _()_, `value.AtomicCollection:Value`, value.Bitfield _()_, `value.Bitfield:NumBits`, value.DeclaredClass _()_, `value.DeclaredClass:GetField` (+16 more)

### Community 57 - "loadlib.rs"
Cohesion: 0.11
Nodes (3): call_remote_entry(), START_TIMEOUT_MS, load_library()

### Community 58 - "Rect\_t" />"
Cohesion: 0.09
Nodes (23): Rect\_t" />, `Rect_t()`, `Rect_t.Clear`, `Rect_t.Contains`, `Rect_t.ContainsPoint`, `Rect_t.Copy`, `Rect_t.height`, `Rect_t.Intersects` (+15 more)

### Community 59 - "simulation.Grenade _()_"
Cohesion: 0.09
Nodes (23): simulation.Grenade _()_, `simulation.Grenade.definition_index`, `simulation.Grenade.detonated`, `simulation.Grenade.detonation_time`, `simulation.Grenade.entity`, `simulation.Grenade.exploded_from_inferno`, `simulation.Grenade.extinguished_by_smoke`, `simulation.Grenade:GetBounces` (+15 more)

### Community 60 - "AgentData"
Cohesion: 0.26
Nodes (6): AgentData, both_agents_parse_with_required_bones(), Mat4, mesh_projection_fits_preview_rect(), parse_agent(), projected_bones_fall_inside_preview_rect()

### Community 61 - "diag.rs"
Cohesion: 0.15
Nodes (11): AddVectoredExceptionHandler(), DLL_BASE, ExceptionPointers, ExceptionRecord, GetModuleHandleA(), init(), install_exception_logger(), LOG (+3 more)

### Community 62 - "storage.rs"
Cohesion: 0.10
Nodes (15): BONE_STRIDE, CHUNK_SLOT_COUNT, CHUNK_SLOT_SIZE, CONFIG_EXTENSION, CONFIRM_TIMEOUT_S, DEFAULT_FONT_SIZE, DEFAULT_MENU_H, DEFAULT_MENU_W (+7 more)

### Community 63 - "font.rs"
Cohesion: 0.14
Nodes (16): ClipRect, MAX_VERTICES, ATLAS, ATLAS_H, ATLAS_W, build_atlas(), CHAR_COUNT, FIRST_CHAR (+8 more)

### Community 64 - "Advanced LuaJIT"
Cohesion: 0.09
Nodes (22): Advanced LuaJIT, Building strings and serializing values, Control flow, Expression adjustment rules, Ffi Crash Course, Further reading, `goto` and `::continue::`, Locking and read-only tables (+14 more)

### Community 65 - "fourplanes\_t _()_"
Cohesion: 0.09
Nodes (22): fourplanes\_t _()_, `fourplanes_t.dist`, `fourplanes_t.nX`, `fourplanes_t.nXAbs`, `fourplanes_t.nY`, `fourplanes_t.nYAbs`, `fourplanes_t.nZ`, `fourplanes_t.nZAbs` (+14 more)

### Community 66 - "ui _()_"
Cohesion: 0.09
Nodes (22): ui _()_, `ui.AddTab`, `ui.ColorKey`, `ui.ConvertDistance`, `ui.GetActiveBinds`, `ui.GetAlpha`, `ui.GetCursorPosition`, `ui.GetPosition` (+14 more)

### Community 67 - "dx11.rs"
Cohesion: 0.11
Nodes (5): MARKER_PX, marker_rgb(), PS_BYTECODE, VERTEX_STRIDE, VS_BYTECODE

### Community 68 - "ModelRenderer"
Cohesion: 0.16
Nodes (6): create_buffer(), create_constant_buffer(), create_texture_srv(), GpuAgent, ModelRenderer, Target

### Community 69 - "DegreeEuler" />"
Cohesion: 0.10
Nodes (21): DegreeEuler" />, `DegreeEuler()`, `DegreeEuler(angles)`, `DegreeEuler(angles)`, `DegreeEuler.Copy`, `DegreeEuler.Init`, `DegreeEuler.Invalidate`, `DegreeEuler.IsValid` (+13 more)

### Community 70 - "motion.Status _()_"
Cohesion: 0.10
Nodes (21): Motion, `motion.Active`, `motion.ApproachMode`, motion.Status _()_, `motion.Status`, `motion.Status.command_number`, `motion.Status.distance`, `motion.Status.phase` (+13 more)

### Community 71 - "QuaternionStorage" />"
Cohesion: 0.10
Nodes (21): QuaternionStorage" />, `-QuaternionStorage`, `QuaternionStorage()`, `QuaternionStorage(angles)`, `QuaternionStorage(angles)`, `QuaternionStorage + b`, `QuaternionStorage - b`, `QuaternionStorage * b` (+13 more)

### Community 72 - "RadianEuler" />"
Cohesion: 0.10
Nodes (21): RadianEuler" />, `RadianEuler()`, `RadianEuler(angles)`, `RadianEuler(angles)`, `RadianEuler.Copy`, `RadianEuler.Init`, `RadianEuler.Invalidate`, `RadianEuler.IsValid` (+13 more)

### Community 73 - "RotationVector" />"
Cohesion: 0.10
Nodes (21): RotationVector" />, `-RotationVector`, `#RotationVector`, `RotationVector()`, `RotationVector + b`, `RotationVector - b`, `RotationVector * b`, `RotationVector / b` (+13 more)

### Community 74 - "Dx11"
Cohesion: 0.11
Nodes (3): Dx11, PreviewCapture, atlas_rgba()

### Community 75 - "registry.rs"
Cohesion: 0.21
Nodes (8): Field, FieldRegistry, flatten(), fnv1a(), fnv1a_known_vectors(), global_registry(), GLOBAL_REGISTRY, registry_covers_all_settings_leaves()

### Community 76 - "VisualSettings"
Cohesion: 0.16
Nodes (11): AmbienceSettings, ChamsMaterialLayer, ChamsPreset, ChamsSettings, GrenadeSettings, HudSettings, ItemEspSettings, Rgba (+3 more)

### Community 77 - "ui.rs"
Cohesion: 0.12
Nodes (9): Vec2, w2s(), ITEM_SPACING_X, ITEM_SPACING_Y, lerp_color(), POPUP_W, ROUNDING, WIDGET_ROUNDING (+1 more)

### Community 78 - "usercmd.rs"
Cohesion: 0.10
Nodes (19): BASE_BITS_BUTTON_PB, BASE_BITS_FORWARD_MOVE, BASE_BITS_MOUSE_DX, BASE_BITS_MOUSE_DY, BASE_BITS_SIDE_MOVE, BASE_BITS_UP_MOVE, BASE_BITS_VIEW_ANGLES, FL_ONGROUND (+11 more)

### Community 79 - "metatable _()_"
Cohesion: 0.10
Nodes (20): metatable _()_, `metatable.__add`, `metatable.__call`, `metatable.__concat`, `metatable.__div`, `metatable.__eq`, `metatable.__gc`, `metatable.__index` (+12 more)

### Community 80 - "particles.Particle _()_"
Cohesion: 0.11
Nodes (19): Particles, `particles.Attachment`, `particles.Create`, particles.Create.Options _()_, `particles.Create.Options.attachment`, `particles.Create.Options.check_fow`, `particles.Create.Options.create_pvs`, `particles.Create.Options.entity` (+11 more)

### Community 81 - "core/src/hooks.rs"
Cohesion: 0.21
Nodes (11): ABS_JMP_LEN, FlushInstructionCache(), GetCurrentProcess(), MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PrologueHook (+3 more)

### Community 82 - "memory.rs"
Cohesion: 0.20
Nodes (14): as_ptr(), call(), MEM_COMMIT, MemoryBasicInformation, PAGE_GUARD, range_readable(), read(), read_cstr_unchecked() (+6 more)

### Community 83 - "state.rs"
Cohesion: 0.16
Nodes (8): PROF_CALLS, PROF_ENTRIES, PROF_ITER_US, PROF_LOCAL_US, PROF_PAWN_US, PROF_PLAYERS, TRACE_COUNT, TRACE

### Community 84 - "Engine"
Cohesion: 0.11
Nodes (18): Engine, engine.DisplayMode _()_, `engine.DisplayMode.back_buffer_count`, `engine.DisplayMode.height`, `engine.DisplayMode.high_dpi`, `engine.DisplayMode.multisample`, `engine.DisplayMode.refresh_rate`, `engine.DisplayMode.vsync` (+10 more)

### Community 85 - "Wallbang Rust — рабочий роадмап"
Cohesion: 0.11
Nodes (17): 1. Зафиксированные решения, 2.1 Инфраструктура, 2.2 Config/профили, 2.3 UI и превью, 2.4 ESP, 2.5 Chams, 2. Текущее состояние, 3. Порядок работ, утвёржденный Хозяином (+9 more)

### Community 86 - "cmd.rs"
Cohesion: 0.15
Nodes (8): Mat4, PlayerSnapshot, Snapshot, LEGIT_MIN_STEP, VK_SPACE, Target, score(), select()

### Community 87 - "QAngle"
Cohesion: 0.21
Nodes (9): angle_diff(), angle_to(), normalize_angle(), QAngle, MutationGuard, on_create_move(), angles_for(), apply_angles() (+1 more)

### Community 88 - "Vec3"
Cohesion: 0.23
Nodes (4): Vec3, extrapolate(), interpolate(), MAX_TICKS

### Community 89 - "Pawn"
Cohesion: 0.16
Nodes (3): read_cstr(), read_u8(), Pawn

### Community 90 - "Cloud"
Cohesion: 0.12
Nodes (17): Cloud, `cloud.GetConfig`, `cloud.GetScript`, `cloud.GetUser`, `cloud.GetUsername`, cloud.Item _()_, `cloud.Item.author`, `cloud.Item.id` (+9 more)

### Community 91 - "WebSocket"
Cohesion: 0.12
Nodes (17): Esp, WebSocket, websocket.Callbacks _()_, `websocket.Callbacks.close`, `websocket.Callbacks.error`, `websocket.Callbacks.message`, `websocket.Callbacks.open`, `websocket.Connect` (+9 more)

### Community 92 - "globals _()_"
Cohesion: 0.12
Nodes (17): Global Variables, globals _()_, `globals.absoluteframetime`, `globals.curtime`, `globals.framecount`, `globals.frametime`, `globals.in_simulation`, `globals.interpolation_amount` (+9 more)

### Community 93 - "ui.Group _()_"
Cohesion: 0.12
Nodes (17): ui.Group _()_, `ui.Group:AddButton`, `ui.Group:AddColorModes`, `ui.Group:AddColorPicker`, `ui.Group:AddColorSet`, `ui.Group:AddCombo`, `ui.Group:AddHotkey`, `ui.Group:AddImage` (+9 more)

### Community 94 - "hk_present"
Cohesion: 0.14
Nodes (13): create_vtable_probe(), GetModuleHandleA(), GetTickCount64(), hk_present(), install(), install_wndproc(), set_frame_callback(), set_trace() (+5 more)

### Community 95 - "resources/src/lib.rs"
Cohesion: 0.15
Nodes (14): atlas_rgba(), CHAM_MATERIALS, fnv1a(), fnv1a_matches_known_vectors(), glyph(), GLYPH_COUNT, GLYPH_H, glyph_table_is_complete() (+6 more)

### Community 96 - "matrix3x4a\_t" />"
Cohesion: 0.12
Nodes (16): matrix3x4a\_t" />, `matrix3x4a_t()`, `matrix3x4a_t(angles, position)`, `matrix3x4a_t * b`, `matrix3x4a_t * b`, `matrix3x4a_t + b`, `matrix3x4a_t - b`, `matrix3x4a_t(m)` (+8 more)

### Community 97 - "initialize"
Cohesion: 0.19
Nodes (15): log(), set_base(), initialize(), install_create_move(), install_frame_stage_notify(), load_scripts(), load_startup_profile(), log_signatures() (+7 more)

### Community 98 - "read_ptr"
Cohesion: 0.17
Nodes (3): EntityList, MAX_ENTRIES, read_ptr()

### Community 99 - "Settings"
Cohesion: 0.17
Nodes (10): finalize_binds(), invalid_binds_are_reset(), key_edges(), Keybind, KeyState, Down, Pressed, Released (+2 more)

### Community 100 - "bit _()_"
Cohesion: 0.13
Nodes (15): bit _()_, `bit.arshift`, `bit.band`, `bit.bnot`, `bit.bor`, `bit.bswap`, `bit.bxor`, `bit.lshift` (+7 more)

### Community 101 - "User Command"
Cohesion: 0.13
Nodes (15): CSGOInterpolationInfo" />, `CSGOInterpolationInfo.dst_tick`, `CSGOInterpolationInfo.frac`, `CSGOInterpolationInfo.src_tick`, Input, simulation _()_, `simulation.Grenade`, `simulation.SimOpt` (+7 more)

### Community 102 - "String"
Cohesion: 0.13
Nodes (15): String, `string.byte`, `string.char`, `string.find`, `string.format`, `string.gmatch`, `string.gsub`, `string.len` (+7 more)

### Community 103 - "filesystem _()_"
Cohesion: 0.14
Nodes (14): File System, filesystem _()_, `filesystem.Delete`, `filesystem.Folder`, `filesystem.Info`, filesystem.Info.Result _()_, `filesystem.Info.Result.atime`, `filesystem.Info.Result.ctime` (+6 more)

### Community 104 - "Random"
Cohesion: 0.14
Nodes (14): Random, `random.Float`, `random.FloatExp`, `random.GaussianFloat`, `random.Int`, `random.Stream`, random.Stream _()_, `random.Stream:Clone` (+6 more)

### Community 105 - "User Interface"
Cohesion: 0.14
Nodes (14): ui.ColorCategoryNode _()_, `#ui.ColorCategoryNode`, `ui.ColorCategoryNode[string|integer]`, ui.ColorPicker _()_, `ui.ColorPicker.default`, `ui.ColorPicker.raw_value`, `ui.ColorPicker.value`, ui.ColorsNode _()_ (+6 more)

### Community 106 - "ui_state.rs"
Cohesion: 0.26
Nodes (7): BarCfg, CATALOG_ITEMS, ExtendedUiState, PAINT_KITS, PlayerOverlayCfg, SkinItem, UiColor

### Community 107 - "mod.rs"
Cohesion: 0.22
Nodes (8): ARMOR_ABSORB, damage(), FALLOFF_START, angle_to_point(), Decision, HISTORY, local_view_angles(), run()

### Community 108 - "Environment"
Cohesion: 0.15
Nodes (13): Bit, boolean _()_, Environment, function _()_, \_G _()_, `int64_t`, integer _()_, nil _()_ (+5 more)

### Community 109 - "ESP"
Cohesion: 0.15
Nodes (13): ESP, `esp.AddBar`, `esp.AddBar.Callback`, `esp.AddItem`, `esp.AddText`, `esp.AddText.Callback`, `esp.Category`, esp.Element _()_ (+5 more)

### Community 110 - "ScriptPerformance _()_"
Cohesion: 0.15
Nodes (13): ScriptPerformance _()_, `ScriptPerformance.alloc_kb`, `ScriptPerformance.calls`, `ScriptPerformance.freed_kb`, `ScriptPerformance.load_ms`, `ScriptPerformance.long_calls`, `ScriptPerformance.max_ms`, `ScriptPerformance.self_ms` (+5 more)

### Community 111 - "Table"
Cohesion: 0.15
Nodes (13): Table, `table.concat`, `table.foreach`, `table.foreachi`, `table.getn`, `table.insert`, `table.maxn`, `table.move` (+5 more)

### Community 113 - "esp.rs"
Cohesion: 0.21
Nodes (8): CHEST_BONE, draw_esp(), FOOT_BONE, format_hp(), format_i32(), HEAD_BONE, STOMACH_BONE, weapon_name()

### Community 114 - ".default"
Cohesion: 0.18
Nodes (7): AppliedSkin, default_active_profile(), EspSettings, MenuSettings, new_visual_and_inventory_fields_round_trip(), SkinSettings, StickerSlot

### Community 115 - "MiscSettings"
Cohesion: 0.24
Nodes (6): AudioSettings, AutobuySettings, GrenadeAutoSettings, MiscSettings, MotionBlurSettings, MovementSettings

### Community 116 - "CloseHandle"
Cohesion: 0.32
Nodes (7): CloseHandle(), CreateThread(), INFINITE, spawn(), thread_proc(), ThreadHandle, WaitForSingleObject()

### Community 117 - "Reading API Pages"
Cohesion: 0.17
Nodes (12): Base, Dot, colon, and field access, Events pages, Generic arrays and schema vectors, Inheritance, Member cards, Multiple returns, Overloads (+4 more)

### Community 118 - "CTransformUnaligned" />"
Cohesion: 0.17
Nodes (12): CTransformUnaligned" />, `CTransformUnaligned()`, `CTransformUnaligned.AsTransform`, `CTransformUnaligned.Copy`, `CTransformUnaligned.IsValid`, `CTransformUnaligned.m_flScale`, `CTransformUnaligned.m_orientation`, `CTransformUnaligned.m_vPosition` (+4 more)

### Community 119 - "JSON"
Cohesion: 0.17
Nodes (12): JSON, `json.decode`, `json.decode_invalid_numbers`, `json.decode_max_depth`, `json.encode`, `json.encode_invalid_numbers`, `json.encode_keep_buffer`, `json.encode_max_depth` (+4 more)

### Community 120 - "panorama.Object _()_"
Cohesion: 0.17
Nodes (12): Panorama, `panorama.Compile`, `panorama.CreatePanel`, `panorama.DispatchEvent`, `panorama.ids`, panorama.Object _()_, `panorama.Object(...)`, `#panorama.Object` (+4 more)

### Community 121 - "sounds.Sound _()_"
Cohesion: 0.17
Nodes (12): Sounds, `sounds.PlayEvent`, sounds.PlayEvent.Options _()_, `sounds.PlayEvent.Options.lifetime`, `sounds.PlayEvent.Options.stop_on_new`, `sounds.PlayFile`, sounds.Sound _()_, `sounds.Sound:Destroy` (+4 more)

### Community 122 - "sigscan/src/main.rs"
Cohesion: 0.36
Nodes (9): main(), parse_pattern(), Pe, scan_all(), scan_section(), Section, u16_at(), u32_at() (+1 more)

### Community 123 - "examples/preview.rs"
Cohesion: 0.31
Nodes (8): adler32(), crc32(), main(), rasterize(), render_one(), save_png(), write_chunk(), Vertex

### Community 126 - "Lua Crash Course"
Cohesion: 0.18
Nodes (11): Classes and inheritance, Comments, Conditionals, Functions, Further reading, Loops, Lua Crash Course, Metatables and metamethods (+3 more)

### Community 127 - "view\.ViewmodelContext _()_"
Cohesion: 0.18
Nodes (11): Cloud, View, view\.ViewContext _()_, `view.ViewContext.angles`, `view.ViewContext.fov`, `view.ViewContext.origin`, view\.ViewmodelContext _()_, `view.ViewmodelContext.angles` (+3 more)

### Community 128 - "Color"
Cohesion: 0.18
Nodes (11): Color, `Color.a`, `Color.b`, `Color.g`, `Color.Hex`, `Color.Lerp`, `Color.r`, `Color(r, g, b, a)` (+3 more)

### Community 129 - "hitbox.rs"
Cohesion: 0.20
Nodes (8): ALL, ARMS, CHEST, extract(), FEET, HEAD, LEGS, STOMACH

### Community 130 - "History"
Cohesion: 0.24
Nodes (4): History, HISTORY_TICKS, MAX_LAG_TICKS, Record

### Community 131 - "CS2 Scripting"
Cohesion: 0.20
Nodes (10): API pages vs Dumped Data, Browse by category, CS2 Scripting, Learn the language, Lua names you will see often, Next pages, Reading These Docs, Start here (+2 more)

### Community 132 - "Input"
Cohesion: 0.20
Nodes (10): Input, input.analog _()_, `input.analog`, `input.analog.mouse_wheel`, `input.analog[string]`, input.buttons _()_, `input.buttons`, `input.buttons[string]` (+2 more)

### Community 133 - "JIT"
Cohesion: 0.20
Nodes (10): JIT, `jit.arch`, `jit.flush`, `jit.off`, `jit.on`, `jit.os`, `jit.status`, `jit.version` (+2 more)

### Community 134 - "AGENTS.md"
Cohesion: 0.22
Nodes (7): Build and verification, Current UI contract, graphify, Hot path rules, Operational gotchas, Real entrypoints and wiring, Safety and defaults

### Community 135 - "ChamsCfg"
Cohesion: 0.33
Nodes (6): ChamsCfg, ChamsLayer, typed_layer_to_ui(), typed_preset_to_ui(), ui_layer_to_typed(), ui_preset_to_typed()

### Community 136 - "Localization"
Cohesion: 0.22
Nodes (9): Add a translation file, Add translations from Lua, Folder layout, How text is chosen, Localization, Requirements, Rules, Test and publish (+1 more)

### Community 137 - "Hitbox _()_"
Cohesion: 0.22
Nodes (9): Entity System, Globals, Hitbox _()_, `Hitbox:GetCapsule`, `Hitbox:GetCenter`, `Hitbox:GetIndex`, `Hitbox:GetOrigin`, `Hitbox:GetOwnerEntity` (+1 more)

### Community 138 - "UserCmd.InputHistoryEntryHandle _()_"
Cohesion: 0.22
Nodes (9): UserCmd.InputHistoryEntryHandle _()_, `UserCmd.InputHistoryEntryHandle.cl_interp`, `UserCmd.InputHistoryEntryHandle.index`, `UserCmd.InputHistoryEntryHandle.player_interp`, `UserCmd.InputHistoryEntryHandle.player_tick`, `UserCmd.InputHistoryEntryHandle.render_tick`, `UserCmd.InputHistoryEntryHandle.sv_interp0`, `UserCmd.InputHistoryEntryHandle.sv_interp1` (+1 more)

### Community 139 - "UserCmd.SubtickMoveStepHandle _()_"
Cohesion: 0.22
Nodes (9): UserCmd.SubtickMoveStepHandle _()_, `UserCmd.SubtickMoveStepHandle.button`, `UserCmd.SubtickMoveStepHandle.index`, `UserCmd.SubtickMoveStepHandle.move`, `UserCmd.SubtickMoveStepHandle.moveangles`, `UserCmd.SubtickMoveStepHandle.preserve`, `UserCmd.SubtickMoveStepHandle.pressed`, `UserCmd.SubtickMoveStepHandle.viewangles` (+1 more)

### Community 141 - "hk_wndproc"
Cohesion: 0.36
Nodes (4): dummy_wndproc(), hk_wndproc(), shutdown(), shutdown()

### Community 142 - "Getting Started"
Cohesion: 0.25
Nodes (8): 1. Unlock Developer mode, 2. Create an extension, 3. Write your first script, 4. Use libraries, 5. Publish on the market, Getting Started, Lua Intelligence, Next steps

### Community 143 - "Anatomy"
Cohesion: 0.25
Nodes (8): Anatomy, Create the extension, Globals And Modules, Hot reload, Minimal layout, The entry point, The manifest, Where extensions live

### Community 144 - "Lifecycle and Events"
Cohesion: 0.25
Nodes (8): Choosing an event, Delayed work, Lifecycle and Events, Localization, main.lua runs on load, Movement code belongs in movement events, Render code belongs in events.render, Shutdown cleanup

### Community 145 - "Requirements"
Cohesion: 0.25
Nodes (8): Cmd, Local modules, Managing requirements, Market modules, Native modules, Requirements, Requiring your own files, The three kinds of requirement

### Community 146 - "Events"
Cohesion: 0.25
Nodes (8): Events, `events.Event`, events.Event.Traits _()_, `events.Event.Traits:Fire`, `events.Event.Traits:Set`, `events.Event.Traits:Unset`, `events[string]`, Hwmon

### Community 147 - "protobuf _()_"
Cohesion: 0.25
Nodes (8): protobuf _()_, `protobuf.HudPrintDest`, protobuf.MessageWrapper _()_, `protobuf.SayText`, `protobuf.SayText2`, `protobuf.TextMsg`, Protocol Buffers, Websocket

### Community 148 - "ui.Element _()_"
Cohesion: 0.25
Nodes (8): ui.Element _()_, `ui.Element.disabled`, `ui.Element:Export`, `ui.Element.icon`, `ui.Element.id`, `ui.Element:Import`, `ui.Element.name`, `ui.Element.visible`

### Community 149 - "wallbang_config"
Cohesion: 0.54
Nodes (8): wallbang, wallbang_config, wallbang_core, wallbang_feature, wallbang_loader, wallbang_lua, wallbang_render, wallbang_resources

### Community 150 - "mutation.rs"
Cohesion: 0.38
Nodes (3): acquire(), is_allowed(), RUNTIME_ALLOWED

### Community 151 - "multipoint.rs"
Cohesion: 0.29
Nodes (5): build(), MAX_SCALE, MIN_SCALE, POINT_GRID, POINT_RADIUS

### Community 153 - "msgpack _()_"
Cohesion: 0.29
Nodes (7): Console, MessagePack, msgpack _()_, `msgpack.pack`, `msgpack.unpack`, `msgpack.unpack_limit`, `msgpack.unpack_one`

### Community 154 - "input.ButtonHandle _()_"
Cohesion: 0.29
Nodes (7): input.ButtonHandle _()_, `input.ButtonHandle.binding`, `input.ButtonHandle.down`, `input.ButtonHandle.pressed`, `input.ButtonHandle.pressed_tick`, `input.ButtonHandle.released`, `input.ButtonHandle.released_tick`

### Community 155 - "Memory"
Cohesion: 0.29
Nodes (7): Memory, `memory.BindPattern`, `memory.BindVirtual`, `memory.CreateInterface`, `memory.FindPattern`, `memory.VirtualThunk`, Nav

### Community 156 - "Time"
Cohesion: 0.29
Nodes (7): Time, `time.Format`, `time.GetEpochMilliseconds`, `time.GetLocalTime`, `time.GetUnixTime`, `time.Schedule`, Utils

### Community 157 - "ui.ActiveBind _()_"
Cohesion: 0.29
Nodes (7): ui.ActiveBind _()_, `ui.ActiveBind.is_active`, `ui.ActiveBind.mode`, `ui.ActiveBind.name`, `ui.ActiveBind.reference`, `ui.ActiveBind.should_show`, `ui.ActiveBind.value`

### Community 158 - "ui.ColorModes _()_"
Cohesion: 0.29
Nodes (7): ui.ColorModes _()_, `ui.ColorModes.categories`, `ui.ColorModes.category`, `ui.ColorModes.default`, `ui.ColorModes.Mode`, `ui.ColorModes.raw_value`, `ui.ColorModes.value`

### Community 159 - "UserCmd.InputHistory _()_"
Cohesion: 0.29
Nodes (7): UserCmd.InputHistory _()_, `UserCmd.InputHistory:Clear`, `UserCmd.InputHistory:EmplaceBack`, `UserCmd.InputHistory:PopBack`, `UserCmd.InputHistory:Remove`, `UserCmd.InputHistory:Resize`, `UserCmd.InputHistory:Size`

### Community 161 - "Globals and Modules"
Cohesion: 0.33
Nodes (6): Common globals, Globals and Modules, Keeping globals clean, Lifecycle And Events, Modules, Standard Lua libraries

### Community 162 - "Font _()_"
Cohesion: 0.33
Nodes (6): Font _()_, `Font.face`, `Font.failed`, `Font.file`, `Font.loaded`, `Font.size`

### Community 163 - "Nav"
Cohesion: 0.33
Nodes (6): Nav, `nav.FindPath`, `nav.GetDistance`, `nav.IsLoaded`, `nav.Snap`, Random

### Community 164 - "render.ShaderOptions _()_"
Cohesion: 0.33
Nodes (6): render.ShaderOptions _()_, `render.ShaderOptions.blur`, `render.ShaderOptions.color`, `render.ShaderOptions.image`, `render.ShaderOptions.params`, `render.ShaderOptions.rounding`

### Community 165 - "simulation.GrenadeDamage _()_"
Cohesion: 0.33
Nodes (6): simulation.GrenadeDamage _()_, `simulation.GrenadeDamage.is_enemy`, `simulation.GrenadeDamage.max_damage`, `simulation.GrenadeDamage.min_damage`, `simulation.GrenadeDamage.passed`, `simulation.GrenadeDamage.player`

### Community 166 - "simulation.GrenadePoint _()_"
Cohesion: 0.33
Nodes (6): simulation.GrenadePoint _()_, `simulation.GrenadePoint.bounced`, `simulation.GrenadePoint.position`, `simulation.GrenadePoint.tick`, `simulation.GrenadePoint.time`, `simulation.GrenadePoint.velocity`

### Community 167 - "TickAndInterpolationAmount\_t" />"
Cohesion: 0.33
Nodes (6): TickAndInterpolationAmount\_t" />, `TickAndInterpolationAmount_t()`, `TickAndInterpolationAmount_t.fraction`, `TickAndInterpolationAmount_t(self)`, `TickAndInterpolationAmount_t.tick`, `TickAndInterpolationAmount_t(tick, fraction)`

### Community 168 - "UserCmd.InButtonHandle _()_"
Cohesion: 0.33
Nodes (6): UserCmd.InButtonHandle _()_, `UserCmd.InButtonHandle.down`, `UserCmd.InButtonHandle.pressed`, `UserCmd.InButtonHandle.released`, `UserCmd.InButtonHandle:RemoveButton`, `UserCmd.InButtonHandle.state`

### Community 169 - "TrackProperty"
Cohesion: 0.33
Nodes (6): Clip, Track, TrackProperty, Rotation, Scale, Translation

### Community 172 - "hitchance.rs"
Cohesion: 0.40
Nodes (3): LCG_ADD, LCG_MULT, roll()

### Community 173 - "Plane"
Cohesion: 0.40
Nodes (5): Frustum\_t" />, `Frustum_t.m_Planes`, Http, Plane, SideType _()_

### Community 174 - "Lua Intelligence"
Cohesion: 0.40
Nodes (5): Lua Intelligence, Overview, Pinned card tabs, Try the hover, What can appear in a card

### Community 175 - "Shader _()_"
Cohesion: 0.40
Nodes (5): Shader _()_, `Shader:GetError`, `Shader:IsReady`, `Shader:IsValid`, `Shader:Release`

### Community 176 - "hide_shots.rs"
Cohesion: 0.83
Nodes (3): build(), HideShot, should_hide()

### Community 177 - "Utilities"
Cohesion: 0.50
Nodes (4): Advanced Luajit, Utilities, utils _()_, `utils.clipboard`

### Community 178 - "Table Clearing"
Cohesion: 0.50
Nodes (4): Functions _()_, Table, `table.clear`, Table Clearing

### Community 179 - "Image _()_"
Cohesion: 0.50
Nodes (4): Image _()_, `Image.failed`, `Image.loaded`, `Image.size`

### Community 180 - "ui.ColorSet _()_"
Cohesion: 0.50
Nodes (4): ui.ColorSet _()_, `ui.ColorSet.default`, `ui.ColorSet.raw_value`, `ui.ColorSet.value`

## Knowledge Gaps
- **18 isolated node(s):** `Up`, `Pressed`, `Down`, `Released`, `Hud` (+13 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 3166 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **16 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Game Events` connect `Game Events` to `neverlose-api.md`?**
  _High betweenness centrality (0.116) - this node is a cross-community bridge._
- **What connects `Up`, `Pressed`, `Down` to the rest of the system?**
  _18 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Game Events` be split into smaller, more focused modules?**
  _Cohesion score 0.007272727272727273 - nodes in this community are weakly interconnected._
- **Why does `User Command` connect `User Command` to `simulation.Movement _()_`, `simulation.GrenadeDamage _()_`, `simulation.GrenadePoint _()_`, `simulation.Player _()_`, `TickAndInterpolationAmount\_t" />`, `UserCmd.InButtonHandle _()_`, `UserCmd.InputHistoryEntryHandle _()_`, `UserCmd.SubtickMoveStepHandle _()_`, `simulation.Weapon _()_`, `neverlose-api.md`, `UserCmd _()_`, `simulation.Grenade _()_`, `UserCmd.InputHistory _()_`?**
  _High betweenness centrality (0.108) - this node is a cross-community bridge._
- **Should `mathlib _()_` be split into smaller, more focused modules?**
  _Cohesion score 0.012345679012345678 - nodes in this community are weakly interconnected._
- **Why does `mathlib _()_` connect `mathlib _()_` to `Math Library`?**
  _High betweenness centrality (0.054) - this node is a cross-community bridge._
- **Should `api.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.0504950495049505 - nodes in this community are weakly interconnected._