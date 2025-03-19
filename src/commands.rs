use std::{path::PathBuf, str::FromStr};

use crate::{arguments::{DecodeArguments, EncodeArguments, PrintArguments, RemoveArguments}, chunk::Chunk, chunk_types::ChunkType, png::Png};

#[derive(PartialEq)]
pub enum CommandsError {
    InvalidPNG,
    InvalidChunk,
    InvalidSave,
}

pub fn encode(args: EncodeArguments) -> Result<Png, CommandsError> {
    let mut png = open_png(&args.file_path);
    if !check_chunks(&png) {
        return Err(CommandsError::InvalidPNG);
    }
    let chunk_type = ChunkType::from_str(&args.chunk_type).unwrap();
    if !chunk_type.is_valid() {
        return Err(CommandsError::InvalidChunk);
    }

    let chunk = Chunk::new(chunk_type, args.message.as_bytes().to_vec());
    png.append_chunk(chunk);

    if let Err(_e) = png.save(&args.file_path) {
        return Err(CommandsError::InvalidSave);
    }
    return Ok(png);
}

pub fn decode(args: DecodeArguments) -> Result<(), CommandsError> {
    let png = open_png(&args.file_path);
    if !check_chunks(&png) {
        return Err(CommandsError::InvalidPNG);
    }
    let chunk_type = ChunkType::from_str(&args.chunk_type).unwrap();
    if !chunk_type.is_valid() {
        return Err(CommandsError::InvalidChunk);
    }

    let tmp = png.chunk_by_type(&args.chunk_type);
    println!("tmp is {:?}", tmp);
    match tmp {
        Some(value) => {
            let aux = value.data();
            let message = String::from_utf8_lossy(&aux);
            println!("The message decoded is {}", message);
        }
        None => {
            return Err(CommandsError::InvalidChunk);
        }
    }
    Ok(())
}

pub fn print(args: PrintArguments) -> Result<(), CommandsError> {
    let png = open_png(&args.file_path);
    if !check_chunks(&png) {
        return Err(CommandsError::InvalidPNG);
    }
    println!("{}", png);
    Ok(())
}

pub fn remove(args: RemoveArguments) -> Result<(), CommandsError> {
    let mut png = open_png(&args.file_path);
    if !check_chunks(&png) {
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

fn check_chunks(png: &Png) -> bool {
    if png.header() != Png::STANDARD_HEADER {
        return false;
    }

    if !png.chunks().iter().any(|e| e.chunk_type().is_valid()) {
        return false;
    }
    true
}

fn open_png(file_path: &PathBuf) -> Png {
    let tmp_png = Png::from_file(file_path.to_path_buf());
    let png = tmp_png.unwrap();
    png
}
