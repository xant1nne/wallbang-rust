// Direct port of wallbang::clantag (Fixed, Typewriter, Scroll, Bounce, Blink) with UTF-8 safety.
use parking_lot::Mutex;
use wallbang_config::settings::ClantagSettings;

pub const MAX_TAG_BYTES: usize = 32;
pub const MIN_STEP_MS: i32 = 250;
pub const MAX_STEP_MS: i32 = 2000;

pub const STYLES: [&str; 5] = ["Fixed", "Typewriter", "Scroll", "Bounce", "Blink"];

static CURRENT_TAG: Mutex<String> = Mutex::new(String::new());
struct NameState { original: Option<String>, last: String, sent_ms: u64 }
static NAME_STATE: Mutex<NameState> = Mutex::new(NameState { original: None, last: String::new(), sent_ms: 0 });
static SHUTDOWN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn request_shutdown() { SHUTDOWN.store(true, std::sync::atomic::Ordering::Release); }
pub fn restored() -> bool { NAME_STATE.lock().original.is_none() }

fn quoted_name(name: &str) -> String {
    name.chars().filter(|c| !c.is_control() && !matches!(c, '"' | ';' | '\\')).take(96).collect()
}

/// This is a name prefix, not a Steam-group clan tag. Called only on game thread.
pub fn sync_name(state: &wallbang_core::state::GameState, now_ms: u64) {
    let settings = state.settings();
    let cfg = settings.misc.clantag.clone();
    let enabled = cfg.enabled && settings.misc.allow_writes && !SHUTDOWN.load(std::sync::atomic::Ordering::Acquire);
    drop(settings);
    let mut runtime = NAME_STATE.lock();
    if now_ms.saturating_sub(runtime.sent_ms) < MIN_STEP_MS as u64 { return; }
    if !enabled {
        if let Some(original) = runtime.original.as_ref() {
            if unsafe { wallbang_core::engine::execute(&format!("name \"{}\"", quoted_name(original))) } {
                runtime.original = None;
                runtime.last.clear();
                runtime.sent_ms = now_ms;
            }
        }
        return;
    }
    if runtime.original.is_none() {
        runtime.original = unsafe { wallbang_core::pawn::Pawn::local_controller() }
            .and_then(wallbang_core::pawn::Pawn::player_name);
    }
    let Some(original) = runtime.original.as_ref() else { return; };
    let tag = tick(&cfg, now_ms);
    let desired = if tag.is_empty() { original.clone() } else { format!("[{tag}] {original}") };
    if desired != runtime.last && unsafe { wallbang_core::engine::execute(&format!("name \"{}\"", quoted_name(&desired))) } {
        runtime.last = desired;
        runtime.sent_ms = now_ms;
    }
}

fn safe_chars(input: &str, limit: usize) -> Vec<char> {
    let mut out = Vec::new();
    let mut used = 0usize;
    for ch in input.chars() {
        if ch.is_control() {
            continue;
        }
        let len = ch.len_utf8();
        if used + len > limit {
            break;
        }
        out.push(ch);
        used += len;
    }
    out
}

pub fn frame(phrase: &str, style: usize, step: u64) -> String {
    let mut glyphs = safe_chars(phrase, MAX_TAG_BYTES);
    if glyphs.is_empty() {
        glyphs = safe_chars("WALLBANG", MAX_TAG_BYTES);
    }
    let n = glyphs.len();
    let mut out = String::new();
    match style {
        0 => {
            for c in &glyphs {
                out.push(*c);
            }
        }
        1 => {
            let phase = (step as usize) % (n * 2 + 2);
            let count = if phase <= n { phase } else { n * 2 + 1 - phase };
            for c in glyphs.iter().take(count) {
                out.push(*c);
            }
        }
        2 => {
            let offset = (step as usize) % n;
            for i in 0..n {
                out.push(glyphs[(i + offset) % n]);
            }
        }
        3 => {
            let phase = (step as usize) % 6;
            let pad = if phase <= 3 { phase } else { 6 - phase };
            for _ in 0..pad {
                out.push(' ');
            }
            for c in &glyphs {
                out.push(*c);
            }
        }
        4 => {
            if step % 2 == 0 {
                for c in &glyphs {
                    out.push(*c);
                }
            }
        }
        _ => {
            for c in &glyphs {
                out.push(*c);
            }
        }
    }
    let mut trimmed = String::new();
    let mut used = 0usize;
    for ch in out.chars() {
        let len = ch.len_utf8();
        if used + len > MAX_TAG_BYTES {
            break;
        }
        trimmed.push(ch);
        used += len;
    }
    trimmed
}

/// Updates the current animated clantag frame.
pub fn tick(cfg: &ClantagSettings, now_ms: u64) -> String {
    if !cfg.enabled {
        let mut cur = CURRENT_TAG.lock();
        cur.clear();
        return String::new();
    }
    let step_ms = cfg.step_ms.clamp(MIN_STEP_MS, MAX_STEP_MS) as u64;
    let phase = now_ms / step_ms;
    let tag = frame(&cfg.phrase, cfg.style, phase);
    let mut cur = CURRENT_TAG.lock();
    if *cur != tag {
        *cur = tag.clone();
    }
    tag
}

pub fn current() -> String {
    CURRENT_TAG.lock().clone()
}
