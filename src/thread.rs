use std::ffi::c_void;

/// Spawn a function pointer on a new thread
pub fn spawn(f: fn()) {
    unsafe { crate::ffi::hapi_thread_spawn(f as *const c_void) }
}
