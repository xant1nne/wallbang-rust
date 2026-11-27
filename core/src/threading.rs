// Manual-map DLL не проходит DllMain, поэтому std::thread::spawn в таком потоке
// умирает с `current thread handle already set during thread spawn`. Создаём
// поток напрямую через CreateThread и не трогаем TLS std::thread.
use std::ffi::c_void;

#[link(name = "kernel32")]
extern "system" {
    fn CreateThread(
        attributes: *mut c_void,
        stack_size: usize,
        start: unsafe extern "system" fn(*mut c_void) -> u32,
        parameter: *mut c_void,
        flags: u32,
        thread_id: *mut u32,
    ) -> *mut c_void;
    fn WaitForSingleObject(handle: *mut c_void, milliseconds: u32) -> u32;
    fn CloseHandle(handle: *mut c_void) -> i32;
}

const INFINITE: u32 = u32::MAX;

unsafe extern "system" fn thread_proc(parameter: *mut c_void) -> u32 {
    if !parameter.is_null() {
        let task = *Box::from_raw(parameter as *mut Box<dyn FnOnce() + Send>);
        task();
    }
    0
}

pub struct ThreadHandle {
    handle: usize,
}

impl ThreadHandle {
    pub fn started(&self) -> bool { self.handle != 0 }
    pub fn join(mut self) {
        let handle = std::mem::replace(&mut self.handle, 0) as *mut c_void;
        if !handle.is_null() {
            unsafe {
                WaitForSingleObject(handle, INFINITE);
                CloseHandle(handle);
            }
        }
    }
}

impl Drop for ThreadHandle {
    fn drop(&mut self) {
        let handle = std::mem::replace(&mut self.handle, 0) as *mut c_void;
        if !handle.is_null() {
            unsafe {
                CloseHandle(handle);
            }
        }
    }
}

/// Поток без std::thread TLS — безопасен внутри manual-map DLL.
pub fn spawn<F>(task: F) -> ThreadHandle
where
    F: FnOnce() + Send + 'static,
{
    let boxed: Box<dyn FnOnce() + Send> = Box::new(task);
    let parameter = Box::into_raw(Box::new(boxed)) as *mut c_void;
    let handle = unsafe { CreateThread(std::ptr::null_mut(), 0, thread_proc, parameter, 0, std::ptr::null_mut()) };
    if handle.is_null() {
        // Вернуть владение задачей, чтобы не утечь, если поток не создался.
        drop(unsafe { Box::from_raw(parameter as *mut Box<dyn FnOnce() + Send>) });
    }
    ThreadHandle {
        handle: handle as usize,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parking_lot::{Mutex, ReentrantMutex, RwLock};
    use std::cell::RefCell;
    use std::sync::{Arc, Barrier};

    #[test]
    fn native_threads_contend_and_reenter_without_compiler_tls_keys() {
        const WORKERS: usize = 6;
        const ITERATIONS: usize = 400;
        let rw = Arc::new(RwLock::new(0usize));
        let mutex = Arc::new(Mutex::new(0usize));
        let recursive = Arc::new(ReentrantMutex::new(RefCell::new(0usize)));
        let barrier = Arc::new(Barrier::new(WORKERS + 1));
        let write_guard = rw.write();
        let mut threads = Vec::new();
        for _ in 0..WORKERS {
            let rw = rw.clone();
            let mutex = mutex.clone();
            let recursive = recursive.clone();
            let barrier = barrier.clone();
            threads.push(spawn(move || {
                barrier.wait();
                // The main thread holds the writer: every native worker must
                // enter the patched shared-lock slow path before it can proceed.
                let _ = *rw.read();
                for _ in 0..ITERATIONS {
                    *rw.write() += 1;
                    *mutex.lock() += 1;
                    let outer = recursive.lock();
                    let inner = recursive.lock();
                    *inner.borrow_mut() += 1;
                    drop(inner);
                    drop(outer);
                }
            }));
        }
        barrier.wait();
        // Give the OS workers time to enqueue; this is a test-only forced wait.
        std::thread::sleep(std::time::Duration::from_millis(40));
        drop(write_guard);
        for thread in threads { thread.join(); }
        assert_eq!(*rw.read(), WORKERS * ITERATIONS);
        assert_eq!(*mutex.lock(), WORKERS * ITERATIONS);
        assert_eq!(*recursive.lock().borrow(), WORKERS * ITERATIONS);
    }
}
