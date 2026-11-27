// Runtime feedback only: native field readback is not proof of a rendered material.
use std::sync::atomic::{AtomicU64, Ordering};

static STATUS: AtomicU64 = AtomicU64::new(0);
pub const WAITING: u8 = 0;
pub const WRITES_BLOCKED: u8 = 1;
pub const BUILD_MISMATCH: u8 = 2;
pub const BACKEND_MISSING: u8 = 3;
pub const DISABLED: u8 = 4;
pub const NO_PAWN: u8 = 5;
pub const NO_TARGETS: u8 = 6;
pub const APPLYING: u8 = 7;
pub const FIELDS_MATCH: u8 = 8;
pub fn update(phase: u8, requested: usize, matched: usize, hud: usize) {
    STATUS.store(phase as u64 | ((requested.min(65535) as u64)<<8)
        | ((matched.min(65535) as u64)<<24) | ((hud.min(65535) as u64)<<40), Ordering::Relaxed);
}
pub fn snapshot() -> u64 { STATUS.load(Ordering::Relaxed) }
pub fn message() -> String {
    let value = snapshot();
    let label = match value as u8 {
        WRITES_BLOCKED => "Writes blocked", BUILD_MISMATCH => "Unsupported game build",
        BACKEND_MISSING => "Native backend unavailable", DISABLED => "Changer disabled",
        NO_PAWN => "Waiting for a living local pawn", NO_TARGETS => "No owned replacement targets",
        APPLYING => "Applying native fields", FIELDS_MATCH => "Fields written; render unverified",
        _ => "Waiting for FrameStageNotify",
    };
    if value as u8 >= APPLYING {
        format!("{label}: {}/{}; HUD {}", (value>>24)&65535,(value>>8)&65535,(value>>40)&65535)
    } else { label.to_owned() }
}
