use std::{fmt::{self}, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct ChunkType {
    chunk_type: [u8; 4],
}

#[derive(Debug)]
pub enum ChunkTypeError {
    InvalidCharacter,
    InvalidThirdByte,
    InvalidArgument,
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(values: [u8; 4]) -> Result<Self, ChunkTypeError> {
        if values.is_empty() {
            return Err(ChunkTypeError::InvalidArgument);
        }

        if values[2].is_ascii_lowercase() {
            return Err(ChunkTypeError::InvalidThirdByte);
        }

        for i in values {
            if !ChunkType::is_byte_valid(i) {
                return Err(ChunkTypeError::InvalidCharacter);
            }
        }

        Ok(ChunkType{chunk_type: values})
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ChunkTypeError::InvalidArgument);
        }

        if s.as_bytes().iter().any(|e| e.is_ascii_digit()) {
            return Err(ChunkTypeError::InvalidCharacter);
        }

        Ok(ChunkType { chunk_type: s.as_bytes().to_vec().try_into().unwrap() })
    }
}

impl fmt::Display for ChunkType {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.chunk_type).unwrap())
    }
}

impl ChunkType {

    pub fn bytes(&self) -> [u8; 4] {
        self.chunk_type
    }

    pub fn is_critical(&self) -> bool {
        self.chunk_type[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.chunk_type[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.chunk_type[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        !self.chunk_type[3].is_ascii_uppercase()
    }

    pub fn is_valid(&self) -> bool {
        if !self.is_reserved_bit_valid() {
            return false;
        }

        if !self.chunk_type.iter().any(|&e| ChunkType::is_byte_valid(e)) {
            return false;
        }
        true
    }

    pub fn is_byte_valid(value: u8) -> bool {
        if (value >= 97 && value <= 122)
        || (value >= 65 && value <= 90) {
            return true;
        }
        false
    }
}
