//! What a commit made.

/// The commit just made, for the UI to show.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommitResult {
    /// Its short hash.
    pub hash: String,
    /// Its message's first line.
    pub subject: String,
}
