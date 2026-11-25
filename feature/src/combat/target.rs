// Config block: null; чистый выбор цели по FOV/дистанции/HP.
use wallbang_core::math::{angle_diff, angle_to, QAngle, Vec3};
use wallbang_core::state::PlayerSnapshot;

use super::Target;

#[inline]
pub fn score(player: &PlayerSnapshot, local: Vec3) -> f32 {
    let dist = player.origin.distance_to(local);
    let hp_bonus = (100 - player.health.min(100)) as f32 * 0.1;
    let spotted_bonus = if player.spotted { 20.0 } else { 0.0 };
    1000.0 - dist + hp_bonus + spotted_bonus
}

pub fn select(
    players: &[&PlayerSnapshot],
    local: Vec3,
    angles: QAngle,
    max_fov: f32,
) -> Option<Target> {
    let mut best: Option<Target> = None;
    for p in players {
        let aim = angle_to(local, p.origin);
        let yaw = angle_diff(aim.yaw, angles.yaw).abs();
        let pitch = angle_diff(aim.pitch, angles.pitch).abs();
        if yaw > max_fov || pitch > max_fov {
            continue;
        }
        let s = score(p, local) - (yaw + pitch) * 2.0;
        if best.map_or(true, |b| s > b.score) {
            best = Some(Target {
                index: p.index,
                address: p.address,
                origin: p.origin,
                velocity: p.velocity,
                health: p.health,
                score: s,
            });
        }
    }
    best
}
