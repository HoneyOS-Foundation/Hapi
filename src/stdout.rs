use std::ffi::CString;

/// Print to honeyos's stdout
#[macro_export]
macro_rules! print {
    ($($t:tt)*) => (hapi::stdout::write(&format_args!($($t)*).to_string()))
}

/// Print a line to honeyos's stdout. With a newline
#[macro_export]
macro_rules! println {
    ($($t:tt)*) => (hapi::stdout::writeln(&format_args!($($t)*).to_string()))
}

/// Clear honeyos's stdout
pub fn clear() {
    // # Safety
    // stdout_clear is garunteed to be safe by honeyos's implementation.
    // If the honeyos kernel is non-standard, there are no safety garuntees and this might be unsound
    unsafe { crate::ffi::hapi_stdout_clear() }
}

/// Clear the last line in honeyos's stdout
pub fn clear_line() {
    // # Safety
    // stdout_clear is garunteed to be safe by honeyos's implementation.
    // If the honeyos kernel is non-standard, there are no safety garuntees and this might be unsound
    unsafe { crate::ffi::hapi_stdout_clear_line() }
}

/// Clear N number of lines in honeyos's stdout.
/// Will only clear up to the amount of lines.
pub fn clear_lines(num: u32) {
    // # Safety
    // stdout_clear is garunteed to be safe by honeyos's implementation.
    // If the honeyos kernel is non-standard, there are no safety garuntees and this might be unsound
    unsafe { crate::ffi::hapi_stdout_clear_lines(num) }
}

/// Write to honeyos's stdout
pub fn write(string: impl Into<String>) {
    let string: String = string.into();
    let string = format!("{}", string);
    let cstring = CString::new(string.clone()).unwrap();
    // # Safety
    // Since the string is garunteed to hae a null terminator, we are garunteed not to write unallocated memory
    unsafe { crate::ffi::hapi_stdout_write(cstring.as_ptr() as *const u8) }
}

/// Write a line to honeyos's stdout
pub fn writeln(string: impl Into<String>) {
    let string: String = string.into();
    let string = format!("{}\n", string);
    let cstring = CString::new(string.clone()).unwrap();
    // # Safety
    // Since the string is garunteed to hae a null terminator, we are garunteed not to write unallocated memory
    unsafe { crate::ffi::hapi_stdout_write(cstring.as_ptr() as *const u8) }
}
