use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "climenv")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init { tag: String },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { tag } => {
            println!("init called with tag: {}", tag);
        }
    }
}
