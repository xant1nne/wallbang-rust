// Config block: 1:1 порт Celerity `src/features/movement/movement.cpp`.
// INTERVAL_PER_TICK / STANDABLE_NORMAL / edge-look clamp / trace distance — те же
// значения, что в оригинале. Вся запись в команду идёт только под MutationGuard
// (вызывается из `combat::cmd::on_create_move` после проверки allow_writes).
//
// Keybinds: в Celerity пустой bind = always-on; у нас UI биндов пока нет, поэтому
// активные фичи работают as always-on, как celerity с пустым ключом.
use parking_lot::Mutex;
use wallbang_config::offsets::{CSGO_INPUT_FORWARD_MOVE, CSGO_INPUT_LEFT_MOVE, CSGO_INPUT_MOUSE_DELTA_X};
use wallbang_core::math::Vec3;
use wallbang_core::pawn::Pawn;
use wallbang_core::state::GameState;
use wallbang_core::trace;
use wallbang_core::usercmd::{
    UserCmd, FL_ONGROUND, IN_DUCK, IN_JUMP, IN_MOVELEFT, IN_MOVERIGHT,
};
use wallbang_core::{memory, modules};

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
}

static STATE: Mutex<MovementState> = Mutex::new(MovementState {
    jumpbug_ducked: false,
});

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
    cmd.remove_buttons(IN_JUMP);
}

/// c_autostrafe::run: в воздухе с зажатым прыжком подставляет side-move и
/// соответствующий MOVE-бит. Направление берётся из mouse delta, при нуле —
/// чередуется по чётности тика (в оригинале — m_command_number).
fn run_autostrafe(cmd: UserCmd, autostrafe: bool) {
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
    let csgo_input = unsafe { modules::csgo_input() }.unwrap_or(0);
    if mouse_delta == 0 && csgo_input != 0 {
        mouse_delta =
            unsafe { memory::read::<i32>(csgo_input + CSGO_INPUT_MOUSE_DELTA_X) }.unwrap_or(0);
    }
    let side = strafe_side(mouse_delta, cmd.client_tick());
    cmd.set_forward_move(0.0);
    cmd.set_side_move(side);
    // Celerity пишет те же значения в i_csgo_input, иначе движок их перезапишет.
    if csgo_input != 0 {
        unsafe {
            memory::write(csgo_input + CSGO_INPUT_FORWARD_MOVE, 0.0f32);
            memory::write(csgo_input + CSGO_INPUT_LEFT_MOVE, side);
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
fn strafe_side(mouse_delta: i32, client_tick: i32) -> f32 {
    if mouse_delta > 1 {
        -1.0
    } else if mouse_delta < -1 {
        1.0
    } else if client_tick % 2 == 0 {
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
/// ставит прыжок. Celerity кладёт его в subtick step `when = 0`; у нас subtick
/// step'ы не пишутся, поэтому бит ставится в текущем тике — геометрия проверки
/// края идентична оригиналу.
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
        cmd.add_buttons(IN_JUMP);
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

/// Порядок вызовов как в Celerity dispatcher: bhop → autostrafe → mini jump →
/// edge jump → jumpbug.
pub fn on_create_move(cmd: UserCmd, state: &GameState) {
    let settings = state.settings().clone();
    let misc = &settings.misc;
    run_bhop(cmd, misc.bhop);
    run_autostrafe(cmd, misc.autostrafe);
    run_mini_jump(cmd, misc.movement.mini_jump);
    run_edge_jump(cmd, misc.movement.edgejump);
    run_jumpbug(cmd, misc.movement.jumpbug);
}

#[cfg(test)]
mod tests {
    use super::strafe_side;

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
