// Snapshot-based Hit & Event feed: detects enemy HP drops and pushes live entries
// into wallbang_core::event_log without fragile game-event vtable hooks.
use parking_lot::Mutex;
use std::collections::HashMap;
use wallbang_core::state::GameState;

#[derive(Default)]
struct FeedState {
    prev_hp: HashMap<usize, i32>,
    last_local_addr: usize,
}

static FEED: std::sync::LazyLock<Mutex<FeedState>> =
    std::sync::LazyLock::new(|| Mutex::new(FeedState::default()));

pub fn tick(state: &GameState) {
    let hit_on = state.settings().misc.hit_log;
    if !hit_on {
        return;
    }
    let snap = state.snapshot();
    if !snap.valid || snap.local_address == 0 {
        return;
    }
    let mut feed = FEED.lock();
    if feed.last_local_addr != snap.local_address {
        feed.prev_hp.clear();
        feed.last_local_addr = snap.local_address;
    }
    for player in &snap.players {
        if player.address == snap.local_address || player.team == snap.local_team {
            continue;
        }
        let cur = player.health.max(0);
        if let Some(&prev) = feed.prev_hp.get(&player.address) {
            if prev > 0 && cur < prev {
                let dmg = prev - cur;
                let name = player.name.as_deref().unwrap_or("enemy");
                wallbang_core::event_log::push_hit(name, "body", dmg, cur);
            }
        }
        feed.prev_hp.insert(player.address, cur);
    }
}
