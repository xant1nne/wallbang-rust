// Config block: TICK_SLEEP_MS задаёт частоту refresh; NAME_CACHE_TTL_MS/NAME_RETRY_MS
// задают TTL кэша имён игроков; все офсеты алиасируются в config::offsets.
use crate::{
    entity::EntityList,
    math::{Mat4, Vec3},
    modules,
    pawn::{sanitize_player_name, Pawn, WeaponFields},
};
use parking_lot::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use wallbang_config::{offsets::{self, DW_VIEW_MATRIX}, Settings};

/// How long a resolved player name is reused before the controller is re-read.
/// Names only change on a reconnect, so this is a cold path.
const NAME_CACHE_TTL_MS: u128 = 4_000;
/// Retry interval after a failed resolve (controller not spawned yet).
const NAME_RETRY_MS: u128 = 250;
/// Upper bound on cached pawn→name entries; a full match never exceeds it.
const NAME_CACHE_CAPACITY: usize = 128;

static TRACE: Mutex<Option<fn(&str)>> = Mutex::new(None);
static TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

// Diagnostic counters for the Present-hook FPS investigation.
static PROF_CALLS: AtomicU32 = AtomicU32::new(0);
static PROF_LOCAL_US: AtomicU64 = AtomicU64::new(0);
static PROF_ITER_US: AtomicU64 = AtomicU64::new(0);
static PROF_PAWN_US: AtomicU64 = AtomicU64::new(0);
static PROF_ENTRIES: AtomicU64 = AtomicU64::new(0);
static PROF_PLAYERS: AtomicU64 = AtomicU64::new(0);

/// Returns and resets aggregated refresh costs (called from the render timing log).
pub fn profile_summary() -> String {
    let n = PROF_CALLS.swap(0, Ordering::Relaxed).max(1) as u64;
    format!(
        "refresh_profile calls={n} local={}us iterate={}us pawns={}us entries={} players={}",
        PROF_LOCAL_US.swap(0, Ordering::Relaxed) / n,
        PROF_ITER_US.swap(0, Ordering::Relaxed) / n,
        PROF_PAWN_US.swap(0, Ordering::Relaxed) / n,
        PROF_ENTRIES.swap(0, Ordering::Relaxed) / n,
        PROF_PLAYERS.swap(0, Ordering::Relaxed) / n,
    )
}

/// Диагностический колбэк из app::diag: только первые вызовы, чтобы не заспамить лог.
pub fn set_trace(callback: fn(&str)) {
    *TRACE.lock() = Some(callback);
}

