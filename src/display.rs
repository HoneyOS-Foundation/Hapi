use std::{ffi::CString, marker::PhantomData};

use crate::util::keys::KeyCode;

/// The errors for the display
#[derive(Debug)]
pub enum DisplayError {
    NotRegistered,
}

/// The display for the process
pub struct Display(String);

/// The display server for honeyos
pub struct DisplayServer;

/// The key buffer stored in the display
#[derive(Debug)]
pub struct KeyPress {
    pub key: i32,
    pub shift: bool,
    pub ctrl: bool,
    _phantom: PhantomData<()>,
}

impl Display {
    /// Push the process's stdout to the display's text buffer
    pub fn push_stdout(&mut self) -> Result<(), DisplayError> {
        let result = unsafe { crate::ffi::hapi_display_push_stdout() };
        if result == -1 {
            return Err(DisplayError::NotRegistered);
        }
        Ok(())
    }

    /// Set the text on the display's text buffer
    pub fn set_text(&mut self, text: impl Into<String>) -> Result<(), DisplayError> {
        let text: String = text.into();
        let result = unsafe { crate::ffi::hapi_display_set_text(text.as_ptr()) };
        if result == -1 {
            return Err(DisplayError::NotRegistered);
        }

        Ok(())
    }

    /// Get the key from the the key buffer and clear it
    pub fn key_buffer(&mut self) -> Option<KeyPress> {
        let key = unsafe { crate::ffi::hapi_display_get_key_buffer() };
        if key <= -1 {
            return None;
        }

        let shift = unsafe { crate::ffi::hapi_display_get_key_shift() } != 0;
        let ctrl = unsafe { crate::ffi::hapi_display_get_key_ctrl() } != 0;

        unsafe { crate::ffi::hapi_display_clear_key() };
        Some(KeyPress {
            key,
            shift,
            ctrl,
            _phantom: PhantomData,
        })
    }
}

impl DisplayServer {
    /// Register a display
    pub fn register() -> Display {
        unsafe { crate::ffi::hapi_display_server_register() };
        Display(crate::process::pid().unwrap())
    }

    /// Claim the display server and display as head display

    pub fn claim(display: &Display) -> Result<(), DisplayError> {
        let id = &display.0;
        let result = unsafe { crate::ffi::hapi_display_server_claim_main(id.as_ptr()) };
        if result == -1 {
            return Err(DisplayError::NotRegistered);
        }
        Ok(())
    }
}

impl KeyPress {
    /// Convert the key buffer to it's textual representation.
    /// Return None if there is no textual representation for the key.
    pub fn to_char(&self) -> Option<char> {
        let code = KeyCode::from(self.key);
        code.to_char(self.shift)
    }
}

impl std::error::Error for DisplayError {}

impl std::fmt::Display for DisplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayError::NotRegistered => {
                writeln!(f, "The current process does not have a display registered")
            }
        }
    }
}
