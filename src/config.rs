use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub paths: BackupPaths,
    pub backup: BackupOptions,
}

#[derive(Debug, Deserialize)]
pub struct BackupPaths {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct BackupOptions {
    pub destination: String,
    pub keep_versions: bool,
    pub max_versions: Option<u32>,
}

pub fn load_config(path: &Path) -> anyhow::Result<Config> {
    let data = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&data)?)
}