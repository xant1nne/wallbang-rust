Leek

# Wallbang Rust — рабочий роадмап

Дата: 25.11.2026. Файл лежит в корне проекта и заменяет старый план из `C:\Users\dmitr\.opencode\plan\wallbang-roadmap.md` как основной источник актуального состояния.

---

## 1. Зафиксированные решения

- Цель: внутренний чит для CS2 на **100% Rust**, target `x86_64-pc-windows-msvc`, stable, `cdylib`.
- Инъекция только своим manual-map loader: `wallbang_loader` запускает CS2 без `-insecure`, ждёт `client.dll`, мапит DLL и вызывает `wb_entry(base)` в удалённом потоке.
- `wb_entry` не возвращается на успехе: паркует поток и тикает Lua там же. Возврат запускает FLS/CRT-деструкторы и падает в `ntdll`.
- `DllMain` — только формальный экспорт; явный демонтаж — `wb_shutdown()`.
- `MUTATIONS_ENABLED = false`, `Misc::allow_writes = false`. Любая запись в игру — только через `MutationGuard` + `allow_writes`.
- GUI только на английском. Общение с Хозяином — по-русски, первая строка `Leek`, последняя `@Leek_Channel`.
- Никаких фейковых hotkeys/индикаторов/спектаторов/ watermark-значений. Контрол в UI появляется только при наличии реального backend'а.
- Профили — `.wb` (JSON внутри), активный профиль `default.wb`, сохранение активного профиля на shutdown. Все новые поля `Settings` обязаны иметь `serde(default)`.
- Основной агент в ESP-превью — свой skinned render. Panorama-мост остаётся только опциональным режимом `Engine: on`.
- Запрещены: runtime `D3DCompile`, покадровый файловый лог, `VirtualQuery` на каждое поле entity, пересоздание RTV каждый кадр, layered-overlay поверх игры.
- Старый C++-проект используется только как справочник. Разрешённый переносimin: ресурсы chams/hitsound; остальное — только после отдельного одобрения и проверки сигнатур/офсетов.

---

## 2. Текущее состояние

### 2.1 Инфраструктура

- Workspace: `config`, `core`, `render`, `feature`, `resources`, `lua`, `loader`, `app`.
- `cargo check --workspace` зелёный после последних правок (.remove fire_area/world_color).
- `cargo test --workspace` зелёный до последних правок; после удаления полей нужно повторить.
- Release-профиль: LTO, `codegen-units=1`, `panic=abort`, `strip=true`, static CRT, `/DYNAMICBASE`.
- Present hook = vtable slot 8, `ResizeBuffers` обрабатывается, WndProc сабклассируется для колеса мыши и блокировки ввода при открытом меню.
- CreateMove detour ставится от `SIG_CREATE_MOVE` из `config/src/signatures.rs`; original вызывается первым.
- Шейдеры предкомпилированы в `.cso`, runtime-компиляции нет.

### 2.2 Config/профили

- `.wb` storage: `list/path_for/exists/save/load/remove`, атомарная запись через temp-файл.
- `Settings.active_profile`, загрузка `default.wb` на старте, сохранение на `wb_shutdown()`.
- Config tab: create/load/save/reset/delete, поиск, подтверждение delete/reset.
- `ExtendedUiState` (`config/src/ui_state.rs`) сериализуется в профиле; `Menu::load_ui_settings` / `Menu::commit_ui_settings` синхронизируют его с типизированными блоками `Settings`.
- Синхронизация покрывает hit logs, tracers, audio, movement, viewmodel, ambience, item ESP, autobuy, skins и `visual.chams`.

### 2.3 UI и превью

- Активные вкладки: 0 Ragebot, 1 Legitbot, 2 Players, 3 Visuals, 4 Skins, 5 Misc, 6 Config.
- Players/Visuals/Skins/Misc подключены к `draw_content_v2`; Config остаётся отдельным экраном.
- Старый C++-порт сайдбара/табов/`draw_esp_preview` остаётся мёртвым кодом и не вызывается.
- Floating ESP preview открывается только на вкладке Players.
- Свой agent render: кэш по `(w,h,enemy,anim,material,color)`, кости проецируются из реального скелета модели, chams материал даёт flat-tint/ignore-z/wireframe Approximation в превью.
- ESP preview toggle'ы синхронизируются с `Settings.esp`, включая weapon/ammo.
- Panorama engine model не тикает в own-mode.

### 2.4 ESP

- Read-only ESP: box, skeleton, name, health, armor, distance, weapon, ammo.
- Weapon/ammo читаются через `Pawn::weapon()` по подтверждённым schema-офсетам; имя берётся из каталога предметов, а не из строки в памяти.
- Snapshot обновляется ~16 мс, timing пишется раз в ~600 кадров; `hook` держится около 0.2–0.5 мс.

### 2.5 Chams

- Игровой путь chams **не включён**: требует подтверждённых `scenesystem.dll` / `materialsystem2.dll` сигнатур, интерфейсов и live-проверки.
- Свой preview chams работает как визуальная аппроксимация и подсвечивает выбранный материал из `CHAM_MATERIALS`.
- Константы и структура хуков из C++ зафиксированы в исследовании; в текущем исходном состоянии их нельзя включать без проверки.

---

## 3. Порядок работ, утвёржденный Хозяином

