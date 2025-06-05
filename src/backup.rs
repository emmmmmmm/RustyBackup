use crate::config::Config;
use crate::journal;
use std::path::PathBuf;
use std::collections::HashSet;
use std::time::SystemTime;
use walkdir::WalkDir;
use globset::{Glob, GlobSetBuilder};

/// Scan and print all files under the configured include paths.
///
/// Entries matching any of the configured exclude patterns will be skipped.
pub fn scan(config: &Config) -> anyhow::Result<()> {
    // Determine changed files within the configured include paths since the
    // beginning of time by default.
    let includes: Vec<PathBuf> = config
        .paths
        .include
        .iter()
        .map(PathBuf::from)
        .collect();

    
    use std::time::{SystemTime, Duration};
    let since = SystemTime::now() - Duration::from_secs(60 * 60); // past hour
    let changed = journal::changed_files(since, &includes)?;
    let changed_set: HashSet<_> = changed.iter().collect();

    let mut builder = GlobSetBuilder::new();
    for pattern in &config.paths.exclude {
        builder.add(Glob::new(pattern)?);
    }
    let excludes = builder.build()?;

    for path in &config.paths.include {
         println!("checking path: {} ", path);
        for entry in WalkDir::new(path).into_iter().filter_entry(|e| !excludes.is_match(e.path())) {
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

/// Placeholder backup implementation.
pub fn run_backup(_config: &Config) -> anyhow::Result<()> {
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


