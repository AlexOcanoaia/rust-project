use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pngapp", about = "Encoder for PNG file", version = "1.0")]
pub struct CliCommands {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode(EncodeArguments),
    Decode(DecodeArguments),
    Print(PrintArguments),
    Remove(RemoveArguments),
}

#[derive(Parser)]
pub struct EncodeArguments {
    file_path: PathBuf,
    chunk_type: String,
    message: String,
    output_file: PathBuf,
}

#[derive(Parser)]
pub struct DecodeArguments {
    file_path: PathBuf,
    chunk_type: String,
}

#[derive(Parser)]
pub struct PrintArguments {
    pub file_path: PathBuf,
}

#[derive(Parser)]
pub struct RemoveArguments {
    file_path: PathBuf,
    chunk_type: String,
}

