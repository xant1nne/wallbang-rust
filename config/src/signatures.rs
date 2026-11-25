// Config block: подписи проверены `tools/sigscan` на локальном client.dll (2026-11-24).
// Каждая запись — name=pattern; `?` — wildcard. Уникальные сигнатуры помечены UNIQUE.

/// client.dll, UNIQUE (RVA 0x178C140): CCSGOInput::CreateMove.
pub const SIG_CREATE_MOVE: &str = "48 8B C4 4C 89 40 ? 48 89 48 ? 55 53 56 57 48 8D A8";
/// Первые 15 байт CreateMove — ровно на границе инструкций (3+4+4+1+1+1+1),
/// поэтому пролог-хук ворует их без риска разрезать инструкцию.
pub const SIG_CREATE_MOVE_PROLOGUE: &str = "48 8B C4 4C 89 40 ? 48 89 48 ? 55 53 56 57";
pub const CREATE_MOVE_STEAL_BYTES: usize = 15;
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
/// TODO_SIG: как подтвердить — см. IDA-скрипт в tools. asphyxia-паттерн на локальном билде
/// даёт 0 совпадений; пересчёт CRC пока не включаем (все мутации по умолчанию выключены).
pub const SIG_CMD_WRITE_MESSAGE: &str =
    "48 89 5C 24 10 48 89 6C 24 18 48 89 7C 24 20 41 56 48 83 EC 20 48 BF";
/// panorama.dll: CUIEngine::RunScript (из Osiris, Windows x64).
pub const SIG_PANORAMA_RUN_SCRIPT: &str =
    "4C 89 4C 24 ? 4C 89 44 24 ? 48 89 54 24 ? 55 53 56 57 41 54 41 55 41 56 41 57 48 8D 6C 24";
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
/// Путь к IDA Pro Хозяина (для реверса недостающих сигнатур).
pub const IDA_PRO_PATH: &str = r"C:\Users\dmitr\Downloads\AyuGram Desktop\IDA Pro\IDA Pro";

#[inline]
pub fn missing() -> &'static [&'static str] {
    &["SIG_CMD_WRITE_MESSAGE"]
}
