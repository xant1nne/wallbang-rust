// Config block: чистые расчёты движения; реальная запись команд появится в M3 через MutationGuard.
use wallbang_core::math::Vec3;

pub const MAX_SPEED: f32 = 250.0;

#[inline]
pub fn strafe_axes(yaw_deg: f32, move_forward: f32, move_side: f32) -> (f32, f32) {
    let yaw = yaw_deg.to_radians();
    (
        move_forward * yaw.cos() - move_side * yaw.sin(),
        move_forward * yaw.sin() + move_side * yaw.cos(),
    )
}

#[inline]
pub fn normalized_move(forward: f32, side: f32) -> (f32, f32) {
    let len = (forward * forward + side * side).sqrt();
    if len > 1.0 {
        (forward / len, side / len)
    } else {
        (forward, side)
    }
}

#[inline]
pub fn predicted_velocity(velocity: Vec3, acceleration: f32, ticks: usize, interval: f32) -> Vec3 {
    velocity.add(Vec3::new(0.0, 0.0, -acceleration * ticks as f32 * interval))
}

#[inline]
pub fn edge_approach(distance: f32, step: f32) -> f32 {
    if distance <= step {
        0.0
    } else {
        distance - step
    }
}
