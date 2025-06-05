use crate::config::Config;
use crate::journal;
use std::collections::HashSet;
use std::time::SystemTime;
use walkdir::WalkDir;

/// Scan and print all files under the configured include paths.
///
/// Currently does not apply exclusion filters.
pub fn scan(config: &Config) -> anyhow::Result<()> {
    // Determine changed files since the beginning of time by default.
    let changed = journal::changed_files(SystemTime::UNIX_EPOCH)?;
    let changed_set: HashSet<_> = changed.iter().collect();

    for path in &config.paths.include {
        for entry in WalkDir::new(path) {
            let entry = entry?;
            if entry.file_type().is_file() {
                if changed_set.is_empty() ||
                    changed_set.contains(&entry.path().to_path_buf())
                {
                    println!("{}", entry.path().display());
                }
            }
        }
    }
    Ok(())
}

