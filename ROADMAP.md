Leek

---

## 1. Текущее состояние

### 1.1 Инфраструктура

- Workspace: `config`, `core`, `render`, `feature`, `resources`, `lua`, `loader`, `app`.
- Target: stable `x86_64-pc-windows-msvc`, static CRT, `/DYNAMICBASE`.
- Единственный деливерабл: `dist\wallbang.exe` — GUI loader со встроенным `wallbang.dll` через `include_bytes!`.
- Present hook: `IDXGISwapChain::Present`, vtable slot 8. RTV пересоздаётся только на `ResizeBuffers`.
- CreateMove: original вызывается первым, дальше `feature::combat::cmd::on_create_move`.
- AutoMakeUserCmd: сигнатура переведена на 2-аргументную форму; call site подтверждён оффлайн (`SIG_AUTO_MAKE_USER_CMD_CALL`).
- Loader: добавлен TLS-index fix через удалённый `TlsAlloc` и direct-`jmp` stub для TLS callbacks.
- Шейдеры только предкомпилированные `.cso`; runtime `D3DCompile` запрещён.
- Диагностика крашей расширена: `symbol_for()`, `stack:` через `RtlCaptureStackBackTrace`, `rip_sym`, `raw_stack_trace()`, расширенный список модулей.
- `graphify update .` после правок выполнялся; `graphify-out/` ожидаемо грязный.

### 1.2 Конфиг и UI

- Профили — JSON под `%LOCALAPPDATA%\wallbang\configs`, активный `default`.
- Активные панели: `draw_esp_real`, `draw_visuals_real`, `draw_unavailable`, `draw_misc_real`. Старые C++-порты помечены `#[allow(dead_code)]` и не вызываются.
- Visuals-панель показывает только контролы с реальным backend'ом: fire area, world color, remove 3D skybox, remove skybox fog, lighting, fog, bloom, gamma, DOF, rain, smoke.
- World material, skybox color/cloud/sun, snow/stars, wetness/wind, projectile/landing indicators в UI **не выведены**: подтверждённого backend'а нет.
- `ragebot.silent` убран из UI.
- ESP preview: вкладка Players, floating panel, own skinned agent render по умолчанию; Panorama-мост disposed, пока панель открыта.
- Menu mouse: пока меню открыто, `hk_wndproc` глотает `WM_INPUT` и `0x0200..0x0210`; `ClipCursor` снимается, системный курсор показывается.

### 1.3 C3 / backend'ы

- `feature/src/cvar.rs` создан и подключён:
  - `engine2.dll!CreateInterface("VEngineCvar007")`;
  - поиск по массиву_CONVAR + schema-полям;
  - чтение/запись bool/int16/uint16/int32/uint32/int64/uint64/float32/float64;
  - сложные типы (string/color/vector) отклоняются;
  - каждая запись идёт через `MutationGuard`;
  - исходные 64 бита запоминаются и восстанавливаются при выключении/финале.
- Подтверждённый layout ConVar для этой сборки: имя `+0x00`, тип `+0x28`, флаги `+0x30`, типизированные значения `+0x58`. Старый вариант `+0x40` как массив значений **не использовать**.
- Таблица типов подтверждена: `0 bool`, `1 int16`, `2 uint16`, `3 int32`, `4 uint32`, `5 int64`, `6 uint64`, `7 float32`, `8 float64`, `9+` complex.
- `feature/src/c3.rs` создан и вызывается из `on_frame`: lighting, fog override, rain particles, bloom, gamma, DOF, smoke reduction.
- `feature/src/world.rs` создан: guarded world color через branch-safe VTableHook, remove 3D skybox, remove skybox fog через CallHook, read-only fire area overlay.
- Подтверждённые cvars: `r_drawskybox`, `r_draw3dskybox`, `sv_skyname`, `r_directlighting`, `r_indirectlighting`, `r_rendersun`, `fog_override_*`, `r_csgo_render_post_bloom_strength`, `r_fullscreen_gamma`, `dof_*`, `r_RainParticleDensity`, smoke/inferno-cvar'ы, `r_csgo_render_post_colorcorrection`, `r_csgo_render_post_local_contrast`, `r_csgo_render_post_film_grain`, `r_csgo_enable_tonemapping`, `r_csgo_postprocess_enable`.
- Snow/stars, wetness/wind, world material и projectile indicators не имеют подтверждённого backend'а.

