//! What a commit made.

use serde::Serialize;

/// The commit just made, for the UI to show.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitResult {
    /// Its short hash.
    pub hash: String,
    /// Its message's first line.
    pub subject: String,
}
