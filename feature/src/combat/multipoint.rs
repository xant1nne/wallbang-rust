// Config block: POINT_GRID, MIN_SCALE, MAX_SCALE — все tunables именованные.
use wallbang_core::math::Vec3;

pub const POINT_GRID: i32 = 3;
pub const MIN_SCALE: f32 = 0.15;
pub const MAX_SCALE: f32 = 1.0;
pub const POINT_RADIUS: f32 = 6.0;

pub fn build(center: Vec3, radius: f32, pointscale: f32) -> Vec<Vec3> {
    let scale = pointscale.clamp(MIN_SCALE, MAX_SCALE);
    let step = radius * scale / POINT_GRID as f32;
    let half = (POINT_GRID - 1) as f32 * 0.5;
    let mut out = Vec::with_capacity((POINT_GRID * POINT_GRID) as usize);
    for x in 0..POINT_GRID {
        for z in 0..POINT_GRID {
            out.push(Vec3::new(
                center.x + (x as f32 - half) * step,
                center.y,
                center.z + (z as f32 - half) * step,
            ));
        }
    }
    out
}

pub fn dynamic_scale(distance: f32, base: f32) -> f32 {
    let d = distance.max(1.0);
    (base * (512.0 / d)).clamp(MIN_SCALE, MAX_SCALE)
}
