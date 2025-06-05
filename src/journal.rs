use anyhow::Result;
use std::path::PathBuf;
use std::time::SystemTime;

use globset::{Glob, GlobSetBuilder};
use walkdir::WalkDir;


/// Return a list of files modified after `since` from the `include_paths`,
/// excluding any paths that match `exclude_patterns`.
///
/// If no changed files are detected, an empty vector is returned.
pub fn changed_files(
    since: SystemTime,
    include_paths: &[PathBuf],
    exclude_patterns: &[String],
) -> Result<Vec<PathBuf>> {
    let mut builder = GlobSetBuilder::new();
    for pattern in exclude_patterns {
        builder.add(Glob::new(pattern)?);
    }
    let excludes = builder.build()?;

    let mut files = Vec::new();

    for include in include_paths {
        for entry in WalkDir::new(include).into_iter().filter_entry(|e| !excludes.is_match(e.path())) {
            let entry = entry?;
            if entry.file_type().is_file() && !excludes.is_match(entry.path()) {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        if modified > since {
                            files.push(entry.path().to_path_buf());
                        }
                    }
                }
            }
        }
    }


    Ok(files)
}

   

