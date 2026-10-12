//! Directory browsing/mutation — list one directory level (a host lazily
//! expands a file tree by calling this per-node), create/rename/remove
//! files and folders, and read/write a file's content — as text (for an
//! editor) or raw bytes (for a binary viewer).
//! Plain `std::fs`; carries no notion of "is this path allowed" — that
//! security boundary (an IDE should only touch files inside an opened
//! project) belongs to the host, one layer up.

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
        let kind = if file_type.is_dir() {
            EntryKind::Dir
        } else {
            EntryKind::File
        };
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

/// Reads a file's entire content as text — an editor's "open file". Fails
/// with [`io::ErrorKind::InvalidData`] for anything that isn't valid UTF-8
/// (an image, an archive, ...) — use [`read_file_bytes`] for those.
pub fn read_file(path: &Path) -> io::Result<String> {
    std::fs::read_to_string(path)
}

/// Reads a file's entire content as raw bytes — for binary files a text
/// editor can't open (an image viewer, a hex view). A host shipping these
/// over IPC should send them as a binary payload (or base64), not as a JSON
/// array of numbers.
pub fn read_file_bytes(path: &Path) -> io::Result<Vec<u8>> {
    std::fs::read(path)
}

/// Overwrites a file's entire content — an editor's "save".
pub fn write_file(path: &Path, contents: &str) -> io::Result<()> {
    std::fs::write(path, contents)
}

/// Binary counterpart to [`write_file`].
pub fn write_file_bytes(path: &Path, contents: &[u8]) -> io::Result<()> {
    std::fs::write(path, contents)
}
