// Config block: подписи проверены `tools/sigscan` на локальном client.dll (2026-11-24).
// Каждая запись — name=pattern; `?` — wildcard. Уникальные сигнатуры помечены UNIQUE.

/// Build 14201: slot 5 RVA B60580, contains AutoMakeUserCmd call B60637.
pub const SIG_CREATE_MOVE: &str = "85 D2 0F 85 ? ? ? ? 48 8B C4 44 88 40 18 89 50 10 48 89 48 08 55 53 56 41 54";
/// Первые 15 байт CreateMove — ровно на границе инструкций (3+4+4+1+1+1+1),
/// поэтому пролог-хук ворует их без риска разрезать инструкцию.
pub const SIG_CREATE_MOVE_PROLOGUE: &str = "48 8B C4 4C 89 40 ? 48 89 48 ? 55 53 56 57";
pub const CREATE_MOVE_STEAL_BYTES: usize = 15;
/// client.dll, UNIQUE (RVA 0xB60637): call site that stores the returned
/// `CUserCmd*` at `[rsp+0x40]`. Patching this `call rel32` is safer than hooking
/// the callee's prologue, which contains a relative call in its stolen bytes.
pub const SIG_AUTO_MAKE_USER_CMD_CALL: &str = "E8 ? ? ? ? 48 89 44 24 40 48 8D 4D F0";
/// Offline-verified target of `SIG_AUTO_MAKE_USER_CMD_CALL` on build 14201.
pub const AUTO_MAKE_USER_CMD_RVA: usize = 0x9447B0;
/// client.dll, UNIQUE (RVA 0xB6BA40): CCSGOInput::GetViewAngles.
pub const SIG_GET_VIEW_ANGLES: &str = "4C 8B C1 85 D2 74 08 48 8D 05 ? ? ? ? C3";
/// client.dll, UNIQUE (RVA 0x9DCE6A): dwEntityList.
pub const SIG_ENTITY_LIST: &str = "48 8B 0D ? ? ? ? 48 89 7C 24 ? 8B FA C1 EB";
/// client.dll, UNIQUE (RVA 0xC0F6E3): dwViewMatrix.
pub const SIG_VIEW_MATRIX: &str = "48 8D 0D ? ? ? ? 48 C1 E0 06";
/// client.dll, UNIQUE (RVA 0xD233A0): dwLocalPlayerPawn.
pub const SIG_LOCAL_PLAYER_PAWN: &str =
    "48 8D 05 ? ? ? ? C3 CC CC CC CC CC CC CC CC 48 83 EC ? 8B 0D";
/// client.dll, UNIQUE (RVA 0xEE2AA): dwCSGOInput.
pub const SIG_CSGO_INPUT: &str = "48 89 05 ? ? ? ? 0F 57 C0 0F 11 05";

/// client.dll, UNIQUE (RVA 0xB67EA0, vtable slot 37): CSource2Client::FrameStageNotify.
pub const SIG_FRAME_STAGE_NOTIFY: &str = "48 89 5C 24 18 48 89 6C 24 20 57 48 83 EC 40 48";
/// client.dll, UNIQUE (RVA 0xD004A0, vtable slot 22): OverrideView.
pub const SIG_OVERRIDE_VIEW: &str = "40 53 57 48 83 EC ? 48 8B FA E8";
/// client.dll, UNIQUE (RVA 0xA9ABC0 на локальном билде): CSkeletonInstance::CalcWorldSpaceBones.
/// Проверено оффлайн-сканером: ровно одно совпадение. Доступ к массиву костей:
/// scene_node + 0x140 (m_modelState) -> +0x80 (m_pBoneArray), stride 0x20.
pub const SIG_WORLD_SPACE_BONES: &str =
    "48 89 4C 24 ? 55 53 56 57 41 54 41 55 41 56 41 57 B8 ? ? ? ? E8 ? ? ? ? 48 2B E0 48 8D 6C 24 ? 48 8B 81";
/// client.dll, UNIQUE (RVA 0x7C8210): CGameParticleManager::CacheParticleEffect.
pub const SIG_CACHE_PARTICLE: &str = "4C 8B DC 53 48 81 EC ? ? ? ? F2 0F 10 05";
/// client.dll, UNIQUE (RVA 0xA116A0): CGameParticleManager::CreateParticleEffect.
pub const SIG_CREATE_PARTICLE: &str =
    "48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? F3 0F 10 1D ? ? ? ? 41 8B F8 8B DA 4C 8D 05";
