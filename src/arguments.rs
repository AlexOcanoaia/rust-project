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
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
}

#[derive(Parser)]
pub struct DecodeArguments {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Parser)]
pub struct PrintArguments {
    pub file_path: PathBuf,
}

#[derive(Parser)]
pub struct RemoveArguments {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

