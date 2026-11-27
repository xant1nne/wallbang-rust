// Windows-only adaptation for hosts that do not register PE static TLS.
// FlsAlloc/GetValue/SetValue manage storage for every calling game/worker thread.
use std::ffi::c_void;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU32, Ordering};

const NO_SLOT: u32 = u32::MAX;

#[link(name = "kernel32")]
extern "system" {
    fn FlsAlloc(callback: Option<unsafe extern "system" fn(*mut c_void)>) -> u32;
    fn FlsFree(index: u32) -> i32;
    fn FlsGetValue(index: u32) -> *mut c_void;
    fn FlsSetValue(index: u32, value: *mut c_void) -> i32;
}

pub(crate) struct Key<T> {
    index: AtomicU32,
    marker: PhantomData<fn() -> T>,
}

impl<T> Key<T> {
    pub(crate) const fn new() -> Self {
        Self { index: AtomicU32::new(NO_SLOT), marker: PhantomData }
    }

    // The returned pointer belongs only to the calling OS fiber. Callers keep
    // it inside their parking operation, matching the previous thread_local use.
    pub(crate) fn get_or_create(&self, make: fn() -> T) -> Option<*const T> {
        let mut index = self.index.load(Ordering::Acquire);
        if index == NO_SLOT {
            let candidate = unsafe { FlsAlloc(Some(drop_value::<T>)) };
            if candidate == NO_SLOT { return None; }
            match self.index.compare_exchange(NO_SLOT, candidate, Ordering::AcqRel, Ordering::Acquire) {
                Ok(_) => index = candidate,
                Err(existing) => { unsafe { FlsFree(candidate); } index = existing; }
            }
        }
        let mut value = unsafe { FlsGetValue(index) }.cast::<T>();
        if value.is_null() {
            value = Box::into_raw(Box::new(make()));
            if unsafe { FlsSetValue(index, value.cast()) } == 0 {
                unsafe { drop(Box::from_raw(value)); }
                return None;
            }
        }
        Some(value)
    }
}

unsafe extern "system" fn drop_value<T>(value: *mut c_void) {
    if !value.is_null() { drop(Box::from_raw(value.cast::<T>())); }
}
