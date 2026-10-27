//! Undo Changes.

use crate::git::GitError;
use crate::git::WriteLock;
use std::path::Path;

use super::{ChangeKind, DiscardItem};
use crate::git::run::{git, has_head, require_repo, run};

/// Undo Changes: an unstaged change goes back to what's staged (an untracked
/// file is deleted), a staged one back to the last commit — index and disk
/// both (a newly added file is deleted).
///
/// # Errors
///
/// No repository at `root`, or what `git` reports.
pub fn discard(lock: &WriteLock, root: &Path, items: Vec<DiscardItem>) -> Result<(), GitError> {
    let top = require_repo(root)?;
    let _lock = lock.hold();
    let head = has_head(&top);
    let (mut untracked, mut worktree, mut added, mut to_head) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for item in items {
        match (item.staged, item.kind) {
            (false, ChangeKind::Untracked) => untracked.push(item.path),
            (false, _) => worktree.push(item.path),
            (true, ChangeKind::Added | ChangeKind::Copied) => added.push(item.path),
            (true, _) if !head => added.push(item.path),
            (true, ChangeKind::Renamed) => {
                added.push(item.path);
                to_head.extend(item.original_path);
            }
            (true, _) => to_head.push(item.path),
        }
    }
    let steps: [(&[&str], &Vec<String>); 4] = [
        (&["clean", "-f", "-q", "--"], &untracked),
        (&["checkout", "-q", "--"], &worktree),
        (&["rm", "-f", "-q", "-r", "--"], &added),
        (&["checkout", "-q", "HEAD", "--"], &to_head),
    ];
    for (args, paths) in steps {
        if !paths.is_empty() {
            run(git(&top).args(args).args(paths))?;
        }
    }
    Ok(())
}