/// client.dll, UNIQUE (RVA 0x9E3010): GetGameParticleManager.
pub const SIG_PARTICLE_MANAGER: &str =
    "48 8B 05 ? ? ? ? C3 CC CC CC CC CC CC CC CC 48 89 5C 24 ? 57 B8";
/// client.dll, UNIQUE (RVA 0x124C5F0): CBaseUserCmdPB::SerializePartialToArray (CRC).
pub const SIG_CMD_SERIALIZE: &str = "48 89 5C 24 18 55 56 57 48 81 EC 90";
/// client.dll, UNIQUE (RVA 0x1234090 на локальном билде): SerializeMoveCrc.
/// Это тот же самый адрес, что давал asphyxia-паттерн SetMessageData `... 48 83 EC 30 49`
/// (у него было 5 кандидатов). Уточнённый паттерн с `49 8B C0` даёт ровно одно совпадение.
pub const SIG_CMD_SET_MESSAGE_DATA: &str = "48 89 5C 24 20 55 56 57 48 83 EC 30 49 8B C0";
/// client.dll, UNIQUE (RVA 0x1483F0): string_copy — копирует собранный proto-буфер
/// в 64-байтный msg перед SerializeMoveCrc. Подтверждено оффлайн 25.11.2026 на
/// установленном client.dll: call-сайт `E8 ? ? ? ? 0F 10 45 88` (RVA 0x1403713)
/// ведёт ровно сюда, прямой пролог даёт одно совпадение. Прежний паттерн
/// `... 48 89 7C 24 20 41 56 ... 48 BF` давал 0 совпадений: в этом билде
/// сохраняются rsi/`48 BE`, а не rdi/`48 BF`.
/// Отдельной функции `WriteMessage` для пересчёта CRC нет: путь CRC — этот
/// копировщик плюс SIG_CMD_SET_MESSAGE_DATA.
pub const SIG_CMD_STRING_COPY: &str =
    "48 89 5C 24 10 48 89 6C 24 18 48 89 74 24 20 41 56 48 83 EC 20 48 BE";
/// client.dll, UNIQUE (RVA 0x1403713): call-сайт string_copy внутри CRC-функции.
/// `rip`-режим sigscan с disp+1 даёт target 0x1483F0; fallback, если пролог выше
/// перестал совпадать после апдейта.
pub const SIG_CMD_STRING_COPY_CALL: &str = "E8 ? ? ? ? 0F 10 45 88";
/// Смещение disp32 внутри SIG_CMD_STRING_COPY_CALL (байты 1..5).
pub const SIG_CMD_STRING_COPY_DISP: usize = 1;
/// panorama.dll: CUIEngine::RunScript (из Osiris, Windows x64).
pub const SIG_PANORAMA_RUN_SCRIPT: &str =
    "4C 89 4C 24 ? 4C 89 44 24 ? 48 89 54 24 ? 55 53 56 57 41 54 41 55 41 56 41 57 48 8D 6C 24";
/// client.dll, UNIQUE (RVA 0x952BE0): C_CSPlayerPawn::GetBonePositionByName.
/// `int(wchar? no) f(pawn, const char* name)` — returns the bone index for a
/// Source 2 bone name ("head_0", "spine_3", ...). Проверено tools/sigscan на
/// локальном client.dll: одно совпадение.
pub const SIG_GET_BONE_POSITION_BY_NAME: &str =
    "40 53 48 83 EC ? 48 8B 89 ? ? ? ? 48 8B DA 48 8B 01 FF 50 ? 48 8B C8";
/// client.dll: trace API (Celerity `world_trace.hpp`). Все пять паттернов
/// проверены tools/sigscan на установленном client.dll — по одному совпадению.
pub const SIG_TRACE_INIT_DATA: &str =
    "48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 48 8D 79 ? 33 F6 C7 47";
pub const SIG_TRACE_INIT_INFO: &str = "40 55 41 55 41 57 48 83 EC 30";
pub const SIG_TRACE_INIT_FILTER: &str =
    "48 89 5C 24 ? 48 89 74 24 ? 57 48 83 EC ? 0F B6 41 ? 33 FF 24";
