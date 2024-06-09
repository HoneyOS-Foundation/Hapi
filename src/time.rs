use std::time::{Duration, SystemTime};

/// Get the time in seconds since the start of the process or thread
pub fn since_startup() -> f64 {
    unsafe { crate::ffi::hapi_time_since_startup() }
}

/// Get the sytem time
pub fn system() -> SystemTime {
    let since_epoch = unsafe { crate::ffi::hapi_time_since_unix_epoch() };
    SystemTime::UNIX_EPOCH + Duration::from_secs_f64(since_epoch)
}
