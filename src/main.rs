//! Minimal Rust Backup Tool Scaffold
//! Designed for learning + Codex integration

mod config;
mod backup;
mod journal;

use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};
use config::Config;

#[derive(Parser, Debug)]
#[command(name = "rustybackup")]
#[command(about = "Minimal backup tool using Rust", long_about = None)]
struct Args {
    /// Path to config file
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,

    /// Action to perform
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Scan configured paths and print discovered files
    Scan,
    /// Perform a backup run (placeholder)
    Backup,
    /// Remove outdated backups (placeholder)
    Vacuum,
    /// Show backup status information (placeholder)
    Status,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config_data = fs::read_to_string(&args.config)?;
    let config: Config = toml::from_str(&config_data)?;

    println!("Loaded config: {:?}", config);

    match args.command {
        Commands::Scan => backup::scan(&config)?,
        Commands::Backup => backup::run_backup(&config)?,
        Commands::Vacuum => backup::vacuum(&config)?,
        Commands::Status => backup::status(&config)?,
    }

    Ok(())
}

/* BUILD INSTRUCTIONS (for Codex or manual use)

1. Ensure Rust is installed:
   https://rustup.rs

2. Create project (if not already):
   cargo new rustybackup --bin

3. Add dependencies to Cargo.toml:
   [dependencies]
   clap = { version = "4.5", features = ["derive"] }
   serde = { version = "1.0", features = ["derive"] }
   toml = "0.8"
   anyhow = "1.0"

4. Run examples:
   cargo run -- scan --config config.toml
   cargo run -- backup --config config.toml

5. Test:
   cargo test
*/