pub const SIG_TRACE_CREATE: &str =
    "48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 41 56 41 57 48 83 EC ? F2 0F 10 02 4D 8D 71 08";
pub const SIG_TRACE_GET_INFO: &str =
    "48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 57 48 81 EC ? ? ? ? 48 8B E9 0F 29 74 24";
/// client.dll: HUD root (Panorama-инъекция). RIP-relative disp32 на +3.
/// Проверено tools/sigscan: match 0xD5495B -> глобал 0x25CF6C8.
pub const SIG_HUD: &str = "48 8B 05 ? ? ? ? 48 85 C0 74 71";
pub const SIG_HUD_DISP: usize = 3;
/// client.dll: pMainMenuPanel. Считается, что обёртка та же, что у HUD:
/// global -> object -> +0x8 CUIPanel*. Требует проверки в live-меню.
/// Проверено tools/sigscan: match 0xD52E53 -> глобал 0x25C2938, disp=25623262.
/// Если адрес не сходится: `cargo run --manifest-path tools/sigscan/Cargo.toml -- rip client.dll MENU '4C 8B 05 ? ? ? ? 83 E9 ?' 3`.
pub const SIG_MAIN_MENU: &str = "4C 8B 05 ? ? ? ? 83 E9 ?";
pub const SIG_MAIN_MENU_DISP: usize = 3;
/// client.dll: pUiEngine (fallback, если create_interface не сработал).
/// Проверено tools/sigscan: match 0x1788820 -> глобал 0x272DC60, disp=16405557.
pub const SIG_PUI_ENGINE: &str = "48 89 78 ? 48 89 0D ? ? ? ?";
pub const SIG_PUI_ENGINE_DISP: usize = 7;
/// inputsystem.dll: pInputSystemSvc. Match RVA 0x29D0, disp32 -> глобал RVA 0x46BC0
/// (совпадает с DW_INPUT_SYSTEM). Проверено tools/sigscan в режиме `rip`.
pub const SIG_INPUT_SYSTEM_SVC: &str =
    "48 8D 05 ? ? ? ? C3 CC CC CC CC CC CC CC CC 40 53 48 83 EC 20 33 DB";
/// Смещение disp32 внутри SIG_INPUT_SYSTEM_SVC (байты 3..7), откуда считается глобал.
pub const SIG_INPUT_SYSTEM_DISP: usize = 3;
/// scenesystem.dll, UNIQUE (RVA 0x3C5A0): CRenderScene drawing entry.
/// Источник: web (HvH sig list 23.09.2026), проверено tools/sigscan на локальном
/// scenesystem.dll 25.11.2026 — одно совпадение. Используется для world color.
pub const SIG_DRAW_SCENE_OBJECT_ARRAY: &str =
    "48 8B C4 48 89 50 ? 48 89 48 ? 55 53 56 57 41 54 41 55 41 56 41 57";
pub const DRAW_SCENE_OBJECT_ARRAY_STEAL_BYTES: usize = 23;
/// Branch-safe alternative: the vtable slot in scenesystem.dll whose value is
/// `CRenderScene::DrawSceneObjectArray`. Verified offline from the local dump.
pub const DRAW_SCENE_OBJECT_ARRAY_RVA: usize = 0x3C5A0;
pub const DRAW_SCENE_OBJECT_ARRAY_SLOT_RVA: usize = 0x5E4550;
/// scenesystem.dll, build 14201: `DrawSceneObject` vtable slot 1 (vtable RVA 0x5E4668,
/// unique lea `48 8D 05 ? ? ? ? 48 89 07 48 8B 7C 24 48` at RVA 0x77D2C).
pub const DRAW_SCENE_OBJECT_RVA: usize = 0x7A540;
pub const DRAW_SCENE_OBJECT_SLOT_RVA: usize = 0x5E4670;
/// scenesystem.dll, UNIQUE (RVA 0x154FB0): skybox draw entry.
/// Источник: web (HvH sig list), проверено tools/sigscan — одно совпадение.
pub const SIG_DRAW_SKYBOX_ARRAY: &str = "45 85 C9 0F 8E ? ? ? ? 4C 8B DC";
/// Branch-safe vtable slot for the skybox draw entry. Verified offline:
/// scenesystem.dll+0x5F6510 holds `base + 0x154FB0`.
pub const DRAW_SKYBOX_ARRAY_RVA: usize = 0x154FB0;
pub const DRAW_SKYBOX_ARRAY_SLOT_RVA: usize = 0x5F6510;
/// client.dll, UNIQUE (RVA 0x283270): fog setup. Web-сигнатура, проверена sigscan.
pub const SIG_SETUP_FOG: &str =
    "48 89 5C 24 ? 48 89 6C 24 ? 48 89 74 24 ? 48 89 4C 24 ? 57 41 54 41 55 41 56 41 57 48 83 EC 20 48 63 02";
