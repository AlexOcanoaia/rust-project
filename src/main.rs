use chunk_types::ChunkType;

mod chunk_types;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    println!("Hello, world!");
    let expected = [82, 117, 83, 116];
    let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();
    println!("expected is {:?}", expected);
    println!("{:?}", actual);

}
