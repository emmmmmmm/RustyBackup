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
        use usn_journal_rs::{journal::UsnJournal, path::PathResolver, volume::Volume};

        // For now we only examine the system drive (C:). A more complete
        // implementation could detect the drive from configured include paths.
        let volume = Volume::from_drive_letter('C')
            .map_err(|e| anyhow!("failed to open volume: {e}"))?;

        let journal = UsnJournal::new(&volume);
        let iter = journal
            .iter()
            .map_err(|e| anyhow!("failed to read USN journal: {e}"))?;

        let mut resolver = PathResolver::new_with_cache(&volume);
        let mut files = Vec::new();

        for entry in iter {
            if entry.time > since && !entry.is_dir() {
                if let Some(path) = resolver.resolve_path(&entry) {
                    if include_paths.iter().any(|p| path.starts_with(p)) {
                        files.push(path);
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
