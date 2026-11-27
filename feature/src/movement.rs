// Config block: 1:1 порт Celerity `src/features/movement/movement.cpp`.
// INTERVAL_PER_TICK / STANDABLE_NORMAL / edge-look clamp / trace distance — те же
// значения, что в оригинале. Вся запись в команду идёт только под MutationGuard
// (вызывается из `combat::cmd::on_create_move` после проверки allow_writes).
//
// Keybinds: пустой bind = always-on; hold/toggle доступны в popup настроек.
use core::sync::atomic::{AtomicU64, Ordering};
use parking_lot::Mutex;

#[repr(C)]
#[derive(Clone, Copy)]
struct KeybdInput {
    w_vk: u16,
    w_scan: u16,
    dw_flags: u32,
    time: u32,
    dw_extra_info: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
union InputUnion {
    ki: KeybdInput,
    _pad: [u8; 32],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Input {
    r#type: u32,
    u: InputUnion,
}

#[link(name = "kernel32")]
extern "system" {
    fn GetTickCount64() -> u64;
    fn GetCurrentProcessId() -> u32;
}

#[link(name = "user32")]
extern "system" {
    fn GetForegroundWindow() -> *mut core::ffi::c_void;
    fn GetWindowThreadProcessId(hwnd: *mut core::ffi::c_void, pid: *mut u32) -> u32;
    fn SendInput(c_inputs: u32, p_inputs: *const Input, cb_size: i32) -> u32;
}
use wallbang_config::offsets::{CSGO_INPUT_FORWARD_MOVE, CSGO_INPUT_LEFT_MOVE, CSGO_INPUT_MOUSE_DELTA_X};
use wallbang_core::math::Vec3;
use wallbang_core::pawn::Pawn;
use wallbang_core::state::GameState;
use wallbang_core::trace;
use wallbang_core::usercmd::{
    UserCmd, FL_ONGROUND, IN_DUCK, IN_JUMP, IN_MOVELEFT, IN_MOVERIGHT,
};
use wallbang_core::{memory, modules};
use wallbang_core::mutation::{self, MutationGuard};

const INTERVAL_PER_TICK: f32 = 0.015625;
const STANDABLE_NORMAL: f32 = 0.7;
const MOVETYPE_NOCLIP: u8 = 7;
const MOVETYPE_LADDER: u8 = 9;
const EDGE_MIN_HORIZ_SPEED: f32 = 5.0;
const EDGE_LOOK_MIN: f32 = 8.0;
const EDGE_LOOK_MAX: f32 = 24.0;
const EDGE_TRACE_DIST: f32 = 128.0;
/// Точность трассы, при которой считаем, что впереди/снизу пусто.
const EDGE_CLEAR_FRACTION: f32 = 0.999;

#[derive(Default)]
struct MovementState {
    /// c_jumpbug::m_ducked
    jumpbug_ducked: bool,
    bind_was_down: [bool; 3],
    bind_toggled: [bool; 3],
    bind_config: [(u16, usize); 3],
}

static STATE: Mutex<MovementState> = Mutex::new(MovementState {
    jumpbug_ducked: false,
    bind_was_down: [false; 3],
    bind_toggled: [false; 3],
    bind_config: [(0, 0); 3],
});
impl MovementState {
    fn bind(&mut self, index: usize, cfg: wallbang_config::settings::MovementBind, enabled: bool) -> bool {
        if !enabled || self.bind_config[index] != (cfg.key, cfg.mode) {
            self.bind_was_down[index] = false;
            self.bind_toggled[index] = false;
            self.bind_config[index] = (cfg.key, cfg.mode);
        }
        if !enabled { return false; }
        let down = cfg.key != 0 && cfg.key <= 0xFF && wallbang_core::input::key_down(cfg.key);
        binding_active(cfg, down, &mut self.bind_was_down[index], &mut self.bind_toggled[index])
    }
}

fn binding_active(cfg: wallbang_config::settings::MovementBind, down: bool, was_down: &mut bool, toggled: &mut bool) -> bool {
    if cfg.key == 0 { return true; }
    if cfg.mode == 2 && down && !*was_down { *toggled = !*toggled; }
    *was_down = down;
    match cfg.mode { 0 => true, 2 => *toggled, _ => down }
}

/// `movement::local_player()`: живой пехотинец не на ladder/noclip.
fn local_player() -> Option<Pawn> {
    let pawn = unsafe { Pawn::local() }?;
    if pawn.health().unwrap_or(0) <= 0 {
        return None;
    }
    match pawn.move_type() {
        Some(MOVETYPE_LADDER) | Some(MOVETYPE_NOCLIP) => None,
        _ => Some(pawn),
    }
}

#[inline]
fn on_ground(pawn: Pawn) -> bool {
    pawn.flags().map(|flags| flags & FL_ONGROUND != 0).unwrap_or(false)
}

/// c_bhop::run: в воздухе снимает IN_JUMP, чтобы следующий прыжок сработал с
/// земли (иначе jump-bug'ится при удержании пробела).
fn run_bhop(cmd: UserCmd, bhop: bool) {
    if !bhop {
        return;
    }
    let Some(pawn) = local_player() else {
        return;
    };
    if on_ground(pawn) {
        return;
    }
    let held = cmd.buttons() | cmd.base_buttons();
    if held & IN_JUMP == 0 {
        return;
    }
    cmd.clear_button_events(IN_JUMP);
}

/// c_autostrafe::run: в воздухе с зажатым прыжком подставляет side-move и
/// соответствующий MOVE-бит. Направление берётся из mouse delta, при нуле —
/// чередуется по чётности тика (в оригинале — m_command_number).
fn run_autostrafe(cmd: UserCmd, autostrafe: bool, guard: &MutationGuard) {
    if !autostrafe {
        return;
    }
    let Some(pawn) = local_player() else {
        return;
    };
    let held = cmd.buttons() | cmd.base_buttons();
    if held & IN_JUMP == 0 || on_ground(pawn) {
        return;
    }
    if cmd.base() == 0 {
        return;
    }
    let mut mouse_delta = cmd.mouse_dx();
    let csgo_input = modules::csgo_input().unwrap_or(0);
    if mouse_delta == 0 && csgo_input != 0 {
        mouse_delta =
            unsafe { memory::read::<i32>(csgo_input + CSGO_INPUT_MOUSE_DELTA_X) }.unwrap_or(0);
    }
    let side = strafe_side(mouse_delta, cmd.command_number());
    cmd.set_forward_move(0.0);
    cmd.set_side_move(side);
    // Celerity пишет те же значения в i_csgo_input, иначе движок их перезапишет.
    if csgo_input != 0 {
        unsafe {
            guard.write(csgo_input + CSGO_INPUT_FORWARD_MOVE, 0.0f32);
            guard.write(csgo_input + CSGO_INPUT_LEFT_MOVE, side);
        }
    }
    cmd.set_button(IN_MOVELEFT, false);
    cmd.set_button(IN_MOVERIGHT, false);
    if side > 0.0 {
        cmd.set_button(IN_MOVELEFT, true);
    } else if side < 0.0 {
        cmd.set_button(IN_MOVERIGHT, true);
    }
}

/// c_autostrafe::run direction rule, isolated for the unit test.
#[inline]
fn strafe_side(mouse_delta: i32, command_number: i64) -> f32 {
    if mouse_delta > 1 {
        -1.0
    } else if mouse_delta < -1 {
        1.0
    } else if command_number % 2 == 0 {
        1.0
    } else {
        -1.0
    }
}

/// c_mini_jump::run: на земле с зажатым прыжком добавляет IN_DUCK.
fn run_mini_jump(cmd: UserCmd, mini_jump: bool) {
    if !mini_jump {
        return;
    }
    let Some(pawn) = local_player() else {
        return;
    };
    if !on_ground(pawn) {
        return;
    }
    let held = cmd.buttons() | cmd.base_buttons();
    if held & IN_JUMP == 0 {
        return;
    }
    cmd.add_buttons(IN_DUCK);
}

/// c_edge_jump::run: на земле, при движении к краю (трасса вперёд-вниз пустая)
/// ставит subtick jump step `when = 0` через игровой protobuf arena.
fn run_edge_jump(cmd: UserCmd, edge_jump: bool) {
    if !edge_jump || !trace::available() {
        return;
    }
    let Some(pawn) = local_player() else {
        return;
    };
    if !on_ground(pawn) {
        return;
    }
    let Some(velocity) = pawn.abs_velocity() else {
        return;
    };
    let speed = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt();
    if speed < EDGE_MIN_HORIZ_SPEED {
        return;
    }
    let Some(origin) = pawn.origin() else {
        return;
    };
    let view_offset = pawn
        .view_offset_value()
        .unwrap_or(Vec3::new(0.0, 0.0, 64.0));
    let eye = origin.add(view_offset);
    let forward = Vec3::new(velocity.x, velocity.y, 0.0).normalized();
    let look = (speed * (1.0 / 64.0) * 2.0).clamp(EDGE_LOOK_MIN, EDGE_LOOK_MAX);
    let ahead = eye.add(forward.scale(look));
    let fraction = trace::trace_fraction(
        ahead,
        Vec3::new(0.0, 0.0, -EDGE_TRACE_DIST),
        pawn.address,
        trace::MASK_SOLID,
    );
    if fraction >= EDGE_CLEAR_FRACTION {
        cmd.push_subtick_button(IN_JUMP, 0.0, true);
    }
}

/// c_jumpbug::run: за тик до приземления ставит IN_DUCK, на земле — IN_JUMP.
fn run_jumpbug(cmd: UserCmd, jumpbug: bool) {
    let mut state = STATE.lock();
    if !jumpbug {
        state.jumpbug_ducked = false;
        return;
    }
    let Some(pawn) = local_player() else {
        state.jumpbug_ducked = false;
        return;
    };
    if on_ground(pawn) {
        if state.jumpbug_ducked {
            cmd.add_buttons(IN_JUMP);
            state.jumpbug_ducked = false;
        }
        return;
    }
    if state.jumpbug_ducked || !trace::available() {
        return;
    }
    let (Some(origin), Some(velocity)) = (pawn.origin(), pawn.abs_velocity()) else {
        return;
    };
    if velocity.z >= 0.0 {
        return;
    }
    let mut end = origin.add(velocity.scale(INTERVAL_PER_TICK));
    end.z -= 2.0;
    let result = trace::trace(
        origin,
        end.sub(origin),
        pawn.address,
        trace::MASK_SOLID,
    );
    if result.fraction >= 1.0 {
        return;
    }
    if result.normal.z < STANDABLE_NORMAL {
        return;
    }
    cmd.add_buttons(IN_DUCK);
    state.jumpbug_ducked = true;
}

static LAST_CREATE_MOVE_MS: AtomicU64 = AtomicU64::new(0);
static LAST_FALLBACK_TAP_MS: AtomicU64 = AtomicU64::new(0);
static FALLBACK_WAS_ON_GROUND: AtomicU64 = AtomicU64::new(0);

#[derive(Default)]
struct FallbackKeys {
    strafe_vk: u16,
    jb_duck_held: bool,
    last_mouse_x: f32,
    has_mouse_x: bool,
}

static FALLBACK_KEYS: Mutex<FallbackKeys> = Mutex::new(FallbackKeys {
    strafe_vk: 0,
    jb_duck_held: false,
    last_mouse_x: 0.0,
    has_mouse_x: false,
});

fn send_key(vk: u16, scan: u16, down: bool) {
    const INPUT_KEYBOARD: u32 = 1;
    const KEYEVENTF_KEYUP: u32 = 0x0002;
    let input = Input {
        r#type: INPUT_KEYBOARD,
        u: InputUnion {
            ki: KeybdInput {
                w_vk: vk,
                w_scan: scan,
                dw_flags: if down { 0 } else { KEYEVENTF_KEYUP },
                time: 0,
                dw_extra_info: 0,
            },
        },
    };
    unsafe {
        let _ = SendInput(1, &input, core::mem::size_of::<Input>() as i32);
    }
}

fn release_fallback_keys(fk: &mut FallbackKeys) {
    if fk.strafe_vk == 0x41 {
        send_key(0x41, 0x1E, false);
    } else if fk.strafe_vk == 0x44 {
        send_key(0x44, 0x20, false);
    }
    fk.strafe_vk = 0;
    if fk.jb_duck_held {
        send_key(0x11, 0x1D, false);
        fk.jb_duck_held = false;
    }
    fk.has_mouse_x = false;
}

fn game_has_input_focus() -> bool {
    unsafe {
        let fg = GetForegroundWindow();
        if fg.is_null() {
            return false;
        }
        let mut pid = 0u32;
        GetWindowThreadProcessId(fg, &mut pid);
        pid != 0 && pid == GetCurrentProcessId()
    }
}

fn send_jump_tap() {
    const INPUT_KEYBOARD: u32 = 1;
    const KEYEVENTF_KEYUP: u32 = 0x0002;
    let inputs = [
        Input {
            r#type: INPUT_KEYBOARD,
            u: InputUnion {
                ki: KeybdInput {
                    w_vk: 0x20,
                    w_scan: 0x39,
                    dw_flags: 0,
                    time: 0,
                    dw_extra_info: 0,
                },
            },
        },
        Input {
            r#type: INPUT_KEYBOARD,
            u: InputUnion {
                ki: KeybdInput {
                    w_vk: 0x20,
                    w_scan: 0x39,
                    dw_flags: KEYEVENTF_KEYUP,
                    time: 0,
                    dw_extra_info: 0,
                },
            },
        },
    ];
    unsafe {
        let _ = SendInput(inputs.len() as u32, inputs.as_ptr(), core::mem::size_of::<Input>() as i32);
    }
}

/// Neverlose-style fallback bhop + autostrafe + edgejump + jumpbug when CreateMove hook is inactive.
pub fn tick_fallback(state: &GameState) {
    let now = unsafe { GetTickCount64() };
    let last_cm = LAST_CREATE_MOVE_MS.load(Ordering::Relaxed);
    if last_cm != 0 && now.saturating_sub(last_cm) < 120 {
        release_fallback_keys(&mut FALLBACK_KEYS.lock());
        return;
    }
    let settings = state.settings().clone();
    let misc = &settings.misc;
    let any_enabled = misc.bhop || misc.autostrafe || misc.movement.edgejump || misc.movement.jumpbug;
    if settings.menu.open || !any_enabled || !game_has_input_focus() {
        FALLBACK_WAS_ON_GROUND.store(0, Ordering::Relaxed);
        release_fallback_keys(&mut FALLBACK_KEYS.lock());
        return;
    }
    let Some(pawn) = local_player() else {
        FALLBACK_WAS_ON_GROUND.store(0, Ordering::Relaxed);
        release_fallback_keys(&mut FALLBACK_KEYS.lock());
        return;
    };
    let grounded = on_ground(pawn);
    let was_grounded = FALLBACK_WAS_ON_GROUND.swap(grounded as u64, Ordering::Relaxed) != 0;

    let (bhop, edgejump, jumpbug) = {
        let mut runtime = STATE.lock();
        (
            runtime.bind(0, misc.movement.bhop_bind, misc.bhop),
            runtime.bind(1, misc.movement.edgejump_bind, misc.movement.edgejump),
            runtime.bind(2, misc.movement.jumpbug_bind, misc.movement.jumpbug),
        )
    };
    wallbang_core::state::set_movement_activity(bhop, edgejump, jumpbug);

    let space_held = wallbang_core::input::key_down(0x20);
    let mut fk = FALLBACK_KEYS.lock();

    // 1. Autostrafe fallback (in air with Space held, tracks mouse X delta)
    if misc.autostrafe && !grounded && space_held {
        let cur_x = wallbang_core::input::cursor_position().map(|p| p.x as f32).unwrap_or(fk.last_mouse_x);
        let dx = if fk.has_mouse_x { cur_x - fk.last_mouse_x } else { 0.0 };
        fk.last_mouse_x = cur_x;
        fk.has_mouse_x = true;
        let want_vk = if dx > 0.5 {
            0x44 // D
        } else if dx < -0.5 {
            0x41 // A
        } else {
            fk.strafe_vk
        };
        if want_vk != fk.strafe_vk {
            if fk.strafe_vk == 0x41 {
                send_key(0x41, 0x1E, false);
            } else if fk.strafe_vk == 0x44 {
                send_key(0x44, 0x20, false);
            }
            if want_vk == 0x41 {
                send_key(0x41, 0x1E, true);
            } else if want_vk == 0x44 {
                send_key(0x44, 0x20, true);
            }
            fk.strafe_vk = want_vk;
        }
    } else if fk.strafe_vk != 0 {
        if fk.strafe_vk == 0x41 {
            send_key(0x41, 0x1E, false);
        } else if fk.strafe_vk == 0x44 {
            send_key(0x44, 0x20, false);
        }
        fk.strafe_vk = 0;
        fk.has_mouse_x = false;
    }

    // 2. Jumpbug fallback (duck while falling, uncrouch + jump tap on landing)
    if jumpbug {
        let vel_z = pawn.velocity().map(|v| v.z).unwrap_or(0.0);
        if !grounded && vel_z < -15.0 {
            if !fk.jb_duck_held {
                send_key(0x11, 0x1D, true);
                fk.jb_duck_held = true;
            }
        } else if grounded && fk.jb_duck_held {
            send_key(0x11, 0x1D, false);
            fk.jb_duck_held = false;
            send_jump_tap();
            LAST_FALLBACK_TAP_MS.store(now, Ordering::Relaxed);
            return;
        }
    } else if fk.jb_duck_held {
        send_key(0x11, 0x1D, false);
        fk.jb_duck_held = false;
    }
    drop(fk);

    // 3. Bhop fallback
    if bhop && grounded && space_held {
        let last_tap = LAST_FALLBACK_TAP_MS.load(Ordering::Relaxed);
        if now.saturating_sub(last_tap) >= 16 {
            send_jump_tap();
            LAST_FALLBACK_TAP_MS.store(now, Ordering::Relaxed);
        }
        return;
    }

    // 4. Edgejump fallback
    if edgejump && was_grounded && !grounded {
        let last_tap = LAST_FALLBACK_TAP_MS.load(Ordering::Relaxed);
        if now.saturating_sub(last_tap) >= 32 {
            send_jump_tap();
            LAST_FALLBACK_TAP_MS.store(now, Ordering::Relaxed);
        }
    }
}

/// Порядок вызовов как в Celerity dispatcher: autostrafe → bhop → mini jump →
/// edge jump → jumpbug.
pub fn on_create_move(cmd: UserCmd, state: &GameState) {
    LAST_CREATE_MOVE_MS.store(unsafe { GetTickCount64() }, Ordering::Relaxed);
    let settings = state.settings().clone();
    let misc = &settings.misc;
    if !misc.allow_writes || !mutation::is_allowed() || modules::build_supported() != Some(true) {
        *STATE.lock() = MovementState::default();
        return;
    }
    let Some(guard) = mutation::acquire() else { return; };
    let (bhop, edgejump, jumpbug) = {
        let mut runtime = STATE.lock();
        (runtime.bind(0, misc.movement.bhop_bind, misc.bhop),
         runtime.bind(1, misc.movement.edgejump_bind, misc.movement.edgejump),
         runtime.bind(2, misc.movement.jumpbug_bind, misc.movement.jumpbug))
    };
    wallbang_core::state::set_movement_activity(bhop, edgejump, jumpbug);
    // Autostrafe must see the original held jump before bhop clears it.
    run_autostrafe(cmd, misc.autostrafe, &guard);
    run_bhop(cmd, bhop);
    run_mini_jump(cmd, misc.movement.mini_jump);
    run_edge_jump(cmd, edgejump);
    run_jumpbug(cmd, jumpbug);
}

#[cfg(test)]
mod tests {
    use super::strafe_side;

    #[test]
    fn toggle_binding_changes_only_on_rising_edges() {
        use wallbang_config::settings::MovementBind;
        let mut previous = false;
        let mut toggled = false;
        let cfg = MovementBind { key: 0x56, mode: 2 };
        for (down, expected) in [(false, false), (true, true), (true, true), (false, true), (true, false)] {
            assert_eq!(super::binding_active(cfg, down, &mut previous, &mut toggled), expected);
        }
        assert!(super::binding_active(MovementBind::default(), false, &mut previous, &mut toggled));
    }

    #[test]
    fn strafe_side_matches_celerity_rules() {
        // Mouse moving right/left overrides the parity fallback.
        assert_eq!(strafe_side(2, 0), -1.0);
        assert_eq!(strafe_side(-2, 0), 1.0);
        // Inside the dead zone the direction alternates per tick.
        assert_eq!(strafe_side(0, 0), 1.0);
        assert_eq!(strafe_side(0, 1), -1.0);
        assert_eq!(strafe_side(1, 2), 1.0);
        assert_eq!(strafe_side(-1, 3), -1.0);
    }
}
