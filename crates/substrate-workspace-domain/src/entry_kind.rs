//! What a folder's entry is.

/// What a folder's entry is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    /// A file, or anything that isn't a directory.
    File,
    /// A directory.
    Directory,
}
