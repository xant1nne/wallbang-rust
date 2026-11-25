Leek

# AGENTS.md

## Build and verification

- Workspace root is `C:\wallbang-rust`; target is stable `x86_64-pc-windows-msvc`. There is **no git repo** here.
- `build.bat` builds exactly one deliverable: **`dist\wallbang.exe`** — the GUI loader with `wallbang.dll` embedded. It sets `TMP`/`TEMP` to `target\tmp`, builds `-p wallbang` (the cdylib) first, then `-p wallbang_loader --bin wallbang_gui` (embeds `target/release/wallbang.dll` via `include_bytes!`), renames `wallbang_gui.exe` to `wallbang.exe`, copies it to the Desktop and deletes the obsolete old variants (`wallbang.dll`, `wallbang_gui.exe`, `wallbang_loader.exe`, `*_packed.exe`, `RUN_WALLBANG.bat`). There are no build modes and no on-disk-DLL variant.
- `.cargo/config.toml` adds `+crt-static` and `/DYNAMICBASE` for manual-map compatibility. Do not remove them.
- Standard checks: `cargo check --workspace`, `cargo test --workspace`, `cargo build --release --workspace`.
- Release outputs: `target\release\wallbang.dll` (payload) and `target\release\wallbang_gui.exe` (renamed to `wallbang.exe` by `build.bat`).
- Offline signature tool is a separate workspace: `cargo run --manifest-path tools/sigscan/Cargo.toml -- pattern <client.dll> NAME=PATTERN ...` (also supports `rip` and `vtable`).
- Offline UI rasterizer: `cargo run -p wallbang_render --example preview` (writes PNG files to the current directory).

## Real entrypoints and wiring

- 8 crates: `config`, `core`, `render`, `feature`, `resources`, `lua`, `loader`, `app`. The `app` crate is the `wallbang` `cdylib`.
- The only binary is **`wallbang.exe`** (crate bin name `wallbang_gui`, `loader/src/bin/wallbang_gui.rs`): a frameless Win32/D3D11 GUI loader drawn with the same `DrawList` as the game menu. One LOAD button runs the manual-map pipeline on a worker thread (attach if CS2 is running, else launch it without `-insecure`, wait for `client.dll`, map the embedded DLL, call `wb_entry`). The DLL payload is embedded at build time via `include_bytes!` of `target/release/wallbang.dll`, so the exe needs no external files. `src/main.rs` (old console loader) was removed.
- `wb_entry` must not return on success: it parks the remote thread and ticks Lua there. Returning runs Rust FLS/CRT destructors and crashes in `ntdll`.
- `DllMain` is only a formal export; startup is `wb_entry`. `wb_shutdown()` is the explicit teardown entry.
- Rendering hooks `IDXGISwapChain::Present` (vtable slot 8) and draws into the game swapchain RTV. Do not restore the abandoned layered-overlay approach (black screen over CS2).
- Shaders are precompiled `.cso` files in `render/src/shaders`. Runtime `D3DCompile` is forbidden: it reproducibly access-violates inside `d3dcompiler_47.dll` under manual map.
- CreateMove detour is installed from `config/src/signatures.rs`; it calls the original first, then `feature::combat::cmd::on_create_move`, which runs the Celerity movement port (`feature/src/movement.rs`) and then combat.
- Movement 1:1 port of Celerity: bhop/autostrafe/mini_jump/edgejump/jumpbug in `feature/src/movement.rs`, trace API in `core/src/trace.rs` (five signatures verified offline on client.dll: TraceInitData 0x87F0A0, TraceInitInfo 0x177A710, TraceInitFilter 0x353620, TraceCreate 0x883380, TraceGetInfo 0x885AD0). Keybinds are always-on (UI has no bind widget yet); edgejump presses IN_JUMP in the current tick because subtick steps are not written yet.

## Safety and defaults

- `MUTATIONS_ENABLED = false` and `Misc::allow_writes = false`. Every game write needs `MutationGuard` plus `allow_writes`; never bypass this.
- All `Settings` fields use `serde(default)`; new fields must too. Profiles are JSON under `%LOCALAPPDATA%\wallbang\configs`.
- GUI strings are English only. Chat and reports to the user are Russian, first line `Leek`, last line `@Leek_Channel`.

## Current UI contract

- `Menu::frame` draws the panel, logo/wordmark, divider, drag handle, a new navigation-only sidebar (`render/src/menu.rs` → `draw_sidebar_v2`), and an empty content pane for every tab. A floating ESP preview panel (`render/src/preview.rs` → `draw_panel`) opens only while the `Players` tab is active. Old C++-ported sidebar/tabs/`draw_esp_preview` remain dead code and must stay inactive.
- The floating ESP preview is now part of the **real ESP path**: `render/src/esp_draw.rs` (moved out of `feature`) draws both the world overlay and the preview through one `draw_target`, and the preview feeds it projected bones of the owned skinned model (`render/src/preview.rs`). The panel's toggles write the real per-target config (`settings.esp.enemies` via the Players overlay mirror), not preview-only state. The Panorama bridge request is disposed while the panel is open: it cannot hand bones back to Rust and its per-frame GPU readback cost frames. `feature/src/esp.rs` only builds screen-space targets from the snapshot.
- ESP is read-only and wired through `set_overlay(wallbang_feature::draw_esp)`, gated by `settings.esp.enabled`. Combat and movement modules are prototypes, not gameplay-verified.

