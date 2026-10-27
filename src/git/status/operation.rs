//! An operation waiting to be finished.

use std::path::Path;

use serde::Serialize;

/// A merge, rebase, cherry-pick or revert waiting to be finished (or aborted).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Operation {
    /// `git rebase`.
    Rebase,
    /// `git merge`.
    Merge,
    /// `git cherry-pick`.
    CherryPick,
    /// `git revert`.
    Revert,
}

impl Operation {
    /// What in `.git` says it's waiting, for each operation.
    const MARKERS: [(&'static [&'static str], Operation); 4] = [
        (&["rebase-merge", "rebase-apply"], Operation::Rebase),
        (&["MERGE_HEAD"], Operation::Merge),
        (&["CHERRY_PICK_HEAD"], Operation::CherryPick),
        (&["REVERT_HEAD"], Operation::Revert),
    ];

    /// The one waiting in `git_dir`, if any.
    pub(crate) fn in_progress(git_dir: &Path) -> Option<Self> {
        Self::MARKERS
            .iter()
            .find(|(files, _)| files.iter().any(|f| git_dir.join(f).exists()))
            .map(|(_, op)| *op)
    }

    /// The git command that continues or aborts it.
    pub(crate) fn command(self) -> &'static str {
        match self {
            Operation::Rebase => "rebase",
            Operation::Merge => "merge",
            Operation::CherryPick => "cherry-pick",
            Operation::Revert => "revert",
        }
    }
}
