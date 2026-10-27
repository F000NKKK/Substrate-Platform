//! How a branch is deleted.

/// How a branch is deleted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeleteMode {
    /// Only when it's merged (`git branch -d`).
    Merged,
    /// Even when it isn't merged anywhere (`git branch -D`).
    Force,
}
