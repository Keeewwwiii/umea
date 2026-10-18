
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "UMEA",
    version,
    about = "A lightweight runtime for web apps"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    New {
        name: String,
    },

    Run {
        path: PathBuf,

        #[arg(long, default_value_t = 2020)]
        port: u16,

        #[arg(long, default_value_t = true)]
        standalone:bool
    },

    Build {
        path: PathBuf,
    },
}