### 1.4 Проверки, которые уже проходили

- `cargo check --workspace` — зелёный.
- `cargo test --workspace` с `TMP`/`TEMP = C:\wallbang-rust\target\tmp` — зелёный, включая тесты cvar encoding.
- `cargo run -p wallbang_render --example preview` — PNG в `target\tmp` перегенерированы.
- Оффлайн sigscan подтвердил уникальные совпадения для CreateMove, AutoMakeUserCmd, SetupFog, scene-system слотов и world-material паттерна.
- SHA256 установленного `client.dll` совпадал с копией, использованной для оффлайн-разбора.

---

## 2. Точка остановки

Работа остановлена посреди добавления **NVIDIA-style color correction как реальной пост-обработки**.

Что уже сделано в этой ветке:

- `feature/src/cvar.rs` — новый backend, подключён в `app/src/lib.rs` (init/on_frame/shutdown).
- `feature/src/c3.rs` — новый файл, подключён в `on_frame`.
- `config/src/settings.rs` — добавлены поля `fog_end`, `weather_density`, `bloom`, `bloom_strength`, `gamma`, `gamma_value`, `dof`, `dof_near_blurry`, `dof_far_blurry`, `smoke_removal`.
- `render/src/menu.rs` — Visuals-панель расширена реальными контролами; вызов `draw_visuals_real` теперь получает `state`.
- `graphify update .` выполнен.

Что **ещё не создано** и является следующим шагом:

- `render/src/shaders/color_vs.hlsl`, `color_ps.hlsl` и скомпилированные `.cso`.
- Структура настроек color correction и её поля в `AmbienceSettings`.
- Двухколоночная UI-секция Color correction.
- Вызов полноэкранного post-pass из `hk_present`.
- Тайминг `grade_us` в логе профиля.

Решение по реализации уже принято: полноэкранный D3D11 pass, который копирует backbuffer в owned SRV-текстуру, применяет HLSL-градинг и рисует результат обратно в backbuffer **до** отрисовки меню/оверлея. Никаких game-memory записей для этого не требуется. Pass должен пропускаться, если `enabled == false` или все параметры нейтральны.

---

## 3. Текущие проблемы

1. **Краши после manual-map.**
   - `0xC0000374` в `ntdll.dll` сразу после `wb_entry: parking remote thread`.
   - Ранее `0xC0000005` в `scenesystem.dll+0x3C5A0` от старого prologue-huka world color; заменён на branch-safe VTableHook.
   - Старый лог не содержит полезного stack trace; новые поля `rip_sym`/`raw_stack` ещё не попали ни в один билд.
   - Нужен новый билд и новый лог, но только после разрешения.

2. **Нет билда, соответствующего исходникам.**
   Последний известный hash `59925A2A...` устарел. Desktop-деливерабл тоже stale. Не делаться без разрешения.

3. **Cvar backend не проверен вживую.**
   Layout и типы подтверждены оффлайн, но `find()`, `read_number()` и записи не прогонялись в реальной игре. `allow_writes` стартует выключенным, поэтому при включении любого C3-тoglе нужно следить за restore.

4. **Недостаток реальных backend'ов.**
   Нет подтверждённых точек для snow/stars, wetness/wind, world material, skybox color/cloud/sun, projectile/landing indicators. Не добавлять контролы “на будущее”.

5. **Color correction ещё не реализована.**
   Дизайн есть, файлов нет. Нужно аккуратно встроить в Present, не сломав timing и не создав feedback backbuffer↔SRV.

6. **Ограничение по MSAA/формату.**
   Полноэкранный pass требует `SampleDesc.Count == 1` и совместимого формата backbuffer. При MSAA > 1 pass пропускать, не падать.

7. **Внешние источники недоступны.**
   Старые дампы/доноры/планы больше не считать существующими. Все сигнатуры и офсеты должны жить в репозитории; новые — добываться через `tools/sigscan` по локально установленной игре.

