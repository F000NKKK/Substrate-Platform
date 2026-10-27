//! What a commit takes in.

/// What a commit takes in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommitScope {
    /// What's staged.
    Staged,
    /// Every change, staged first (Commit All).
    All,
}
