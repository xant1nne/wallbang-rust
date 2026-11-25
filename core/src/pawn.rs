// Config block: DW_LOCAL_PLAYER_PAWN/LOCAL_CONTROLLER, PAWN_*/schema-офсеты.
use crate::{entity::EntityList, math::Vec3, memory, modules, weapons};
use std::sync::Arc;
use wallbang_config::offsets::{
    COLLISION_MAXS, COLLISION_MINS, CONTROLLER_MONEY_SERVICES, CONTROLLER_PLAYER_NAME,
    DW_LOCAL_PLAYER_CONTROLLER, DW_LOCAL_PLAYER_PAWN, ITEM_SERVICES_DEFUSER, ITEM_SERVICES_HELMET,
    MAX_OWNED_WEAPONS, MONEY_ACCOUNT, NET_VECTOR_DATA, NET_VECTOR_SIZE, PAWN_ABS_ORIGIN,
    PAWN_ABS_VELOCITY, PAWN_ARMOR, PAWN_COLLISION, PAWN_CONTROLLER_HANDLE, PAWN_FLAGS,
    PAWN_HEALTH, PAWN_ITEM_SERVICES, PAWN_LIFE_STATE, PAWN_MOVE_TYPE, PAWN_SCENE_NODE,
    PAWN_SCOPED, PAWN_SPOTTED, PAWN_TEAM, PAWN_VELOCITY, PAWN_VIEW_OFFSET, PAWN_WEAPON_SERVICES,
    VDATA_MAX_CLIP1, VDATA_WEAPON_NAME, WEAPON_C4_DEF_INDEX, WEAPON_CLIP1, WEAPON_DEF_INDEX,
    WEAPON_SERVICE_ACTIVE, WEAPON_SERVICES_WEAPONS, WEAPON_VDATA,
};

/// Font atlas is printable ASCII only, so player names are folded to that
/// alphabet before they enter the snapshot. Anything else becomes '?'.
pub const MAX_NAME_CHARS: usize = 24;
/// Minimal plausible user-mode address; anything below is not dereferenced.
const MIN_USER_ADDRESS: usize = 0x10000;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Pawn {
    pub address: usize,
}

/// Hot snapshot of the fields used by `GameState::refresh()`. Validating one
/// contiguous range once per pawn removes ~10 VirtualQuery syscalls per player.
#[derive(Clone, Copy, Debug, Default)]
pub struct PawnFields {
    pub health: i32,
    pub armor: i32,
    pub team: i32,
    pub life_state: u8,
    pub spotted: bool,
    pub scoped: bool,
    pub velocity: Vec3,
    pub view_offset: Vec3,
    pub origin: Vec3,
    /// World-space AABB from `m_pCollision`; zeroed when the pawn has no
    /// collision property yet.
    pub bounds_min: Vec3,
    pub bounds_max: Vec3,
}

/// Active weapon data used by player ESP. Everything numeric, plus the display
/// text resolved from `CCSWeaponBaseVData::m_szName` (shared `Arc`, so the
/// snapshot clones stay cheap).
#[derive(Clone, Debug, Default)]
pub struct WeaponFields {
    pub def_index: u16,
    pub clip: i32,
    pub max_clip: i32,
    pub name: Option<Arc<str>>,
}

impl Pawn {
    #[inline]
    pub const fn new(address: usize) -> Self {
        Self { address }
    }

    #[inline]
    pub unsafe fn local() -> Option<Self> {
        let client = modules::module_base("client.dll")?;
        let address = memory::read_ptr(client + DW_LOCAL_PLAYER_PAWN)?;
        Some(Self::new(address))
    }

    #[inline]
    pub unsafe fn local_controller() -> Option<usize> {
        let client = modules::module_base("client.dll")?;
        memory::read_ptr(client + DW_LOCAL_PLAYER_CONTROLLER)
    }

