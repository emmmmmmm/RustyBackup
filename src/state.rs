use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
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

impl State {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&data)?)
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let data = toml::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }
}
