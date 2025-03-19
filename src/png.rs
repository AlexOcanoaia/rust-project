use std::{fmt::{self}, fs::File, io::{self, BufWriter, Read, Write}, path::{Path, PathBuf}, str::FromStr};

use crate::{chunk::Chunk, chunk_types::ChunkType};

#[derive(Debug, Clone)]
pub struct Png {
    header: [u8; 8],
    chunks: Vec<Chunk>,
}

#[derive(Debug)]
pub enum PngErrors {
    InvalidHeader,
    InvalidArgument,
    ChunkNotFound,
}

impl TryFrom<&[u8]> for  Png {
    type Error = PngErrors;

    fn try_from(values: &[u8]) -> Result<Self, Self::Error> {
        if values.len() < 8 {
            return Err(PngErrors::InvalidArgument);
        }
        let (first, second) = values.split_at(8);
        let result = copy_values(first);

        if !check(&result, Png::STANDARD_HEADER) {
            return Err(PngErrors::InvalidHeader);
        }

        let mut chunks: Vec<Chunk> = Vec::new();
        let mut count = 0;
        while let Some(_value) = second.get(count) {
            let tmp_array = &second[count..(count + 4)];
            let tmp_value = u32::from_be_bytes(tmp_array.try_into().expect("Length should be 4"));
            let length = 12 + tmp_value;
            let chunk = Chunk::try_from(&second[count..(count + length as usize)]).unwrap();
            chunks.push(chunk);
            count = count + length as usize;
        }

        Ok(Self {
            header: result,
            chunks,
        } )
    }
}

impl fmt::Display for Png {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_bytes())
    }
}

impl Png {
    pub const STANDARD_HEADER: &[u8] = &[137, 80, 78, 71, 13, 10, 26, 10];

    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        let header = copy_values(Png::STANDARD_HEADER);
        Self {
            header,
            chunks,
        }
    }

    pub fn save(&self, file_path: &Path) -> io::Result<()> {
        let file = File::create(file_path)?;
        let mut writer = BufWriter::new(file);

        writer.write_all(Png::STANDARD_HEADER)?;

        for chunk in &self.chunks() {
            self.write_chunk(chunk, &mut writer)?;
        }
        Ok(())
    }

    pub fn write_chunk(&self, chunk: &Chunk, writer: &mut BufWriter<File>) -> io::Result<()> {
        let bytes = chunk.as_bytes();
        writer.write_all(&bytes)?;
        Ok(())
    }

    pub fn chunks(&self) -> Vec<Chunk> {
        self.chunks.clone()
    }

    pub fn header(&self) -> &[u8; 8] {
        &self.header
    }

    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    pub fn remove_first_chunk(&mut self, chunk_type: &str) -> Result<Chunk, PngErrors> {
        let chunk = self.chunk_by_type(chunk_type);
        if chunk == None {
            return Err(PngErrors::ChunkNotFound);
        }

        let tmp = chunk.unwrap();
        if let Some(index) = self.chunks.iter().position(|e| *e == tmp) {
            self.chunks.remove(index);
        }
        Ok(tmp)
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<Chunk> {
        for value in &self.chunks() {
            if value.chunk_type() == &ChunkType::from_str(chunk_type).unwrap() {
                return Some(value.clone());
            }
        }
        return None;
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.append(&mut self.header.to_vec());
        for value in &self.chunks {
            result.append(&mut value.as_bytes());
        }
        result
    }

    pub fn from_file(file_path: PathBuf) -> Result<Self, PngErrors> {
        let mut file = File::open(file_path).unwrap();

        let mut data:  Vec<u8> = Vec::new();
        file.read_to_end(&mut data).unwrap();

        let tmp: &[u8] = &data;
        let png = Png::try_from(tmp);
        png
    }
}

fn check(arr1: &[u8; 8], arr2: &[u8]) -> bool {
    let length1 = arr1.len();
    let length2 = arr2.len();
    if length1 != length2 {
        return false;
    }

    for i in 0..length1 {
        if arr1[i] != arr2[i] {
            return false;
        }
    }
    true
}

fn copy_values(arr: &[u8]) -> [u8; 8] {
    let mut result = [0u8; 8];
    for i in 0..=7 {
        result[i] = arr[i];
    }
    result
}
