// Config: Windows SMTC, Spotify only; no game offsets. Polling runs off Present.
use parking_lot::Mutex;
use std::sync::{Arc, atomic::{AtomicBool, AtomicU8, Ordering}};
use std::time::{Duration, Instant};
use windows::Media::Control::*;
use windows::Storage::Streams::{Buffer, DataReader, InputStreamOptions};
use windows::Win32::System::WinRT::{RoInitialize, RoUninitialize, RO_INIT_MULTITHREADED};

const POLL_MS: u64 = 500;
const MAX_ART_BYTES: u64 = 4 * 1024 * 1024;
#[derive(Clone)]
pub struct Track {
    pub title: String, pub artist: String, pub playing: bool,
    pub position: f64, pub duration: f64, pub sampled: Instant,
    pub art: Option<Arc<image::RgbaImage>>, pub revision: u64,
}
struct Shared { stop: AtomicBool, command: AtomicU8, track: Mutex<Option<Track>> }
struct Worker { shared: Arc<Shared>, handle: wallbang_core::threading::ThreadHandle }
static WORKER: Mutex<Option<Worker>> = Mutex::new(None);
static ART_RECT: Mutex<Option<(f32, f32, f32, f32)>> = Mutex::new(None);
pub fn set_art_rect(rect: Option<(f32, f32, f32, f32)>) { *ART_RECT.lock() = rect; }
pub fn art_request() -> Option<((f32, f32, f32, f32), Track)> {
    let rect = (*ART_RECT.lock())?;
    Some((rect, snapshot()?))
}

pub fn snapshot() -> Option<Track> {
    let mut worker = WORKER.lock();
    if worker.is_none() {
        let shared = Arc::new(Shared { stop: AtomicBool::new(false), command: AtomicU8::new(0), track: Mutex::new(None) });
        let task = shared.clone();
        let handle = wallbang_core::threading::spawn(move || run(task));
        *worker = Some(Worker { shared, handle });
    }
    let track = worker.as_ref()?.shared.track.lock().clone();
    track
}
pub fn command(value: u8) {
    if let Some(worker) = WORKER.lock().as_ref() { worker.shared.command.store(value, Ordering::Release); }
}
pub fn shutdown() {
    if let Some(worker) = WORKER.lock().take() {
        worker.shared.stop.store(true, Ordering::Release);
        worker.handle.join();
    }
}
fn artwork(session: &GlobalSystemMediaTransportControlsSession) -> Option<Arc<image::RgbaImage>> {
    let props = session.TryGetMediaPropertiesAsync().ok()?.join().ok()?;
    let stream = props.Thumbnail().ok()?.OpenReadAsync().ok()?.join().ok()?;
    let size = stream.Size().ok()?;
    if size == 0 || size > MAX_ART_BYTES { return None; }
    let buffer = Buffer::Create(size as u32).ok()?;
    let read = stream.ReadAsync(&buffer, size as u32, InputStreamOptions::None).ok()?.join().ok()?;
    let mut bytes = vec![0; read.Length().ok()? as usize];
    DataReader::FromBuffer(&read).ok()?.ReadBytes(&mut bytes).ok()?;
    let picture = image::load_from_memory(&bytes).ok()?.thumbnail(256, 256).to_rgba8();
    Some(Arc::new(picture))
}
fn run(shared: Arc<Shared>) {
    if unsafe { RoInitialize(RO_INIT_MULTITHREADED) }.is_err() { return; }
    let manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync().and_then(|op| op.join());
    if let Ok(manager) = manager {
        let mut previous = String::new();
        let mut art = None;
        let mut revision = 0;
        while !shared.stop.load(Ordering::Acquire) {
            let session = manager.GetSessions().ok().and_then(|sessions| sessions.into_iter().find(|s|
                s.SourceAppUserModelId().is_ok_and(|id| id.to_string().to_lowercase().contains("spotify"))));
            if let Some(session) = session {
                match shared.command.swap(0, Ordering::AcqRel) {
                    1 => { let _ = session.TrySkipPreviousAsync().and_then(|op| op.join()); }
                    2 => { let _ = session.TryTogglePlayPauseAsync().and_then(|op| op.join()); }
                    3 => { let _ = session.TrySkipNextAsync().and_then(|op| op.join()); }
                    _ => {}
                }
                if let Ok(props) = session.TryGetMediaPropertiesAsync().and_then(|op| op.join()) {
                    let title = props.Title().map(|v| v.to_string()).unwrap_or_default();
                    let artist = props.Artist().map(|v| v.to_string()).unwrap_or_default();
                    let key = format!("{artist}\n{title}");
                    if key != previous { art = artwork(&session); revision += 1; previous = key; }
                    let timeline = session.GetTimelineProperties().ok();
                    let start = timeline.as_ref().and_then(|t| t.StartTime().ok()).map(|t| t.Duration).unwrap_or(0);
                    let position = timeline.as_ref().and_then(|t| t.Position().ok()).map(|t| t.Duration - start).unwrap_or(0) as f64 / 10_000_000.0;
                    let duration = timeline.and_then(|t| t.EndTime().ok()).map(|t| t.Duration - start).unwrap_or(0) as f64 / 10_000_000.0;
                    let playing = session.GetPlaybackInfo().and_then(|v| v.PlaybackStatus()).is_ok_and(|v| v == GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing);
                    *shared.track.lock() = Some(Track { title, artist, playing, position, duration, sampled: Instant::now(), art: art.clone(), revision });
                }
            } else {
                shared.command.store(0, Ordering::Release);
                *shared.track.lock() = None;
                previous.clear();
                art = None;
            }
            for _ in 0..POLL_MS / 25 {
                if shared.stop.load(Ordering::Acquire) || shared.command.load(Ordering::Acquire) != 0 { break; }
                std::thread::sleep(Duration::from_millis(25));
            }
        }
    }
    unsafe { RoUninitialize(); }
}
