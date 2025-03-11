use arguments::{CliCommands, Commands};
use clap::Parser;
use commands::{encode, print, remove, CommandsError};

mod chunk_types;
mod chunk;
mod png;
mod arguments;
mod commands;

fn main() {
    let cli = CliCommands::parse();

    match cli.command {
        Commands::Print(args) => {
            match print(args) {
                Err(_e) => {
                    println!("The PNG file is invalid");
                }
                Ok(_) => {}
            }
        }
        Commands::Remove(args) => {
            match remove(args) {
                Err(_e) => {
                    println!("The PNG file is invalid");
                }
                Ok(_) => {}
            }
        }
        Commands::Encode(args) => {
            match encode(args) {
                Err(e) => {
                    if e == CommandsError::InvalidPNG {
                        println!("The PNG file is invalid");
                    }
                    if e == CommandsError::InvalidChunk {
                        println!("The chunk type is invalid");
                    }
                }
                Ok(_) => {}
            }
        }
        Commands::Decode(_args) => {

        }
        _ => {
            println!("Wrong command");
        }
    }
}
