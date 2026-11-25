// Config block: CMD_*/BASE_*/PB_*/HISTORY_* — из config::offsets (asphyxia-verified layout).
// Запись сюда делает только feature-слой и только под MutationGuard.
use crate::math::{QAngle, Vec3};
use crate::memory;
use wallbang_config::offsets::*;

pub const IN_ATTACK: u64 = 1 << 0;
pub const IN_JUMP: u64 = 1 << 1;
pub const IN_DUCK: u64 = 1 << 2;
pub const IN_FORWARD: u64 = 1 << 3;
pub const IN_BACK: u64 = 1 << 4;
pub const IN_USE: u64 = 1 << 5;
pub const IN_LEFT: u64 = 1 << 7;
pub const IN_RIGHT: u64 = 1 << 8;
pub const IN_MOVELEFT: u64 = 1 << 9;
pub const IN_MOVERIGHT: u64 = 1 << 10;
pub const IN_RELOAD: u64 = 1 << 13;

pub const BASE_BITS_BUTTON_PB: u32 = 0x2;
pub const BASE_BITS_VIEW_ANGLES: u32 = 0x4;
pub const BASE_BITS_FORWARD_MOVE: u32 = 0x20;
pub const BASE_BITS_SIDE_MOVE: u32 = 0x40;
pub const BASE_BITS_UP_MOVE: u32 = 0x80;
pub const BASE_BITS_MOUSE_DX: u32 = 0x800;
pub const BASE_BITS_MOUSE_DY: u32 = 0x1000;

pub const HISTORY_BITS_VIEW_ANGLES: u32 = 0x1;

pub const FL_ONGROUND: u32 = 1;

const MAX_HISTORY_ENTRIES: i32 = 64;

