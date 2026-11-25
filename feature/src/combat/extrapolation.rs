// Config block: MAX_TICKS=12 — лимит экстраполяции из меню ragebot.
use wallbang_core::math::Vec3;

pub const MAX_TICKS: usize = 12;

#[inline]
pub fn extrapolate(origin: Vec3, velocity: Vec3, ticks: usize, tick_interval: f32) -> Vec3 {
    let t = ticks.min(MAX_TICKS) as f32 * tick_interval;
    origin.add(velocity.scale(t))
}

#[inline]
pub fn interpolate(previous: Vec3, next: Vec3, fraction: f32) -> Vec3 {
    let f = fraction.clamp(0.0, 1.0);
    previous.add(next.sub(previous).scale(f))
}
