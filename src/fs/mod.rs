pub mod dir;
pub mod error;
pub mod file;
pub mod fslabel;

pub use file::*;

use self::{error::Error, fslabel::FsLabel};

/// Represents a ram file system
pub struct RamFileSystem;

impl RamFileSystem {
    /// Initialzzie a ram file system and mount it at the label
    pub fn init(label: FsLabel) -> Result<(), Error> {
        let result = unsafe { crate::ffi::hapi_fs_init_ramfs(label.into()) };
        if result < 0 {
            return Err(Error::FsAlreadyExists(label));
        }
        Ok(())
    }
}
