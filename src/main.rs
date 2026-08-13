mod commands;
mod helper;
mod nostr;
mod tests;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Keygen => {
            key_gen();
        }

        Commands::Init { tag } => {
            //complete work is left for now this is just a testing code for my understanding
            let files = match helper::env_files::load_project_files() {
                Ok(files) => files,
                Err(e) => {
                    eprintln!("error: {}", e);
                    return Err(e);
                }
            };

            commands::init::init(tag, &files.env_contents, &files.trusted_pubkeys).await;
        }

        Commands::Pull { tag } => {
            commands::pull::pull(tag).await;
        }

        Commands::Push { tag } => {
            println!("Push called with tag: {}", tag);
        }

        Commands::AddUser { tag, pubkey } => {
            println!("AddUser called with tag: {}, pubkey: {}", tag, pubkey);
        }
    }
    Ok(())
}
