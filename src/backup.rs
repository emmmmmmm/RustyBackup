use crate::config::Config;
use crate::journal;
use crate::state::load_or_init_state;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
struct Status {
    state: String,
}

#[derive(Serialize, Deserialize)]
struct FileList {
    files: Vec<PathBuf>,
}

#[derive(Serialize, Deserialize)]
struct TempBackup {
    status: Status,
    incomplete: FileList,
    completed: FileList,
}

impl TempBackup {
    fn new(files: Vec<PathBuf>) -> Self {
        Self {
            status: Status {
                state: "in_progress".to_string(),
            },
            incomplete: FileList { files },
            completed: FileList { files: Vec::new() },
        }
    }

    fn load(path: &Path) -> anyhow::Result<Self> {
        let data = fs::read_to_string(path)?;
        Ok(toml::from_str(&data)?)
    }

    fn save(&self, path: &Path) -> anyhow::Result<()> {
        let data = toml::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }
}

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

/// Perform a backup run with simple resumable logic.
pub fn run_backup(config: &Config) -> anyhow::Result<()> {
    let dest = PathBuf::from(&config.backup.destination);
    fs::create_dir_all(&dest)?;

    let state_path = dest.join("state.toml");
    let state = load_or_init_state(&state_path)?;
    let since: SystemTime = state.latest.timestamp.into();

    let includes: Vec<PathBuf> = config
        .paths
        .include
        .iter()
        .map(PathBuf::from)
        .collect();

    let changed = journal::changed_files(since, &includes, &config.paths.exclude)?;

    let temp_path = dest.join(".backup.temp");
    let mut progress = if temp_path.exists() {
        let tmp = TempBackup::load(&temp_path)?;
        if tmp.status.state == "in_progress" {
            tmp
        } else {
            TempBackup::new(changed)
        }
    } else {
        TempBackup::new(changed)
    };

    progress.save(&temp_path)?;

    while let Some(path) = progress.incomplete.files.first().cloned() {
        let dest_file = dest.join(
            path.file_name()
                .map(|s| s.to_owned())
                .unwrap_or_else(|| path.as_os_str().to_owned()),
        );
        fs::copy(&path, &dest_file)?;
        progress.completed.files.push(path.clone());
        progress.incomplete.files.remove(0);
        progress.save(&temp_path)?;
    }

    progress.status.state = "completed".to_string();
    progress.save(&temp_path)?;

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


