//! Directory browsing/mutation — list one directory level (a host lazily
//! expands a file tree by calling this per-node), and create/rename/remove
//! files and folders. Plain `std::fs`; carries no notion of "is this path
//! allowed" — that security boundary (an IDE should only touch files inside
//! an opened project) belongs to the host, one layer up.

use std::io;
use std::path::{Path, PathBuf};

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryKind {
    File,
    Dir,
}

#[derive(Debug, Clone, Serialize)]
pub struct DirEntryInfo {
    pub name: String,
    pub path: PathBuf,
    pub kind: EntryKind,
}

/// One directory level, not recursive — a file tree widget calls this again
/// for a folder node the moment it's expanded, rather than loading an entire
/// subtree up front.
pub fn list_dir(path: &Path) -> io::Result<Vec<DirEntryInfo>> {
    let mut entries = Vec::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let kind = if file_type.is_dir() { EntryKind::Dir } else { EntryKind::File };
        entries.push(DirEntryInfo {
            name: entry.file_name().to_string_lossy().into_owned(),
            path: entry.path(),
            kind,
        });
    }
    entries.sort_by(|a, b| match (a.kind, b.kind) {
        (EntryKind::Dir, EntryKind::File) => std::cmp::Ordering::Less,
        (EntryKind::File, EntryKind::Dir) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });
    Ok(entries)
}

pub fn create_file(path: &Path) -> io::Result<()> {
    std::fs::File::create(path).map(|_| ())
}

pub fn create_dir(path: &Path) -> io::Result<()> {
    std::fs::create_dir(path)
}

pub fn rename(from: &Path, to: &Path) -> io::Result<()> {
    std::fs::rename(from, to)
}

/// Removes a file or a directory (recursively) at `path`.
pub fn remove(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        std::fs::remove_dir_all(path)
    } else {
        std::fs::remove_file(path)
    }
}
