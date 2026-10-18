
mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};

type AppResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result: AppResult<()> = match cli.command {
        Commands::New { name } => {
            commands::new::execute(&name)
                .map_err(|error| Box::new(error) as Box<dyn std::error::Error + Send + Sync>)
        }

        Commands::Run { path, port, standalone  } => {
            commands::run::execute(path, port, standalone).await
        }

        Commands::Build { path } => {
            println!("Building app at: {}", path.display());
            Ok(())
        }
    };

    if let Err(error) = result {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}
