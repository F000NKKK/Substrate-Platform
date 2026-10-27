//! Finding the repository a folder is in.

use std::path::{Path, PathBuf};
use std::process::Stdio;

use substrate_git_domain::GitError;

use super::command::git;
use super::failure::{NO_REPO_SIGNS, friendly, spawn_error};
use super::run::run;

/// The repository `root` (a project folder) belongs to: its working tree's
/// top folder — canonical, like project roots, so paths built on it match
/// the ones in open tabs — and its `.git` directory. `None` when it isn't
/// in a repository at all.
pub(crate) fn repo_of(root: &Path) -> Result<Option<(PathBuf, PathBuf)>, GitError> {
    let out = git(root)
        .args(["rev-parse", "--show-toplevel", "--absolute-git-dir"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(spawn_error)?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        // A bare repository or the inside of `.git` has no working tree to show either.
        if NO_REPO_SIGNS.iter().any(|sign| err.contains(sign)) {
            return Ok(None);
        }
        return Err(friendly(err.trim()));
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let mut lines = text.lines();
    let (Some(top), Some(git_dir)) = (lines.next(), lines.next()) else {
        return Ok(None);
    };
    let top = PathBuf::from(top);
    Ok(Some((
        top.canonicalize().unwrap_or(top),
        PathBuf::from(git_dir),
    )))
}

/// The working tree's top folder of the repository `root` is in.
pub(crate) fn require_repo(root: &Path) -> Result<PathBuf, GitError> {
    repo_of(root)?
        .map(|(top, _)| top)
        .ok_or(GitError::NotARepository)
}

/// The working tree's top folder of the repository `root` is in, if it's in one.
///
/// # Errors
///
/// What `git` reports when it can't tell.
pub fn repo_root(root: &Path) -> Result<Option<PathBuf>, GitError> {
    Ok(repo_of(root)?.map(|(top, _)| top))
}

/// Whether the repository has a commit checked out.
pub(crate) fn has_head(top: &Path) -> bool {
    run(git(top).args(["rev-parse", "--verify", "--quiet", "HEAD"])).is_ok()
}

/// The branch checked out — `None` on a detached HEAD.
pub(crate) fn current_branch(top: &Path) -> Option<String> {
    run(git(top).args(["symbolic-ref", "--quiet", "--short", "HEAD"]))
        .ok()
        .map(|b| b.trim().to_string())
}

/// Whether the branch checked out tracks an upstream.
pub(crate) fn has_upstream(top: &Path) -> bool {
    run(git(top).args(["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"])).is_ok()
}
