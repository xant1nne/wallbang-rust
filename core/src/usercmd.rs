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
pub const BASE_BITS_FORWARD_MOVE: u32 = 0x40;
pub const BASE_BITS_SIDE_MOVE: u32 = 0x80;
pub const BASE_BITS_UP_MOVE: u32 = 0x100;
pub const BASE_BITS_MOUSE_DX: u32 = 0x1000;
pub const BASE_BITS_MOUSE_DY: u32 = 0x2000;

pub const HISTORY_BITS_VIEW_ANGLES: u32 = 0x1;

pub const FL_ONGROUND: u32 = 1;

const MAX_HISTORY_ENTRIES: i32 = 64;
const MAX_SUBTICK_STEPS: i32 = 64;
static SUBTICK_FUNCTIONS: std::sync::OnceLock<Option<(usize, usize)>> = std::sync::OnceLock::new();

/// Resolve once during initialization, not on the first edge-jump tick.
pub fn initialize_subtick() -> bool {
    use wallbang_config::signatures::{SIG_SUBTICK_ALLOC_SITE, SIG_REPEATED_PUSH};
    if crate::modules::build_supported() != Some(true) { return false; }
    SUBTICK_FUNCTIONS.get_or_init(|| {
        let site = crate::pattern::scan_module("client.dll", SIG_SUBTICK_ALLOC_SITE)?;
        let rel = unsafe { memory::read::<i32>(site + 4)? };
        let alloc = (site + 8).wrapping_add_signed(rel as isize);
        Some((alloc, crate::pattern::scan_module("client.dll", SIG_REPEATED_PUSH)?))
    }).is_some()
}

unsafe fn guarded_write<T: Copy>(address: usize, value: T) -> bool {
    crate::mutation::acquire().is_some_and(|guard| guard.write(address, value))
}

