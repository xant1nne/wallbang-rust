// Config block: индексы костей и hitbox-групп фиксируются в config::offsets после дампа.
use wallbang_core::{bone::BoneArray, math::Vec3};

pub const HEAD: usize = 6;
pub const CHEST: usize = 4;
pub const STOMACH: usize = 3;
pub const ARMS: usize = 9;
pub const LEGS: usize = 14;
pub const FEET: usize = 16;

pub const ALL: [usize; 6] = [HEAD, CHEST, STOMACH, ARMS, LEGS, FEET];

/// Возвращает world-позиции выбранных hitbox-точек по костям модели.
pub fn extract(pawn: usize, selected: &[bool; 6]) -> Vec<Vec3> {
    let mut out = Vec::with_capacity(6);
    if pawn == 0 {
        return out;
    }
    let Some(bones) = (unsafe { BoneArray::from_pawn(pawn) }) else {
        return out;
    };
    for (index, bone) in ALL.iter().enumerate() {
        if !selected[index] {
            continue;
        }
        if let Some(point) = unsafe { bones.position(*bone) } {
            out.push(point);
        }
    }
    out
}
