// Config block: офсеты из C:\Users\dmitr\Downloads\output (cs2-dumper, build 14201, 2026-11-24).
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
// i_csgo_input: in_thirdperson 0x229, movement fields 0x258..0x26C, view_angles 0x2D0
// (Celerity i_csgo_input.hpp).
pub const CSGO_INPUT_IN_THIRDPERSON: usize = 0x229;
pub const CSGO_INPUT_FORWARD_MOVE: usize = 0x258;
pub const CSGO_INPUT_LEFT_MOVE: usize = 0x25C;
pub const CSGO_INPUT_MOUSE_DELTA_X: usize = 0x264;
pub const CSGO_INPUT_VIEW_ANGLES: usize = 0x2D0;
// CViewSetup (Celerity c_view_setup.hpp): world/viewmodel fov, origin, angles,
// explicit-aspect flag.
pub const VIEW_SETUP_WORLD_FOV: usize = 0x498;
pub const VIEW_SETUP_VIEWMODEL_FOV: usize = 0x49C;
pub const VIEW_SETUP_ORIGIN: usize = 0x4A0;
pub const VIEW_SETUP_VIEW_ANGLE: usize = 0x4B8;
pub const VIEW_SETUP_ASPECT_RATIO: usize = 0x4D4;
pub const VIEW_SETUP_FLAGS: usize = 0x551;
/// CViewSetup::m_view_flags explicit-aspect bit (Celerity aspect_ratio).
pub const VIEW_FLAG_EXPLICIT_ASPECT: u8 = 1 << 1;
pub const DW_GLOBAL_VARS: usize = 0x222AAD0;
pub const DW_PREDICTION: usize = 0x255E9C0;
pub const DW_GLOW_MANAGER: usize = 0x255B110;
pub const DW_INPUT_SYSTEM: usize = 0x46BC0; // inputsystem.dll

// C_Inferno (local schema dump 2026-11-24): fire hull, read-only overlay.
pub const INFERNO_FIRE_POSITIONS: usize = 0x1108; // VectorWS[64], stride 0xC
pub const INFERNO_FIRE_COUNT: usize = 0x1A48; // int32
pub const INFERNO_LIFETIME: usize = 0x1A50; // float32
pub const INFERNO_MAX_FIRES: usize = 64;
// CEntityIdentity::m_designerName — CUtlSymbolLarge (const char*).
pub const ENTITY_DESIGNER_NAME: usize = 0x20;

pub const INPUT_RELATIVE_MOUSE_FLAG_OFFSET: usize = 84;
pub const INPUT_SET_RELATIVE_MOUSE_VFUNC: usize = 76;

