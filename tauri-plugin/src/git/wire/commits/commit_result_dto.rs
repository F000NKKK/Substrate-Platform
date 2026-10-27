//! What a commit made, on the wire.

use serde::Serialize;
use substrate_git_domain::commits::CommitResult;

/// The commit just made.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CommitResultDto {
    hash: String,
    subject: String,
}

impl From<CommitResult> for CommitResultDto {
    fn from(result: CommitResult) -> Self {
        Self {
            hash: result.hash,
            subject: result.subject,
        }
    }
}
