// Config block: DW_LOCAL_PLAYER_PAWN/LOCAL_CONTROLLER, PAWN_*/schema-офсеты.
use crate::{entity::EntityList, math::Vec3, memory, modules};
use wallbang_config::offsets::{
    CONTROLLER_PLAYER_NAME, DW_LOCAL_PLAYER_CONTROLLER, DW_LOCAL_PLAYER_PAWN, PAWN_ABS_ORIGIN,
    PAWN_ARMOR, PAWN_CONTROLLER_HANDLE, PAWN_FLAGS, PAWN_HEALTH, PAWN_LIFE_STATE, PAWN_SCENE_NODE,
    PAWN_SPOTTED, PAWN_TEAM, PAWN_VELOCITY, PAWN_VIEW_OFFSET, PAWN_WEAPON_SERVICES, VDATA_MAX_CLIP1,
    WEAPON_CLIP1, WEAPON_DEF_INDEX, WEAPON_SERVICE_ACTIVE, WEAPON_VDATA,
};

/// Font atlas is printable ASCII only, so player names are folded to that
/// alphabet before they enter the snapshot. Anything else becomes '?'.
pub const MAX_NAME_CHARS: usize = 24;

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
    pub velocity: Vec3,
    pub view_offset: Vec3,
    pub origin: Vec3,
}

/// Active weapon data used by player ESP. Everything is numeric so the hot
/// snapshot never allocates or reads weapon-name strings.
#[derive(Clone, Copy, Debug, Default)]
pub struct WeaponFields {
    pub def_index: u16,
    pub clip: i32,
    pub max_clip: i32,
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
    #[inline]
    pub unsafe fn fields(self) -> Option<PawnFields> {
        const MIN_USER_ADDRESS: usize = 0x10000;
        if self.address < MIN_USER_ADDRESS {
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
        Some(PawnFields {
            health: memory::read_unchecked::<i32>(self.address + PAWN_HEALTH)?,
            armor: memory::read_unchecked::<i32>(self.address + PAWN_ARMOR).unwrap_or(0),
            team: memory::read_unchecked::<u8>(self.address + PAWN_TEAM)? as i32,
            life_state: memory::read_unchecked::<u8>(self.address + PAWN_LIFE_STATE)?,
            spotted: memory::read_unchecked::<u8>(self.address + PAWN_SPOTTED)
                .unwrap_or(0)
                != 0,
            velocity: Vec3::new(vel[0], vel[1], vel[2]),
            view_offset: Vec3::new(view[0], view[1], view[2]),
            origin,
        })
    }

    /// Resolves `m_hActiveWeapon` through the entity list and reads the numeric
    /// weapon fields needed by ESP. Uses the same low 15 bits as the game's
    /// handle decoder; a zero/invalid handle simply leaves the snapshot empty.
    #[inline]
    pub unsafe fn weapon(self) -> Option<WeaponFields> {
        const MIN_USER_ADDRESS: usize = 0x10000;
        let services = memory::read_unchecked::<usize>(self.address + PAWN_WEAPON_SERVICES)?;
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
        let vdata = memory::read_unchecked::<usize>(weapon + WEAPON_VDATA)?;
        let max_clip = if vdata >= MIN_USER_ADDRESS {
            memory::read_unchecked::<i32>(vdata + VDATA_MAX_CLIP1).unwrap_or(0)
        } else {
            0
        };
        Some(WeaponFields {
            def_index: memory::read_unchecked::<u16>(weapon + WEAPON_DEF_INDEX).unwrap_or(0),
            clip: memory::read_unchecked::<i32>(weapon + WEAPON_CLIP1).unwrap_or(0),
            max_clip,
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
        let offset = unsafe { memory::read::<[f32; 3]>(self.address + PAWN_VIEW_OFFSET) }
            .unwrap_or([0.0, 0.0, 64.0]);
        Some(Vec3::new(
            origin.x + offset[0],
            origin.y + offset[1],
            origin.z + offset[2],
        ))
    }

    #[inline]
    pub fn spotted(self) -> bool {
        unsafe { memory::read_u8(self.address + PAWN_SPOTTED).unwrap_or(0) != 0 }
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
