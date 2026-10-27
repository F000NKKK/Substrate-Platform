//! A commit in the history.

/// A commit in a branch's history.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commit {
    /// The full hash.
    pub hash: String,
    /// The abbreviated hash.
    pub short_hash: String,
    /// Its parents' hashes.
    pub parents: Vec<String>,
    /// The author's name.
    pub author: String,
    /// The author's email.
    pub email: String,
    /// Unix seconds.
    pub date: i64,
    /// Branch and tag names pointing here (`%D`).
    pub refs: Vec<String>,
    /// The message's first line.
    pub subject: String,
}
