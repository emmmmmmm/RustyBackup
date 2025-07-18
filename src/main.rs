//! A Minimal Rust Backup Tool 

// TODO: 
// - vacuum function to remove outdated backups (using max_versions from config file)
// - decide how to handle deleted files (maybe just move to "history")
// 


mod config;
mod backup;
mod journal;
mod state;
mod utils;

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

    /// Perform a full scan checking the backup destination
    #[arg(long, default_value_t = false)]
    fullscan: bool,

    /// Action to perform
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Scan configured paths and print discovered files
    Scan,
    /// Perform a backup run
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
        Commands::Scan => backup::scan(&config, args.fullscan)?,
        Commands::Backup => backup::run_backup(&config)?,
        Commands::Vacuum => backup::vacuum(&config)?,
        Commands::Status => backup::status(&config)?,
    }

    Ok(())
}
