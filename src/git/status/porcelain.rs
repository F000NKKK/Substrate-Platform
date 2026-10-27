//! Reading `git status`: running it, and parsing its porcelain v2.

use crate::git::GitError;
use std::path::Path;

use super::{ChangeKind, FileChange, Operation, RepoStatus};
use crate::git::run::{git, repo_of, run};

/// Parses `git status --porcelain=v2 --branch --show-stash -z`.
#[must_use]
pub fn parse(raw: &str) -> RepoStatus {
    let mut status = RepoStatus::default();
    let mut records = raw.split('\0');
    while let Some(record) = records.next() {
        if let Some(header) = record.strip_prefix("# ") {
            branch_header(&mut status, header);
            continue;
        }
        match record.chars().next() {
            // `1 XY sub mH mI mW hH hI path`.
            Some('1') => changed(&mut status, record, ORDINARY_FIELDS, None),
            // `2` adds `Xscore` before the path, and the original path after it.
            Some('2') => {
                let original = records.next();
                changed(&mut status, record, ORDINARY_FIELDS + 1, original);
            }
            Some('u') => {
                if let Some(path) = record.splitn(11, ' ').nth(10) {
                    let change = FileChange::new(path, None, ChangeKind::Conflicted);
                    status.conflicted.push(change);
                }
            }
            Some('?') => {
                if let Some(path) = record.get(2..) {
                    let change = FileChange::new(path, None, ChangeKind::Untracked);
                    status.unstaged.push(change);
                }
            }
            _ => {}
        }
    }
    status
}

/// Fields of an ordinary changed entry, its path last.
const ORDINARY_FIELDS: usize = 9;

/// One changed entry of `fields` fields: its staged and its unstaged change.
fn changed(status: &mut RepoStatus, record: &str, fields: usize, original: Option<&str>) {
    let fields: Vec<&str> = record.splitn(fields, ' ').collect();
    let Some(&path) = fields.last() else { return };
    let xy: Vec<char> = fields
        .get(1)
        .map(|s| s.chars().collect())
        .unwrap_or_default();
    if let Some(kind) = xy.first().copied().and_then(ChangeKind::from_code) {
        let original = matches!(kind, ChangeKind::Renamed | ChangeKind::Copied)
            .then_some(original)
            .flatten();
        status.staged.push(FileChange::new(path, original, kind));
    }
    if let Some(kind) = xy.get(1).copied().and_then(ChangeKind::from_code) {
        status.unstaged.push(FileChange::new(path, None, kind));
    }
}

/// One `# branch.*` / `# stash` header of porcelain v2.
fn branch_header(status: &mut RepoStatus, header: &str) {
    let (key, value) = header.split_once(' ').unwrap_or((header, ""));
    match key {
        "branch.oid" if value != "(initial)" => status.head = Some(value.to_string()),
        "branch.head" if value != "(detached)" => status.branch = Some(value.to_string()),
        "branch.upstream" => status.upstream = Some(value.to_string()),
        "branch.ab" => {
            for part in value.split(' ') {
                if let Some(n) = part.strip_prefix('+') {
                    status.ahead = n.parse().unwrap_or(0);
                } else if let Some(n) = part.strip_prefix('-') {
                    status.behind = n.parse().unwrap_or(0);
                }
            }
        }
        "stash" => status.stashes = value.parse().unwrap_or(0),
        _ => {}
    }
}

/// The repository's state for Git Changes and the status bar — `None` when
/// `root` isn't in a repository.
///
/// # Errors
///
/// What `git` reports when it can't read the repository.
pub fn read(root: &Path) -> Result<Option<RepoStatus>, GitError> {
    let Some((top, git_dir)) = repo_of(root)? else {
        return Ok(None);
    };
    // `--no-optional-locks`: a refresh never takes `index.lock` from under a commit.
    let raw = run(git(&top).args([
        "--no-optional-locks",
        "status",
        "--porcelain=v2",
        "--branch",
        "--show-stash",
        "--untracked-files=all",
        "-z",
    ]))?;
    let mut status = parse(&raw);
    status.set_absolute_paths(&top);
    status.repo_root = top.to_string_lossy().into_owned();
    status.operation = Operation::in_progress(&git_dir);
    Ok(Some(status))
}
