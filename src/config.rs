use serde::Deserialize;

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
    pub max_versions: Option<u32>,
}
