use crate::config::Config;
use walkdir::WalkDir;

/// Scan and print all files under the configured include paths.
///
/// Currently does not apply exclusion filters.
pub fn scan(config: &Config) -> anyhow::Result<()> {
    for path in &config.paths.include {
        for entry in WalkDir::new(path) {
            let entry = entry?;
            if entry.file_type().is_file() {
                println!("{}", entry.path().display());
            }
        }
    }
    Ok(())
}

