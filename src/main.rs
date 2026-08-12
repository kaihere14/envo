mod commands;
mod helper;

use crate::commands::key_gen::key_gen;

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
            println!("init called with tag: {}", tag);
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
