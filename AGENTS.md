Leek

# AGENTS.md

## Build and verification

- Workspace root is `C:\wallbang-rust`; target is stable `x86_64-pc-windows-msvc`. There is **no git repo** here.
- `.cargo/config.toml` adds `+crt-static` and `/DYNAMICBASE` for manual-map compatibility. Do not remove them.
- Standard checks: `cargo check --workspace`, `cargo test --workspace`, `cargo build --release --workspace`.
- Release outputs: `target\release\wallbang.dll` and `target\release\wallbang_loader.exe`.
- Offline signature tool is a separate workspace: `cargo run --manifest-path tools/sigscan/Cargo.toml -- pattern <client.dll> NAME=PATTERN ...` (also supports `rip` and `vtable`).
- Offline UI rasterizer: `cargo run -p wallbang_render --example preview` (writes PNG files to the current directory).

## Real entrypoints and wiring

- 8 crates: `config`, `core`, `render`, `feature`, `resources`, `lua`, `loader`, `app`. The `app` crate is the `wallbang` `cdylib`.
- `wallbang_loader [--steam PATH] [--dll PATH] [--wait MS] [--attach] [--loadlib]` launches CS2 **without `-insecure`**, waits for `client.dll`, manual-maps the DLL, then calls exported `wb_entry(base)` in a remote thread. `WAIT_TIMEOUT` is success.
- `wb_entry` must not return on success: it parks the remote thread and ticks Lua there. Returning runs Rust FLS/CRT destructors and crashes in `ntdll`.
- `DllMain` is only a formal export; startup is `wb_entry`. `wb_shutdown()` is the explicit teardown entry.
- Rendering hooks `IDXGISwapChain::Present` (vtable slot 8) and draws into the game swapchain RTV. Do not restore the abandoned layered-overlay approach (black screen over CS2).
- Shaders are precompiled `.cso` files in `render/src/shaders`. Runtime `D3DCompile` is forbidden: it reproducibly access-violates inside `d3dcompiler_47.dll` under manual map.
- CreateMove detour is installed from `config/src/signatures.rs`; it calls the original first, then `feature::combat::cmd::on_create_move`.

## Safety and defaults

- `MUTATIONS_ENABLED = false` and `Misc::allow_writes = false`. Every game write needs `MutationGuard` plus `allow_writes`; never bypass this.
- All `Settings` fields use `serde(default)`; new fields must too. Profiles are JSON under `%LOCALAPPDATA%\wallbang\configs`.
- GUI strings are English only. Chat and reports to the user are Russian, first line `Leek`, last line `@Leek_Channel`.

## Current UI contract

- `Menu::frame` draws the panel, logo/wordmark, divider, drag handle, a new navigation-only sidebar (`render/src/menu.rs` → `draw_sidebar_v2`), and an empty content pane for every tab. A floating ESP preview panel (`render/src/preview.rs` → `draw_panel`) opens only while the `Players` tab is active. Old C++-ported sidebar/tabs/`draw_esp_preview` remain dead code and must stay inactive.
- The floating ESP preview uses a Panorama `MapPlayerPreviewPanel` bridge (`core/src/panorama.rs`, `render/src/native_preview.rs`) for a real animated CS2 player model and bone-projected ESP. The procedural mesh in `render/src/preview.rs` is only a fallback when the bridge is unavailable. Panorama script is throttled to ~120 ms. Do not add fake HUD hotkeys, indicators, spectators, or watermark values.
- ESP is read-only and wired through `set_overlay(wallbang_feature::draw_esp)`, gated by `settings.esp.enabled`. Combat and movement modules are prototypes, not gameplay-verified.

## Hot path rules

- Snapshot refresh is throttled to ~16 ms in `render/src/hooks.rs`; `EntityList::iterate()` caches the module base and validates chunks once; `Pawn::fields()` batches reads after one address check.
- Do not reintroduce per-field `VirtualQuery`, per-frame file logging, or per-frame RTV recreation. Timing is logged once per ~600 frames; keep `hook` around 0.2–0.5 ms.

## Operational gotchas

- Real Desktop is `C:\Users\dmitr\OneDrive\Desktop`; `RUN_WALLBANG.bat` points to `Desktop\wallbang.dll`. It can vanish; recreate it if needed.
- `SIG_CMD_WRITE_MESSAGE` is still `NOT FOUND`; CRC recalculation stays disabled. Particle detours, kill effects, and skin scene are placeholders.
- `snapshot valid=false players=0` on the main menu is expected. Validate offsets only in a live match.
- For build 14201, `DW_HIGHEST_ENTITY_INDEX` is `0x270`; the dumper's `0x2120` returns ~229 and silently truncates players in later slots.
- `CSource2Client::FrameStageNotify` is vtable slot **36** in build 14201 (slot 37 is a different function). `app/src/lib.rs` finds the slot by comparing vtable entries with `SIG_FRAME_STAGE_NOTIFY` instead of hardcoding it.
- If `render::install` stalls at `vtable probe`, the DLL was mapped before DXGI was ready. Restart CS2, wait until its window responds, then use `wallbang_loader --attach --dll <path>`.
- Ask before rebuilding, running the loader, or launching CS2 after source-only edits; the last built DLL may not match the source.
- `cargo test --workspace` writes profile fixtures to the system `%TEMP%`; Defender blocks that write from the test exe (`PermissionDenied`). Run tests with `TMP`/`TEMP` pointed at `C:\wallbang-rust\target\tmp`.
- Player names: pawn `m_hController` (`0x14BC`) → `CCSPlayerController::m_sSanitizedPlayerName` (`0x878`), folded to printable ASCII and cached in `core/src/state.rs` (`NAME_CACHE_TTL_MS`), never resolved per frame.
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
