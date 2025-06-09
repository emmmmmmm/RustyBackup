use anyhow::Result;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::collections::HashMap;
use crate::config::Config;

use globset::{Glob, GlobSetBuilder};
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};


/// Return a list of files modified after `since` from the `include_paths`,
/// excluding any paths that match `exclude_patterns`.
///
/// If no changed files are detected, an empty vector is returned.
pub fn changed_files(
    since: SystemTime,
    include_paths: &[PathBuf],
    exclude_patterns: &[String],
    destination: &Path,
    check_destination: bool,
) -> Result<Vec<PathBuf>> {
    let mut builder = GlobSetBuilder::new();
    for pattern in exclude_patterns {
        builder.add(Glob::new(pattern)?);
    }
    let excludes = builder.build()?;

    let mut files = Vec::new();

    let style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta})")
        .unwrap()
        .progress_chars("##-");

    for include in include_paths {
        let entries: Vec<_> = WalkDir::new(include)
            .into_iter()
            .filter_entry(|e| !excludes.is_match(e.path()))
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .collect();

        // these don't really work, because walkdir is what's slowing us down i think. // TODO
        let pb = ProgressBar::new(entries.len() as u64);
        pb.set_style(style.clone());
        pb.set_message(include.display().to_string());

        for entry in entries {
            let path = entry.path();
            pb.inc(1);
            if entry.file_type().is_file() && !excludes.is_match(path) {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        let needs_update = if modified > since {
                            true
                        } else if check_destination {
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
                        } else {
                            false
                        };

                        if needs_update {
                            files.push(path.to_path_buf());
                            //println!( "added {} ", path.display() );
                        }
                    }
                }
            }
        }
        pb.finish_with_message(format!("{} scanned", include.display()));
    }

    Ok(files)
}

/// Scan the backup destination for files that no longer exist in the source
/// directories. Returns a list of backup file paths that should be moved to the
/// `History` folder.
pub fn find_removed_files(dest: &Path, config: &Config) -> Result<Vec<PathBuf>> {
    let mut roots: HashMap<String, PathBuf> = HashMap::new();
    for include in &config.paths.include {
        let p = PathBuf::from(include);
        let label = p
            .to_string_lossy()
            .replace(':', "")
            .replace('\\', "-")
            .replace('/', "-");
        roots.insert(label, p);
    }

    let mut removed = Vec::new();
    for (label, src_root) in &roots {
        let backup_root = dest.join(label);
        if !backup_root.exists() {
            continue;
        }

        for entry in WalkDir::new(&backup_root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let backup_path = entry.path();
            let rel = backup_path.strip_prefix(&backup_root).unwrap();
            let src_path = src_root.join(rel);
            if !src_path.exists() {
                removed.push(backup_path.to_path_buf());
            }
        }
    }

    Ok(removed)
}

   