8. **IDA Python не настроен.**
   helper-скрипты на Python через shell не запускаются. Не делать это блокирующим: использовать только то, что уже перенесено в код, и встроенный sigscan.

9. **graphify-out грязный.**
   Это ожидаемо после правок. Не коммитить/push; при исчезновении CLI просто оставить граф как есть.

10. **Никаких коммитов/push.**
    Все изменения незакоммичены.

---

## 4. Матрица C3

| Функция | Статус | Backend |
|---|---|---|
| Fire area | Реализовано, read-only | `feature/src/world.rs`, entity/snapshot |
| World color | Код есть, branch-safe hook | guarded scene-object tint, restore |
| Remove 3D skybox | Код есть, source-only | scenesystem VTableHook |
| Remove skybox fog | Код есть, source-only | client CallHook на `SetupFog` |
| Lighting | Код есть | cvars `r_directlighting`, `r_indirectlighting`, `r_rendersun` |
| Fog override | Код есть | `fog_override_enable/max_density/end` |
| Rain | Код есть | `r_RainParticleDensity` |
| Bloom | Код есть | `r_csgo_render_post_bloom_strength` |
| Gamma | Код есть | `r_fullscreen_gamma` |
| DOF | Код есть | `dof_enabled`, `dof_near_blurry`, `dof_far_blurry` |
| Smoke reduction | Код есть | smoke cvars |
| Color correction | **В работе** | планируется собственный D3D11 post-pass + реальные engine cvars |
| Skybox filename/color | Нет backend'а | только cvars `r_drawskybox`, `sv_skyname`; цвет/облака/солнце — без подтверждённой точки |
| World material | Нет backend'а | паттерн есть, hook не подключён |
| Snow/stars | Нет backend'а | в бинарниках не найдено |
| Wetness/wind | Нет backend'а | в бинарниках не найдено |
| Projectile/landing indicators | Нет backend'а | требуется read-only entity path |
| Game chams | Отключено | только own preview-аппроксимация |

---


### Фаза B — Color correction (приоритет сейчас)

Файлы и шаги:

1. `render/src/shaders/color_vs.hlsl` и `color_ps.hlsl`:
   - fullscreen triangle/quad через уже существующий `Vertex`;
   - `cbuffer Grade : register(b0)` с тремя `float4`:
     - `p0 = brightness, contrast, saturation, hue`;
     - `p1 = vibrance, temperature, tint, sharpen`;
     - `p2 = shadows, midtones, highlights, reserved`.
   - sample из `Texture2D : register(t0)` + `SamplerState : register(s0)`;
   - hue rotation в YIQ;
   - vibrance по насыщенности пикселя;
   - temperature/tint как лёгкий white balance;
   - shadows/midtones/highlights по luma-весам;
   - sharpen как unsharp mask по 4 соседям;
   - `clamp(0,1)` на выходе, альфу сохранить.
2. Скомпилировать через системный `fxc.exe` из Windows Kits (`/T vs_5_0`, `/T ps_5_0`, `/E main`), положить `.cso` рядом. Runtime-компиляция запрещена.
3. `config/src/settings.rs`:
   - новый `#[derive(Clone, Debug, Serialize, Deserialize)] #[serde(default)] pub struct ColorGradeSettings`;
   - поля: `enabled`, `brightness`, `contrast`, `saturation`, `hue`, `vibrance`, `temperature`, `tint`, `shadows`, `midtones`, `highlights`, `sharpen`;
   - дефолты: `false`, `0`, `1`, `1`, `0`, `0`, `0`, `0`, `0`, `0`, `0`, `0`;
   - поле `pub color_grade: ColorGradeSettings` внутри `AmbienceSettings`.
4. `render/src/dx11.rs`:
   - `include_bytes!` для новых `.cso`;
   - owned texture + SRV, пересоздание при размере/формате;
   - DEFAULT constant buffer, обновление через `UpdateSubresource`;
   - метод `apply_color_grade(swapchain, &settings)`:
     - `CopyResource(copy, backbuffer)`;
     - update cbuffer;
     - bind RTV/input layout/VS/PS/SRV/sampler/cbuffer;
     - `Draw(3)`;
     - unbind SRV;
     - пропуск при MSAA и при нейтральных параметрах.
