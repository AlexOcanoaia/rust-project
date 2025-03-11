use std::{path::PathBuf, str::FromStr};

use crate::{arguments::{DecodeArguments, EncodeArguments, PrintArguments, RemoveArguments}, chunk::Chunk, chunk_types::ChunkType, png::Png};

#[derive(PartialEq)]
pub enum CommandsError {
    InvalidPNG,
    InvalidChunk,
}

pub fn encode(args: EncodeArguments) -> Result<(), CommandsError> {
    let mut png = open_png(args.file_path);
    if !check_chunks(png.clone()) {
        return Err(CommandsError::InvalidPNG);
    }
    let chunk_type = ChunkType::from_str(&args.chunk_type).unwrap();
    if !chunk_type.is_valid() {
        return Err(CommandsError::InvalidChunk);
    }

    let chunk = Chunk::new(chunk_type, args.message.as_bytes().to_vec());


    png.append_chunk(chunk);
    Ok(())
}

pub fn decode(args: DecodeArguments) -> Result<(), ()> {
    todo!()
}

pub fn print(args: PrintArguments) -> Result<(), CommandsError> {
    let png = open_png(args.file_path);
    if !check_chunks(png.clone()) {
        return Err(CommandsError::InvalidPNG);
    }
    println!("{}", png);
    Ok(())
}

pub fn remove(args: RemoveArguments) -> Result<(), CommandsError> {
    let mut png = open_png(args.file_path);
    if !check_chunks(png.clone()) {
        return Err(CommandsError::InvalidPNG);
    }

    let result = png.remove_first_chunk(args.chunk_type.as_str());

    match result {
        Err(_e) => {
            println!("The chunk doesn't exist");
        }
        Ok(_) => {}
    }
    Ok(())    
}

fn check_chunks(png: Png) -> bool {
    if png.header() != Png::STANDARD_HEADER {
        return false;
    }

    if !png.chunks().iter().any(|e| e.chunk_type().is_valid()) {
        return false;
    }
    true
}

fn open_png(file_path: PathBuf) -> Png {
    let tmp_png = Png::from_file(file_path);
    let png = tmp_png.unwrap();
    png
}