/// client.dll, UNIQUE (RVA 0xD1DBE8): smoke render. Web-сигнатура, проверена sigscan.
pub const SIG_RENDER_SMOKE: &str = "5C 24 28 48 89 44 24 20 E8 ? ? ? ? 48 8B 5C 24 60";
/// client.dll, 2 matches (RVA 0x184CFD0/0x184E6B0): scene material prepare.
/// Источник: web (HvH sig list), проверено sigscan; используется только после
/// Additional validation в early-init, поэтому в хуки пока не входит.
pub const SIG_PREPARE_SCENE_MATERIAL: &str = "48 89 5C 24 08 48 89 74 24 10 57 48 83 EC 30 48 8B 59 20";
/// Путь к IDA Pro Хозяина (для реверса недостающих сигнатур).
pub const IDA_PRO_PATH: &str = r"C:\Users\dmitr\Downloads\AyuGram Desktop\IDA Pro\IDA Pro";

/// Unique in local newclient/client.dll: callsite B58C79, allocator 4F8D00.
pub const SIG_SUBTICK_ALLOC_SITE: &str = "48 8B 0F E8 ? ? ? ? 48 8B D0 48 8B CF E8 ? ? ? ? 48 8B D0 48 8B 0B 8B 42 10";
pub const SIG_REPEATED_PUSH: &str = "48 89 5C 24 08 57 48 83 EC 20 48 8B D9 48 8B FA 48 8B 49 10 48 85 C9";
/// Build 14201: legacy Celerity label ApplyEconCustomization is OnDataChanged.
pub const SIG_ECON_DATA_CHANGED: &str = "48 89 5C 24 08 57 48 83 EC 20 8B FA 48 8B D9 E8 ? ? ? ? 48 8B CB E8 ? ? ? ? 48 85 C0 74";
/// Native rebuild path, shared by C_EconEntity and HUD model global refresh.
pub const SIG_BUILD_ECON_MATERIALS: &str = "48 85 C9 0F 84 ? ? ? ? 55 56 57 41 56 41 57 48 8D AC 24 60 FD FF FF 48 81 EC A0 03 00 00 48 8B 05 ? ? ? ? 45 0F B6 F9 45 0F B6 F0 48 8B FA 48 8B F1";
pub const SIG_ECON_MATERIAL_CONTEXT: &str = "40 53 48 83 EC 20 48 8B D9 48 85 C9 75 08 32 C0 48 83 C4 20 5B C3 48 8D 0D ? ? ? ?";
pub const SIG_REGENERATE_SKIN: &str = "48 89 5C 24 10 48 89 6C 24 18 48 89 74 24 20 57 41 56 41 57 48 83 EC 20 44 0F B6 F2 48 8B F9 E8 ? ? ? ? 45 33 FF";
pub const SIG_SET_MODEL: &str = "40 53 48 83 EC ? 48 8B D9 4C 8B C2 48 8B 0D ? ? ? ? 48 8D 54 24";
pub const SIG_SET_MESH_GROUP: &str = "48 89 5C 24 08 48 89 74 24 10 57 48 83 EC 20 48 8D 99 40 01 00 00";
pub const SIG_UPDATE_SUBCLASS: &str = "4C 8B DC 53 48 81 EC ? ? ? ? 48 8B 41";
pub const SIG_SET_BODY_GROUP_SITE: &str = "E8 ? ? ? ? EB 0C 48 8B CF";

#[inline]
pub fn missing() -> &'static [&'static str] {
    &[]
}
