use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::time::SystemTime;

/// Return a list of files modified since the given timestamp using the
/// NTFS USN Change Journal when available. Only files under `include_paths`
/// will be returned.
#[allow(unused_variables)]
pub fn changed_files(since: SystemTime, include_paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    #[cfg(target_os = "windows")]
    {
        use std::collections::HashSet;
        use std::path::{Component, Prefix};
        use usn_journal_rs::{journal::UsnJournal, path::PathResolver, volume::Volume};

        // Determine the set of drive letters from the include paths. If none of the
        // paths provide a drive (e.g. they are relative), default to the system drive
        // "C" so the function still operates in a best-effort manner.
        let mut drives = HashSet::new();
        for path in include_paths {
            if let Ok(abs) = path.canonicalize() {
                if let Some(Component::Prefix(prefix)) = abs.components().next() {
                    match prefix.kind() {
                        Prefix::Disk(d) | Prefix::VerbatimDisk(d) => {
                            drives.insert((d as char).to_ascii_uppercase());
                        }
                        _ => {}
                    }
                }
            }
        }
        if drives.is_empty() {
            drives.insert('C');
        }

        let mut files = Vec::new();

        for drive in drives {
            let volume = Volume::from_drive_letter(drive)
                .map_err(|e| anyhow!("failed to open volume {drive}: {e}"))?;

            let journal = UsnJournal::new(&volume);
            let iter = journal
                .iter()
                .map_err(|e| anyhow!("failed to read USN journal on {drive}: {e}"))?;

            let mut resolver = PathResolver::new_with_cache(&volume);

            for entry in iter {
                if entry.time > since && !entry.is_dir() {
                    if let Some(path) = resolver.resolve_path(&entry) {
                        if include_paths.iter().any(|p| path.starts_with(p)) {
                            files.push(path);
                        }
                    }
                }
            }
        }

        Ok(files)
    }

    #[cfg(not(target_os = "windows"))]
    {
        use walkdir::WalkDir;

        let mut files = Vec::new();

        for include in include_paths {
            for entry in WalkDir::new(include) {
                let entry = entry?;
                if entry.file_type().is_file() {
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
}
