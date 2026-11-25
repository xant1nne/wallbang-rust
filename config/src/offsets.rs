// Config block: офсеты из C:\Users\dmitr\Downloads\output (cs2-dumper, 2026-11-24 20:32 UTC).
// Единственный не-schema офсет — MODEL_STATE_BONE_ARRAY (IDA, client.dll).

/// Адрес client.dll получаем через GetModuleHandleA в рантайме.
pub const CLIENT_BASE: usize = 0; // module base, не константа дампа

pub const DW_ENTITY_LIST: usize = 0x27145B0;
pub const DW_GAME_ENTITY_SYSTEM: usize = 0x27145B0;
/// Проверено в билде 14201: 0x2120 из дампа даёт 229 и обрезает игроков в слотах 237/417;
/// реальный highest entity index лежит по 0x270.
pub const DW_HIGHEST_ENTITY_INDEX: usize = 0x270;
/// Массив указателей чанков внутри CGameEntitySystem.
pub const ENTITY_LIST_OFFSET: usize = 0x10;
pub const DW_LOCAL_PLAYER_PAWN: usize = 0x255EAB8;
pub const DW_LOCAL_PLAYER_CONTROLLER: usize = 0x2536300;
pub const DW_VIEW_MATRIX: usize = 0x25643D0;
pub const DW_VIEW_ANGLES: usize = 0x2574BE8;
pub const DW_CSGO_INPUT: usize = 0x2574560;
pub const DW_GLOBAL_VARS: usize = 0x222AAD0;
pub const DW_PREDICTION: usize = 0x255E9C0;
pub const DW_GLOW_MANAGER: usize = 0x255B110;
pub const DW_INPUT_SYSTEM: usize = 0x46BC0; // inputsystem.dll

pub const INPUT_RELATIVE_MOUSE_FLAG_OFFSET: usize = 84;
pub const INPUT_SET_RELATIVE_MOUSE_VFUNC: usize = 76;

// C_BaseEntity: m_iHealth 0x34C, m_iTeamNum 0x3E7, m_pGameSceneNode 0x330.
pub const PAWN_HEALTH: usize = 0x34C;
pub const PAWN_LIFE_STATE: usize = 0x354;
pub const PAWN_TEAM: usize = 0x3E7;
pub const PAWN_FLAGS: usize = 0x3F4;
pub const PAWN_VELOCITY: usize = 0x430;
// C_BaseModelEntity::m_vecViewOffset (CNetworkViewOffsetVector, xyz + w).
pub const PAWN_VIEW_OFFSET: usize = 0xF60;
// C_CSPlayerPawn::m_ArmorValue.
pub const PAWN_ARMOR: usize = 0x1F00;
// C_CSPlayerPawn::m_entitySpottedState 0x1EB8 + EntitySpottedState_t::m_bSpotted 0x8.
pub const PAWN_SPOTTED: usize = 0x1EC0;
// C_CSPlayerPawn::m_bIsScoped / m_iShotsFired.
pub const PAWN_SCOPED: usize = 0x1ED0;
pub const PAWN_SHOTS_FIRED: usize = 0x1EE8;
// Относительно CGameSceneNode (не pawn): CGameSceneNode::m_vecAbsOrigin 0xC8.
pub const PAWN_ABS_ORIGIN: usize = 0xC8;
pub const PAWN_SCENE_NODE: usize = 0x330;
// C_BasePlayerPawn::m_pWeaponServices / CPlayer_WeaponServices::m_hActiveWeapon.
pub const PAWN_WEAPON_SERVICES: usize = 0x12F0;
pub const WEAPON_SERVICE_ACTIVE: usize = 0x60;
// C_BasePlayerWeapon::m_iClip1 and C_EconEntity::m_AttributeManager::m_Item::m_iItemDefinitionIndex.
pub const WEAPON_CLIP1: usize = 0x1928;
pub const WEAPON_DEF_INDEX: usize = 0x1290 + 0x50 + 0x1BA;
// C_BaseEntity::m_nSubclassID + 0x8 points at weapon vdata.
pub const WEAPON_VDATA: usize = 0x380 + 0x8;
// CCSWeaponBaseVData::m_iMaxClip1 (the weapon name string lives at +0x720).
pub const VDATA_MAX_CLIP1: usize = 0x4D0;
// CSkeletonInstance::m_modelState 0x140; CModelState::m_pBoneArray 0x80 (IDA).
pub const SCENE_NODE_MODEL_STATE: usize = 0x140;
pub const MODEL_STATE_BONE_ARRAY: usize = 0x80;
// CCSPlayerController::m_sSanitizedPlayerName 0x878 (CUtlString: ptr, len).
pub const CONTROLLER_PLAYER_NAME: usize = 0x878;
// C_BasePlayerPawn::m_hController (CHandle<CBasePlayerController>), cs2-dumper 2026-11-24.
pub const PAWN_CONTROLLER_HANDLE: usize = 0x14BC;
pub const ENGINE_BUILD_NUMBER: usize = 0x6B2D1C;
pub const NETCHANNEL_CLIENT: usize = 0x9AFB70;

