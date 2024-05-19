use std::str::FromStr;

use super::error::Error;

/// The label for a mounted file system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[rustfmt::skip]
pub enum FsLabel {
    A,B,C,D,E,
    F,G,H,I,J,
    K,L,M,N,O,
    P,Q,R,S,T,
    U,V,W,X,Y,
    Z,
}

impl FsLabel {
    /// Extract the fs label from a path
    pub fn extract_from_path(path: &str) -> Result<Self, Error> {
        let (fs_label_str, _) = path.split_at(3);
        if !path.contains(':') {
            return Err(Error::NoFsLabel(path.to_owned()));
        }

        let fs_char = fs_label_str
            .get(0..1)
            .ok_or(Error::NoFsLabel(path.to_owned()))?;
        fs_char.parse()
    }
}

impl std::fmt::Display for FsLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

impl Into<u8> for FsLabel {
    fn into(self) -> u8 {
        match self {
            Self::A => 'a' as u8,
            Self::B => 'b' as u8,
            Self::C => 'c' as u8,
            Self::D => 'd' as u8,
            Self::E => 'e' as u8,
            Self::F => 'f' as u8,
            Self::G => 'g' as u8,
            Self::H => 'h' as u8,
            Self::I => 'i' as u8,
            Self::J => 'j' as u8,
            Self::K => 'k' as u8,
            Self::L => 'l' as u8,
            Self::M => 'm' as u8,
            Self::N => 'n' as u8,
            Self::O => 'o' as u8,
            Self::P => 'p' as u8,
            Self::Q => 'q' as u8,
            Self::R => 'r' as u8,
            Self::S => 's' as u8,
            Self::T => 't' as u8,
            Self::U => 'u' as u8,
            Self::V => 'v' as u8,
            Self::W => 'w' as u8,
            Self::X => 'x' as u8,
            Self::Y => 'y' as u8,
            Self::Z => 'z' as u8,
        }
    }
}

impl FromStr for FsLabel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            "e" => Ok(Self::E),
            "f" => Ok(Self::F),
            "g" => Ok(Self::G),
            "h" => Ok(Self::H),
            "i" => Ok(Self::I),
            "j" => Ok(Self::J),
            "k" => Ok(Self::K),
            "l" => Ok(Self::L),
            "m" => Ok(Self::M),
            "n" => Ok(Self::N),
            "o" => Ok(Self::O),
            "p" => Ok(Self::P),
            "q" => Ok(Self::Q),
            "r" => Ok(Self::R),
            "s" => Ok(Self::S),
            "t" => Ok(Self::T),
            "u" => Ok(Self::U),
            "v" => Ok(Self::V),
            "w" => Ok(Self::W),
            "x" => Ok(Self::X),
            "y" => Ok(Self::Y),
            "z" => Ok(Self::Z),
            _ => Err(Error::NotAFsLabel(s.to_owned())),
        }
    }
}
