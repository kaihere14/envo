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
    Push { tag: String },
    Pull { tag: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Keygen => {
            key_gen();
        }
        Commands::Push { tag } => {
            let files = match helper::env_files::load_project_files() {
                Ok(files) => files,
                Err(e) => {
                    helper::log::fail(&format!("{}", e));
                    return Err(e);
                }
            };
            commands::push::push(tag, &files.env_contents, &files.trusted_pubkeys).await;
        }
        Commands::Pull { tag } => {
            commands::pull::pull(tag).await;
        }
    }
    Ok(())
}
