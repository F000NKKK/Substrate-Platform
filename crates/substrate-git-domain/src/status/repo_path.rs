//! Repository-relative paths as files on disk.

use std::path::Path;

/// `path` (repo-relative, `/`-separated as git prints it) as a file path under `top`.
#[must_use]
pub fn absolute(top: &Path, path: &str) -> String {
    let mut full = top.to_path_buf();
    full.extend(path.split('/').filter(|s| !s.is_empty()));
    full.to_string_lossy().into_owned()
}
