// Config block: ядро не знает о UI; все используемые офсеты алиасируются из wallbang_config::offsets.
pub mod bone;
pub mod entity;
pub mod hooks;
pub mod input;
pub mod math;
pub mod memory;
pub mod modules;
pub mod mutation;
pub mod panorama;
pub mod pattern;
pub mod pawn;
pub mod state;
pub mod threading;
pub mod tick;
pub mod usercmd;

use std::sync::{Arc, OnceLock};
use state::GameState;

static STATE: OnceLock<Arc<GameState>> = OnceLock::new();

/// Единственный инстанс состояния процесса.
pub fn state() -> Arc<GameState> {
    STATE.get_or_init(|| Arc::new(GameState::new())).clone()
}
