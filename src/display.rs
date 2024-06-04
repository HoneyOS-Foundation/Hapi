use std::{ffi::CString, marker::PhantomData};

use crate::util::keys::KeyCode;

/// The errors for the display
#[derive(Debug)]
pub enum DisplayError {
    NotRegistered,
}

/// The os's display
pub struct Display(PhantomData<()>);

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
    pub fn push_stdout() -> Result<(), DisplayError> {
        let result = unsafe { crate::ffi::hapi_display_push_stdout() };
        if result == -1 {
            return Err(DisplayError::NotRegistered);
        }
        Ok(())
    }

    /// Set the text on the display's text buffer
    pub fn set_text(text: impl Into<String>) -> Result<(), DisplayError> {
        let text: String = text.into();
        let text_cstr = CString::new(text.clone()).unwrap();

        let result = unsafe { crate::ffi::hapi_display_set_text(text_cstr.as_ptr() as *const u8) };
        if result == -1 {
            return Err(DisplayError::NotRegistered);
        }

        Ok(())
    }

    /// Get the key from the the key buffer and clear it
    pub fn key_buffer() -> Option<KeyPress> {
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