5. `render/src/hooks.rs`:
   - вызвать pass после построения `DrawList`, до `renderer.draw(list.vertices())`;
   - завести `GRADE_US` и добавить `grade=Nus` в строку `timing`.
6. `render/src/menu.rs`:
   - разделить Visuals на две колонки;
   - слева оставить текущие реальные контролы, справа — Color correction;
   - все слайдеры реально влияют на shader;
   - никаких отключённых/фейковых элементов.
7. Проверить: `cargo check`, `cargo test` с TMP/TEMP в `target\tmp`, preview PNG.

### Фаза C — реальные engine post-cvars

Подключить только после `cvar::find()` проверки:

- `r_csgo_render_post_colorcorrection`;
- `r_csgo_render_post_local_contrast`;
- `r_csgo_render_post_film_grain`;
- `r_csgo_enable_tonemapping`;
- `r_csgo_postprocess_enable`;
- `cl_disable_postprocessing` — только если логика инверсии подтверждена.

Каждый контрол появляется в UI только если `find()` нашёл cvar и тип поддерживается записью. Иначе запись в лог, но не фейковый чекбокс.

### Фаза D — оставшиеся обязательные C3

Для каждой функции:

1. Найти паттерн/слот через `tools/sigscan` по локально установленной игре.
2. Зафиксировать константу в `config/src/signatures.rs` или `config/src/offsets.rs` с пометкой источника.
3. Проверить уникальное совпадение оффлайн.
4. Сделать hook с fail-closed поведением.
5. Добавить `MutationGuard` + restore, если нужна запись.
6. Добавить контрол в UI только после шага 4.
7. Не переносить код из внешних доноров.

### Фаза E — доставка

1. `build.bat` только после разрешения.
2. Hash `dist\wallbang.exe` == hash на Desktop.
3. Старые варианты удаляются build-скриптом: `wallbang.dll`, `wallbang_gui.exe`, `wallbang_loader.exe`, `*_packed.exe`, `RUN_WALLBANG.bat`.
4. Новый лог после запуска — обязателен для диагностики.

### Фаза F — поддержка

- `cargo check --workspace`.
- `cargo test --workspace` с `TMP`/`TEMP = C:\wallbang-rust\target\tmp`.
- `cargo run -p wallbang_render --example preview`.
- `graphify update .`, если CLI доступен.
- Никаких коммитов/push без прямого указания.

---

## 6. Полезные команды

```powershell
cargo check --workspace

$env:TMP='C:\wallbang-rust\target\tmp'
$env:TEMP='C:\wallbang-rust\target\tmp'
cargo test --workspace

cargo run -p wallbang_render --example preview

cargo run --manifest-path tools/sigscan/Cargo.toml -- pattern <local_client_or_engine_dll> NAME=PATTERN

graphify update .
```

Путь к DLL для sigscan брать из локально установленной игры, не из внешних дампов.

---

## 7. Критерии приёмки

- Все обязательные C3-функции либо реализованы с реальным backend'ом, либо явно помечены как “no confirmed backend” без контрола в UI.
- Color correction работает как полноэкранный post-pass, все слайдеры влияют на результат, pass не ломает меню/ESP-оверлей.
- `hook` в среднем `0.2–0.5 мс`, `grade` измеряется отдельно.
- Любая запись в игру требует `allow_writes` и `MutationGuard`, исходные значения восстанавливаются.
- Новые `Settings`-поля имеют `serde(default)`, старые профили грузятся без миграции.
- GUI только на английском.
- КС/loader/build не запускались без разрешения.
- Коммит/push не делался.
- Внешние доноры и старые копии не использовались.

---

## 8. Чек-лист для продолжения с другого устройства

1. Прочитать только этот файл и исходники проекта.
2. Не искать внешние дампы/доноры/планы.
3. Проверить `git status --short`: всё должно быть незакоммичено.
4. Начать с Фазы B (color correction), не трогая КС.
5. После готовности source-only прогнать check/test/preview.
6. Спросить разрешение на сборку и запуск/attach.
7. Получить новый лог и только затем возвращаться к Фазе A.

@Leek_Channel
