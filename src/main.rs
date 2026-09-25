mod commands;
mod helper;
mod nostr;
mod tests;
use crate::commands::key_gen::*;
use clap::{Parser, Subcommand};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "envo")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Keygen,
    Push {
        /// Tag to publish under. Defaults to the one saved in `.envo-config`
        /// by the last successful push or pull in this directory.
        tag: Option<String>,
    },
    Pull {
        /// Tag to fetch. Defaults to the one saved in `.envo-config` by the
        /// last successful push or pull in this directory.
        tag: Option<String>,
        /// npub of the publisher to trust for this tag. Required the first
        /// time a tag is pulled; remembered afterwards.
        #[arg(long)]
        owner: Option<String>,
    },
}

/// Reports failure through `helper::log` and the exit status, never by
/// returning `Err` from `main`: that would print Rust's own `Error:` line
/// alongside ours, and output belongs to `helper::log` alone.
#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Keygen => {
            key_gen();
            Ok(())
        }
        Commands::Push { tag } => run_push(tag).await,
        Commands::Pull { tag, owner } => run_pull(tag, owner).await,
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            helper::log::fail(&format!("{}", e));
            ExitCode::FAILURE
        }
    }
}

/// Reads the project files `push` needs, then hands them over already parsed.
async fn run_push(tag: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let tag = helper::project_config::resolve_tag(tag)?;
    let files = helper::env_files::load_project_files()?;

    commands::push::push(tag, &files.env_contents, &files.trusted_pubkeys).await
}

/// Resolves the tag to pull, falling back to the saved default.
async fn run_pull(
    tag: Option<String>,
    owner: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let tag = helper::project_config::resolve_tag(tag)?;

    commands::pull::pull(tag, owner).await
}
