use std::io::{Cursor, Read};

use crate::chunk_types::ChunkType;

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

        Ok(Self {
            length,
            chunk_type,
            data,
            crc,
        })
    }
}