/// Помечает поле как «выставленное» и для nHasBits, и для nCachedBits (его читает proto).
#[inline]
fn set_bits(pb: usize, bits: u32) {
    unsafe {
        let has = memory::read::<u32>(pb + PB_HAS_BITS).unwrap_or(0);
        memory::write(pb + PB_HAS_BITS, has | bits);
        let cached = memory::read::<u64>(pb + PB_CACHED_BITS).unwrap_or(0);
        memory::write(pb + PB_CACHED_BITS, cached | bits as u64);
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct UserCmd {
    address: usize,
}

impl UserCmd {
    pub unsafe fn from_raw(address: usize) -> Option<Self> {
        if address == 0 {
            return None;
        }
        let base = memory::read_ptr(address + CMD_BASE_CMD)?;
        if base == 0 {
            return None;
        }
        Some(Self { address })
    }

    #[inline]
    pub fn address(self) -> usize {
        self.address
    }

    #[inline]
    pub fn base(self) -> usize {
        unsafe { memory::read_ptr(self.address + CMD_BASE_CMD).unwrap_or(0) }
    }

    #[inline]
    pub fn view_angles(self) -> Option<QAngle> {
        unsafe {
            let base = self.base();
            if base == 0 {
                return None;
            }
            let angles = memory::read_ptr(base + BASE_VIEW_ANGLES)?;
            let raw = memory::read::<[f32; 3]>(angles + PB_ANGLE_VALUE)?;
            Some(QAngle::new(raw[0], raw[1], raw[2]))
        }
    }

    /// Пишет углы в base->pViewAngles и во все input-history записи (subtick).
    /// Движение корректируется на дельту yaw, иначе forward/side «уводят» игрока.
    pub fn set_view_angles(self, angles: QAngle) -> bool {
        unsafe {
            let base = self.base();
            if base == 0 {
                return false;
            }
            let old = self.view_angles();
            let mut ok = false;
            if let Some(pb) = memory::read_ptr(base + BASE_VIEW_ANGLES) {
                set_bits(pb, BASE_BITS_VIEW_ANGLES);
                ok |= memory::write(pb + PB_ANGLE_VALUE, [angles.pitch, angles.yaw, angles.roll]);
            }
            set_bits(base, BASE_BITS_VIEW_ANGLES);

            let count = memory::read::<i32>(self.address + CMD_HISTORY_COUNT).unwrap_or(0);
            if count > 0 {
                if let Some(rep) = memory::read_ptr(self.address + CMD_HISTORY_REP) {
                    for index in 0..count.min(MAX_HISTORY_ENTRIES) {
                        let Some(entry) = memory::read_ptr(rep + 8 + index as usize * 8) else {
                            continue;
                        };
                        set_bits(entry, HISTORY_BITS_VIEW_ANGLES);
                        if let Some(pb) = memory::read_ptr(entry + HISTORY_VIEW_ANGLES) {
                            ok |= memory::write(
                                pb + PB_ANGLE_VALUE,
                                [angles.pitch, angles.yaw, angles.roll],
                            );
                        }
                    }
                }
            }

            if let Some(old) = old {
                let delta = (angles.yaw - old.yaw).to_radians();
                if delta.abs() > 0.0001 {
                    let (sin, cos) = delta.sin_cos();
                    let forward = self.forward_move();
                    let side = self.side_move();
                    self.set_forward_move(forward * cos + side * sin);
                    self.set_side_move(-forward * sin + side * cos);
                }
            }
            ok
        }
    }

    #[inline]
    pub fn buttons(self) -> u64 {
        unsafe { memory::read::<u64>(self.address + CMD_BUTTONS + CMD_BUTTON_VALUE).unwrap_or(0) }
    }

    /// Синхронизирует CUserCmd::nButtons и protobuf CInButtonStatePB.
    pub fn set_buttons(self, value: u64) -> bool {
        unsafe {
            let old = self.buttons();
            let changed = old ^ value;
            let mut ok = memory::write(self.address + CMD_BUTTONS + CMD_BUTTON_VALUE, value);
            ok |= memory::write(
                self.address + CMD_BUTTONS + CMD_BUTTON_VALUE + 8,
                changed,
            );
            let base = self.base();
            if base != 0 {
                if let Some(pb) = memory::read_ptr(base + BASE_BUTTON_STATE) {
                    set_bits(pb, 0x1);
                    ok |= memory::write(pb + PB_BUTTON_VALUE, value);
                    memory::write(pb + PB_BUTTON_VALUE + 8, changed);
                }
                set_bits(base, BASE_BITS_BUTTON_PB);
            }
            ok
        }
    }

    #[inline]
    pub fn add_buttons(self, bits: u64) -> bool {
        self.set_buttons(self.buttons() | bits)
    }

    #[inline]
    pub fn remove_buttons(self, bits: u64) -> bool {
        self.set_buttons(self.buttons() & !bits)
    }

    /// Reads the protobuf `CInButtonStatePB::nValue` copy of the same buttons.
    #[inline]
    pub fn base_buttons(self) -> u64 {
        unsafe {
            let base = self.base();
            if base == 0 {
                return 0;
            }
            let Some(pb) = memory::read_ptr(base + BASE_BUTTON_STATE) else {
                return 0;
            };
            memory::read::<u64>(pb + PB_BUTTON_VALUE).unwrap_or(0)
        }
    }

    /// Sets or clears a single button bit, no-op when it already matches.
    pub fn set_button(self, bit: u64, down: bool) -> bool {
        let current = self.buttons();
        let next = if down { current | bit } else { current & !bit };
        next == current || self.set_buttons(next)
    }

    #[inline]
    pub fn forward_move(self) -> f32 {
        unsafe { memory::read_f32(self.base() + BASE_FORWARD_MOVE).unwrap_or(0.0) }
    }

    pub fn set_forward_move(self, value: f32) -> bool {
        unsafe {
            let base = self.base();
            if base == 0 {
                return false;
            }
            set_bits(base, BASE_BITS_FORWARD_MOVE);
            memory::write(base + BASE_FORWARD_MOVE, value)
        }
    }

    #[inline]
    pub fn side_move(self) -> f32 {
        unsafe { memory::read_f32(self.base() + BASE_SIDE_MOVE).unwrap_or(0.0) }
    }

    pub fn set_side_move(self, value: f32) -> bool {
        unsafe {
            let base = self.base();
            if base == 0 {
                return false;
            }
            set_bits(base, BASE_BITS_SIDE_MOVE);
            memory::write(base + BASE_SIDE_MOVE, value)
        }
    }

    #[inline]
    pub fn up_move(self) -> f32 {
        unsafe { memory::read_f32(self.base() + BASE_UP_MOVE).unwrap_or(0.0) }
    }

    pub fn set_up_move(self, value: f32) -> bool {
        unsafe {
            let base = self.base();
            if base == 0 {
                return false;
            }
            set_bits(base, BASE_BITS_UP_MOVE);
            memory::write(base + BASE_UP_MOVE, value)
        }
    }

    #[inline]
    pub fn mouse_dx(self) -> i32 {
        unsafe { memory::read_i32(self.base() + BASE_MOUSE_DX).unwrap_or(0) }
    }

    #[inline]
    pub fn mouse_dy(self) -> i32 {
        unsafe { memory::read_i32(self.base() + BASE_MOUSE_DY).unwrap_or(0) }
    }

    pub fn set_mouse_delta(self, dx: i32, dy: i32) -> bool {
        unsafe {
            let base = self.base();
            if base == 0 {
                return false;
            }
            set_bits(base, BASE_BITS_MOUSE_DX | BASE_BITS_MOUSE_DY);
            memory::write(base + BASE_MOUSE_DX, dx) & memory::write(base + BASE_MOUSE_DY, dy)
        }
    }

    #[inline]
    pub fn client_tick(self) -> i32 {
        unsafe { memory::read_i32(self.base() + BASE_CLIENT_TICK).unwrap_or(0) }
    }

    pub fn set_client_tick(self, tick: i32) -> bool {
        unsafe {
            let base = self.base();
            if base == 0 {
                return false;
            }
            set_bits(base, 0x10);
            memory::write(base + BASE_CLIENT_TICK, tick)
        }
    }

    /// Флаги локального пехотинца (m_fFlags).
    pub fn local_flags() -> u32 {
        unsafe {
            crate::pawn::Pawn::local()
                .and_then(|pawn| memory::read::<u32>(pawn.address + PAWN_FLAGS))
                .unwrap_or(0)
        }
    }

    pub fn local_velocity() -> Vec3 {
        unsafe {
            crate::pawn::Pawn::local()
                .and_then(|pawn| memory::read::<[f32; 3]>(pawn.address + PAWN_VELOCITY))
                .map(|v| Vec3::new(v[0], v[1], v[2]))
                .unwrap_or_default()
        }
    }

    pub fn local_eye_position() -> Option<Vec3> {
        let pawn = unsafe { crate::pawn::Pawn::local() }?;
        pawn.eye_position()
    }
}
