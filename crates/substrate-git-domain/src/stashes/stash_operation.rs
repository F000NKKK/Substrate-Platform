//! What is done with a stash.

/// What is done with an existing stash.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StashOperation {
    /// Apply it and keep it.
    Apply,
    /// Apply it and drop it.
    Pop,
    /// Drop it.
    Drop,
}
