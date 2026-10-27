//! The project's files and folders, as a file explorer's search lists them.

use std::path::Path;

use serde::Serialize;

use super::Layout;
use super::project::project_walk;

/// What a walk entry is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum WalkEntryKind {
    /// A file.
    #[serde(rename = "file")]
    File,
    /// A folder.
    #[serde(rename = "dir")]
    Directory,
}

/// A file or folder of the project, as the project walk gives one.
#[derive(Clone, Serialize, Debug, PartialEq)]
pub struct WalkEntry {
    /// Its name.
    pub name: String,
    /// Its path.
    pub path: String,
    /// File or folder.
    pub kind: WalkEntryKind,
}

/// Every file and folder under `root` (not `root` itself) that the project walk keeps.
#[must_use]
pub fn project_entries(root: &Path, layout: &Layout) -> Vec<WalkEntry> {
    project_walk(root, layout)
        .filter_map(Result::ok)
        .filter(|entry| entry.depth() > 0)
        .filter_map(|entry| {
            let kind = match entry.file_type() {
                Some(t) if t.is_dir() => WalkEntryKind::Directory,
                Some(t) if t.is_file() => WalkEntryKind::File,
                _ => return None,
            };
            Some(WalkEntry {
                name: entry.file_name().to_string_lossy().into_owned(),
                path: entry.path().to_string_lossy().into_owned(),
                kind,
            })
        })
        .collect()
}
