use arguments::{CliCommands, Commands};
use clap::Parser;
use commands::{print, remove};

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
        _ => {
            println!("Another command");
        }
    }
}
