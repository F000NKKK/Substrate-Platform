//! A commit in the history, on the wire.

use serde::Serialize;
use substrate_git_domain::commits::Commit;

/// A commit in a branch's history.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CommitDto {
    hash: String,
    short_hash: String,
    parents: Vec<String>,
    author: String,
    email: String,
    date: i64,
    refs: Vec<String>,
    subject: String,
}

impl From<Commit> for CommitDto {
    fn from(commit: Commit) -> Self {
        Self {
            hash: commit.hash,
            short_hash: commit.short_hash,
            parents: commit.parents,
            author: commit.author,
            email: commit.email,
            date: commit.date,
            refs: commit.refs,
            subject: commit.subject,
        }
    }
}
