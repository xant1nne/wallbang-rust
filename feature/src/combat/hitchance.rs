// Config block: LCG-константы детерминированного симулятора разлёта.
const LCG_MULT: u64 = 6364136223846793005;
const LCG_ADD: u64 = 1442695040888963407;

#[inline]
fn next(seed: &mut u64) -> f32 {
    *seed = seed.wrapping_mul(LCG_MULT).wrapping_add(LCG_ADD);
    ((*seed >> 33) as f32) / ((1u64 << 31) as f32)
}

/// Доля точек, попавших в конус разлёта. Возвращает 0..100.
pub fn roll(accuracy: f32, spread: f32, seed: u32, point_count: usize) -> f32 {
    if point_count == 0 {
        return 0.0;
    }
    let mut rng = seed as u64;
    let cone = (spread + (100.0 - accuracy).max(0.0) * 0.01).max(0.001);
    let mut passed = 0usize;
    for _ in 0..point_count {
        let dx = next(&mut rng) - 0.5;
        let dy = next(&mut rng) - 0.5;
        if (dx * dx + dy * dy).sqrt() <= cone {
            passed += 1;
        }
    }
    passed as f32 / point_count as f32 * 100.0
}
