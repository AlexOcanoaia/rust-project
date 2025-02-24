use std::str::FromStr;

use chunk_types::ChunkType;


mod chunk_types;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    println!("Hello, world!");
}