    /// Reads every field used by the world snapshot. `EntityList::iterate()`
    /// already validates the chunk pages, and every slot in a live chunk is a
    /// valid pawn pointer, so re-checking each pawn with VirtualQuery cost
    /// ~16 ms per refresh on this machine. Cheap plausibility checks keep the
    /// hot path fault-free on stale slots without the syscall storm.
    ///
    /// The health/team/life-state gate runs FIRST: `refresh()` calls this for
    /// every one of ~500 entity slots and only a handful are players, so the
    /// scattered scene-node/collision reads must not run for the rest (that
    /// ordering was what pushed a refresh to 45 ms in the playtest).
    #[inline]
    pub unsafe fn fields(self) -> Option<PawnFields> {
        if self.address < MIN_USER_ADDRESS {
            return None;
        }
        let health = memory::read_unchecked::<i32>(self.address + PAWN_HEALTH)?;
        if health <= 0 {
            return None;
        }
        let life_state = memory::read_unchecked::<u8>(self.address + PAWN_LIFE_STATE)?;
        if life_state != 0 {
            return None;
        }
        let team = memory::read_unchecked::<u8>(self.address + PAWN_TEAM)? as i32;
        if !(2..=3).contains(&team) {
            return None;
        }
        let node = memory::read_unchecked::<usize>(self.address + PAWN_SCENE_NODE)?;
        let origin = if node >= MIN_USER_ADDRESS {
            let v = memory::read_unchecked::<[f32; 3]>(node + PAWN_ABS_ORIGIN)?;
            Vec3::new(v[0], v[1], v[2])
        } else {
            Vec3::default()
        };
        let vel = memory::read_unchecked::<[f32; 3]>(self.address + PAWN_VELOCITY)?;
        let view = memory::read_unchecked::<[f32; 3]>(self.address + PAWN_VIEW_OFFSET)
            .unwrap_or([0.0, 0.0, 64.0]);
        // AABB из CCollisionProperty: crouch/jump/ragdoll позы уже учтены движком,
        // в отличие от жёсткого origin±72. mins/maxs — в локальных координатах.
        let collision = memory::read_unchecked::<usize>(self.address + PAWN_COLLISION).unwrap_or(0);
        let (bounds_min, bounds_max) = if collision >= MIN_USER_ADDRESS {
            match (
                memory::read_unchecked::<[f32; 3]>(collision + COLLISION_MINS),
                memory::read_unchecked::<[f32; 3]>(collision + COLLISION_MAXS),
            ) {
                (Some(mins), Some(maxs)) => (
                    Vec3::new(origin.x + mins[0], origin.y + mins[1], origin.z + mins[2]),
                    Vec3::new(origin.x + maxs[0], origin.y + maxs[1], origin.z + maxs[2]),
                ),
                _ => (Vec3::default(), Vec3::default()),
            }
        } else {
            (Vec3::default(), Vec3::default())
        };
        Some(PawnFields {
            health,
            armor: memory::read_unchecked::<i32>(self.address + PAWN_ARMOR).unwrap_or(0),
            team,
            life_state,
            spotted: memory::read_unchecked::<u8>(self.address + PAWN_SPOTTED)
                .unwrap_or(0)
                != 0,
            scoped: memory::read_unchecked::<u8>(self.address + PAWN_SCOPED).unwrap_or(0) != 0,
            velocity: Vec3::new(vel[0], vel[1], vel[2]),
            view_offset: Vec3::new(view[0], view[1], view[2]),
            origin,
            bounds_min,
            bounds_max,
        })
    }

    /// Resolves `m_hActiveWeapon` through the entity list and reads the numeric
    /// weapon fields needed by ESP. `with_name` gates the `m_szName` string read
    /// (it costs a validation + a cstr copy, so it only runs when weapon text is
    /// actually enabled). Uses the same low 15 bits as the game's handle decoder.
    #[inline]
    pub unsafe fn weapon(self, with_name: bool) -> Option<WeaponFields> {
        let services = memory::read_ptr(self.address + PAWN_WEAPON_SERVICES)?;
        if services < MIN_USER_ADDRESS {
            return None;
        }
        let handle = memory::read_unchecked::<u32>(services + WEAPON_SERVICE_ACTIVE)?;
        if handle == 0 || handle == u32::MAX {
            return None;
        }
        let weapon = EntityList::slot((handle & 0x7FFF) as usize)?;
        if weapon < MIN_USER_ADDRESS {
            return None;
        }
        // vdata is a raw pointer; every read below it is validated so a weapon
        // that is mid-spawn can never fault the render thread.
        let vdata = memory::read_ptr(weapon + WEAPON_VDATA).unwrap_or(0);
        let max_clip = if vdata >= MIN_USER_ADDRESS {
            memory::read::<i32>(vdata + VDATA_MAX_CLIP1).unwrap_or(0)
        } else {
            0
        };
        // Real weapon name: vdata + m_szName is a `const char*` such as
        // "weapon_ak47"; Celerity normalises + maps it to a display string.
        let name = if with_name && vdata >= MIN_USER_ADDRESS {
            memory::read_ptr(vdata + VDATA_WEAPON_NAME)
                .filter(|&pointer| {
                    pointer >= MIN_USER_ADDRESS && memory::validate_range(pointer, 64)
                })
                .and_then(|pointer| memory::read_cstr_unchecked(pointer, 64))
                .and_then(|raw| weapons::display_for_raw(&raw))
        } else {
            None
        };
        Some(WeaponFields {
            def_index: memory::read_unchecked::<u16>(weapon + WEAPON_DEF_INDEX).unwrap_or(0),
            clip: memory::read_unchecked::<i32>(weapon + WEAPON_CLIP1).unwrap_or(0),
            max_clip,
            name,
        })
    }

