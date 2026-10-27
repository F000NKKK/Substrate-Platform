//! What a stash takes.

/// What a new stash takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StashMode {
    /// Stash All — untracked files included.
    All,
    /// Stash All and Keep Staged.
    KeepIndex,
    /// Stash Staged.
    Staged,
}
