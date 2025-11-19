use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod scanner;

#[derive(Parser)]
#[command(name = "tracky-cli")]
#[command(about = "Music library scanner & validator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a folder and output basic info
    Scan {
        /// Path to folder containing music files
        path: PathBuf,
    },

    /// Validate tags in a folder (does not save)
    Validate {
        /// Path to folder containing music files
        path: PathBuf,
    },

    /// Build a SQLite database from the folder
    BuildDb {
        /// Path to folder containing music files
        path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
            let files = scanner::scan_folder(&path);
            println!("Found {} audio files:", files.len());
            for f in files {
                println!(" - {}", f.path.display());
            }
        }
        Commands::Validate { path } => {
            println!("VALIDATE → {:?}", path);
        }
        Commands::BuildDb { path } => {
            println!("BUILD-DB → {:?}", path);
        }
    }
}
