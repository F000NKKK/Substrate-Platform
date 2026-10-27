//! What was committed.

use std::path::Path;

use substrate_git_domain::GitError;
use substrate_git_domain::commits::Commit;
use substrate_git_domain::status::FileChange;

/// What was committed: the last message, a branch's log, a commit's files.
pub trait History {
    /// The last commit's full message — what Amend starts from.
    ///
    /// # Errors
    ///
    /// No repository at `root`, or what `git` reports.
    fn last_message(&self, root: &Path) -> Result<String, GitError>;

    /// A page of `rev`'s history (HEAD when empty), newest first.
    ///
    /// # Errors
    ///
    /// No repository at the folder, or what `git` reports.
    fn log(&self, root: &Path, rev: &str, skip: u32, max: u32) -> Result<Vec<Commit>, GitError>;

    /// The files a commit changed, compared with its first parent.
    ///
    /// # Errors
    ///
    /// No repository at the folder, or what `git` reports.
    fn commit_files(&self, root: &Path, hash: &str) -> Result<Vec<FileChange>, GitError>;
}