1. **Config** — сделать. Текущее состояние: готово, нужна финальная проверка тестами после удаления полей.
2. **C1 Visuals/Skins/Misc + movement/HUD toggles** — принято Хозяином: “с1. базар, претезний нет”. Дополнительно: motion blur в Misc/Camera с persistent `enabled/strength/mode/on_velocity`.
3. **C2 Chams из `C:\wallbang-main-clone`** — в работе: готов own-preview слой; игровой слой ждёт валидации сигнатур и `MutationGuard`.
4. **C3 World/skybox/weather** — текущий приоритет. Важно: в C++-чите **погода, сцена и скайбокс работают**, а **fire area и world color отсутствуют**. Эти два контрола удалены из Rust UI/конфига и возвращаться не должны.
5. **C4 Инвентарь и skin-changer scene** — по образцу `C:\wb_toolchain\docs\1234.jpg`; только после C3.

---

## 4. C3: что переносить

### 4.1 Разрешено как read-only

- Skybox filename discovery: фиксированные 16 путей + сканирование `game/csgo/materials/skybox`, исключение `_fog`/`_lighting`.
- Inferno/fire overlay — это **ESP projectile overlay**, не “fire area” из ambience: read-only hull и timer по `C_Inferno`.
- Projectile/landing indicators — читаются из snapshot/entity list без записи.

### 4.2 Требует engine-хуков и записей

Всё ниже — только после оффлайн-проверки сигнатур на build 14201 и обёртывания записей в `MutationGuard` + `allow_writes`:

- Skybox material/color: `draw_skybox_array`; запись material binding и sky-color в дескриптор с обязательным restore после original.
- World material: `draw_scene_object_array` / `draw_scene_object`, stride primitive `0x70`, material `+0x20`, color `+0x50`, name vfunc 0.
- Lighting: `light_scene_object`, цвет по `scene_object +0xE4/+0xE8/+0xEC`.
- Fog/wetness/wind/bloom/gamma/DOF: `setup_fog`, `set_shader_param`, `set_shader_param_i`, `set_postprocess_vec`; restore исходных указателей/значений.
- Weather particles: VPCF `particles/embedded/{snow,rain,stars}.vpcf`, `particle_manager`, create/set_transform/control_point/destroy, `m_fRoundStartTime`.
- Smoke removal: vfunc 32/33, opacity floats по `+1280 + 16*i`; отдельный read-only/разрешённый write feature.
- “Remove skybox fog”: `prepare_scene_material`, параметры `g_flBrightnessExposureBias` / `g_flRenderOnlyExposureBias`.
- “Remove 3D skybox”: cvar `r_draw3dskybox` через `cvar->unlock_all()`.

### 4.3 Где смотреть в C++

- `world.hpp`, `world/impl/scene.cpp`, `world/impl/weather.cpp`, `world/impl/smoke.cpp`.
- `signatures/patterns.cpp`, `core/hooks/impl/cheat.cpp`.
- `systems/impl/materials.cpp` для KV3/material_create.
- `esp/projectile/projectile.overlay.cpp` для inferno overlay.

### 4.4 Правило переноса

Сначала добавить pattern-константы в `config/src/signatures.rs`, проверить их оффлайн:

```powershell
cargo run --manifest-path tools/sigscan/Cargo.toml -- pattern <dll> NAME=PATTERN
```

Только после уникального совпадения создавать hook. Никаких “похожих” офсетов и адресов из старой сборки.

---

## 5. Инструменты

- `target/` и `tools/sigscan/target/` — чисто build-артефакты, в архив проекта не попадают. После любой сборки их можно удалять: исходники и `Cargo.lock` восстанавливают сборку.
- `$log` в корне — мусор от IDA, удалён.
- Graphify: требуется только локальный CLI для OpenCode. Статус: **установлен и подключён** (`uv` + `graphifyy` в профиле Хозяина, `graphify opencode install --project`). Граф построен: `graphify-out/graph.json`, `graph.html`, `GRAPH_REPORT.md` (~10 МБ).
- Целевые команды после установки uv:

```powershell
uv tool install --python 3.12 graphifyy
graphify opencode install --project
graphify update .
```

- Результат должен появиться как `graphify-out/graph.json`, `graph.html`, `GRAPH_REPORT.md`, плюс `.opencode/plugins/graphify.js`, `.opencode/skills/graphify/SKILL.md` и секция в `AGENTS.md`.

---

## 6. Ближайшие шаги

1. `cargo test --workspace` после удаления fire_area/world_color.
2. Добавить в `config/src/signatures.rs` непроверенные pattern-константы для chams/world/weather с пометкой “clone reference, requires sigscan”.
3. Прогнать sigscan по локальным DLL и зафиксировать только уникальные совпадения.
4. Сначала сделать read-only inferno/projectile overlay — он не требует записи в игру.
5. После этого портировать skybox/world/lighting/fog/weather по одному hook'у с restore и `MutationGuard`.
6. Не делать release build, копирование DLL на Desktop, запуск loader и запуск CS2 без явного разрешения Хозяина.

---

## 7. Критерии приёмки

- `cargo check/test/build --workspace` зелёные.
- `hook` в среднем ~0.2–0.5 мс, без покадровых логов и лишних `VirtualQuery`.
- В UI нет fire area и world color; погода/сцена/скайбокс либо реализованы, либо спрятаны до реализации.
- Любая запись в игру требует `allow_writes` и `MutationGuard`.
- GUI только на английском; общение с Хозяином по-русски.
- Panorama не включается в own-mode; свой рендер агента остаётся основным.

@Leek_Channel
