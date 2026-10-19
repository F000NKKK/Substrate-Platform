//! Long runs, streamed to an [`Output`]: fetch, pull, push, sync; merge and
//! rebase; finishing or aborting what's in progress; clone and init.

use super::branch::{current, has_upstream};
use super::messages as m;
use super::run::{Output, git, repo_of, require_repo, run, stream, stream_git, write_lock};
use super::status::Operation;
use crate::message::Message;
use serde::Deserialize;
use std::path::Path;

/// The remote a branch pushes to when nothing else says: git's convention.
const ORIGIN: &str = "origin";

/// Where a branch without an upstream gets pushed: its own `pushRemote`,
/// the repository's `remote.pushDefault`, `origin`, else the only remote.
fn push_remote(top: &Path, branch: &str) -> Result<String, Message> {
    let config = |key: &str| {
        run(git(top).args(["config", "--get", key]))
            .ok()
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
    };
    if let Some(remote) =
        config(&format!("branch.{branch}.pushRemote")).or_else(|| config("remote.pushDefault"))
    {
        return Ok(remote);
    }
    let remotes = run(git(top).arg("remote"))?;
    let remotes: Vec<&str> = remotes
        .lines()
        .map(str::trim)
        .filter(|r| !r.is_empty())
        .collect();
    if remotes.contains(&ORIGIN) {
        return Ok(ORIGIN.to_string());
    }
    match remotes[..] {
        [only] => Ok(only.to_string()),
        [] => Err(Message::new(m::NO_PUSH_REMOTE)),
        _ => Err(Message::new(m::NO_ORIGIN)),
    }
}

fn push(out: Output, top: &Path) -> Result<(), Message> {
    if has_upstream(top) {
        return stream_git(out, top, &["push", "--progress"]);
    }
    // Push and create the upstream, as Visual Studio does for a new branch.
    let branch = current(top).ok_or_else(|| Message::new(m::DETACHED_PUSH))?;
    let remote = push_remote(top, &branch)?;
    stream_git(
        out,
        top,
        &["push", "--progress", "--set-upstream", &remote, &branch],
    )
}

fn pull(out: Output, top: &Path) -> Result<(), Message> {
    if !has_upstream(top) {
        return Err(Message::new(m::NO_UPSTREAM));
    }
    stream_git(out, top, &["pull", "--progress"])
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum RemoteOp {
    Fetch,
    Pull,
    Push,
    /// Pull, then push.
    Sync,
}

/// Fetch / Pull / Push / Sync.
pub fn remote(root: &Path, op: RemoteOp, out: Output) -> Result<(), Message> {
    let top = require_repo(root)?;
    let _lock = write_lock();
    match op {
        RemoteOp::Fetch => stream_git(out, &top, &["fetch", "--all", "--progress"]),
        RemoteOp::Pull => pull(out, &top),
        RemoteOp::Push => push(out, &top),
        RemoteOp::Sync => pull(out, &top).and_then(|()| push(out, &top)),
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum BranchOp {
    /// Merge the branch into the current one.
    Merge,
    /// Rebase the current branch onto it.
    Rebase,
}

/// Merge into Current / Rebase onto — streamed, since either can stop on conflicts.
pub fn integrate(root: &Path, op: BranchOp, branch: &str, out: Output) -> Result<(), Message> {
    let top = require_repo(root)?;
    let _lock = write_lock();
    match op {
        BranchOp::Merge => stream_git(out, &top, &["merge", "--no-edit", branch]),
        BranchOp::Rebase => stream_git(out, &top, &["rebase", branch]),
    }
}

/// Aborts (or, `go_on`, continues) the merge/rebase/cherry-pick/revert in progress.
pub fn operation(root: &Path, go_on: bool, out: Output) -> Result<(), Message> {
    let Some((top, git_dir)) = repo_of(root)? else {
        return Err(Message::new(m::NOT_A_REPOSITORY));
    };
    let _lock = write_lock();
    let op =
        Operation::in_progress(&git_dir).ok_or_else(|| Message::new(m::NOTHING_IN_PROGRESS))?;
    // A merge is continued by committing it.
    if go_on && op == Operation::Merge {
        return stream_git(out, &top, &["commit", "--no-edit"]);
    }
    let command = op.command();
    stream_git(
        out,
        &top,
        &[command, if go_on { "--continue" } else { "--abort" }],
    )
}

/// The folder a clone of `url` goes into by default — its last path segment, minus `.git`.
pub fn clone_dir_name(url: &str) -> String {
    let trimmed = url.trim().trim_end_matches(['/', '\\']);
    let last = trimmed.rsplit(['/', '\\', ':']).next().unwrap_or(trimmed);
    let name = last.strip_suffix(".git").unwrap_or(last);
    if name.is_empty() {
        "repository".to_string()
    } else {
        name.to_string()
    }
}

/// Clone a Repository: into `parent/<name>` (the URL's name when empty);
/// the new working tree's path.
pub fn clone(url: &str, parent: &Path, name: &str, out: Output) -> Result<String, Message> {
    let name = if name.trim().is_empty() {
        clone_dir_name(url)
    } else {
        name.trim().to_string()
    };
    let dest = parent.join(&name);
    if dest.exists()
        && std::fs::read_dir(&dest)
            .map(|mut d| d.next().is_some())
            .unwrap_or(true)
    {
        return Err(Message::new(m::CLONE_TARGET_NOT_EMPTY).arg("path", dest.display()));
    }
    let dest_text = dest.to_string_lossy().into_owned();
    stream(
        out,
        git(parent).args(["clone", "--progress", "--", url, &dest_text]),
        &format!("clone {url} {dest_text}"),
    )?;
    Ok(dest_text)
}

/// Create Git Repository: `git init` in `root`, on `branch` (git's own default when empty).
pub fn init(root: &Path, branch: &str, out: Output) -> Result<(), Message> {
    let mut args = vec!["init"];
    if !branch.trim().is_empty() {
        args.extend(["--initial-branch", branch.trim()]);
    }
    stream_git(out, root, &args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_clone_folders() {
        assert_eq!(
            clone_dir_name("https://github.com/owner/Some-App.git"),
            "Some-App"
        );
        assert_eq!(clone_dir_name("git@github.com:owner/other.git"), "other");
        assert_eq!(clone_dir_name("git@host:repo"), "repo");
        assert_eq!(clone_dir_name("C:\\repos\\local\\"), "local");
    }
}
