//! An operation waiting to be finished.

/// A merge, rebase, cherry-pick or revert waiting to be finished (or aborted).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operation {
    /// `git rebase`.
    Rebase,
    /// `git merge`.
    Merge,
    /// `git cherry-pick`.
    CherryPick,
    /// `git revert`.
    Revert,
}
