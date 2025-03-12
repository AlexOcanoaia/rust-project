use arguments::{CliCommands, Commands};
use clap::Parser;
use commands::{decode, encode, print, remove, CommandsError};

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
                Err(e) => {
                    print_errors(e);
                }
                Ok(_) => {}
            }
        }
        Commands::Remove(args) => {
            match remove(args) {
                Err(e) => {
                    print_errors(e);
                }
                Ok(_) => {}
            }
        }
        Commands::Encode(args) => {
            match encode(args) {
                Err(e) => {
                    print_errors(e);
                }
                Ok(_) => {}
            }
        }
        Commands::Decode(args) => {
            match decode(args) {
                Err(e)  => {
                    print_errors(e);
                }
                Ok(_) => {}
            }
        }
    }
}

fn print_errors(error: CommandsError) {
    if error == CommandsError::InvalidPNG {
        println!("The PNG file is invalid");
    }
    if error == CommandsError::InvalidChunk {
        println!("The chunk type is invalid");
    }
}
