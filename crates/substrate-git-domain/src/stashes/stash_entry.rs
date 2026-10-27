//! One stash.

/// One entry of the stash list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StashEntry {
    /// Its place in the list, newest first (`stash@{index}`).
    pub index: u32,
    /// What it was stashed with.
    pub message: String,
    /// Unix seconds.
    pub date: i64,
}
