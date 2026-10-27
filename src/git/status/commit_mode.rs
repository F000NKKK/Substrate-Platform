//! A new commit or an amend.

/// Whether a commit is a new one or replaces the last.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommitMode {
    /// A new commit on top of the current one.
    New,
    /// Replaces the last commit; an empty message keeps its message.
    Amend,
}
