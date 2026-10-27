//! What a walk entry is.

/// What a walk entry is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalkEntryKind {
    /// A file.
    File,
    /// A folder.
    Directory,
}
