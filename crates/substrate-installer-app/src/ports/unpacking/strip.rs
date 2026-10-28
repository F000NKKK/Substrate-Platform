//! Whether the archive's one top folder is dropped.

/// Whether the one folder an archive's entries are all in is dropped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strip {
    /// Entries land as the archive names them.
    KeepLayout,
    /// The top folder every entry is in is left out (a source archive's
    /// `project-1.0/`, a JDK's `jdk-21/`).
    DropTopFolder,
}
