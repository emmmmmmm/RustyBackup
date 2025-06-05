use crate::config::Config;
use crate::journal;
use crate::state::load_or_init_state;
use std::path::PathBuf;
use std::time::SystemTime;

/// Scan and print all files under the configured include paths.
///
/// Entries matching any of the configured exclude patterns will be skipped.
pub fn scan(config: &Config) -> anyhow::Result<()> {
    let includes: Vec<PathBuf> = config
        .paths
        .include
        .iter()
        .map(PathBuf::from)
        .collect();

    let state_path = PathBuf::from(&config.backup.destination).join("state.toml");
    let state = load_or_init_state(&state_path)?;
    let since: SystemTime = state.latest.timestamp.into();

    let files = journal::changed_files(since, &includes, &config.paths.exclude)?;

    println!(
        "Found {} changed files since {}",
        files.len(),
        state.latest.timestamp
    );

    for path in files {
        println!("{}", path.display());
    }

    Ok(())
}

/// Placeholder backup implementation.
pub fn run_backup(config: &Config) -> anyhow::Result<()> {
    let state_path = PathBuf::from(&config.backup.destination).join("state.toml");
    let _state = load_or_init_state(&state_path)?;
    println!("Performing backup...");
    Ok(())
}

/// Placeholder vacuum implementation.
pub fn vacuum(_config: &Config) -> anyhow::Result<()> {
    println!("Vacuuming old backups...");
    Ok(())
}

/// Placeholder status implementation.
pub fn status(_config: &Config) -> anyhow::Result<()> {
    println!("Backup status: OK");
    Ok(())
}


