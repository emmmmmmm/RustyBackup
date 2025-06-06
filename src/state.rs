use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use crate::backup;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupState {
    pub latest: LatestBackup,
    pub stats: BackupStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LatestBackup {
    pub timestamp: DateTime<Local>,
    pub snapshot_id: String,
    pub destination: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupStats {
    pub files_synced: u64,
    pub bytes_copied: u64,
    pub duration_ms: u64,
}

impl Default for BackupStats {
    fn default() -> Self {
        Self {
            files_synced: 0,
            bytes_copied: 0,
            duration_ms: 0,
        }
    }
}

impl Default for BackupState {
    fn default() -> Self {
        Self {
            latest: LatestBackup {
                timestamp: DateTime::<Local>::from(SystemTime::UNIX_EPOCH),
                snapshot_id: String::new(),
                destination: PathBuf::new(),
            },
            stats: BackupStats::default(),
        }
    }
}

impl BackupState {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&data)?)
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let data = toml::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn record_backup(&mut self, progress: &backup::TempBackup, dest: &Path) {
        self.latest.timestamp = progress.timestamp;
        self.latest.snapshot_id = progress.snapshot_id.to_string();
        self.latest.destination = dest.to_path_buf();

        self.stats.files_synced = progress.completed.files.len() as u64;
        self.stats.bytes_copied = 0; // still untracked — stub for now
        self.stats.duration_ms = progress.duration.as_millis() as u64;
    }


}

/// Load the backup state from `path`, or initialize it if it does not exist.
pub fn load_or_init_state(path: &Path) -> anyhow::Result<BackupState> {
    if path.exists() {
        BackupState::load(path)
    } else {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let state = BackupState::default();
        state.save(path)?;
        Ok(state)
    }
}
