mod commands;
mod utils;

use clap::{Parser, Subcommand};
use dotenv::dotenv;

#[derive(Subcommand)]
enum Commands {
    Init,
    New {
        name: String,
    },
    List,
    Run {
        file_name: Option<String>,
        #[clap(long, help = "Execute all pending seeders")]
        all: bool,
    },
    Status,
}

#[derive(Parser)]
#[command(name = "grow")]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();

    commands::ensure_seeders_dir();

    match &cli.command {
        Commands::Init => commands::init_seeder(),
        Commands::New { name } => commands::create_seeder(name),
        Commands::List => commands::list_seeders(),
        Commands::Run { file_name, all } => {
            if let Err(e) = commands::run_seeder(file_name.as_ref(), *all).await
            {
                eprintln!("\x1b[1;31;91m[ERROR] {e}\x1b[0m");
            }
        }
        Commands::Status => {
            if let Err(e) = commands::list_seeders_status().await {
                eprintln!("\x1b[1;31;91m[ERROR] {e}\x1b[0m");
            }
        }
    }
}
