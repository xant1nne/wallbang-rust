// Config block: ARMOR_ABSORB, FALLOFF_START, MIN_DAMAGE_CLAMP.
const ARMOR_ABSORB: f32 = 0.5;
const FALLOFF_START: f32 = 500.0;

#[inline]
pub fn damage(base: f32, hitgroup_mult: f32, armor: f32, distance: f32) -> f32 {
    let falloff = if distance > FALLOFF_START {
        (1.0 - (distance - FALLOFF_START) / 4000.0).clamp(0.25, 1.0)
    } else {
        1.0
    };
    let raw = base * hitgroup_mult * falloff;
    raw * (1.0 - armor * ARMOR_ABSORB)
}

#[inline]
pub fn lethal(hp: i32, damage: f32) -> bool {
    damage >= hp as f32
}
