use chunk::Chunk;
use chunk_types::ChunkType;
use png::Png;

mod chunk_types;
mod chunk;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;


fn main() {
    println!("Hello, world!");
}
