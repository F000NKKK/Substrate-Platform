//! Staging and unstaging changes.

use crate::git::GitError;
use std::path::Path;

use crate::git::run::{git, has_head, require_repo, run, write_lock};

/// Stages `paths` (repo-relative) — every change when empty. Deletions too.
///
/// # Errors
///
/// No repository at `root`, or what `git` reports.
pub fn stage(root: &Path, paths: &[String]) -> Result<(), GitError> {
    let top = require_repo(root)?;
    let _lock = write_lock();
    run(git(&top).args(["add", "--all", "--"]).args(paths)).map(drop)
}

/// Unstages `paths` — everything when empty. Before the first commit there's
/// no HEAD to reset to, so files simply leave the index.
///
/// # Errors
///
/// No repository at `root`, or what `git` reports.
pub fn unstage(root: &Path, paths: &[String]) -> Result<(), GitError> {
    let top = require_repo(root)?;
    let _lock = write_lock();
    if !has_head(&top) {
        let mut command = git(&top);
        command.args(["rm", "--cached", "-r", "-q", "--"]);
        if paths.is_empty() {
            command.arg(".");
        } else {
            command.args(paths);
        }
        return run(&mut command).map(drop);
    }
    if paths.is_empty() {
        run(git(&top).args(["reset", "-q"])).map(drop)
    } else {
        run(git(&top).args(["reset", "-q", "--"]).args(paths)).map(drop)
    }
}
