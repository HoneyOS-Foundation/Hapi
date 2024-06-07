use std::os::raw::c_void;

#[link(wasm_import_module = "hapi")]
extern "C" {
    /// Logs a string to the js console as info
    pub fn hapi_js_console_log_info(string: *const u8);
    /// Logs a string to the js console as a warning
    pub fn hapi_js_console_log_warn(string: *const u8);
    /// Logs a string to the js console as an error
    pub fn hapi_js_console_log_error(string: *const u8);
    /// Clear the process's stdout
    pub fn hapi_stdout_clear();
    /// Clear last line in process's stdout
    pub fn hapi_stdout_clear_line();
    /// Print a string to process's stdout
    pub fn hapi_stdout_write(string: *const u8);
    /// Write the proccess id to the buffer
    /// ### Safety
    /// - The buffer size must be at least 37-bytes or unallocated memory will be written to.
    pub fn hapi_process_get_pid(buffer: *mut u8);
    /// Write the current working directory to the buffer
    /// ### Safety
    /// - The buffer size must be at least the size of `hapi_process_get_cwd_length` or unallocated memory will be written to.
    pub fn hapi_process_get_cwd(buffer: *mut u8);
    // Get the string length of current working directory
    pub fn hapi_process_get_cwd_length() -> u32;
    /// Sets the current working directory for the process.
    /// ### Note
    /// There are no checks to see if the working directory is valid
    /// ### Returns
    /// - `0` On success
    /// - `-1` If the path is invalid
    pub fn hapi_process_set_cwd(path: *const u8) -> i32;
    /// Spawn a wasm binary as a subprocess.
    /// Writes the pid of the process to the provided buffer, unless null.
    /// ### Safety
    /// - The provided buffer must be at least 37-bytes of length or unallocated memory will be written to
    /// ### Returns
    /// - `0` On success
    /// - `-1` On failure
    pub fn hapi_process_spawn_subprocess(bin: *const u8, bin_len: u32, pid_out: *mut u8);
    /// Returns true if the process is alive
    pub fn hapi_process_alive(id: *const u8) -> i32;
    /// Write the stoud of a process to a buffer
    /// ### Safety
    /// - The out buffer must be equal to `hapi_process_stdout_length` or unallocated memory will be written to.
    /// - The id must be at least 37-bytes in length and a valid string or unallocated memory will be read from.
    /// ### Returns
    /// - `0` On success
    /// - `-1` on failure
    pub fn hapi_process_stdout(id: *const u8, out_buffer: *mut u8);
    /// Returns the current length of the stdout buffer
    /// ### Returns
    /// - The length of the stdout buffer
    /// - `-1` If the id cannot be read from memory
    /// ### Safety
    /// - The id must be at least 37-bytes in length and a valid string or unallocated memory will be read from.
    pub fn hapi_process_stdout_length(id: *const u8) -> i32;
    /// Allocate a block of memory and return it's pointer.
    /// ### Returns
    /// - The pointer to the block
    /// - `NULL` if the memory allocation failed
    pub fn hapi_mem_alloc(size: u32) -> *mut c_void;
    /// Reallocate a block of memory and return the new pointer.
    /// ### Returns
    /// - The pointer to the block
    /// - `NULL` if the memory allocation failed
    pub fn hapi_mem_realloc(ptr: *mut c_void, size: u32) -> *mut c_void;
    // hapi_mem_free
    // Free a block of memory
    pub fn hapi_mem_free(ptr: *mut c_void);
    /// Attempt to take control of the display
    /// ### Returns
    /// - `0` On success
    /// - `-1` If the display is occupied
    pub fn hapi_display_assume_control() -> i32;
    /// Override the control over the display
    pub fn hapi_display_override_control();
    /// Release the control over the display.
    /// ### Returns
    /// - `0` On success
    /// - `-1` If the display is occupied
    pub fn hapi_display_release_control() -> i32;
    /// Take away the control over the display from the currently controling process,
    /// regardless of whether the process has control.
    pub fn hapi_display_displace_control(); // "We workers must take control of the means of display" - GetAGripGal
    /// Push stdout to the display's text-mode buffer.
    /// Do nothing if the process does not have control of the display.
    /// ### Returns
    /// - `0` On success
    /// - `-1` If the process doesn't have control over the display
    pub fn hapi_display_push_stdout() -> i32;
    /// Set the text in the displays text-mode buffer.
    /// Do nothing if the process does not have control of the display.
    /// ### Returns
    /// - `0` On success
    /// - `-1` If the process doesn't have control over the display
    /// - `-2` If the string is invalid
    pub fn hapi_display_set_text(text: *const u8) -> i32;
    /// Get the key in the displays key buffer. Clears it afterwards.
    /// Do nothing if the process does not have control of the display.
    /// ### Returns
    /// - The key stored in the key buffer
    /// - `-1` If the process doesn't have control over the display
    /// - `-2` or if the key buffer is empty.
    pub fn hapi_display_get_key_buffer() -> i32;
    /// Whether or not the shift key is in the key buffer
    /// Do nothing if the process does not have control of the display.
    /// ### Returns
    /// - Whether the shift key is pressed in the key buffer
    /// - `-1` If the process doesn't have control over the display
    pub fn hapi_display_get_key_shift() -> i32;
    /// Whether or not the control key is in the key buffer
    /// Do nothing if the process does not have control of the display.
    /// ### Returns
    /// - Whether the ctrl key is pressed in the key buffer
    /// - `-1` If the process doesn't have control over the display
    pub fn hapi_display_get_key_ctrl() -> i32;
    /// Clears the key buffer of the display
    /// Do nothing if the process does not have control of the display.
    /// ### Returns
    /// - `0` On Success
    /// - `-1` If the process doesn't have control over the display
    pub fn hapi_display_clear_key();
    /// Get the time in seconds since the start of the process
    pub fn hapi_time_since_startup() -> f64;
    /// Returns a pointer to the user agent.
    /// ### Returns
    /// - The user agent on success
    /// - `NULL`` if failed allocate the string.
    pub fn hapi_browser_user_agent() -> *const u8;
    /// Returns the length of the user agent string
    pub fn hapi_browser_user_agent_length() -> u32;
    /// Returns whether the browser is online
    pub fn hapi_browser_is_online() -> u32;
    /// Create a network request and return it's id.
    /// ### Returns:
    /// - The id of the request success
    /// - NULL if the request method was invalid, or when failed to parse headers as json.
    /// ### Methods:
    /// - Get = 0
    /// - Head = 1
    /// - Post = 2
    /// - Put = 3
    /// - Delete = 4
    /// - Connect = 5
    /// - Options = 6
    /// - Trace = 7
    /// - Patch = 8
    pub fn hapi_network_request(url: *const u8, method: u32, headers: *const u8) -> *const u8;
    /// Create a network request to the local server and return it's id.
    /// ### Returns:
    /// - The id of the request on success
    /// - NULL if the request method was invalid, or when failed to parse headers as json.
    /// ### Methods:
    /// - Get = 0
    /// - Head = 1
    /// - Post = 2
    /// - Put = 3
    /// - Delete = 4
    /// - Connect = 5
    /// - Options = 6
    /// - Trace = 7
    /// - Patch = 8
    pub fn hapi_network_request_local(url: *const u8, method: u32, headers: *const u8)
        -> *const u8;
    /// Check the status of the request
    /// ### Returns
    /// - `-1` if the request does not exists.
    /// - `0`if the request is pending.
    /// - `1`if the request succeeded.
    /// - `2`if the request failed.
    /// - `3`if the request is still pending
    pub fn hapi_network_request_status(id: *const u8) -> i32;
    /// Check the lenght of the data in bytes.
    /// ### Returns
    /// - The data length on success
    /// - -1 if the request does not exist.
    pub fn hapi_network_request_data_length(id: *const u8) -> i32;
    /// Check the data in a request
    /// ### Returns
    /// - The data on success
    /// - NULL if the request does not exist,
    /// - NULL if the request has failed or is still pending,
    /// - NULL if the memory allocation failed.
    pub fn hapi_network_request_data(id: *const u8) -> *const u8;
    /// Drop the request from memory.
    /// Does nothing if the request does not exist.
    pub fn hapi_network_request_drop(id: *const u8);
    /// Register a ram filesystem with the provided label.
    /// ### Returns
    /// - `0` On success
    /// - `-1` If the label char is invalid
    /// - `-2` If the label is already occupied
    /// ### Panics
    /// Panics if the filesystem is poisoned.
    pub fn hapi_fs_init_ramfs(label: u8) -> i32;
    /// Create a file at the path.
    /// ### Returns
    /// - `0` On success
    /// - `-1` If the directory doesn't exist
    /// - `-2` If a file with the name already exists
    /// - `-3` If the path string is invalid
    /// ### Panics
    /// Panics if the filesystem is poisoned.
    pub fn hapi_fs_file_create(path: *const u8) -> i32;
    /// Find a file at disk and return it's id
    /// ### Returns
    /// - `0` On success
    /// - `-1` if the file does not exist or if the path is incorrect.
    /// ### Panics
    /// Panics if the filesystem is poisoned.
    /// ### Safety
    /// The destination buffer must be the size of a UUID (37 bytes),
    /// otherwise the remaining bytes will be written to unallocated memory and can cause UB.
    pub fn hapi_fs_file_get(path: *const u8, id_buf: *mut u8) -> i32;
    /// Write a set amount of bytes to a file
    /// ### Returns
    /// - `0` On success
    /// - `-1` if the file does not exist or if the path is incorrect.
    /// - `-2` If the file label does not correspond to an active fs
    /// - `-3` If there is not enough space
    /// ### Panics
    /// Panics if the filesystem is poisoned.
    /// ### Safety
    /// If the size of the buffer is smaller than the reported, unallocated memory will be read from and can cause UB.
    pub fn hapi_fs_file_write(
        fs_label: u8,
        file_id: *const u8,
        offset: u32,
        size: u32,
        buffer: *const u8,
    ) -> i32;
    /// Read a set amount of bytes from the file and write it to a buffer
    /// ### Returns
    /// - `0` On success
    /// - `-1` if the file does not exist or if the path is incorrect.
    /// - `-2` If the file label does not correspond to an active fs
    /// ### Panics
    /// Panics if the filesystem is poisoned.
    /// ### Safety
    /// If the size of the buffer is smaller than the reported, unallocated memory will be written to and can cause UB.
    pub fn hapi_fs_file_read(
        fs_label: u8,
        file_id: *const u8,
        offset: u32,
        size: u32,
        buffer: *mut u8,
    ) -> i32;
    /// Return a file's length
    /// ### Returns
    /// - `0` On success
    /// - `-1` if the file does not exist or if the path is incorrect.
    /// - `-2` If the fs label does not correspond to an active fs
    /// ### Panics
    /// Panics if the filesystem is poisoned.
    pub fn hapi_fs_file_size(fs_label: u8, file_id: *const u8) -> i32;
    /// Create a directory at the path.
    /// ### Returns
    /// - `0` On success
    /// - `-1` If the directory doesn't exist
    /// - `-2` If a directory with the name already exists
    /// - `-3` If the path string is invalid
    /// ### Panics
    /// Panics if the filesystem is poisoned.
    pub fn hapi_fs_directory_create(path: *const u8) -> i32;
    /// Find a directory at disk and return it's id
    /// ### Returns
    /// - `0` On success
    /// - `-1` if the directory does not exist or if the path is incorrect.
    /// - `-2` If the fs label does not correspond to an active fs
    /// ### Panics
    /// Panics if the filesystem is poisoned.
    /// ### Safety
    /// The destination buffer must be the size of a UUID (37 bytes),
    /// otherwise the remaining bytes will be written to unallocated memory and can cause UB.
    pub fn hapi_fs_directory_get(path: *const u8, id_buf: *mut u8) -> i32;
    // Spawn a function pointer on a new thread
    pub fn hapi_thread_spawn(f_ptr: *const c_void);
}
