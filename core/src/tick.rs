// Config block: TICK_SLEEP_MS из wallbang_config::constants.
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;
use wallbang_config::constants::TICK_SLEEP_MS;

use crate::threading::ThreadHandle;

pub struct TickLoop {
    stop: Arc<AtomicBool>,
    handle: Option<ThreadHandle>,
}

impl TickLoop {
    pub fn start<F>(callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        let stop = Arc::new(AtomicBool::new(false));
        let worker_stop = Arc::clone(&stop);
        let callback = Arc::new(callback);
        let handle = crate::threading::spawn(move || {
            let delay = Duration::from_millis(TICK_SLEEP_MS);
            while !worker_stop.load(Ordering::Relaxed) {
                callback();
                thread::sleep(delay);
            }
        });
        Self {
            stop,
            handle: Some(handle),
        }
    }

    pub fn stop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            handle.join();
        }
    }
}

impl Drop for TickLoop {
    fn drop(&mut self) {
        self.stop();
    }
}
