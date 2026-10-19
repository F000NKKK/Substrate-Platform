//! What's changed — status, staging, undo, commit.

use super::run::{absolute, git, has_head, repo_of, require_repo, run, run_with_input, write_lock};
use crate::message::Message;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChangeKind {
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    TypeChanged,
    Untracked,
    Conflicted,
}

impl ChangeKind {
    pub(crate) fn from_code(code: char) -> Option<Self> {
        Some(match code {
            'M' => Self::Modified,
            'A' => Self::Added,
            'D' => Self::Deleted,
            'R' => Self::Renamed,
            'C' => Self::Copied,
            'T' => Self::TypeChanged,
            'U' => Self::Conflicted,
            _ => return None,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileChange {
    /// Repo-relative, `/`-separated.
    pub path: String,
    /// The path before a rename or copy.
    pub orig_path: Option<String>,
    pub kind: ChangeKind,
    /// The file on disk (filled in once the repository root is known).
    pub abs_path: String,
}

#[derive(Debug, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoStatus {
    pub repo_root: String,
    /// `None` on a detached HEAD.
    pub branch: Option<String>,
    /// The commit checked out — `None` before the first commit.
    pub head: Option<String>,
    pub upstream: Option<String>,
    pub ahead: u32,
    pub behind: u32,
    pub staged: Vec<FileChange>,
    pub unstaged: Vec<FileChange>,
    pub conflicted: Vec<FileChange>,
    pub stashes: u32,
    /// A merge, rebase, cherry-pick or revert waiting to be finished (or aborted).
    pub operation: Option<Operation>,
}

pub(crate) fn change(path: &str, orig_path: Option<&str>, kind: ChangeKind) -> FileChange {
    FileChange {
        path: path.to_string(),
        orig_path: orig_path.map(str::to_string),
        kind,
        abs_path: String::new(),
    }
}

/// Parses `git status --porcelain=v2 --branch --show-stash -z`.
pub fn parse_status(raw: &str) -> RepoStatus {
    let mut status = RepoStatus::default();
    let mut records = raw.split('\0');
    while let Some(record) = records.next() {
        if let Some(header) = record.strip_prefix("# ") {
            branch_header(&mut status, header);
            continue;
        }
        match record.chars().next() {
            Some('1') | Some('2') => {
                let renamed = record.starts_with('2');
                // `1 XY sub mH mI mW hH hI path`; `2` adds `Xscore` before the path.
                let fields: Vec<&str> = record.splitn(if renamed { 10 } else { 9 }, ' ').collect();
                let Some(&path) = fields.last() else { continue };
                let xy: Vec<char> = fields
                    .get(1)
                    .map(|s| s.chars().collect())
                    .unwrap_or_default();
                let orig = if renamed { records.next() } else { None };
                if let Some(kind) = xy.first().copied().and_then(ChangeKind::from_code) {
                    let orig = matches!(kind, ChangeKind::Renamed | ChangeKind::Copied)
                        .then_some(orig)
                        .flatten();
                    status.staged.push(change(path, orig, kind));
                }
                if let Some(kind) = xy.get(1).copied().and_then(ChangeKind::from_code) {
                    status.unstaged.push(change(path, None, kind));
                }
            }
            Some('u') => {
                if let Some(path) = record.splitn(11, ' ').nth(10) {
                    status
                        .conflicted
                        .push(change(path, None, ChangeKind::Conflicted));
                }
            }
            Some('?') => {
                if let Some(path) = record.get(2..) {
                    status
                        .unstaged
                        .push(change(path, None, ChangeKind::Untracked));
                }
            }
            _ => {}
        }
    }
    status
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

/// A merge, rebase, cherry-pick or revert waiting to be finished (or aborted).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Operation {
    Rebase,
    Merge,
    CherryPick,
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

/// Fills in each change's file on disk.
pub fn set_abs_paths(status: &mut RepoStatus, top: &Path) {
    for file in status
        .staged
        .iter_mut()
        .chain(&mut status.unstaged)
        .chain(&mut status.conflicted)
    {
        file.abs_path = absolute(top, &file.path);
    }
}

/// The repository's state for Git Changes and the status bar — `None` when
/// `root` isn't in a repository.
pub fn status(root: &Path) -> Result<Option<RepoStatus>, Message> {
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
    let mut status = parse_status(&raw);
    set_abs_paths(&mut status, &top);
    status.repo_root = top.to_string_lossy().into_owned();
    status.operation = Operation::in_progress(&git_dir);
    Ok(Some(status))
}

/// Stages `paths` (repo-relative) — every change when empty. Deletions too.
pub fn stage(root: &Path, paths: &[String]) -> Result<(), Message> {
    let top = require_repo(root)?;
    let _lock = write_lock();
    run(git(&top).args(["add", "--all", "--"]).args(paths)).map(drop)
}

/// Unstages `paths` — everything when empty. Before the first commit there's
/// no HEAD to reset to, so files simply leave the index.
pub fn unstage(root: &Path, paths: &[String]) -> Result<(), Message> {
    let top = require_repo(root)?;
    let _lock = write_lock();
    if !has_head(&top) {
        let mut cmd = git(&top);
        cmd.args(["rm", "--cached", "-r", "-q", "--"]);
        if paths.is_empty() {
            cmd.arg(".");
        } else {
            cmd.args(paths);
        }
        return run(&mut cmd).map(drop);
    }
    if paths.is_empty() {
        run(git(&top).args(["reset", "-q"])).map(drop)
    } else {
        run(git(&top).args(["reset", "-q", "--"]).args(paths)).map(drop)
    }
}

/// A change to undo.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscardItem {
    pub path: String,
    pub orig_path: Option<String>,
    pub kind: ChangeKind,
    pub staged: bool,
}

/// Undo Changes: an unstaged change goes back to what's staged (an untracked
/// file is deleted), a staged one back to the last commit — index and disk
/// both (a newly added file is deleted).
pub fn discard(root: &Path, items: Vec<DiscardItem>) -> Result<(), Message> {
    let top = require_repo(root)?;
    let _lock = write_lock();
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
                to_head.extend(item.orig_path);
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitResult {
    pub hash: String,
    pub subject: String,
}

/// Commits what's staged — or, with `stage_all`, every change (Commit All).
/// `amend` with an empty message keeps the last commit's.
pub fn commit(
    root: &Path,
    message: &str,
    amend: bool,
    stage_all: bool,
) -> Result<CommitResult, Message> {
    let top = require_repo(root)?;
    let _lock = write_lock();
    if stage_all {
        run(git(&top).args(["add", "--all"]))?;
    }
    let mut cmd = git(&top);
    cmd.args(["commit", "-q", "--cleanup=strip"]);
    if amend {
        cmd.arg("--amend");
    }
    if amend && message.trim().is_empty() {
        run(cmd.arg("--no-edit"))?;
    } else {
        run_with_input(cmd.args(["-F", "-"]), message)?;
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
pub fn last_message(root: &Path) -> Result<String, Message> {
    let top = require_repo(root)?;
    run(git(&top).args(["log", "-1", "--format=%B"])).map(|m| m.trim_end().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_branch_headers() {
        let raw = "# branch.oid 1234abcd\0# branch.head main\0# branch.upstream origin/main\0# branch.ab +2 -3\0# stash 4\0";
        let status = parse_status(raw);
        assert_eq!(status.head.as_deref(), Some("1234abcd"));
        assert_eq!(status.branch.as_deref(), Some("main"));
        assert_eq!(status.upstream.as_deref(), Some("origin/main"));
        assert_eq!((status.ahead, status.behind, status.stashes), (2, 3, 4));
    }

    #[test]
    fn parses_initial_and_detached() {
        let status = parse_status("# branch.oid (initial)\0# branch.head (detached)\0");
        assert_eq!(
            (status.head, status.branch, status.upstream),
            (None, None, None)
        );
    }

    #[test]
    fn splits_staged_and_unstaged() {
        let raw = concat!(
            "1 M. N... 100644 100644 100644 aaa bbb src/lib.rs\0",
            "1 .M N... 100644 100644 100644 aaa aaa README.md\0",
            "1 MD N... 100644 100644 000000 aaa bbb both ways.txt\0",
            "1 A. N... 000000 100644 100644 000 bbb new file.rs\0",
            "? untracked dir/file with spaces.txt\0",
        );
        let status = parse_status(raw);
        let staged: Vec<_> = status
            .staged
            .iter()
            .map(|f| (f.path.as_str(), f.kind))
            .collect();
        let unstaged: Vec<_> = status
            .unstaged
            .iter()
            .map(|f| (f.path.as_str(), f.kind))
            .collect();
        assert_eq!(
            staged,
            [
                ("src/lib.rs", ChangeKind::Modified),
                ("both ways.txt", ChangeKind::Modified),
                ("new file.rs", ChangeKind::Added)
            ]
        );
        assert_eq!(
            unstaged,
            [
                ("README.md", ChangeKind::Modified),
                ("both ways.txt", ChangeKind::Deleted),
                ("untracked dir/file with spaces.txt", ChangeKind::Untracked)
            ]
        );
    }

    #[test]
    fn parses_renames_with_their_origin() {
        let raw = concat!(
            "2 R. N... 100644 100644 100644 aaa aaa R100 new name.rs\0old name.rs\0",
            "1 .M N... 100644 100644 100644 a a after.rs\0"
        );
        let status = parse_status(raw);
        assert_eq!(
            status.staged,
            [change(
                "new name.rs",
                Some("old name.rs"),
                ChangeKind::Renamed
            )]
        );
        // The record after the rename's extra field still parses.
        assert_eq!(
            status.unstaged,
            [change("after.rs", None, ChangeKind::Modified)]
        );
    }

    #[test]
    fn parses_conflicts() {
        let status =
            parse_status("u UU N... 100644 100644 100644 100644 a b c conflicted file.rs\0");
        assert_eq!(
            status.conflicted,
            [change("conflicted file.rs", None, ChangeKind::Conflicted)]
        );
        assert!(status.staged.is_empty() && status.unstaged.is_empty());
    }

    /// End to end against a scratch repository — skipped where git isn't installed.
    #[test]
    fn status_of_a_scratch_repository() {
        let dir = std::env::temp_dir().join(format!("substrate-git-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        if run(git(&dir).args(["init", "-q", "--initial-branch", "main"])).is_err() {
            return;
        }
        std::fs::write(dir.join("a.txt"), "one\n").unwrap();
        let s = status(&dir).unwrap().expect("a repository");
        assert_eq!((s.branch.as_deref(), s.head), (Some("main"), None));
        assert_eq!(
            s.unstaged,
            [FileChange {
                abs_path: absolute(&dir.canonicalize().unwrap(), "a.txt"),
                ..change("a.txt", None, ChangeKind::Untracked)
            }]
        );
        stage(&dir, &["a.txt".into()]).unwrap();
        let staged = status(&dir).unwrap().unwrap().staged;
        assert_eq!((staged.len(), staged[0].kind), (1, ChangeKind::Added));
        let _ = std::fs::remove_dir_all(&dir);
        assert!(status(&std::env::temp_dir()).is_ok());
    }
}
