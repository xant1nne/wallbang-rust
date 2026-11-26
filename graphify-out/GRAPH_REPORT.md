# Graph Report - wallbang-rust  (2026-11-26)

## Corpus Check
- 88 files · ~175,062 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 15 file(s) not represented in the graph (top: .cso 4, .hlsl 4, .glb 2)

## Summary
- 4789 nodes · 6485 edges · 214 communities (176 shown, 38 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 93 edges (avg confidence: 0.83)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `4d851045`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

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
- wallbang_gui.rs
- registry.rs
- VisualSettings
- esp_draw.rs
- usercmd.rs
- metatable _()_
- particles.Particle _()_
- trace.rs
- memory.rs
- state.rs
- Engine
- Wallbang Rust — рабочий роадмап
- movement.rs
- cvar.rs
- Vec3
- Pawn
- Cloud
- WebSocket
- globals _()_
- ui.Group _()_
- hk_present
- PanelMode
- matrix3x4a\_t" />
- read_ptr
- Settings
- bit _()_
- User Command
- String
- filesystem _()_
- Random
- User Interface
- ui_state.rs
- mindamage.rs
- Environment
- ESP
- ScriptPerformance _()_
- Table
- bone.rs
- ui.rs
- runtime.rs
- MiscSettings
- CloseHandle
- Reading API Pages
- CTransformUnaligned" />
- JSON
- panorama.Object _()_
- sounds.Sound _()_
- sigscan/src/main.rs
- bindings.rs
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
- world.rs
- UserCmd.SubtickMoveStepHandle _()_
- .render
- hk_wndproc
- EventBus
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
- weapons.rs
- .refresh
- Font _()_
- Nav
- render.ShaderOptions _()_
- Database
- simulation.GrenadePoint _()_
- TickAndInterpolationAmount\_t" />
- UserCmd.InButtonHandle _()_
- TrackProperty
- core/src/hooks.rs
- .camera
- hitchance.rs
- Plane
- UserCmd.SubtickMoves _()_
- Shader _()_
- hide_shots.rs
- esp.rs
- mutation.rs
- Image _()_
- ui.ColorSet _()_
- shaders.rs
- Euler Angles
- Math Library
- Matrix 4x4
- MENU_OPEN
- sigscan
- resources/src/lib.rs
- build_guard_accepts_only_the_dumped_build
- dw_offsets_are_resolved
- pixel_font.rs
- UserCmd.InputHistoryEntryHandle _()_
- Globals and Modules
- arc
- visuals.rs
- lua/src/lib.rs
- Table Allocation
- Table Clearing
- VTableHook
- log _()_
- module_base

## God Nodes (most connected - your core abstractions)
1. `Game Events` - 275 edges
2. `mathlib _()_` - 162 edges
3. `DrawList` - 113 edges
4. `entity _()_` - 85 edges
5. `simulation.Movement _()_` - 74 edges
6. `Vector` - 74 edges
7. `simulation.Player _()_` - 70 edges
8. `VectorAligned" />` - 66 edges
9. `Quaternion` - 64 edges
10. `Render` - 64 edges

## Surprising Connections (you probably didn't know these)
- `alloc_remote_tls_index()` --calls--> `GetModuleHandleA()`  [INFERRED]
  loader/src/entry_call.rs → app/src/diag.rs
- `map_image()` --calls--> `GetModuleHandleA()`  [INFERRED]
  loader/src/map.rs → app/src/diag.rs
- `draw_world_overlay()` --calls--> `w2s()`  [INFERRED]
  feature/src/world.rs → core/src/math.rs
- `create_window()` --calls--> `GetSystemMetrics()`  [INFERRED]
  loader/src/bin/wallbang_gui.rs → core/src/modules.rs
- `alloc_remote_tls_index()` --calls--> `GetProcAddress()`  [INFERRED]
  loader/src/entry_call.rs → core/src/modules.rs

## Import Cycles
- 1-file cycle: `render/src/esp_draw.rs -> render/src/esp_draw.rs`
- 1-file cycle: `feature/src/movement.rs -> feature/src/movement.rs`
- 1-file cycle: `core/src/pawn.rs -> core/src/pawn.rs`
- 1-file cycle: `render/src/pixel_font.rs -> render/src/pixel_font.rs`
- 1-file cycle: `core/src/weapons.rs -> core/src/weapons.rs`
- 1-file cycle: `config/src/offsets.rs -> config/src/offsets.rs`
- 2-file cycle: `core/src/modules.rs -> core/src/pattern.rs -> core/src/modules.rs`
- 2-file cycle: `render/src/draw.rs -> render/src/dx11.rs -> render/src/draw.rs`
- 2-file cycle: `lua/src/api.rs -> lua/src/runtime.rs -> lua/src/api.rs`
- 3-file cycle: `lua/src/api.rs -> lua/src/runtime.rs -> lua/src/bindings.rs -> lua/src/api.rs`
- 3-file cycle: `lua/src/api.rs -> lua/src/lib.rs -> lua/src/runtime.rs -> lua/src/api.rs`
- 4-file cycle: `lua/src/api.rs -> lua/src/lib.rs -> lua/src/runtime.rs -> lua/src/bindings.rs -> lua/src/api.rs`

## Communities (214 total, 38 thin omitted)

### Community 0 - "Game Events"
Cohesion: 0.01
Nodes (275): `achievement_earned`, `achievement_earned_local`, `achievement_event`, `achievement_info_loaded`, `achievement_write_failed`, `add_bullet_hit_marker`, `add_player_sonar_icon`, `ammo_pickup` (+267 more)

### Community 1 - "mathlib _()_"
Cohesion: 0.01
Nodes (162): mathlib _()_, `mathlib.AlmostEqual`, `mathlib.AngleDiff`, `mathlib.AngleIMatrix`, `mathlib.AngleMatrix`, `mathlib.AngleMod`, `mathlib.AngleNormalize`, `mathlib.AngleNormalizePositive` (+154 more)

### Community 2 - "api.rs"
Cohesion: 0.20
Nodes (18): args_to_string(), event_set(), event_unset(), json_to_lua(), lua_to_json(), lua_to_json_inner(), make_event_object(), MAX_DATABASE_KEY_LEN (+10 more)

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
Cohesion: 0.02
Nodes (97): BASE_BUTTON_STATE, BASE_CLIENT_TICK, BASE_FORWARD_MOVE, BASE_HAS_BITS, BASE_MOUSE_DX, BASE_MOUSE_DY, BASE_RANDOM_SEED, BASE_SIDE_MOVE (+89 more)

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
Nodes (24): CURSOR_SHOWING, DRAW, DX, ESP_OVERLAY, ESP_US, FRAME, HOOK_US, LAST_REFRESH_MS (+16 more)

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
Nodes (28): CHEST, clear_model_request(), draw_esp_overlay(), ELBOW_L, ELBOW_R, EspPreviewState, FOOT_L, FOOT_R (+20 more)

### Community 22 - "GameState"
Cohesion: 0.09
Nodes (4): GameState, DrawList, Menu, Style

### Community 23 - "AABB\_t" />"
Cohesion: 0.05
Nodes (41): AABB\_t" />, `AABB_t()`, `AABB_t.AddAABBToBounds`, `AABB_t.AddAABBToBounds`, `AABB_t.AddPointToBounds`, `AABB_t.AddSphereToBounds`, `AABB_t + b`, `AABB_t.ClosestPoint` (+33 more)

### Community 24 - "QAngle" />"
Cohesion: 0.05
Nodes (41): Angle, Color, QAngle" />, `-QAngle`, `QAngle + b`, `QAngle - b`, `QAngle * b`, `QAngle / b` (+33 more)

### Community 25 - "UserCmd _()_"
Cohesion: 0.05
Nodes (41): UserCmd _()_, `UserCmd.attack1_start_history_index`, `UserCmd.attack2_start_history_index`, `UserCmd.client_tick`, `UserCmd:Clone`, `UserCmd:Finalized`, `UserCmd.forwardmove`, `UserCmd.has_new_snapshot` (+33 more)

### Community 27 - "hwmon _()_"
Cohesion: 0.05
Nodes (39): Hardware Monitor, hwmon _()_, `hwmon.average_fps`, `hwmon.cpu_base_frequency`, `hwmon.cpu_cores`, `hwmon.cpu_cores_count`, `hwmon.cpu_count`, `hwmon.cpu_frequency` (+31 more)

### Community 28 - "Vector4D" />"
Cohesion: 0.05
Nodes (39): Vector4D" />, `-Vector4D`, `#Vector4D`, `Vector4D()`, `Vector4D.Abs`, `Vector4D.AsVector2D`, `Vector4D.AsVector3D`, `Vector4D + b` (+31 more)

### Community 29 - "panorama.rs"
Cohesion: 0.11
Nodes (8): GetUiEngineFn, hud_panel(), menu_panel(), RunScriptFn, wrapper_panel(), MAX_SIG_BYTES, Pattern, scan_module()

### Community 30 - "find_process.rs"
Cohesion: 0.11
Nodes (7): module_in_snapshot(), name_eq(), pid_in_snapshot(), POLL_INTERVAL_MS, process_handle(), wait_for_module(), wait_for_process()

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
Cohesion: 0.08
Nodes (25): AntiAimSettings, AutosSettings, BAR, CombatSettings, DuckPeekSettings, EMPTY_GROUP, EMPTY_LEGIT_GROUP, FLAG_ARMOR (+17 more)

### Community 40 - "signatures.rs"
Cohesion: 0.04
Nodes (47): AUTO_MAKE_USER_CMD_RVA, CREATE_MOVE_STEAL_BYTES, DRAW_SCENE_OBJECT_ARRAY_RVA, DRAW_SCENE_OBJECT_ARRAY_SLOT_RVA, DRAW_SCENE_OBJECT_ARRAY_STEAL_BYTES, DRAW_SKYBOX_ARRAY_RVA, DRAW_SKYBOX_ARRAY_SLOT_RVA, IDA_PRO_PATH (+39 more)

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
Cohesion: 0.16
Nodes (10): build_number(), build_supported(), CreateInterfaceFn, GetModuleHandleA(), GetProcAddress(), GetSystemMetrics(), screen_size(), SM_CXSCREEN (+2 more)

### Community 48 - "schema.Type _()_"
Cohesion: 0.07
Nodes (27): schema _()_, schema.Scope _()_, `schema.Scope[string]`, `schema[string]`, Schema System, schema.Type _()_, `#schema.Type`, `schema.Type:Atomic` (+19 more)

### Community 49 - "ui.Variable _()_"
Cohesion: 0.07
Nodes (27): ui.Variable _()_, `ui.Variable:AddCallback`, `ui.Variable:AddColorModes`, `ui.Variable:AddColorPicker`, `ui.Variable:AddColorSet`, `ui.Variable:AddGroup`, `ui.Variable:Chip`, `ui.Variable.default` (+19 more)

### Community 50 - "app/src/lib.rs"
Cohesion: 0.09
Nodes (34): init(), log(), set_base(), AUTO_MAKE_USER_CMD_ORIGINAL, AutoMakeUserCmdFn, cmd_base(), CREATE_MOVE_TRAMPOLINE, CreateMoveFn (+26 more)

### Community 51 - "native_preview.rs"
Cohesion: 0.13
Nodes (22): is_menu_open(), available(), build_script(), CAPTURE_FRAME, CAPTURE_GRACE_FRAMES, CAPTURE_LAST_OK, Driver, DRIVER (+14 more)

### Community 52 - "input.rs"
Cohesion: 0.14
Nodes (16): cursor_position(), CursorController, GetAsyncKeyState(), GetCursorPos(), Input, INPUT_MOUSE, key_down(), MOUSE (+8 more)

### Community 53 - "neverlose-api.md"
Cohesion: 0.06
Nodes (31): 1. Get the definitions, 1. Unlock Developer mode, 2. Add `.luarc.json` to your extension, 2. Create an extension, 3. Set up your editor, 3. Write your first script, 4. Check that it works, 4. Use libraries (+23 more)

### Community 54 - "Console"
Cohesion: 0.08
Nodes (25): Console, console.Command _()_, `console.Command(...)`, console.CommandLine _()_, `console.CommandLine(...)`, `console.CommandLine.is_valid`, `console.CommandLine.name`, `console.ConVarFlags_t` (+17 more)

### Community 55 - "Schema Values"
Cohesion: 0.08
Nodes (24): Cheat, Schema Values, value.AtomicCollection _()_, `value.AtomicCollection:Value`, value.Bitfield _()_, `value.Bitfield:NumBits`, value.DeclaredClass _()_, `value.DeclaredClass:GetField` (+16 more)

### Community 57 - "loadlib.rs"
Cohesion: 0.13
Nodes (3): START_TIMEOUT_MS, TLS_OUT_OF_INDEXES, load_library()

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
Cohesion: 0.20
Nodes (15): AddVectoredExceptionHandler(), DLL_BASE, ExceptionPointers, ExceptionRecord, GetModuleHandleA(), install_exception_logger(), LOG, log_path() (+7 more)

### Community 62 - "storage.rs"
Cohesion: 0.09
Nodes (15): BONE_STRIDE, CHUNK_SLOT_COUNT, CHUNK_SLOT_SIZE, CONFIG_EXTENSION, CONFIRM_TIMEOUT_S, DEFAULT_FONT_SIZE, DEFAULT_MENU_H, DEFAULT_MENU_W (+7 more)

### Community 63 - "font.rs"
Cohesion: 0.08
Nodes (26): adler32(), crc32(), main(), rasterize(), render_one(), save_png(), write_chunk(), ClipRect (+18 more)

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
Cohesion: 0.07
Nodes (7): Dx11, MARKER_PX, marker_rgb(), PreviewCapture, PS_BYTECODE, VERTEX_STRIDE, VS_BYTECODE

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

### Community 74 - "wallbang_gui.rs"
Cohesion: 0.04
Nodes (40): ACCENT, BG, BG_COLORREF, BTN, BUSY, button_rect(), CLICK, CLIENT_H (+32 more)

### Community 75 - "registry.rs"
Cohesion: 0.21
Nodes (8): Field, FieldRegistry, flatten(), fnv1a(), fnv1a_known_vectors(), global_registry(), GLOBAL_REGISTRY, registry_covers_all_settings_leaves()

### Community 76 - "VisualSettings"
Cohesion: 0.15
Nodes (10): AmbienceSettings, ChamsMaterialLayer, ChamsPreset, ChamsSettings, GrenadeSettings, HudSettings, ItemEspSettings, TracerSettings (+2 more)

### Community 77 - "esp_draw.rs"
Cohesion: 0.10
Nodes (31): EspPlayerSettings, EspSettings, has_flag(), Rgba, Vec2, w2s(), draw_targets(), project_bones() (+23 more)

### Community 78 - "usercmd.rs"
Cohesion: 0.09
Nodes (21): BASE_BITS_BUTTON_PB, BASE_BITS_FORWARD_MOVE, BASE_BITS_MOUSE_DX, BASE_BITS_MOUSE_DY, BASE_BITS_SIDE_MOVE, BASE_BITS_UP_MOVE, BASE_BITS_VIEW_ANGLES, FL_ONGROUND (+13 more)

### Community 79 - "metatable _()_"
Cohesion: 0.10
Nodes (20): metatable _()_, `metatable.__add`, `metatable.__call`, `metatable.__concat`, `metatable.__div`, `metatable.__eq`, `metatable.__gc`, `metatable.__index` (+12 more)

### Community 80 - "particles.Particle _()_"
Cohesion: 0.11
Nodes (19): Particles, `particles.Attachment`, `particles.Create`, particles.Create.Options _()_, `particles.Create.Options.attachment`, `particles.Create.Options.check_fow`, `particles.Create.Options.create_pvs`, `particles.Create.Options.entity` (+11 more)

### Community 81 - "trace.rs"
Cohesion: 0.11
Nodes (19): available(), FnCreate, FnGetInfo, FnInitData, FnInitFilter, FnInitInfo, functions(), GameTrace (+11 more)

### Community 82 - "memory.rs"
Cohesion: 0.14
Nodes (16): as_ptr(), call(), MEM_COMMIT, MemoryBasicInformation, PAGE_GUARD, range_readable(), read(), read_cstr() (+8 more)

### Community 83 - "state.rs"
Cohesion: 0.12
Nodes (15): BUILD_WARNED, CachedName, NAME_CACHE_CAPACITY, NAME_CACHE_TTL_MS, NAME_RETRY_MS, PROF_CALLS, PROF_ENTRIES, PROF_ITER_US (+7 more)

### Community 84 - "Engine"
Cohesion: 0.11
Nodes (18): Engine, engine.DisplayMode _()_, `engine.DisplayMode.back_buffer_count`, `engine.DisplayMode.height`, `engine.DisplayMode.high_dpi`, `engine.DisplayMode.multisample`, `engine.DisplayMode.refresh_rate`, `engine.DisplayMode.vsync` (+10 more)

### Community 85 - "Wallbang Rust — рабочий роадмап"
Cohesion: 0.11
Nodes (17): 1. Зафиксированные решения, 2.1 Инфраструктура, 2.2 Config/профили, 2.3 UI и превью, 2.4 ESP, 2.5 Chams, 2. Текущее состояние, 3. Порядок работ, утвёржденный Хозяином (+9 more)

### Community 86 - "movement.rs"
Cohesion: 0.11
Nodes (21): EDGE_CLEAR_FRACTION, EDGE_LOOK_MAX, EDGE_LOOK_MIN, EDGE_MIN_HORIZ_SPEED, EDGE_TRACE_DIST, INTERVAL_PER_TICK, local_player(), MovementState (+13 more)

### Community 87 - "cvar.rs"
Cohesion: 0.08
Nodes (38): group(), safe(), sync(), apply(), available(), Backend, BACKEND, Cvar (+30 more)

### Community 88 - "Vec3"
Cohesion: 0.13
Nodes (6): BoneArray, Mat3x4, Vec3, extrapolate(), interpolate(), MAX_TICKS

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
Cohesion: 0.10
Nodes (19): ClipCursor(), create_vtable_probe(), CursorInfo, enforce_menu_cursor(), GetCursorInfo(), GetModuleHandleA(), GetTickCount64(), hk_present() (+11 more)

### Community 95 - "PanelMode"
Cohesion: 0.20
Nodes (8): create_interface(), PanelMode, Hud, MainMenu, Panorama, resolve_panel(), RunResult, ui_engine_interface()

### Community 96 - "matrix3x4a\_t" />"
Cohesion: 0.12
Nodes (16): matrix3x4a\_t" />, `matrix3x4a_t()`, `matrix3x4a_t(angles, position)`, `matrix3x4a_t * b`, `matrix3x4a_t * b`, `matrix3x4a_t + b`, `matrix3x4a_t - b`, `matrix3x4a_t(m)` (+8 more)

### Community 99 - "Settings"
Cohesion: 0.10
Nodes (16): finalize_binds(), invalid_binds_are_reset(), key_edges(), Keybind, KeyState, Down, Pressed, Released (+8 more)

### Community 100 - "bit _()_"
Cohesion: 0.13
Nodes (15): bit _()_, `bit.arshift`, `bit.band`, `bit.bnot`, `bit.bor`, `bit.bswap`, `bit.bxor`, `bit.lshift` (+7 more)

### Community 101 - "User Command"
Cohesion: 0.13
Nodes (15): CSGOInterpolationInfo" />, `CSGOInterpolationInfo.dst_tick`, `CSGOInterpolationInfo.frac`, `CSGOInterpolationInfo.src_tick`, Input, simulation _()_, `simulation.Grenade`, simulation.GrenadeDamage _()_ (+7 more)

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
Cohesion: 0.23
Nodes (8): BarCfg, CATALOG_ITEMS, ExtendedUiState, PAINT_KITS, PlayerOverlayCfg, SkinItem, UiColor, esp_target_from_overlay()

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

### Community 112 - "bone.rs"
Cohesion: 0.08
Nodes (20): HEAD, LEFT_ELBOW, LEFT_FOOT, LEFT_HAND, LEFT_HIP, LEFT_KNEE, LEFT_SHOULDER, NECK (+12 more)

### Community 113 - "ui.rs"
Cohesion: 0.18
Nodes (7): ITEM_SPACING_X, ITEM_SPACING_Y, lerp_color(), POPUP_W, ROUNDING, WIDGET_ROUNDING, WINDOW_PAD

### Community 114 - "runtime.rs"
Cohesion: 0.13
Nodes (6): create_lua(), Deferred, report_callback_error(), Runtime, Scheduled, SHARED_LUA

### Community 115 - "MiscSettings"
Cohesion: 0.24
Nodes (6): AudioSettings, AutobuySettings, GrenadeAutoSettings, MiscSettings, MotionBlurSettings, MovementSettings

### Community 116 - "CloseHandle"
Cohesion: 0.23
Nodes (10): CloseHandle(), CreateThread(), INFINITE, spawn(), thread_proc(), ThreadHandle, WaitForSingleObject(), alloc_remote_tls_index() (+2 more)

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

### Community 123 - "bindings.rs"
Cohesion: 0.21
Nodes (19): angle_table(), bit_value(), clear_table(), color_table(), metatable(), mp_decode(), mp_encode(), mp_write_size() (+11 more)

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

### Community 138 - "world.rs"
Cohesion: 0.06
Nodes (25): hk_frame_stage_notify(), apply_world_color(), DrawSceneFn, DrawSkyboxFn, FOG_HOOKS, hk_draw_scene_object_array(), hk_draw_skybox_array(), hk_setup_fog() (+17 more)

### Community 139 - "UserCmd.SubtickMoveStepHandle _()_"
Cohesion: 0.22
Nodes (9): UserCmd.SubtickMoveStepHandle _()_, `UserCmd.SubtickMoveStepHandle.button`, `UserCmd.SubtickMoveStepHandle.index`, `UserCmd.SubtickMoveStepHandle.move`, `UserCmd.SubtickMoveStepHandle.moveangles`, `UserCmd.SubtickMoveStepHandle.preserve`, `UserCmd.SubtickMoveStepHandle.pressed`, `UserCmd.SubtickMoveStepHandle.viewangles` (+1 more)

### Community 141 - "hk_wndproc"
Cohesion: 0.36
Nodes (4): dummy_wndproc(), hk_wndproc(), shutdown(), shutdown()

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
Cohesion: 0.57
Nodes (8): wallbang, wallbang_config, wallbang_core, wallbang_feature, wallbang_loader, wallbang_lua, wallbang_render, wallbang_resources

### Community 150 - "mutation.rs"
Cohesion: 0.11
Nodes (18): angle_diff(), angle_to(), normalize_angle(), QAngle, PlayerSnapshot, LEGIT_MIN_STEP, on_create_move(), angle_to_point() (+10 more)

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

### Community 160 - "weapons.rs"
Cohesion: 0.39
Nodes (6): WeaponFields, display_for_raw(), display_name(), display_names_match_the_table(), keys_are_normalised_like_celerity(), normalize_key()

### Community 161 - ".refresh"
Cohesion: 0.18
Nodes (6): Mat4, name_is_capped_and_control_bytes_folded(), name_is_folded_to_font_alphabet(), sanitize_player_name(), Snapshot, trace()

### Community 162 - "Font _()_"
Cohesion: 0.33
Nodes (6): Font _()_, `Font.face`, `Font.failed`, `Font.file`, `Font.loaded`, `Font.size`

### Community 163 - "Nav"
Cohesion: 0.33
Nodes (6): Nav, `nav.FindPath`, `nav.GetDistance`, `nav.IsLoaded`, `nav.Snap`, Random

### Community 164 - "render.ShaderOptions _()_"
Cohesion: 0.33
Nodes (6): render.ShaderOptions _()_, `render.ShaderOptions.blur`, `render.ShaderOptions.color`, `render.ShaderOptions.image`, `render.ShaderOptions.params`, `render.ShaderOptions.rounding`

### Community 165 - "Database"
Cohesion: 0.38
Nodes (4): Database, database_path(), encoded_len(), json_keys()

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

### Community 170 - "core/src/hooks.rs"
Cohesion: 0.15
Nodes (14): CREATE_MOVE, ABS_JMP_LEN, CallHook, FlushInstructionCache(), GetCurrentProcess(), MEM_COMMIT, MEM_RELEASE, MEM_RESERVE (+6 more)

### Community 173 - "Plane"
Cohesion: 0.40
Nodes (5): Frustum\_t" />, `Frustum_t.m_Planes`, Http, Plane, SideType _()_

### Community 174 - "UserCmd.SubtickMoves _()_"
Cohesion: 0.33
Nodes (6): UserCmd.SubtickMoves _()_, `UserCmd.SubtickMoves:Clear`, `UserCmd.SubtickMoves:EmplaceBack`, `UserCmd.SubtickMoves:PopBack`, `UserCmd.SubtickMoves:Remove`, `UserCmd.SubtickMoves:Size`

### Community 175 - "Shader _()_"
Cohesion: 0.40
Nodes (5): Shader _()_, `Shader:GetError`, `Shader:IsReady`, `Shader:IsValid`, `Shader:Release`

### Community 176 - "hide_shots.rs"
Cohesion: 0.83
Nodes (3): build(), HideShot, should_hide()

### Community 177 - "esp.rs"
Cohesion: 0.15
Nodes (6): draw_esp(), FALLBACK_HEIGHT, HEALTH_ANIM, PROJECTION_MARGIN, UNIT_TO_METRE, draw_world_overlay()

### Community 178 - "mutation.rs"
Cohesion: 0.31
Nodes (4): acquire(), is_allowed(), MutationGuard, RUNTIME_ALLOWED

### Community 179 - "Image _()_"
Cohesion: 0.50
Nodes (4): Image _()_, `Image.failed`, `Image.loaded`, `Image.size`

### Community 180 - "ui.ColorSet _()_"
Cohesion: 0.50
Nodes (4): ui.ColorSet _()_, `ui.ColorSet.default`, `ui.ColorSet.raw_value`, `ui.ColorSet.value`

### Community 187 - "resources/src/lib.rs"
Cohesion: 0.15
Nodes (14): atlas_rgba(), CHAM_MATERIALS, fnv1a(), fnv1a_matches_known_vectors(), glyph(), GLYPH_COUNT, GLYPH_H, glyph_table_is_complete() (+6 more)

### Community 190 - "pixel_font.rs"
Cohesion: 0.21
Nodes (15): build_table(), CHAR_SPACING, draw(), EMPTY, Glyph, glyph(), glyph_pass(), GLYPHS (+7 more)

### Community 191 - "UserCmd.InputHistoryEntryHandle _()_"
Cohesion: 0.22
Nodes (9): UserCmd.InputHistoryEntryHandle _()_, `UserCmd.InputHistoryEntryHandle.cl_interp`, `UserCmd.InputHistoryEntryHandle.index`, `UserCmd.InputHistoryEntryHandle.player_interp`, `UserCmd.InputHistoryEntryHandle.player_tick`, `UserCmd.InputHistoryEntryHandle.render_tick`, `UserCmd.InputHistoryEntryHandle.sv_interp0`, `UserCmd.InputHistoryEntryHandle.sv_interp1` (+1 more)

### Community 193 - "Globals and Modules"
Cohesion: 0.33
Nodes (6): Common globals, Globals and Modules, Keeping globals clean, Lifecycle And Events, Modules, Standard Lua libraries

### Community 198 - "lua/src/lib.rs"
Cohesion: 0.40
Nodes (4): MAX_DATABASE_BYTES, MAX_DATABASE_DEPTH, MAX_DATABASE_KEYS, MAX_SCHEDULED

### Community 199 - "Table Allocation"
Cohesion: 0.50
Nodes (4): Aabb, Functions _()_, Table Allocation, `table.new`

### Community 200 - "Table Clearing"
Cohesion: 0.50
Nodes (4): Functions _()_, Table, `table.clear`, Table Clearing

### Community 201 - "VTableHook"
Cohesion: 0.22
Nodes (4): AUTO_MAKE_USER_CMD, FRAME_STAGE, LUA, VTableHook

### Community 202 - "log _()_"
Cohesion: 0.33
Nodes (6): log _()_, `log.Console`, `log.Event`, `log.Print`, Logging, Time

### Community 204 - "module_base"
Cohesion: 0.24
Nodes (11): EntityList, read_ptr(), csgo_input(), input_system(), module_base(), module_size(), run_script_address(), install() (+3 more)

## Knowledge Gaps
- **21 isolated node(s):** `InvalidPe`, `VirtualAlloc`, `Write`, `Reloc`, `Import` (+16 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 3412 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **38 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Game Events` connect `Game Events` to `neverlose-api.md`?**
  _High betweenness centrality (0.070) - this node is a cross-community bridge._
- **What connects `InvalidPe`, `VirtualAlloc`, `Write` to the rest of the system?**
  _21 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Game Events` be split into smaller, more focused modules?**
  _Cohesion score 0.007272727272727273 - nodes in this community are weakly interconnected._
- **Why does `User Command` connect `User Command` to `UserCmd.InputHistoryEntryHandle _()_`, `simulation.Movement _()_`, `simulation.GrenadePoint _()_`, `simulation.Player _()_`, `TickAndInterpolationAmount\_t" />`, `UserCmd.InButtonHandle _()_`, `UserCmd.SubtickMoveStepHandle _()_`, `UserCmd.SubtickMoves _()_`, `simulation.Weapon _()_`, `neverlose-api.md`, `UserCmd _()_`, `simulation.Grenade _()_`, `UserCmd.InputHistory _()_`?**
  _High betweenness centrality (0.064) - this node is a cross-community bridge._
- **Should `mathlib _()_` be split into smaller, more focused modules?**
  _Cohesion score 0.012345679012345678 - nodes in this community are weakly interconnected._
- **Why does `User Interface` connect `User Interface` to `ui _()_`, `ui.Variable _()_`, `ui.ColorSet _()_`, `neverlose-api.md`, `ui.Element _()_`, `ui.Group _()_`, `ui.ActiveBind _()_`, `ui.ColorModes _()_`?**
  _High betweenness centrality (0.041) - this node is a cross-community bridge._
- **Should `entity _()_` be split into smaller, more focused modules?**
  _Cohesion score 0.023529411764705882 - nodes in this community are weakly interconnected._