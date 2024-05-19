use super::fslabel::FsLabel;

/// Represents a fs error
#[derive(Debug)]
pub enum Error {
    NoFsLabel(String),
    NotAFsLabel(String),
    NoSuchFile(String),
    NoSuchFileWithId(String),
    FileExists(String),
    NoSuchDirectory(String),
    DirExists(String),
    NoSuchFs(FsLabel),
    NotEnoughSpace(FsLabel),
    FsAlreadyExists(FsLabel),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoFsLabel(s) => writeln!(f, "The path: {} contains no fs label", s),
            Self::NotAFsLabel(s) => writeln!(f, "{} is not a valid fs label", s),
            Self::NoSuchFile(s) => writeln!(f, "No such file: {}", s),
            Self::NoSuchFileWithId(s) => writeln!(f, "No such file with id: {}", s),
            Self::FileExists(s) => writeln!(f, "File already exists at: {}", s),
            Self::NoSuchDirectory(s) => writeln!(f, "No such directory: {}", s),
            Self::NoSuchFs(l) => writeln!(f, "No fs mounted at: {}", l),
            Self::NotEnoughSpace(l) => writeln!(f, "No space left on device: {}", l),
            Self::DirExists(s) => writeln!(f, "Dir already exists at: {}", s),
            Self::FsAlreadyExists(l) => writeln!(
                f,
                "Could not mount file sytem. A file system is already mounted at: {}",
                l
            ),
        }
    }
}
