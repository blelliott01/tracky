use clap::{Parser, Subcommand};
use std::path::Path;

mod json_models;
mod models;
mod scanner;
mod tag_reader;
mod tag_rules;
mod tag_validator;

#[derive(Parser)]
#[command(name = "tracky-cli")]
#[command(about = "Music library scanner & validator", long_about = None)]
struct Cli {
    #[arg(global = true, long = "json")]
    pub json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scan { path: String },
    Validate { path: String },
    Read { file: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
            let path = std::path::Path::new(&path);

            if !path.exists() {
                eprintln!("Path does not exist: {}", path.display());
                std::process::exit(1);
            }

            println!("Scanning {} ...", path.display());

            // 🔥 This is the correct call — matches your Swift scanner
            let files = scanner::scan_folder(path);

            // You likely want to filter like Swift:
            // only .m4a files
            let m4a: Vec<_> = files
                .into_iter()
                .filter(|f| f.path.to_lowercase().ends_with(".m4a"))
                .collect();

            for f in &m4a {
                println!("{} ({} bytes)", f.path, f.size);
            }

            println!("Done. {} m4a files found.", m4a.len());
        }

        Commands::Validate { path } => {
            println!("VALIDATE → {:?}", path);
        }

        Commands::Read { file } => {
            let p = Path::new(&file);

            if !p.exists() {
                eprintln!("File does not exist: {}", p.display());
                std::process::exit(1);
            }

            match tag_reader::read_tags(p) {
                Some(tags) => {
                    println!("{:#?}", tags);
                }
                None => {
                    eprintln!("Could not read tags for {}", file);
                }
            }
        }
    }
}
