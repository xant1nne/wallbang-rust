// Config block: VK_SPACE=0x20; LEGIT_MIN_STEP=0.02; вся запись — только под MutationGuard.
use wallbang_core::math::{angle_diff, angle_to, QAngle};
use wallbang_core::mutation;
use wallbang_core::state::GameState;
use wallbang_core::usercmd::{UserCmd, IN_ATTACK};

const LEGIT_MIN_STEP: f32 = 0.02;

/// Вызывается из detour'а CCSGOInput::CreateMove (игровой поток).
pub fn on_create_move(cmd: UserCmd, state: &GameState) {
    if cmd.base() == 0 || !state.settings().misc.allow_writes || !mutation::is_allowed() {
        return;
    }
    let Some(_guard) = mutation::acquire() else {
        return;
    };

    // Movement: порт Celerity (bhop/autostrafe/mini_jump/edgejump/jumpbug).
    crate::movement::on_create_move(cmd, state);

    let settings = state.settings().clone();
    if !settings.combat.ragebot.enabled && !settings.combat.legit.enabled {
        return;
    }
    if !settings.combat.ragebot.enabled
        && settings.combat.legit.enabled
        && !settings.combat.legit.group().aimbot
    {
        return;
    }

    let decision = super::run(state, cmd.client_tick().max(0) as u32);
    if !decision.fire || decision.target.is_none() {
        return;
    }
    let Some(from) = UserCmd::local_eye_position() else {
        return;
    };
    let want = angle_to(from, decision.point).normalized();

    if settings.combat.ragebot.enabled {
        // silent: углы живут только в команде, живой обзор не трогаем.
        cmd.set_view_angles(want);
        cmd.add_buttons(IN_ATTACK);
    } else {
        let current = cmd.view_angles().unwrap_or(want);
        let smooth = settings.combat.legit.group().smooth as f32;
        let step = (1.0 - smooth / 100.0).clamp(LEGIT_MIN_STEP, 1.0);
        let next = QAngle::new(
            current.pitch + angle_diff(want.pitch, current.pitch) * step,
            current.yaw + angle_diff(want.yaw, current.yaw) * step,
            0.0,
        )
        .normalized();
        cmd.set_view_angles(next);
        super::silent::apply_angles(next);
        cmd.add_buttons(IN_ATTACK);
    }
}
