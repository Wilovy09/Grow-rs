use clap::{Parser, Subcommand};
mod functions;
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
#[command(about = "Seeders", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|err| {
        eprintln!(
            "Error: {}. Please, be sure to set the `DATABASE_URL` environment variable.",
            err
        );
        std::process::exit(1);
    });

    match &cli.command {
        Commands::Init => functions::init_seeder(),
        Commands::New { name } => functions::create_seeder(name),
        Commands::List => functions::list_seeders(),
        Commands::Run { file_name } => {
            if let Err(e) = functions::infer_database(db_url, file_name.as_ref()).await {
                eprintln!("Error while executing seeders: {}", e);
            }
        }
    }
}
