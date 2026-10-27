//! Branches: listing, switching, creating, deleting.

use super::run::{git, require_repo, run, write_lock};
use crate::message::Message;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Branch {
    /// `main`, `origin/main`.
    pub name: String,
    pub remote: bool,
    pub current: bool,
    pub upstream: Option<String>,
    pub ahead: u32,
    pub behind: u32,
    /// The upstream was deleted on the remote.
    pub gone: bool,
    pub hash: String,
    /// The tip commit's date (unix seconds) and subject.
    pub date: i64,
    pub subject: String,
}

const BRANCH_FORMAT: &str = "--format=%(refname)%1f%(refname:short)%1f%(objectname:short)%1f%(upstream:short)%1f%(upstream:track,nobracket)%1f%(HEAD)%1f%(committerdate:unix)%1f%(contents:subject)";
const REMOTES_PREFIX: &str = "refs/remotes/";

/// `ahead 1, behind 2` / `gone` / empty → (ahead, behind, gone).
fn parse_track(track: &str) -> (u32, u32, bool) {
    let mut result = (0, 0, track.trim() == "gone");
    for part in track.split(", ") {
        if let Some(n) = part.strip_prefix("ahead ") {
            result.0 = n.trim().parse().unwrap_or(0);
        } else if let Some(n) = part.strip_prefix("behind ") {
            result.1 = n.trim().parse().unwrap_or(0);
        }
    }
    result
}

fn parse_branches(raw: &str) -> Vec<Branch> {
    raw.lines()
        .filter_map(|line| {
            let fields: Vec<&str> = line.split('\x1f').collect();
            let [refname, name, hash, upstream, track, head, date, subject] = fields[..] else {
                return None;
            };
            // `origin/HEAD` is just a pointer to the remote's default branch.
            if refname.ends_with("/HEAD") {
                return None;
            }
            let (ahead, behind, gone) = parse_track(track);
            Some(Branch {
                name: name.to_string(),
                remote: refname.starts_with(REMOTES_PREFIX),
                current: head == "*",
                upstream: (!upstream.is_empty()).then(|| upstream.to_string()),
                ahead,
                behind,
                gone,
                hash: hash.to_string(),
                date: date.parse().unwrap_or(0),
                subject: subject.to_string(),
            })
        })
        .collect()
}

/// Where the branch to switch to is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BranchSource {
    /// A local branch, checked out as it is.
    Local,
    /// A remote-tracking branch (`origin/feature`): a local branch tracking it is made.
    Remote,
}

/// Whether a new branch is checked out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BranchCheckout {
    /// The current branch stays checked out.
    Stay,
    /// The new branch is checked out.
    Switch,
}

/// How a branch is deleted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeleteMode {
    /// Only when it's merged (`git branch -d`).
    Merged,
    /// Even when it isn't merged anywhere (`git branch -D`).
    Force,
}

/// Every local and remote-tracking branch.
pub fn list(root: &Path) -> Result<Vec<Branch>, Message> {
    let top = require_repo(root)?;
    Ok(parse_branches(&run(git(&top).args([
        "for-each-ref",
        BRANCH_FORMAT,
        "refs/heads",
        "refs/remotes",
    ]))?))
}

/// Checks out a branch. A remote one (`origin/feature`) gets a local branch
/// tracking it, as Visual Studio does.
pub fn switch(root: &Path, name: &str, source: BranchSource) -> Result<(), Message> {
    let top = require_repo(root)?;
    let _lock = write_lock();
    let mut command = git(&top);
    command.arg("switch");
    if source == BranchSource::Remote {
        command.arg("--track");
    }
    run(command.arg(name)).map(drop)
}

/// New Branch: from `start` (the current commit when empty), checked out or not.
pub fn create(
    root: &Path,
    name: &str,
    start: &str,
    checkout: BranchCheckout,
) -> Result<(), Message> {
    let top = require_repo(root)?;
    let _lock = write_lock();
    let mut command = git(&top);
    match checkout {
        BranchCheckout::Switch => command.args(["switch", "-c", name]),
        BranchCheckout::Stay => command.args(["branch", name]),
    };
    if !start.is_empty() {
        command.arg(start);
    }
    run(&mut command).map(drop)
}

/// Deletes a local branch, merged or (`DeleteMode::Force`) not.
pub fn delete(root: &Path, name: &str, mode: DeleteMode) -> Result<(), Message> {
    let top = require_repo(root)?;
    let _lock = write_lock();
    let flag = match mode {
        DeleteMode::Merged => "-d",
        DeleteMode::Force => "-D",
    };
    run(git(&top).args(["branch", flag, name])).map(drop)
}

/// The branch checked out — `None` on a detached HEAD.
pub(crate) fn current(top: &Path) -> Option<String> {
    run(git(top).args(["symbolic-ref", "--quiet", "--short", "HEAD"]))
        .ok()
        .map(|b| b.trim().to_string())
}

/// Whether the branch checked out tracks an upstream.
pub(crate) fn has_upstream(top: &Path) -> bool {
    run(git(top).args(["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"])).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_branches_and_tracking() {
        let raw = "refs/heads/main\x1fmain\x1fabc\x1forigin/main\x1fahead 1, behind 2\x1f*\x1f1700000000\x1fTip\n\
                   refs/heads/old\x1fold\x1fdef\x1forigin/old\x1fgone\x1f \x1f1600000000\x1fOld\n\
                   refs/remotes/origin/HEAD\x1forigin\x1fabc\x1f\x1f\x1f \x1f1700000000\x1fTip\n\
                   refs/remotes/origin/main\x1forigin/main\x1fabc\x1f\x1f\x1f \x1f1700000000\x1fTip\n";
        let branches = parse_branches(raw);
        assert_eq!(branches.len(), 3);
        assert!(branches[0].current && !branches[0].remote);
        assert_eq!((branches[0].ahead, branches[0].behind), (1, 2));
        assert!(branches[1].gone && !branches[1].current);
        assert!(branches[2].remote && branches[2].upstream.is_none());
        assert_eq!(branches[2].name, "origin/main");
    }
}
