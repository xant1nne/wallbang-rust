// Config block: PAWN_SCENE_NODE=0x330, SCENE_NODE_MODEL_STATE=0x140, MODEL_STATE_BONE_ARRAY=0x80, BONE_STRIDE=0x20.
// IDA: CalculateWorldSpaceBones читает *(scene_node + 0x1C0), каждая кость — CTransform 0x20:
//   +0x00 VectorAligned xyz(+w), +0x10 Quaternion (x,y,z,w).
use crate::{math::Mat3x4, memory};
use wallbang_config::constants::BONE_STRIDE;
use wallbang_config::offsets::{MODEL_STATE_BONE_ARRAY, PAWN_SCENE_NODE, SCENE_NODE_MODEL_STATE};

#[derive(Clone, Copy, Debug, Default)]
pub struct BoneArray {
    base: usize,
}

impl BoneArray {
    #[inline]
    pub unsafe fn from_pawn(pawn: usize) -> Option<Self> {
        let scene_node = memory::read_ptr(pawn + PAWN_SCENE_NODE)?;
        let model_state = scene_node + SCENE_NODE_MODEL_STATE;
        let base = memory::read_ptr(model_state + MODEL_STATE_BONE_ARRAY)?;
        Some(Self { base })
    }

    #[inline]
    pub fn is_valid(self) -> bool {
        self.base != 0
    }

    /// CTransform -> Mat3x4 (row-major): поворот из quaternion, позиция в 4-й колонке.
    #[inline]
    pub unsafe fn bone(self, index: usize) -> Option<Mat3x4> {
        if self.base == 0 {
            return None;
        }
        let raw = memory::read::<[f32; 8]>(self.base + index * BONE_STRIDE)?;
        let (x, y, z) = (raw[0], raw[1], raw[2]);
        let (qx, qy, qz, qw) = (raw[4], raw[5], raw[6], raw[7]);
        let r00 = 1.0 - 2.0 * (qy * qy + qz * qz);
        let r01 = 2.0 * (qx * qy - qz * qw);
        let r02 = 2.0 * (qx * qz + qy * qw);
        let r10 = 2.0 * (qx * qy + qz * qw);
        let r11 = 1.0 - 2.0 * (qx * qx + qz * qz);
        let r12 = 2.0 * (qy * qz - qx * qw);
        let r20 = 2.0 * (qx * qz - qy * qw);
        let r21 = 2.0 * (qy * qz + qx * qw);
        let r22 = 1.0 - 2.0 * (qx * qx + qy * qy);
        Some(Mat3x4::from_raw(&[
            r00, r01, r02, x, r10, r11, r12, y, r20, r21, r22, z,
        ]))
    }

    #[inline]
    pub unsafe fn position(self, index: usize) -> Option<crate::math::Vec3> {
        if self.base == 0 {
            return None;
        }
        let raw = memory::read::<[f32; 3]>(self.base + index * BONE_STRIDE)?;
        Some(crate::math::Vec3::new(raw[0], raw[1], raw[2]))
    }
}
