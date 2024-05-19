use std::{ffi::CString, path::PathBuf};

use super::{error::Error, fslabel::FsLabel};

/// Represetns a directory on a file system
#[derive(Debug, Clone)]
pub struct Directory {
    fs_label: FsLabel,
    id: CString,
}

impl Directory {
    /// Open a directory
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, Error> {
        let path: PathBuf = path.into();
        let path = path.to_str().unwrap();

        let path_cstr = CString::new(path).unwrap();
        let mut id_buf = vec![0; 37];

        let result = unsafe {
            crate::ffi::hapi_fs_directory_get(
                path_cstr.as_ptr() as *const u8,
                &mut id_buf[0] as *mut u8,
            )
        };

        let fs_label = FsLabel::extract_from_path(path)?;

        if result < 0 {
            return Err(match result {
                -1 => Error::NoSuchDirectory(path.to_owned()),
                -2 => Error::NoSuchFs(fs_label),
                _ => Error::NoSuchDirectory(path.to_owned()),
            });
        }

        let id_str = CString::from_vec_with_nul(id_buf).unwrap();
        Ok(Self {
            fs_label,
            id: id_str,
        })
    }

    /// Create a directory
    pub fn create(path: impl Into<PathBuf>) -> Result<Self, Error> {
        let path: PathBuf = path.into();
        let path = path.to_str().unwrap().to_string();

        let path_cstr = CString::new(path.clone()).unwrap();

        let result =
            unsafe { crate::ffi::hapi_fs_directory_create(path_cstr.as_ptr() as *const u8) };
        if result < 0 {
            return Err(match result {
                -1 => Error::NoSuchDirectory(path.to_owned()),
                -2 => Error::DirExists(path.to_owned()),
                -3 => Error::NoSuchDirectory(path.to_owned()),
                _ => Error::NoSuchDirectory(path.to_owned()),
            });
        }

        Self::open(path)
    }

    /// Get the id
    pub fn id(&self) -> &str {
        self.id.to_str().unwrap()
    }

    /// Get the fs label
    pub fn fs(&self) -> FsLabel {
        self.fs_label
    }
}
