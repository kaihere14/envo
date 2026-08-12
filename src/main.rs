mod commands;
mod helper;

use crate::commands::key_gen::*;
use crate::helper::key_valid::*;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "envo")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Keygen,

    Init {
        tag: String,
    },

    Pull {
        tag: String,
    },

    Push {
        tag: String,
    },

    AddUser {
        tag: String,
        #[arg(long)]
        pubkey: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Keygen => {
            key_gen();
        }

        Commands::Init { tag } => {
            let keys = load_keys();

            let (public_key, private_key) = match keys {
                Ok((public_key, private_key)) => (public_key, private_key),
                Err(e) => {
                    eprintln!("Error loading keys: {}", e);
                    return;
                }
            };

            let valid = is_valid_keypair(&public_key, &private_key);

            if !valid {
                eprintln!("Invalid keypair");
                return;
            }

            println!("Keys loaded successfully and your tag is : {}", tag);
        }

        Commands::Pull { tag } => {
            println!("Pull called with tag: {}", tag);
        }

        Commands::Push { tag } => {
            println!("Push called with tag: {}", tag);
        }

        Commands::AddUser { tag, pubkey } => {
            println!("AddUser called with tag: {}, pubkey: {}", tag, pubkey);
        }
    }
}
