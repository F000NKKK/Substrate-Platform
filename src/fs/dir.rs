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
    // Folders first, then case-insensitive by name.
    entries.sort_by_cached_key(|e| (e.kind != EntryKind::Dir, e.name.to_lowercase()));
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

/// Copies a file or a whole folder to `to` (which mustn't exist): Copy and
/// Paste, Add Existing Item.
pub fn copy(from: &Path, to: &Path) -> io::Result<()> {
    if from.is_dir() {
        std::fs::create_dir(to)?;
        for entry in std::fs::read_dir(from)? {
            let entry = entry?;
            copy(&entry.path(), &to.join(entry.file_name()))?;
        }
        Ok(())
    } else {
        std::fs::copy(from, to).map(|_| ())
    }
}

/// Binary counterpart to [`write_file`].
pub fn write_file_bytes(path: &Path, contents: &[u8]) -> io::Result<()> {
    std::fs::write(path, contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("substrate-dir-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn list_dir_puts_dirs_first_then_sorts_case_insensitively() {
        let dir = temp_dir("list");
        for file in ["b.txt", "A.txt", "c.txt"] {
            create_file(&dir.join(file)).unwrap();
        }
        for sub in ["zeta", "Alpha"] {
            create_dir(&dir.join(sub)).unwrap();
        }

        let names: Vec<_> = list_dir(&dir)
            .unwrap()
            .into_iter()
            .map(|e| (e.name, e.kind))
            .collect();
        assert_eq!(
            names,
            vec![
                ("Alpha".to_string(), EntryKind::Dir),
                ("zeta".to_string(), EntryKind::Dir),
                ("A.txt".to_string(), EntryKind::File),
                ("b.txt".to_string(), EntryKind::File),
                ("c.txt".to_string(), EntryKind::File),
            ]
        );
        remove(&dir).unwrap();
    }

    #[test]
    fn binary_files_round_trip_but_fail_as_text() {
        let dir = temp_dir("bytes");
        let path = dir.join("image.png");
        let bytes = [0x89, b'P', b'N', b'G', 0xff, 0x00, 0xfe];
        write_file_bytes(&path, &bytes).unwrap();

        assert_eq!(read_file_bytes(&path).unwrap(), bytes);
        assert_eq!(
            read_file(&path).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
        remove(&dir).unwrap();
    }
}