// C_BaseEntity: m_iHealth 0x34C, m_iTeamNum 0x3E7, m_pGameSceneNode 0x330, m_pCollision 0x340.
pub const PAWN_HEALTH: usize = 0x34C;
pub const PAWN_LIFE_STATE: usize = 0x354;
pub const PAWN_TEAM: usize = 0x3E7;
pub const PAWN_FLAGS: usize = 0x3F4;
pub const PAWN_VELOCITY: usize = 0x430;
/// C_BaseEntity::m_vecAbsVelocity (м/с в мировых координатах) — нужен movement.
pub const PAWN_ABS_VELOCITY: usize = 0x3F8;
/// C_BaseEntity::m_MoveType (MOVETYPE_*).
pub const PAWN_MOVE_TYPE: usize = 0x525;
pub const PAWN_COLLISION: usize = 0x340;
// CCollisionProperty::m_vecMins / m_vecMaxs (Vector) в мировых координатах после + origin.
pub const COLLISION_MINS: usize = 0x40;
pub const COLLISION_MAXS: usize = 0x4C;
// C_BaseModelEntity::m_vecViewOffset (CNetworkViewOffsetVector, xyz + w).
pub const PAWN_VIEW_OFFSET: usize = 0xF60;
// C_CSPlayerPawn::m_ArmorValue.
pub const PAWN_ARMOR: usize = 0x1F00;
// C_CSPlayerPawn::m_entitySpottedState 0x1EB8 + EntitySpottedState_t::m_bSpotted 0x8.
pub const PAWN_SPOTTED: usize = 0x1EC0;
/// EntitySpottedState_t::m_bSpottedByMask (uint32[2]) at +0xC.
pub const PAWN_SPOTTED_BY_MASK: usize = 0x1EC4;
// C_CSPlayerPawn::m_bIsScoped / m_iShotsFired.
pub const PAWN_SCOPED: usize = 0x1ED0;
pub const PAWN_SHOTS_FIRED: usize = 0x1EE8;
// Относительно CGameSceneNode (не pawn): CGameSceneNode::m_vecAbsOrigin 0xC8.
pub const PAWN_ABS_ORIGIN: usize = 0xC8;
pub const PAWN_SCENE_NODE: usize = 0x330;
// C_BasePlayerPawn::m_pWeaponServices / CPlayer_WeaponServices::m_hActiveWeapon.
pub const PAWN_WEAPON_SERVICES: usize = 0x12F0;
pub const PAWN_OBSERVER_SERVICES: usize = 0x1308;
pub const OBSERVER_TARGET: usize = 0x4C;
pub const WEAPON_SERVICE_ACTIVE: usize = 0x60;
// CPlayer_WeaponServices::m_hMyWeapons (C_NetworkUtlVectorBase<CHandle>).
pub const WEAPON_SERVICES_WEAPONS: usize = 0x48;
/// C_NetworkUtlVectorBase: `u32 m_size` at +0, element pointer at +8.
pub const NET_VECTOR_SIZE: usize = 0x00;
pub const NET_VECTOR_DATA: usize = 0x08;
/// Handle array cap used when scanning a player's weapon list for C4.
pub const MAX_OWNED_WEAPONS: usize = 64;
/// C_EconItemView::m_iItemDefinitionIndex для C4.
pub const WEAPON_C4_DEF_INDEX: i16 = 49;
// C_BasePlayerPawn::m_pItemServices → CCSPlayer_ItemServices.
pub const PAWN_ITEM_SERVICES: usize = 0x12F8;
pub const ITEM_SERVICES_DEFUSER: usize = 0x48;
pub const ITEM_SERVICES_HELMET: usize = 0x49;
// CCSPlayerController::m_pInGameMoneyServices → m_iAccount.
pub const CONTROLLER_MONEY_SERVICES: usize = 0x818;
pub const MONEY_ACCOUNT: usize = 0x40;
// C_BasePlayerWeapon::m_iClip1 and C_EconEntity::m_AttributeManager::m_Item::m_iItemDefinitionIndex.
pub const WEAPON_CLIP1: usize = 0x1928;
pub const WEAPON_DEF_INDEX: usize = 0x1290 + 0x50 + 0x1BA;
// Econ schema descriptors read offline from wb_ida/newclient/client.dll, 26.11.2026.
pub const ECON_ITEM: usize = 0x1290 + 0x50;
pub const ECON_ATTRIBUTES_INITIALIZED: usize = 0x1288;
pub const ECON_OWNER_XUID: usize = 0x18A0;
pub const ECON_FALLBACK_PAINT: usize = 0x18A8;
pub const ECON_FALLBACK_SEED: usize = 0x18AC;
pub const ECON_FALLBACK_WEAR: usize = 0x18B0;
pub const ECON_FALLBACK_STATTRAK: usize = 0x18B4;
pub const ITEM_RESTORE_MATERIAL: usize = 0x1B8;
pub const ITEM_DEFINITION: usize = 0x1BA;
pub const ITEM_QUALITY: usize = 0x1BC;
pub const ITEM_ID_HIGH: usize = 0x1D0;
pub const ITEM_ID_LOW: usize = 0x1D4;
pub const ITEM_INITIALIZED: usize = 0x1E8;
pub const ITEM_ATTRIBUTE_VECTOR: usize = 0x208 + 0x08;
pub const ITEM_CUSTOM_NAME: usize = 0x2F8;
pub const ITEM_INITIALIZED_TAGS: usize = 0x5A8;
pub const PAWN_ECON_GLOVES: usize = 0x17A0;
pub const PAWN_REAPPLY_GLOVES: usize = 0x179D;
pub const PAWN_HUD_ARMS: usize = 0x1DD8;
pub const ENTITY_IDENTITY: usize = 0x10;
pub const IDENTITY_HANDLE: usize = 0x10;
pub const IDENTITY_FLAGS: usize = 0x30;
pub const ENTITY_OWNER_HANDLE: usize = 0x520;
pub const ENTITY_SUBCLASS: usize = 0x380;
pub const SCENE_CHILD: usize = 0x40;
pub const SCENE_SIBLING: usize = 0x48;
pub const SCENE_OWNER: usize = 0x30;
pub const MODEL_STATE_NAME: usize = 0xA8;
pub const MODEL_STATE_MESH_MASK: usize = 0x208;
// C_BaseEntity::m_nSubclassID + 0x8 points at weapon vdata.
pub const WEAPON_VDATA: usize = 0x380 + 0x8;
// CCSWeaponBaseVData::m_iMaxClip1 (the weapon name string lives at +0x720).
pub const VDATA_MAX_CLIP1: usize = 0x4D0;
/// CCSWeaponBaseVData::m_szName — `const char*` ("weapon_ak47").
pub const VDATA_WEAPON_NAME: usize = 0x720;
// CSkeletonInstance::m_modelState 0x140; CModelState::m_pBoneArray 0x80 (IDA).
pub const SCENE_NODE_MODEL_STATE: usize = 0x140;
pub const MODEL_STATE_BONE_ARRAY: usize = 0x80;
// CCSPlayerController::m_sSanitizedPlayerName 0x878 (CUtlString: ptr, len).
pub const CONTROLLER_PLAYER_NAME: usize = 0x878;
pub const CONTROLLER_PING: usize = 0x838;
// C_BasePlayerPawn::m_hController (CHandle<CBasePlayerController>), cs2-dumper 2026-11-24.
pub const PAWN_CONTROLLER_HANDLE: usize = 0x14BC;
pub const ENGINE_BUILD_NUMBER: usize = 0x6B2D1C;
pub const NETCHANNEL_CLIENT: usize = 0x9AFB70;