    #[inline]
    pub fn health(self) -> Option<i32> {
        unsafe { memory::read_i32(self.address + PAWN_HEALTH) }
    }

    #[inline]
    pub fn team(self) -> Option<i32> {
        unsafe { memory::read_u8(self.address + PAWN_TEAM).map(|v| v as i32) }
    }

    #[inline]
    pub fn flags(self) -> Option<u32> {
        unsafe { memory::read::<u32>(self.address + PAWN_FLAGS) }
    }

    /// `m_MoveType` (MOVETYPE_*). Movement-фичи отключаются на ladder/noclip.
    #[inline]
    pub fn move_type(self) -> Option<u8> {
        unsafe { memory::read_u8(self.address + PAWN_MOVE_TYPE) }
    }

    /// `m_vecAbsVelocity` — скорость в мировых координатах (movement/jumpbug).
    #[inline]
    pub fn abs_velocity(self) -> Option<Vec3> {
        unsafe { memory::read::<[f32; 3]>(self.address + PAWN_ABS_VELOCITY) }
            .map(|v| Vec3::new(v[0], v[1], v[2]))
    }

    #[inline]
    pub fn armor(self) -> i32 {
        unsafe { memory::read_i32(self.address + PAWN_ARMOR).unwrap_or(0) }
    }

    #[inline]
    pub fn life_state(self) -> Option<u8> {
        unsafe { memory::read_u8(self.address + PAWN_LIFE_STATE) }
    }

    #[inline]
    pub fn velocity(self) -> Option<Vec3> {
        unsafe { memory::read::<[f32; 3]>(self.address + PAWN_VELOCITY) }
            .map(|v| Vec3::new(v[0], v[1], v[2]))
    }

    #[inline]
    pub fn eye_position(self) -> Option<Vec3> {
        let origin = self.origin()?;
        let offset = self.view_offset_value().unwrap_or(Vec3::new(0.0, 0.0, 64.0));
        Some(origin.add(offset))
    }

    /// `m_vecViewOffset` (CNetworkViewOffsetVector, xyz) как Vec3.
    #[inline]
    pub fn view_offset_value(self) -> Option<Vec3> {
        unsafe { memory::read::<[f32; 3]>(self.address + PAWN_VIEW_OFFSET) }
            .map(|v| Vec3::new(v[0], v[1], v[2]))
    }

    #[inline]
    pub fn spotted(self) -> bool {
        unsafe { memory::read_u8(self.address + PAWN_SPOTTED).unwrap_or(0) != 0 }
    }

    /// `m_bIsScoped` (камера/FOV используют это каждый кадр).
    #[inline]
    pub fn scoped(self) -> bool {
        unsafe { memory::read_u8(self.address + PAWN_SCOPED).unwrap_or(0) != 0 }
    }

    #[inline]
    pub fn alive(self) -> bool {
        self.health().unwrap_or(0) > 0
    }

    #[inline]
    pub fn origin(self) -> Option<Vec3> {
        let node = unsafe { memory::read_ptr(self.address + PAWN_SCENE_NODE) }?;
        unsafe { memory::read::<[f32; 3]>(node + PAWN_ABS_ORIGIN) }
            .map(|v| Vec3::new(v[0], v[1], v[2]))
    }

    #[inline]
    pub fn is_enemy(self, local_team: i32) -> bool {
        match self.team() {
            Some(team) => team != local_team,
            None => true,
        }
    }

    pub fn player_name(controller: usize) -> Option<String> {
        unsafe {
            if controller == 0 || !memory::validate_range(controller, CONTROLLER_PLAYER_NAME + 8) {
                return None;
            }
            let string = memory::read_unchecked::<usize>(controller + CONTROLLER_PLAYER_NAME)?;
            if string == 0 || !memory::validate_range(string, 128) {
                return None;
            }
            memory::read_cstr_unchecked(string, 128)
        }
    }

    /// Resolves `m_hController` through the entity list. `CHandle` keeps the
    /// entity index in the low 15 bits, exactly like `m_hActiveWeapon`.
    #[inline]
    pub unsafe fn controller(self) -> Option<usize> {
        const MIN_USER_ADDRESS: usize = 0x10000;
        let handle = memory::read_unchecked::<u32>(self.address + PAWN_CONTROLLER_HANDLE)?;
        if handle == 0 || handle == u32::MAX {
            return None;
        }
        let controller = EntityList::slot((handle & 0x7FFF) as usize)?;
        (controller >= MIN_USER_ADDRESS).then_some(controller)
    }

