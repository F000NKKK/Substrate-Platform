//! One entry of a folder.

use std::path::PathBuf;

use crate::EntryKind;

/// One entry of a folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    /// Its file name.
    pub name: String,
    /// Its full path.
    pub path: PathBuf,
    /// File or directory.
    pub kind: EntryKind,
}