/// Engine build these offsets and signatures were dumped for (cs2-dumper
/// 2026-11-24). On any other build the module, entity layout and every
/// `dw*` offset are unverified, so the snapshot refuses to read game memory.
pub const EXPECTED_BUILD_NUMBER: u32 = 14201;

/// True when `actual` is the build these offsets were dumped for.
#[inline]
pub const fn build_matches(actual: u32) -> bool {
    actual == EXPECTED_BUILD_NUMBER
}

// ---- CUserCmd (client.dll) ----
// Сверено с рабочим internal'ом (asphyxia-cs2) и проверено сигнатурой CreateMove
// на локальном client.dll: сигнатура уникальна (RVA 0x178C140).
/// CUserCmd::csgoUserCmd.pBaseCmd.
pub const CMD_BASE_CMD: usize = 0x40;
pub const CMD_COMMAND_NUMBER: usize = 0x08;
/// CUserCmd::nButtons (CInButtonState) — vtable в +0x0.
pub const CMD_BUTTONS: usize = 0x58;
/// CInButtonState::nValue.
pub const CMD_BUTTON_VALUE: usize = 0x08;
/// CUserCmd::csgoUserCmd.inputHistoryField.nCurrentSize.
pub const CMD_HISTORY_COUNT: usize = 0x30;
/// CUserCmd::csgoUserCmd.inputHistoryField.pRep (Rep_t: int + T*[]).
pub const CMD_HISTORY_REP: usize = 0x38;
/// CUserCmd::csgoUserCmd.nHasBits.
pub const CMD_HAS_BITS: usize = 0x20;

/// CBaseUserCmdPB::nHasBits.
pub const BASE_HAS_BITS: usize = 0x10;
pub const BASE_SUBTICK_FIELD: usize = 0x18;
pub const REPEATED_COUNT: usize = 0x08;
pub const REPEATED_REP: usize = 0x10;
pub const REPEATED_ELEMENTS: usize = 0x08;
pub const SUBTICK_BUTTON: usize = 0x18;
pub const SUBTICK_PRESSED: usize = 0x20;
pub const SUBTICK_WHEN: usize = 0x24;
pub const SUBTICK_MESSAGE_SIZE: usize = 0x38;
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
pub const PB_HAS_BITS: usize = 0x10;
/// Protobuf cached serialized byte size (u32), not a second has-bits mask.
pub const PB_CACHED_BITS: usize = 0x14;
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

#[cfg(test)]
mod tests {
    use super::{build_matches, EXPECTED_BUILD_NUMBER, is_resolved};

    #[test]
    fn build_guard_accepts_only_the_dumped_build() {
        assert!(build_matches(EXPECTED_BUILD_NUMBER));
        assert!(!build_matches(EXPECTED_BUILD_NUMBER + 1));
        assert!(!build_matches(EXPECTED_BUILD_NUMBER - 1));
        assert!(!build_matches(0));
    }

    #[test]
    fn dw_offsets_are_resolved() {
        assert!(is_resolved());
    }
}