/// Protobuf has-bits live at +0x10. +0x14 is a cached BYTE SIZE, not a bitmask.
#[inline]
fn set_bits(pb: usize, bits: u32) {
    unsafe {
        let has = memory::read::<u32>(pb + PB_HAS_BITS).unwrap_or(0);
        guarded_write(pb + PB_HAS_BITS, has | bits);
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

    pub fn command_number(self) -> i64 {
        unsafe { memory::read(self.address + CMD_COMMAND_NUMBER).unwrap_or(0) }
    }

    /// Clear all three button copies and matching queued subtick events (Celerity bhop).
    pub fn clear_button_events(self, bits: u64) {
        let Some(guard) = crate::mutation::acquire() else { return; };
        unsafe {
            for i in 0..3 {
                let address = self.address + CMD_BUTTONS + CMD_BUTTON_VALUE + i * 8;
                if let Some(value) = memory::read::<u64>(address) { guard.write(address, value & !bits); }
            }
            let base = self.base();
            if base == 0 { return; }
            if let Some(pb) = memory::read_ptr(base + BASE_BUTTON_STATE) {
                for i in 0..3 {
                    if let Some(value) = memory::read::<u64>(pb + PB_BUTTON_VALUE + i * 8) {
                        guard.write(pb + PB_BUTTON_VALUE + i * 8, value & !bits);
                    }
                }
                set_bits(pb, 7);
                set_bits(base, BASE_BITS_BUTTON_PB);
            }
            let field = base + BASE_SUBTICK_FIELD;
            let count = memory::read::<i32>(field + REPEATED_COUNT).unwrap_or(0);
            if !(0..=MAX_SUBTICK_STEPS).contains(&count) { return; }
            let Some(rep) = memory::read_ptr(field + REPEATED_REP) else { return; };
            for i in 0..count as usize {
                let Some(step) = memory::read_ptr(rep + REPEATED_ELEMENTS + i * 8) else { continue; };
                if let Some(button) = memory::read::<u64>(step + SUBTICK_BUTTON) {
                    if button & bits != 0 {
                        guard.write(step + SUBTICK_BUTTON, button & !bits);
                        guard.write(step + SUBTICK_PRESSED, false);
                        set_bits(step, 3);
                    }
                }
            }
        }
    }

    /// Allocate via the engine's protobuf arena, never the Rust allocator.
    pub fn push_subtick_button(self, button: u64, when: f32, pressed: bool) -> bool {
        let Some(guard) = crate::mutation::acquire() else { return false; };
        if crate::modules::build_supported() != Some(true) || !when.is_finite() { return false; }
        if !initialize_subtick() { return false; }
        let Some((alloc, push)) = SUBTICK_FUNCTIONS.get().copied().flatten() else { return false; };
        unsafe {
            let base = self.base();
            if base == 0 { return false; }
            let field = base + BASE_SUBTICK_FIELD;
            let count = memory::read::<i32>(field + REPEATED_COUNT).unwrap_or(-1);
            if !(0..MAX_SUBTICK_STEPS).contains(&count) { return false; }
            let rep = memory::read_ptr(field + REPEATED_REP).unwrap_or(0);
            let capacity = memory::read::<i32>(rep).unwrap_or(0);
            let step = if rep != 0 && count < capacity {
                let Some(step) = memory::read_ptr(rep + REPEATED_ELEMENTS + count as usize * 8) else { return false; };
                if !memory::validate_range(step, SUBTICK_MESSAGE_SIZE) { return false; }
                guard.write(field + REPEATED_COUNT, count + 1);
                step
            } else {
                let arena = memory::read_ptr(field).unwrap_or(0);
                if arena == 0 { return false; }
                let allocate: unsafe extern "system" fn(usize) -> usize = std::mem::transmute(alloc);
                let append: unsafe extern "system" fn(usize, usize) -> usize = std::mem::transmute(push);
                let step = allocate(arena);
                if step == 0 { return false; }
                append(field, step)
            };
            if !memory::validate_range(step, SUBTICK_MESSAGE_SIZE) { return false; }
            // Preserve the protobuf header/arena, clear the reused payload and flags.
            guard.write(step + SUBTICK_BUTTON, [0u8; 32]);
            guard.write(step + PB_HAS_BITS, 7u32);
            guard.write(step + SUBTICK_BUTTON, button);
            guard.write(step + SUBTICK_PRESSED, pressed);
            guard.write(step + SUBTICK_WHEN, when.clamp(0.0, 1.0))
        }
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
                set_bits(pb, 7);
                ok |= guarded_write(pb + PB_ANGLE_VALUE, [angles.pitch, angles.yaw, angles.roll]);
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
                            set_bits(pb, 7);
                            ok |= guarded_write(
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
            let mut ok = guarded_write(self.address + CMD_BUTTONS + CMD_BUTTON_VALUE, value);
            ok |= guarded_write(
                self.address + CMD_BUTTONS + CMD_BUTTON_VALUE + 8,
                changed,
            );
            let base = self.base();
            if base != 0 {
                if let Some(pb) = memory::read_ptr(base + BASE_BUTTON_STATE) {
                    set_bits(pb, 0x1);
                    ok |= guarded_write(pb + PB_BUTTON_VALUE, value);
                    guarded_write(pb + PB_BUTTON_VALUE + 8, changed);
                }
                set_bits(base, BASE_BITS_BUTTON_PB);
            }
            ok
        }
    }

    #[inline]
    pub fn add_buttons(self, bits: u64) -> bool {
        self.set_button(bits, true)
    }

    #[inline]
    pub fn remove_buttons(self, bits: u64) -> bool {
        self.set_button(bits, false)
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

    /// Celerity IN_BUTTON_DOWN / IN_BUTTON_UP: clear transition words even if
    /// the primary word already matches, and synchronize the protobuf copy.
    pub fn set_button(self, bit: u64, down: bool) -> bool {
        let Some(guard) = crate::mutation::acquire() else { return false; };
        unsafe {
            let mut ok = true;
            let base = self.base();
            let pb = memory::read_ptr(base + BASE_BUTTON_STATE).unwrap_or(0);
            for address in [self.address + CMD_BUTTONS + CMD_BUTTON_VALUE, pb + PB_BUTTON_VALUE] {
                if address == PB_BUTTON_VALUE { continue; }
                for index in 0..3 {
                    let old = memory::read::<u64>(address + index * 8).unwrap_or(0);
                    let value = if index == 0 && down { old | bit } else { old & !bit };
                    ok &= guard.write(address + index * 8, value);
                }
            }
            if pb != 0 { set_bits(pb, 7); }
            if base != 0 { set_bits(base, BASE_BITS_BUTTON_PB); }
            ok
        }
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
            guarded_write(base + BASE_FORWARD_MOVE, value)
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
            guarded_write(base + BASE_SIDE_MOVE, value)
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
            guarded_write(base + BASE_UP_MOVE, value)
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
            guarded_write(base + BASE_MOUSE_DX, dx) & guarded_write(base + BASE_MOUSE_DY, dy)
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
            set_bits(base, 0x20);
            guarded_write(base + BASE_CLIENT_TICK, tick)
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn movement_writes_preserve_protobuf_header_and_clear_subtick_jump() {
        let mut command = [0u64; 32];
        let mut base = [0u64; 32];
        let mut buttons = [0u64; 16];
        let mut step = [0u64; 8];
        let mut rep = [1u64, step.as_mut_ptr() as u64];
        let command_address = command.as_mut_ptr() as usize;
        let base_address = base.as_mut_ptr() as usize;
        let buttons_address = buttons.as_mut_ptr() as usize;
        let step_address = step.as_mut_ptr() as usize;
        unsafe {
            memory::write(command_address + CMD_BASE_CMD, base_address);
            memory::write(base_address + BASE_BUTTON_STATE, buttons_address);
            memory::write(base_address + 8, 0x123456789ABCDEF0u64);
            memory::write(base_address + PB_CACHED_BITS, 123u32);
            memory::write(base_address + BASE_SUBTICK_FIELD + REPEATED_COUNT, 1i32);
            memory::write(base_address + BASE_SUBTICK_FIELD + REPEATED_REP, rep.as_mut_ptr() as usize);
            for i in 0..3 {
                memory::write(command_address + CMD_BUTTONS + CMD_BUTTON_VALUE + i * 8, IN_JUMP | IN_DUCK);
                memory::write(buttons_address + PB_BUTTON_VALUE + i * 8, IN_JUMP | IN_DUCK);
            }
            memory::write(step_address + SUBTICK_BUTTON, IN_JUMP);
            memory::write(step_address + SUBTICK_PRESSED, true);
            let cmd = UserCmd::from_raw(command_address).unwrap();
            crate::mutation::set_allowed(false);
            assert!(!cmd.set_forward_move(0.75));
            assert_eq!(cmd.forward_move(), 0.0);
            crate::mutation::set_allowed(true);
            struct Reset;
            impl Drop for Reset { fn drop(&mut self) { crate::mutation::set_allowed(false); } }
            let _reset = Reset;
            assert!(cmd.set_forward_move(0.75));
            assert_eq!(memory::read::<u64>(base_address + 8), Some(0x123456789ABCDEF0));
            assert_eq!(memory::read::<u32>(base_address + PB_CACHED_BITS), Some(123));
            assert_eq!(memory::read::<u32>(base_address + PB_HAS_BITS).unwrap() & BASE_BITS_FORWARD_MOVE, BASE_BITS_FORWARD_MOVE);
            cmd.clear_button_events(IN_JUMP);
            for i in 0..3 {
                assert_eq!(memory::read::<u64>(command_address + CMD_BUTTONS + CMD_BUTTON_VALUE + i * 8), Some(IN_DUCK));
                assert_eq!(memory::read::<u64>(buttons_address + PB_BUTTON_VALUE + i * 8), Some(IN_DUCK));
            }
            assert_eq!(memory::read::<u64>(step_address + SUBTICK_BUTTON), Some(0));
            assert_eq!(memory::read::<bool>(step_address + SUBTICK_PRESSED), Some(false));
            assert!(cmd.set_button(IN_DUCK, true));
            assert_eq!(memory::read::<u64>(buttons_address + PB_BUTTON_VALUE + 8), Some(0));
            assert_eq!(memory::read::<u64>(buttons_address + PB_BUTTON_VALUE + 16), Some(0));
        }
    }
}
