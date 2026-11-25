// Config block: MUTATIONS_ENABLED стартует выключенным; runtime-флаг тоже стартует выключенным.
use std::sync::atomic::{AtomicBool, Ordering};
use wallbang_config::constants::MUTATIONS_ENABLED;

static RUNTIME_ALLOWED: AtomicBool = AtomicBool::new(false);

/// Держится только при явном разрешении Хозяина. Без него write<T> недоступен.
pub struct MutationGuard {
    _private: (),
}

impl MutationGuard {
    #[inline]
    pub unsafe fn write<T: Copy>(&self, address: usize, value: T) -> bool {
        crate::memory::write(address, value)
    }
}

#[inline]
pub fn set_allowed(allowed: bool) {
    RUNTIME_ALLOWED.store(allowed, Ordering::SeqCst);
}

#[inline]
pub fn is_allowed() -> bool {
    MUTATIONS_ENABLED || RUNTIME_ALLOWED.load(Ordering::SeqCst)
}

#[inline]
pub fn acquire() -> Option<MutationGuard> {
    if is_allowed() {
        Some(MutationGuard { _private: () })
    } else {
        None
    }
}