    /// Sanitized `m_sSanitizedPlayerName` for this pawn's controller.
    #[inline]
    pub unsafe fn name(self) -> Option<String> {
        let controller = self.controller()?;
        Self::player_name(controller)
    }

    /// `m_pInGameMoneyServices → m_iAccount` (ESP cash flag).
    #[inline]
    pub unsafe fn money(self) -> Option<i32> {
        let controller = self.controller()?;
        let services = memory::read_ptr(controller + CONTROLLER_MONEY_SERVICES)?;
        if services < MIN_USER_ADDRESS {
            return None;
        }
        memory::read::<i32>(services + MONEY_ACCOUNT)
    }

    /// `(has_helmet, has_defuser)` from `CCSPlayer_ItemServices`.
    #[inline]
    pub unsafe fn item_services(self) -> Option<(bool, bool)> {
        let services = memory::read_ptr(self.address + PAWN_ITEM_SERVICES)?;
        if services < MIN_USER_ADDRESS {
            return None;
        }
        let helmet = memory::read_u8(services + ITEM_SERVICES_HELMET).unwrap_or(0) != 0;
        let defuser = memory::read_u8(services + ITEM_SERVICES_DEFUSER).unwrap_or(0) != 0;
        Some((helmet, defuser))
    }

    /// Walks `m_hMyWeapons` (`C_NetworkUtlVectorBase<CHandle>`) looking for C4.
    #[inline]
    pub unsafe fn has_c4(self) -> bool {
        let Some(services) = memory::read_ptr(self.address + PAWN_WEAPON_SERVICES) else {
            return false;
        };
        if services < MIN_USER_ADDRESS {
            return false;
        }
        let size = memory::read::<u32>(services + WEAPON_SERVICES_WEAPONS + NET_VECTOR_SIZE)
            .unwrap_or(0) as usize;
        if size == 0 || size > MAX_OWNED_WEAPONS {
            return false;
        }
        let Some(data) =
            memory::read_ptr(services + WEAPON_SERVICES_WEAPONS + NET_VECTOR_DATA)
        else {
            return false;
        };
        if data < MIN_USER_ADDRESS || !memory::validate_range(data, size * 4) {
            return false;
        }
        for index in 0..size {
            let handle = memory::read_unchecked::<u32>(data + index * 4).unwrap_or(0);
            if handle == 0 || handle == u32::MAX {
                continue;
            }
            let Some(weapon) = EntityList::slot((handle & 0x7FFF) as usize) else {
                continue;
            };
            if weapon < MIN_USER_ADDRESS {
                continue;
            }
            let def_index =
                memory::read::<u16>(weapon + WEAPON_DEF_INDEX).unwrap_or(0) as i16;
            if def_index == WEAPON_C4_DEF_INDEX {
                return true;
            }
        }
        false
    }
}

/// Folds a raw server-side name into the ASCII alphabet the font atlas can
/// draw: control bytes and non-ASCII run `?`, runs of `?` collapse, the result
/// is trimmed and capped at [`MAX_NAME_CHARS`]. Empty names yield `None`.
pub fn sanitize_player_name(raw: &str) -> Option<String> {
    let mut out = String::with_capacity(MAX_NAME_CHARS);
    let mut previous_question = false;
    for ch in raw.chars() {
        let folded = if ch.is_ascii_graphic() || ch == ' ' {
            ch
        } else {
            '?'
        };
        if folded == '?' {
            if previous_question {
                continue;
            }
            previous_question = true;
        } else {
            previous_question = false;
        }
        if out.len() >= MAX_NAME_CHARS {
            break;
        }
        out.push(folded);
    }
    let trimmed = out.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_owned())
}

#[cfg(test)]
mod tests {
    use super::{sanitize_player_name, MAX_NAME_CHARS};

    #[test]
    fn name_is_folded_to_font_alphabet() {
        assert_eq!(sanitize_player_name("Leek").as_deref(), Some("Leek"));
        assert_eq!(sanitize_player_name("  spaced  ").as_deref(), Some("spaced"));
        assert_eq!(sanitize_player_name("").as_deref(), None);
        assert_eq!(sanitize_player_name("   ").as_deref(), None);
        assert_eq!(sanitize_player_name("\u{0301}\u{0301}").as_deref(), Some("?"));
    }

    #[test]
    fn name_is_capped_and_control_bytes_folded() {
        let long = "A".repeat(80);
        assert_eq!(
            sanitize_player_name(&long).map(|name| name.len()),
            Some(MAX_NAME_CHARS)
        );
        assert_eq!(
            sanitize_player_name("a\nb\tc\u{0}d").as_deref(),
            Some("a?b?c?d")
        );
        assert_eq!(sanitize_player_name("line\n\nbreak").as_deref(), Some("line?break"));
    }
}
