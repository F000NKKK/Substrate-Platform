//! Which version of a file a diff side shows.

/// Which version of a file a diff side shows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SideVersion {
    /// Nothing: the file didn't exist there.
    Nothing,
    /// What's staged.
    Index,
    /// The file on disk.
    Worktree,
    /// A commit (anything `git rev-parse` takes).
    Revision(String),
}
