use std::ffi::{CStr, CString};

/// Get the process id
pub fn pid() -> Option<String> {
    let ptr = unsafe { crate::ffi::hapi_process_get_pid() };
    // # Safety
    // Since the string is garunteed to be null ternimated, there is no way of accessing unallocated memory
    let cstring = unsafe { CStr::from_ptr(ptr as *const i8) };
    let mut string = cstring.to_string_lossy().to_string();
    let _ = string.split_off(36);
    let string = string.to_owned();

    unsafe { crate::mem::free(ptr as *mut u8) };
    Some(string)
}

/// Get the current working directory
pub fn cwd() -> String {
    let ptr = unsafe { crate::ffi::hapi_process_get_cwd() };
    let cstring = unsafe { CStr::from_ptr(ptr as *const i8) };
    let string = cstring.to_string_lossy().to_string();
    let string = string.to_owned();

    unsafe { crate::mem::free(ptr as *mut u8) };

    string
}

/// Set the current working directory
pub fn set_cwd(wd: &str) {
    let cstring = CString::new(wd).unwrap();
    unsafe { crate::ffi::hapi_process_set_cwd(cstring.as_ptr() as *const u8) };
}

/// Represents a process
pub struct Process(String);

impl Process {
    /// Spawn a wasm binary as a subprocess and return it's pid
    pub fn spawn_sub(bin: &[u8]) -> Option<Self> {
        let bin = bin.to_vec();
        let pid_ptr =
            unsafe { crate::ffi::hapi_process_spawn_subprocess(bin.as_ptr(), bin.len() as u32) };

        if pid_ptr == std::ptr::null() {
            return None;
        }

        // # Safety
        // Since the string is garunteed to be null ternimated, there is no way of accessing unallocated memory
        let cstring = unsafe { CStr::from_ptr(pid_ptr as *const i8) };
        let mut string = cstring.to_string_lossy().to_string();
        let _ = string.split_off(36);
        let string = string.to_owned();

        unsafe { crate::mem::free(pid_ptr as *mut u8) };

        Some(Self(string))
    }

    /// Return the pid
    pub fn pid(&self) -> String {
        self.0.clone()
    }

    /// Check if the process is alive
    pub fn alive(&self) -> bool {
        let id = &self.0;
        let id_cstr = CString::new(id.clone()).unwrap();
        let alive = unsafe { crate::ffi::hapi_process_alive(id_cstr.as_ptr() as *const u8) };
        alive > 0
    }

    /// Fetch the stdout of the process
    pub fn stdout(&self) -> Option<String> {
        let id = &self.0;
        let id_cstr = CString::new(id.clone()).unwrap();
        let stdout_ptr = unsafe { crate::ffi::hapi_process_stdout(id_cstr.as_ptr() as *const u8) };

        log::info!("Stdout: {:#02x}", stdout_ptr as u32);

        if stdout_ptr == std::ptr::null() {
            return None;
        }

        // # Safety
        // Since the string is garunteed to be null ternimated, there is no way of accessing unallocated memory
        let cstring = unsafe { CStr::from_ptr(stdout_ptr as *const i8) };
        let string = cstring.to_string_lossy().to_string();
        let string = string.to_owned();

        unsafe { crate::mem::free(stdout_ptr as *mut u8) };

        Some(string)
    }
}
