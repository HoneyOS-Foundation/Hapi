use std::{ffi::CString, path::PathBuf};

use super::{error::Error, fslabel::FsLabel};

/// Represents a file on file system
#[derive(Debug, Clone)]
pub struct File {
    fs_label: FsLabel,
    id: CString,
}

impl File {
    /// Open a file
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, Error> {
        let path: PathBuf = path.into();
        let path = path.to_str().unwrap();

        let path_cstr = CString::new(path).unwrap();
        let mut id_buf = vec![0; 37];

        let result = unsafe {
            crate::ffi::hapi_fs_file_get(path_cstr.as_ptr() as *const u8, &mut id_buf[0] as *mut u8)
        };

        if result < 0 {
            return Err(Error::NoSuchFile(path.to_owned()));
        }

        let id_str = CString::from_vec_with_nul(id_buf).unwrap();
        return Ok(Self {
            id: id_str,
            fs_label: FsLabel::extract_from_path(path).unwrap(),
        });
    }

    /// Create a file
    pub fn create(path: impl Into<PathBuf>) -> Result<Self, Error> {
        let path: PathBuf = path.into();
        let path = path.to_str().unwrap();

        let path_cstr = CString::new(path).unwrap();

        let result = unsafe { crate::ffi::hapi_fs_file_create(path_cstr.as_ptr() as *const u8) };
        if result < 0 {
            return Err(match result {
                -1 => Error::NoSuchDirectory(path.to_owned()),
                -2 => Error::FileExists(path.to_owned()),
                -3 => Error::NoSuchDirectory(path.to_owned()),
                _ => Error::NoSuchDirectory(path.to_owned()),
            });
        }

        Self::open(path)
    }

    /// Read data from the file
    pub fn read(&self, offset: usize, size: usize) -> Result<Vec<u8>, Error> {
        let mut buffer = vec![0u8; size];
        let result = unsafe {
            crate::ffi::hapi_fs_file_read(
                self.fs_label.into(),
                self.id.as_ptr() as *const u8,
                offset as u32,
                size as u32,
                &mut buffer[0] as *mut u8,
            )
        };
        if result < 0 {
            return Err(match result {
                -2 => Error::NoSuchFs(self.fs_label),
                _ => Error::NoSuchFileWithId(self.id.to_string_lossy().into()),
            });
        }

        Ok(buffer)
    }

    /// Read all the data from the file
    pub fn read_all(&self) -> Result<Vec<u8>, Error> {
        self.read(0, self.size()?)
    }

    /// Write data to the file
    pub fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), Error> {
        let result = unsafe {
            crate::ffi::hapi_fs_file_write(
                self.fs_label.into(),
                self.id.as_ptr() as *const u8,
                offset as u32,
                data.len() as u32,
                &data[0] as *const u8,
            )
        };

        match result {
            -1 => Err(Error::NoSuchFileWithId(self.id.to_string_lossy().into())),
            -2 => Err(Error::NoSuchFs(self.fs_label)),
            -3 => Err(Error::NotEnoughSpace(self.fs_label)),
            _ => Ok(()),
        }
    }

    /// Get the file size
    pub fn size(&self) -> Result<usize, Error> {
        let result = unsafe {
            crate::ffi::hapi_fs_file_size(self.fs_label.into(), self.id.as_ptr() as *const u8)
        };
        if result < 0 {
            return Err(match result {
                -1 => Error::NoSuchFileWithId(self.id.to_string_lossy().into()),
                -2 => Error::NoSuchFs(self.fs_label),
                _ => Error::NoSuchFileWithId(self.id.to_string_lossy().into()),
            });
        }
        Ok(result as usize)
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