## Hot path rules

- Snapshot refresh is throttled to ~16 ms in `render/src/hooks.rs`; `EntityList::iterate()` caches the module base and validates chunks once; `Pawn::fields()` batches reads after one address check.
- Do not reintroduce per-field `VirtualQuery`, per-frame file logging, or per-frame RTV recreation. Timing is logged once per ~600 frames; keep `hook` around 0.2–0.5 ms.

## Operational gotchas

- Real Desktop is `C:\Users\dmitr\OneDrive\Desktop`; the deliverable there is `wallbang.exe` (self-contained). Old `RUN_WALLBANG.bat` / `wallbang.dll` / separate loader exes are obsolete and `build.bat` deletes them.
- CRC helpers are confirmed offline on the installed `client.dll`: `SIG_CMD_SERIALIZE` (0x124C5F0), `SIG_CMD_SET_MESSAGE_DATA` (0x1234090, SerializeMoveCrc) and `SIG_CMD_STRING_COPY` (0x1483F0, the buffer copy helper; `E8 ? ? ? ? 0F 10 45 88` call site at 0x1403713 is its xref). There is no separate `WriteMessage` function for CRC. CRC recalculation itself stays off until mutations are enabled.
- `snapshot valid=false players=0` on the main menu is expected. Validate offsets only in a live match.
- For build 14201, `DW_HIGHEST_ENTITY_INDEX` is `0x270`; the dumper's `0x2120` returns ~229 and silently truncates players in later slots.
- `CSource2Client::FrameStageNotify` is vtable slot **36** in build 14201 (slot 37 is a different function). `app/src/lib.rs` finds the slot by comparing vtable entries with `SIG_FRAME_STAGE_NOTIFY` instead of hardcoding it.
- If `render::install` stalls at `vtable probe`, the DLL was mapped before DXGI was ready. Restart CS2, wait until its window responds, then use `wallbang_loader --attach --dll <path>`.
- Ask before rebuilding, running the loader, or launching CS2 after source-only edits; the last built DLL may not match the source.
- `cargo test --workspace` writes profile fixtures to the system `%TEMP%`; Defender blocks that write from the test exe (`PermissionDenied`). Run tests with `TMP`/`TEMP` pointed at `C:\wallbang-rust\target\tmp`.
- Player names: pawn `m_hController` (`0x14BC`) → `CCSPlayerController::m_sSanitizedPlayerName` (`0x878`), folded to printable ASCII and cached in `core/src/state.rs` (`NAME_CACHE_TTL_MS`), never resolved per frame.
- Menu mouse: while the menu is open `hk_wndproc` swallows `WM_INPUT` (raw mouse) and the 0x0200..0x0210 message range, and `render::hooks::update_menu_cursor` releases the game `ClipCursor` + forces the OS cursor visible (and hides it back on close). Blocking only legacy `WM_MOUSEMOVE` was not enough — CS2 reads the camera from raw input. Checkbox "gear dots" open their popup (three clickable dots left of the switch); popups drop down below the row.
- Camera (fov/thirdperson/aspect) is **disabled in the build**: its OverrideView hook patched the first qword in the client.dll image that equalled the OverrideView address, which was not the client-mode vtable slot — CS2 got an access violation inside our DLL right after injection. `feature/src/camera.rs` and the hook were removed. Re-add only through the ClientMode instance (Celerity `ClientMode` rip-relative signature) plus slot validation.
- The `graphify` CLI is not installed on this machine, so `graphify update .` cannot be run after edits; `graphify-out/` stays as-is until the tool appears.
- Roadmap: `C:\Users\dmitr\.opencode\plan\wallbang-roadmap.md`.

@Leek_Channel

## graphify

This project has a knowledge graph at graphify-out/ with god nodes, community structure, and cross-file relationships.

When the user types `/graphify`, use the installed graphify skill or instructions before doing anything else.

Rules:
- When graphify-out/graph.json exists, run `graphify query "<question>"` before your first grep, search or source-file read for a codebase question, and put this rule in every subagent prompt that explores code. Read raw files only for lines the graph or an earlier read already located. Use `graphify path "<A>" "<B>"` for relationships and `graphify explain "<concept>"` for focused concepts. When the answer is under a Markdown heading, `graphify section "<heading>"` prints that section's text, so the .md need not be opened. These return a scoped subgraph, usually much smaller than GRAPH_REPORT.md or raw grep output.
- Dirty graphify-out/ files are expected after hooks or incremental updates; dirty graph files are not a reason to skip graphify. Only skip graphify if the task is about stale or incorrect graph output, or the user explicitly says not to use it.
- If graphify-out/wiki/index.md exists, use it for broad navigation instead of raw source browsing.
- Read graphify-out/GRAPH_REPORT.md only for broad architecture review or when query/path/explain do not surface enough context.
- After modifying code, run `graphify update .` to keep the graph current (AST-only, no API cost).
- A committed graphify-out/graph.json is rewritten whole on every build, so a large diff to it in a pull request is expected. Do not flag it as churn or suggest skipping the graph rebuild or its hooks.
