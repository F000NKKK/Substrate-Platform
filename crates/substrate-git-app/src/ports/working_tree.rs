//! The working tree and the index.

use std::path::Path;

use substrate_git_domain::GitError;
use substrate_git_domain::commits::{CommitMode, CommitResult, CommitScope};
use substrate_git_domain::status::{DiscardItem, RepoStatus};

/// The working tree and the index: read, stage, undo, commit. The writes
/// here are not serialized; [`crate::changes`] does that.
pub trait WorkingTree {
    /// The repository's state for Git Changes and the status bar — `None`
    /// when `root` isn't in a repository.
    ///
    /// # Errors
    ///
    /// What `git` reports when it can't read the repository.
    fn status(&self, root: &Path) -> Result<Option<RepoStatus>, GitError>;

    /// Stages `paths` (repo-relative) — every change when empty. Deletions too.
    ///
    /// # Errors
    ///
    /// No repository at `root`, or what `git` reports.
    fn stage(&self, root: &Path, paths: &[String]) -> Result<(), GitError>;

    /// Unstages `paths` — everything when empty.
    ///
    /// # Errors
    ///
    /// No repository at `root`, or what `git` reports.
    fn unstage(&self, root: &Path, paths: &[String]) -> Result<(), GitError>;

    /// Undo Changes: an unstaged change goes back to what's staged (an
    /// untracked file is deleted), a staged one back to the last commit.
    ///
    /// # Errors
    ///
    /// No repository at `root`, or what `git` reports.
    fn discard(&self, root: &Path, items: &[DiscardItem]) -> Result<(), GitError>;

    /// Commits what `scope` takes in, as a new commit or amending the last.
    ///
    /// # Errors
    ///
    /// No repository at `root`, or what `git` reports.
    fn commit(
        &self,
        root: &Path,
        message: &str,
        mode: CommitMode,
        scope: CommitScope,
    ) -> Result<CommitResult, GitError>;
}
