// Config block: запись углов — только через MutationGuard, который по умолчанию выключен.
use wallbang_core::math::{angle_to, QAngle, Vec3};
use wallbang_core::mutation::{self, MutationGuard};
use wallbang_config::offsets::DW_VIEW_ANGLES;

#[inline]
pub fn angles_for(from: Vec3, to: Vec3) -> QAngle {
    angle_to(from, to).normalized()
}

/// Применяет углы в client.dll::dwViewAngles. Ничего не делает без явного разрешения на запись.
pub fn apply_angles(angles: QAngle) -> bool {
    let Some(guard) = mutation::acquire() else {
        return false;
    };
    let Some(client) = wallbang_core::modules::module_base("client.dll") else {
        return false;
    };
    unsafe { write_angles(&guard, client + DW_VIEW_ANGLES, angles) }
}

#[inline]
unsafe fn write_angles(guard: &MutationGuard, address: usize, angles: QAngle) -> bool {
    guard.write(address, [angles.pitch, angles.yaw, angles.roll])
}
