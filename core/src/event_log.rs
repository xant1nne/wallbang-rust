// Thread-safe HUD event log ring buffer for Hit, Miss, and Vote Revealer notifications.
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

const MAX_ENTRIES: usize = 8;
const ENTRY_TTL: Duration = Duration::from_secs(6);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogKind {
    Hit,
    Miss,
    Vote,
}

#[derive(Clone, Debug)]
pub struct LogEntry {
    pub kind: LogKind,
    pub prefix: String,
    pub highlight: String,
    pub suffix: String,
    pub created: Instant,
}

impl LogEntry {
    pub fn alpha(&self, now: Instant) -> f32 {
        let age = now.saturating_duration_since(self.created).as_secs_f32();
        let ttl = ENTRY_TTL.as_secs_f32();
        if age >= ttl {
            0.0
        } else if age > ttl - 0.8 {
            ((ttl - age) / 0.8).clamp(0.0, 1.0)
        } else {
            1.0
        }
    }
}

static LOGS: Mutex<VecDeque<LogEntry>> = Mutex::new(VecDeque::new());

fn push(kind: LogKind, prefix: String, highlight: String, suffix: String) {
    let mut q = LOGS.lock();
    if q.len() >= MAX_ENTRIES {
        q.pop_front();
    }
    q.push_back(LogEntry {
        kind,
        prefix,
        highlight,
        suffix,
        created: Instant::now(),
    });
}

pub fn push_hit(target: &str, hitgroup: &str, damage: i32, remaining_hp: i32) {
    push(
        LogKind::Hit,
        format!("hit {} in {} for ", target, hitgroup),
        format!("{} dmg", damage),
        format!(" ({} hp left)", remaining_hp.max(0)),
    );
}

pub fn push_miss(target: &str, reason: &str) {
    push(
        LogKind::Miss,
        format!("missed {} due to ", target),
        reason.to_owned(),
        String::new(),
    );
}

pub fn push_vote(voter: &str, choice: &str) {
    push(
        LogKind::Vote,
        format!("{} voted ", voter),
        choice.to_owned(),
        String::new(),
    );
}
pub fn push_vote_started() {
    push(LogKind::Vote, "vote ".to_owned(), "started".to_owned(), String::new());
}

/// Returns active (non-expired) log entries filtered by enabled flags.
pub fn snapshot(hit: bool, miss: bool, vote: bool) -> Vec<(LogEntry, f32)> {
    let now = Instant::now();
    let mut q = LOGS.lock();
    q.retain(|e| now.saturating_duration_since(e.created) < ENTRY_TTL);
    q.iter()
        .filter(|e| match e.kind {
            LogKind::Hit => hit,
            LogKind::Miss => miss,
            LogKind::Vote => vote,
        })
        .map(|e| (e.clone(), e.alpha(now)))
        .collect()
}
