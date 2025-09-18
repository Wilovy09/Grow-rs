mod commands;
mod utils;

use clap::{Parser, Subcommand};
use dotenv::dotenv;

#[derive(Subcommand)]
enum Commands {
    Init,
    New { name: String },
    List,
    Run { file_name: Option<String> },
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

    match &cli.command {
        Commands::Init => commands::init_seeder(),
        Commands::New { name } => commands::create_seeder(name),
        Commands::List => commands::list_seeders(),
        Commands::Run { file_name } => {
            if let Err(e) = commands::run_seeder(file_name.as_ref()).await {
                eprintln!("\x1b[1;31;91m[ERROR] {e}\x1b[0m");
            }
        }
    }
}
