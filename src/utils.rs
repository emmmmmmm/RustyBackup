use std::path::{Path, PathBuf};

/// Normalize path separators so `\` becomes `/`.
pub fn normalize_path(path: &Path) -> PathBuf {
    PathBuf::from(path.to_string_lossy().replace('\\', "/"))
}

