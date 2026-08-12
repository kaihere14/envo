mod commands;
mod helper;

use crate::commands::key_gen::*;

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
            //complete work is left for now this is just a testing code for my understanding
            let env_contents = std::fs::read_to_string(".env").expect("No file found");
            commands::init::init(tag, &env_contents);
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
