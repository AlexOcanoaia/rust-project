use arguments::{CliCommands, Commands};
use clap::Parser;
use commands::print;

mod chunk_types;
mod chunk;
mod png;
mod arguments;
mod commands;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;


fn main() {
    println!("Hello, world!");

    let cli = CliCommands::parse();

    match cli.command {
        Commands::Print(args) => {
            print(args);
        }
        _ => {}
    }
}