fn trace(message: &str) {
    if TRACE_COUNT.fetch_add(1, Ordering::Relaxed) < 60 {
        if let Some(callback) = *TRACE.lock() {
            callback(message);
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct PlayerSnapshot {
    pub index: usize,
    pub address: usize,
    pub health: i32,
    pub armor: i32,
    pub team: i32,
    pub spotted: bool,
    pub alive: bool,
    pub origin: Vec3,
    pub velocity: Vec3,
    pub enemy: bool,
    /// Sanitized `m_sSanitizedPlayerName`; `None` until the controller spawns.
    pub name: Option<Arc<str>>,
    pub weapon: WeaponFields,
}

/// One cached name lookup. `last` is the instant the controller was read, so a
/// pawn whose controller has not spawned yet is retried after NAME_RETRY_MS
/// instead of on every snapshot refresh.
struct CachedName {
    name: Option<Arc<str>>,
    last: Instant,
}

#[derive(Clone, Debug, Default)]
pub struct Snapshot {
    pub valid: bool,
    pub local_address: usize,
    pub local_team: i32,
    pub local_origin: Vec3,
    pub view_matrix: Mat4,
    pub screen_w: f32,
    pub screen_h: f32,
    pub players: Vec<PlayerSnapshot>,
    pub client_tick: u32,
}

pub struct GameState {
    settings: RwLock<Settings>,
    snapshot: RwLock<Snapshot>,
    names: Mutex<HashMap<usize, CachedName>>,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        Self {
            settings: RwLock::new(Settings::default()),
            snapshot: RwLock::new(Snapshot::default()),
            names: Mutex::new(HashMap::new()),
        }
    }

    #[inline]
    pub fn settings(&self) -> RwLockReadGuard<'_, Settings> {
        self.settings.read()
    }

    #[inline]
    pub fn settings_mut(&self) -> RwLockWriteGuard<'_, Settings> {
        self.settings.write()
    }

    #[inline]
    pub fn snapshot(&self) -> RwLockReadGuard<'_, Snapshot> {
        self.snapshot.read()
    }

    /// Cached pawn→name resolution. The snapshot walk only re-reads the
    /// controller when the cache entry is missing or stale, so the 16 ms hot
    /// path never pays for a string read per frame.
    fn resolve_name(&self, pawn: usize) -> Option<Arc<str>> {
        let now = Instant::now();
        {
            let cache = self.names.lock();
            if let Some(entry) = cache.get(&pawn) {
                let ttl = if entry.name.is_some() {
                    NAME_CACHE_TTL_MS
                } else {
                    NAME_RETRY_MS
                };
                if now.saturating_duration_since(entry.last).as_millis() < ttl {
                    return entry.name.clone();
                }
            }
        }
        let resolved = unsafe {
            Pawn::new(pawn)
                .name()
                .and_then(|raw| sanitize_player_name(&raw))
                .map(Arc::<str>::from)
        };
        let mut cache = self.names.lock();
        if cache.len() >= NAME_CACHE_CAPACITY && !cache.contains_key(&pawn) {
            // Pawn addresses are recycled between matches; clearing keeps the
            // map bounded and drops the previous match's names in one step.
            cache.clear();
        }
        cache.insert(
            pawn,
            CachedName {
                name: resolved.clone(),
                last: now,
            },
        );
        resolved
    }

    /// Read-only snapshot, вызывается из тик-лупа и Present-хука.
    pub fn refresh(&self) {
        if !offsets::is_resolved() {
            trace("refresh: offsets not resolved");
            return;
        }
        let client = match modules::module_base("client.dll") {
            Some(b) => b,
            None => {
                trace("refresh: client.dll missing");
                return;
            }
        };
        let mut snap = Snapshot::default();
        snap.screen_w = modules::screen_size().0;
        snap.screen_h = modules::screen_size().1;

        unsafe {
            if let Some(matrix) = crate::memory::read::<Mat4>(client + DW_VIEW_MATRIX) {
                snap.view_matrix = matrix;
            }
            let t = std::time::Instant::now();
            if let Some(local) = Pawn::local() {
                if let Some(fields) = local.fields() {
                    snap.local_address = local.address;
                    snap.local_team = fields.team;
                    snap.local_origin = fields.origin;
                }
            }
            PROF_LOCAL_US.fetch_add(t.elapsed().as_micros() as u64, Ordering::Relaxed);
            let t = std::time::Instant::now();
            let entries = EntityList::iterate();
            PROF_ENTRIES.fetch_add(entries.len() as u64, Ordering::Relaxed);
            PROF_ITER_US.fetch_add(t.elapsed().as_micros() as u64, Ordering::Relaxed);
            let t = std::time::Instant::now();
            for (index, instance) in entries {
                let Some(fields) = Pawn::new(instance).fields() else {
                    continue;
                };
                if fields.health <= 0 || fields.life_state != 0 {
                    continue;
                }
                if !(2..=3).contains(&fields.team) {
                    continue;
                }
                snap.players.push(PlayerSnapshot {
                    index,
                    address: instance,
                    health: fields.health,
                    armor: fields.armor,
                    team: fields.team,
                    spotted: fields.spotted,
                    alive: true,
                    origin: fields.origin,
                    velocity: fields.velocity,
                    enemy: fields.team != snap.local_team,
                    name: self.resolve_name(instance),
                    weapon: Pawn::new(instance).weapon().unwrap_or_default(),
                });
            }
            PROF_PAWN_US.fetch_add(t.elapsed().as_micros() as u64, Ordering::Relaxed);
            PROF_PLAYERS.fetch_add(snap.players.len() as u64, Ordering::Relaxed);
        }
        snap.valid = snap.local_address != 0;
        *self.snapshot.write() = snap;
        PROF_CALLS.fetch_add(1, Ordering::Relaxed);
    }
}
