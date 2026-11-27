// Config: owned DX11 inspect request only. No game offsets or Panorama scenes.
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Instant;

#[derive(Clone, Debug, PartialEq)]
pub struct Request {
    pub rect: [f32; 4],
    pub definition: i16,
    pub paint: u32,
    pub yaw: f32,
    pub pitch: f32,
    pub zoom: f32,
    pub show_model: bool,
}
static REQUEST: Mutex<Option<Request>> = Mutex::new(None);
static READY: AtomicBool = AtomicBool::new(false);
static ERROR: Mutex<Option<String>> = Mutex::new(None);
static TOTAL_US: AtomicU64 = AtomicU64::new(0);
static MAX_US: AtomicU64 = AtomicU64::new(0);
static COUNT: AtomicU64 = AtomicU64::new(0);
pub fn set_request(request: Option<Request>) {
    let mut current = REQUEST.lock();
    if current.as_ref().map(|r| (r.definition, r.paint))
        != request.as_ref().map(|r| (r.definition, r.paint))
    {
        *ERROR.lock() = None;
        report_capture(false);
    }
    *current = request;
}
pub fn report_error(error: String) {
    *ERROR.lock() = Some(error);
}
pub fn error() -> Option<String> {
    ERROR.lock().clone()
}
pub fn request() -> Option<Request> {
    REQUEST.lock().clone().filter(|r| r.show_model)
}
pub fn park() {
    set_request(None);
}
pub fn shutdown() {
    park();
}
pub fn capture_ready() -> bool {
    READY.load(Ordering::Relaxed)
}
pub fn report_capture(ready: bool) {
    READY.store(ready, Ordering::Relaxed);
}
pub fn profile(start: Instant) {
    let us = start.elapsed().as_micros() as u64;
    TOTAL_US.fetch_add(us, Ordering::Relaxed);
    MAX_US.fetch_max(us, Ordering::Relaxed);
    COUNT.fetch_add(1, Ordering::Relaxed);
}
pub fn take_profile() -> (u64, u64) {
    let n = COUNT.swap(0, Ordering::Relaxed).max(1);
    (
        TOTAL_US.swap(0, Ordering::Relaxed) / n,
        MAX_US.swap(0, Ordering::Relaxed),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn closing_and_switching_finish_clear_the_old_frame_immediately() {
        let make = |paint| Request {
            rect: [10., 20., 640., 320.],
            definition: 9,
            paint,
            yaw: 0.,
            pitch: 0.,
            zoom: 1.,
            show_model: true,
        };
        set_request(Some(make(279)));
        report_capture(true);
        assert!(capture_ready());
        set_request(Some(make(0)));
        assert!(!capture_ready());
        assert_eq!(request().unwrap().paint, 0);
        report_capture(true);
        park();
        assert!(request().is_none());
        assert!(!capture_ready());
    }
}
