use std::{fmt, io::{Cursor, Read}};

use crc32fast::Hasher;

use crate::chunk_types::ChunkType;

#[derive(Debug, Clone)]
struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

#[derive(Debug)]
enum ChunkError {
    InvalidArgument,
    InvalidLengthSize,
    InvalidCrc,
    NotUTF8,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;

    fn try_from(values: &[u8]) -> Result<Self, Self::Error> {
        if values.len() < 12 {
            return Err(ChunkError::InvalidArgument);
        }

        let mut cursor = Cursor::new(values);

        let mut tmp_buffer = [0u8; 4];
        cursor.read_exact(&mut tmp_buffer).unwrap();
        let length = u32::from_be_bytes(tmp_buffer);

        cursor.read_exact(&mut tmp_buffer).unwrap();
        let chunk_type = ChunkType::try_from(tmp_buffer).unwrap();

        if values.len() < (length as usize + 12) {
            return Err(ChunkError::InvalidLengthSize);
        }

        let mut data = vec![0u8; length as usize];

        cursor.read_exact(&mut data).unwrap();
        cursor.read_exact(&mut tmp_buffer).unwrap();

        let crc = u32::from_be_bytes(tmp_buffer);

        let expected_crc = calculate_crc(&chunk_type, data.clone());

        if crc != expected_crc {
            return Err(ChunkError::InvalidCrc);
        }

        Ok(Self {
            length,
            chunk_type,
            data,
            crc,
        })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data_as_string().unwrap())
    }
}

impl Chunk {

    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let length = data.len() as u32;
        let crc = calculate_crc(&chunk_type, data.clone());
        Self {
            length,
            chunk_type,
            data,
            crc,
        }
    }

    fn length(&self) -> u32 {
        self.length
    }

    fn crc(&self) -> u32 {
        self.crc
    }

    fn data(&self) -> Vec<u8> {
        self.data.clone()
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data_as_string(&self) -> Result<String, ChunkError> {
        if !self.data.iter().any(|&e| ChunkType::is_byte_valid(e)) {
            return Err(ChunkError::NotUTF8);
        }

        let result = String::from_utf8(self.data.clone()).unwrap();

        Ok(result)
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.append(&mut self.length.to_be_bytes().to_vec());
        result.append(&mut self.chunk_type.bytes().to_vec());
        result.append(&mut self.data.clone());
        result.append(&mut self.crc.to_be_bytes().to_vec());
        result
    }
}

fn calculate_crc(chunk_type: &ChunkType, data: Vec<u8>) -> u32 {
    let mut hash = Hasher::new();
    hash.update(&chunk_type.bytes());
    hash.update(&data);

    hash.finalize()
}
