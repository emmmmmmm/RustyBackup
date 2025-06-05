use anyhow::Result;
use std::path::PathBuf;
use std::time::SystemTime;


#[allow(unused_variables)]
pub fn changed_files(since: SystemTime, include_paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
   
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

   

