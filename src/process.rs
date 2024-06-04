use std::ffi::CString;

/// Get the process id
pub fn pid() -> Option<String> {
    const PID_LENGTH: usize = std::mem::size_of::<u8>() * 37;
    let mut pid_buf = vec![0u8; PID_LENGTH];

    unsafe { crate::ffi::hapi_process_get_pid(&mut pid_buf[0] as *mut u8) };

    let string = CString::from_vec_with_nul(pid_buf).ok()?;
    let string = string.to_string_lossy().to_string();
    Some(string)
}

/// Get the current working directory
pub fn cwd() -> Option<String> {
    let cwd_length = unsafe { crate::ffi::hapi_process_get_cwd_length() } as usize;
    let mut cwd_buf = vec![0u8; cwd_length];

    unsafe { crate::ffi::hapi_process_get_cwd(&mut cwd_buf[0] as *mut u8) };

    let string = CString::from_vec_with_nul(cwd_buf).ok()?;
    let string = string.to_string_lossy().to_string();

    Some(string)
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

        const PID_LENGTH: usize = std::mem::size_of::<u8>() * 37;
        let mut pid_buf = vec![0u8; PID_LENGTH];

        unsafe {
            crate::ffi::hapi_process_spawn_subprocess(
                bin.as_ptr(),
                bin.len() as u32,
                &mut pid_buf[0] as *mut u8,
            )
        };

        let string = CString::from_vec_with_nul(pid_buf).ok()?;
        let string = string.to_string_lossy().to_string();

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

        const PID_LENGTH: usize = std::mem::size_of::<u8>() * 37;
        let mut pid_buf = vec![0u8; PID_LENGTH];

        unsafe {
            crate::ffi::hapi_process_stdout(
                id_cstr.as_ptr() as *const u8,
                &mut pid_buf[0] as *mut u8,
            )
        };

        let string = CString::from_vec_with_nul(pid_buf).ok()?;
        let string = string.to_string_lossy().to_string();

        Some(string)
    }
}
