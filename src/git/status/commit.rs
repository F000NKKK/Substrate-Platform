//! Committing, and the message Amend starts from.

use std::path::Path;

use super::{CommitMode, CommitResult, CommitScope};
use crate::git::run::{git, require_repo, run, run_with_input, write_lock};
use crate::message::Message;

/// Commits what `scope` takes in, as a new commit or amending the last (`mode`).
///
/// # Errors
///
/// No repository at `root`, or what `git` reports.
pub fn commit(
    root: &Path,
    message: &str,
    mode: CommitMode,
    scope: CommitScope,
) -> Result<CommitResult, Message> {
    let top = require_repo(root)?;
    let _lock = write_lock();
    if scope == CommitScope::All {
        run(git(&top).args(["add", "--all"]))?;
    }
    let mut command = git(&top);
    command.args(["commit", "-q", "--cleanup=strip"]);
    if mode == CommitMode::Amend {
        command.arg("--amend");
    }
    if mode == CommitMode::Amend && message.trim().is_empty() {
        run(command.arg("--no-edit"))?;
    } else {
        run_with_input(command.args(["-F", "-"]), message)?;
    }
    let out = run(git(&top).args(["log", "-1", "--format=%h%x1f%s"]))?;
    let (hash, subject) = out
        .trim_end()
        .split_once('\x1f')
        .unwrap_or((out.trim_end(), ""));
    Ok(CommitResult {
        hash: hash.to_string(),
        subject: subject.to_string(),
    })
}

/// The last commit's full message — what Amend starts from.
///
/// # Errors
///
/// No repository at `root`, or what `git` reports.
pub fn last_message(root: &Path) -> Result<String, Message> {
    let top = require_repo(root)?;
    run(git(&top).args(["log", "-1", "--format=%B"])).map(|m| m.trim_end().to_string())
}
