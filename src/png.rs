use crate::chunk::Chunk;

#[derive(Debug)]
pub struct Png {
    header: [u8; 8],
    chunks: Vec<Chunk>,
}

#[derive(Debug)]
pub enum PngErrors {
    InvalidHeader,
    InvalidArgument,
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
        while let Some(value) = second.get(count) {
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

impl Png {
    pub const STANDARD_HEADER: &[u8] = &[137, 80, 78, 71, 13, 10, 26, 10];

    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        let header = copy_values(Png::STANDARD_HEADER);
        Self {
            header,
            chunks,
        }
    }

    pub fn chunks(&self) -> Vec<Chunk> {
        self.chunks.clone()
    }

    pub fn header(&self) -> &[u8; 8] {
        &self.header
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
