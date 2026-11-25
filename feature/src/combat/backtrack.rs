// Config block: HISTORY_TICKS / MAX_LAG_TICKS — размер кольцевого буфера истории позиций.
use wallbang_core::math::Vec3;

pub const HISTORY_TICKS: usize = 64;
pub const MAX_LAG_TICKS: usize = 12;

#[derive(Clone, Copy, Debug)]
pub struct Record {
    pub player_index: usize,
    pub tick: u32,
    pub origin: Vec3,
    pub velocity: Vec3,
}

#[derive(Debug, Default)]
pub struct History {
    records: Vec<Record>,
}

impl History {
    pub const fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    pub fn record(&mut self, player_index: usize, tick: u32, origin: Vec3, velocity: Vec3) {
        self.records.push(Record {
            player_index,
            tick,
            origin,
            velocity,
        });
        if self.records.len() > HISTORY_TICKS * 8 {
            self.records.drain(..HISTORY_TICKS);
        }
    }

    /// Самая свежая запись игрока в окне [tick-max_lag, tick].
    pub fn select_tick(&self, player_index: usize, current_tick: u32, max_lag: usize) -> Option<Record> {
        let limit = max_lag.min(MAX_LAG_TICKS) as u32;
        self.records
            .iter()
            .rev()
            .find(|r| {
                r.player_index == player_index
                    && current_tick >= r.tick
                    && current_tick - r.tick <= limit
            })
            .copied()
    }

    pub fn clear(&mut self) {
        self.records.clear();
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.records.len()
    }
}
