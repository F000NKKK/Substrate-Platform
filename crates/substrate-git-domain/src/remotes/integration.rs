//! How a branch is brought into the current one.

/// How another branch is brought into the current one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Integration {
    /// Merge the branch into the current one.
    Merge,
    /// Rebase the current branch onto it.
    Rebase,
}
