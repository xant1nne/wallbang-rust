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
