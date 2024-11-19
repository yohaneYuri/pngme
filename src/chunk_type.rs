use std::{error::Error, fmt::{self, Display}, str::{self, FromStr}};

#[derive(Debug)]
pub enum ChunkTypeError {
    IllegalByte,
    UnexpectedLength,
}

impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IllegalByte => write!(f, "Invalid chunk type: bytes must in alphabetic"),
            Self::UnexpectedLength => write!(f, "Error while converting from str: length must be 4"),
        }
    }
}

impl Error for ChunkTypeError {}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ChunkType(u32);

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }

    pub fn is_valid(&self) -> bool {
        self.bytes().iter().all(|&byte| byte.is_ascii_alphabetic()) && self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        self.bytes()[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.bytes()[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes()[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes()[3].is_ascii_lowercase()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;
    
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if value.iter().all(|byte| !byte.is_ascii_alphabetic()) {
            Err(Self::Error::IllegalByte)
        } else {
            Ok(Self(u32::from_be_bytes(value)))
        }
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(Self::Err::UnexpectedLength);
        }
        if !s.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(Self::Err::IllegalByte);
        }

        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&s.as_bytes());

        Ok(Self(u32::from_be_bytes(bytes)))
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", str::from_utf8(&self.bytes()).unwrap())
    }
}
