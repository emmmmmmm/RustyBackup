use crate::config::Config;
use walkdir::WalkDir;
use globset::{Glob, GlobSetBuilder};

/// Scan and print all files under the configured include paths.
///
/// Entries matching any of the configured exclude patterns will be skipped.
pub fn scan(config: &Config) -> anyhow::Result<()> {
    let mut builder = GlobSetBuilder::new();
    for pattern in &config.paths.exclude {
        builder.add(Glob::new(pattern)?);
    }
    let excludes = builder.build()?;

    for path in &config.paths.include {
        for entry in WalkDir::new(path).into_iter().filter_entry(|e| !excludes.is_match(e.path())) {
            let entry = entry?;
            if entry.file_type().is_file() {
                println!("{}", entry.path().display());
            }
        }
    }
    Ok(())
}

