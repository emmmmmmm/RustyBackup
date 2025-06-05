use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::time::SystemTime;

/// Return a list of files modified since the given timestamp using the
/// NTFS USN Change Journal when available.
#[allow(unused_variables)]
pub fn changed_files(since: SystemTime) -> Result<Vec<PathBuf>> {
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
                    files.push(path);
                }
            }
        }

        Ok(files)
    }

    #[cfg(not(target_os = "windows"))]
    {
        // Non-Windows platforms do not have the NTFS USN journal.
        let _ = since;
        Ok(Vec::new())
    }
}
