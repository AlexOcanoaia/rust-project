use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct ChunkType {
    chunk_type: [u8; 4],
}

#[derive(Debug)]
pub enum ChunkTypeError {
    InvalidCharacter,
    InvalidThirdByte,
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(values: [u8; 4]) -> Result<Self, ChunkTypeError> {
        if values[2].is_ascii_lowercase() {
            return Err(ChunkTypeError::InvalidThirdByte);
        }

        Ok(ChunkType{chunk_type: values})
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ChunkType { chunk_type: s.as_bytes().to_vec().try_into().unwrap() })
    }
}

impl ChunkType {

    pub fn bytes(self) -> [u8; 4] {
        self.chunk_type
    }
}