// ---- CUserCmd (client.dll) ----
// Сверено с рабочим internal'ом (asphyxia-cs2) и проверено сигнатурой CreateMove
// на локальном client.dll: сигнатура уникальна (RVA 0x178C140).
/// CUserCmd::csgoUserCmd.pBaseCmd.
pub const CMD_BASE_CMD: usize = 0x40;
/// CUserCmd::nButtons (CInButtonState) — vtable в +0x0.
pub const CMD_BUTTONS: usize = 0x58;
/// CInButtonState::nValue.
pub const CMD_BUTTON_VALUE: usize = 0x08;
/// CUserCmd::csgoUserCmd.inputHistoryField.nCurrentSize.
pub const CMD_HISTORY_COUNT: usize = 0x30;
/// CUserCmd::csgoUserCmd.inputHistoryField.pRep (Rep_t: int + T*[]).
pub const CMD_HISTORY_REP: usize = 0x38;
/// CUserCmd::csgoUserCmd.nHasBits.
pub const CMD_HAS_BITS: usize = 0x18;

/// CBaseUserCmdPB::nHasBits.
pub const BASE_HAS_BITS: usize = 0x08;
/// CBaseUserCmdPB::pInButtonState.
pub const BASE_BUTTON_STATE: usize = 0x38;
/// CBaseUserCmdPB::pViewAngles (CMsgQAngle: CBasePB 0x18 + QAngle).
pub const BASE_VIEW_ANGLES: usize = 0x40;
pub const BASE_FORWARD_MOVE: usize = 0x50;
pub const BASE_SIDE_MOVE: usize = 0x54;
pub const BASE_UP_MOVE: usize = 0x58;
pub const BASE_CLIENT_TICK: usize = 0x4C;
pub const BASE_RANDOM_SEED: usize = 0x64;
pub const BASE_MOUSE_DX: usize = 0x68;
pub const BASE_MOUSE_DY: usize = 0x6C;

/// CBasePB::nHasBits (для CInButtonStatePB и CMsgQAngle).
pub const PB_HAS_BITS: usize = 0x08;
/// CBasePB::nCachedBits — именно его proto-сериализатор считает «has bits».
pub const PB_CACHED_BITS: usize = 0x0C;
/// CInButtonStatePB::nValue.
pub const PB_BUTTON_VALUE: usize = 0x18;
/// CMsgQAngle::angValue (QAngle).
pub const PB_ANGLE_VALUE: usize = 0x18;
/// CCSGOInputHistoryEntryPB::pViewAngles.
pub const HISTORY_VIEW_ANGLES: usize = 0x18;

const DW_OFFSETS: [(&str, usize, &str); 12] = [
    ("DW_ENTITY_LIST", DW_ENTITY_LIST, "client.dll"),
    ("DW_GAME_ENTITY_SYSTEM", DW_GAME_ENTITY_SYSTEM, "client.dll"),
    ("DW_HIGHEST_ENTITY_INDEX", DW_HIGHEST_ENTITY_INDEX, "client.dll"),
    ("DW_LOCAL_PLAYER_PAWN", DW_LOCAL_PLAYER_PAWN, "client.dll"),
    ("DW_LOCAL_PLAYER_CONTROLLER", DW_LOCAL_PLAYER_CONTROLLER, "client.dll"),
    ("DW_VIEW_MATRIX", DW_VIEW_MATRIX, "client.dll"),
    ("DW_VIEW_ANGLES", DW_VIEW_ANGLES, "client.dll"),
    ("DW_CSGO_INPUT", DW_CSGO_INPUT, "client.dll"),
    ("DW_GLOBAL_VARS", DW_GLOBAL_VARS, "client.dll"),
    ("DW_PREDICTION", DW_PREDICTION, "client.dll"),
    ("DW_GLOW_MANAGER", DW_GLOW_MANAGER, "client.dll"),
    ("DW_INPUT_SYSTEM", DW_INPUT_SYSTEM, "inputsystem.dll"),
];

#[inline]
pub fn is_resolved() -> bool {
    DW_OFFSETS.iter().all(|&(_, value, _)| value != 0)
}

pub fn describe_missing() -> Vec<&'static str> {
    DW_OFFSETS
        .iter()
        .filter(|&&(_, value, _)| value == 0)
        .map(|&(name, _, _module)| {
            if name == "DW_INPUT_SYSTEM" {
                "DW_INPUT_SYSTEM: inputsystem.dll + dwInputSystem"
            } else {
                "offset: cs2-dumper output/offsets.rs → client.dll"
            }
        })
        .collect()
}
