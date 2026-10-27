//! A repository's state.

use std::path::Path;

use super::{FileChange, Operation, absolute};

/// A repository's state for Git Changes: branch, head, upstream, changes, operation.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct RepoStatus {
    /// The working tree's top folder.
    pub repo_root: String,
    /// `None` on a detached HEAD.
    pub branch: Option<String>,
    /// The commit checked out — `None` before the first commit.
    pub head: Option<String>,
    /// The branch it tracks.
    pub upstream: Option<String>,
    /// Commits the upstream lacks.
    pub ahead: u32,
    /// Commits only the upstream has.
    pub behind: u32,
    /// Changes in the index.
    pub staged: Vec<FileChange>,
    /// Changes in the working tree.
    pub unstaged: Vec<FileChange>,
    /// Files with merge conflicts.
    pub conflicted: Vec<FileChange>,
    /// How many stashes there are.
    pub stashes: u32,
    /// A merge, rebase, cherry-pick or revert waiting to be finished (or aborted).
    pub operation: Option<Operation>,
}

impl RepoStatus {
    /// Fills in each change's file on disk, under the working tree's top folder `top`.
    pub fn set_absolute_paths(&mut self, top: &Path) {
        for file in self
            .staged
            .iter_mut()
            .chain(&mut self.unstaged)
            .chain(&mut self.conflicted)
        {
            file.absolute_path = absolute(top, &file.path);
        }
    }
}
