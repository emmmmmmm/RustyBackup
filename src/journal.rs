use anyhow::Result;
use std::path::{Path, PathBuf};
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
    destination: &Path,
) -> Result<Vec<PathBuf>> {
    let mut builder = GlobSetBuilder::new();
    for pattern in exclude_patterns {
        builder.add(Glob::new(pattern)?);
    }
    let excludes = builder.build()?;

    let mut files = Vec::new();

    for include in include_paths {
        for entry in WalkDir::new(include)
            .into_iter()
            .filter_entry(|e| !excludes.is_match(e.path()))
        {
            let entry = entry?;
            let path = entry.path();
            if entry.file_type().is_file() && !excludes.is_match(path) {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        let needs_update = modified > since || {
                            // compute destination path and check if it exists
                            let (normalized_root, relative) = include_paths
                                .iter()
                                .find_map(|root| {
                                    let root_path = Path::new(root);
                                    path.strip_prefix(root_path).ok().map(|rel| {
                                        let root_label = root_path
                                            .to_string_lossy()
                                            .replace(':', "")
                                            .replace('\\', "-")
                                            .replace('/', "-");
                                        (root_label, rel)
                                    })
                                })
                                .unwrap_or_else(|| ("UnknownSource".into(), path));

                            let dest_file = destination
                                .join(normalized_root)
                                .join(relative);
                            !dest_file.exists()
                        };

                        if needs_update {
                            files.push(path.to_path_buf());
                        }
                    }
                }
            }
        }
    }

    Ok(files)
}

   

