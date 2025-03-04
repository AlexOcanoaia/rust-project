use std::path::PathBuf;

use crate::{arguments::{DecodeArguments, EncodeArguments, PrintArguments, RemoveArguments}, png::Png};

pub enum CommandsError {
    InvalidPNG,
}

pub fn encode(args: EncodeArguments) -> Result<(), ()> {
    todo!()
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

