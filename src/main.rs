use clap::{Parser, Subcommand};
mod functions;

#[derive(Subcommand)]
enum Commands {
    Init,
    New { name: String },
    List,
    Run { file_name: Option<String> },
}

#[derive(Parser)]
#[command(name = "seed")]
#[command(about = "Seeders", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init => functions::init_seeder(),
        Commands::New { name } => {
            println!("{name}")
        }
        Commands::List => todo!(),
        Commands::Run { file_name } => {
            if let Some(file_name) = file_name {
                println!("Running seeder: {file_name}");
            } else {
                println!("Running all seeders");
            }
        }
    }
}
