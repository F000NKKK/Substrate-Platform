//! History: a branch's commits, and the files a commit changed.

use std::path::Path;

use serde::Serialize;

use super::GitError;
use super::run::{absolute, git, has_head, require_repo, run};
use super::status::{ChangeKind, FileChange};

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Commit {
    pub hash: String,
    pub short_hash: String,
    pub parents: Vec<String>,
    pub author: String,
    pub email: String,
    /// Unix seconds.
    pub date: i64,
    /// Branch and tag names pointing here (`%D`).
    pub refs: Vec<String>,
    pub subject: String,
}

const LOG_FORMAT: &str = "--format=%H%x1f%h%x1f%P%x1f%an%x1f%ae%x1f%at%x1f%D%x1f%s%x1e";

fn parse_log(raw: &str) -> Vec<Commit> {
    raw.split('\x1e')
        .filter_map(|record| {
            let fields: Vec<&str> = record.trim_start_matches('\n').split('\x1f').collect();
            let [hash, short, parents, author, email, date, refs, subject] = fields[..] else {
                return None;
            };
            Some(Commit {
                hash: hash.to_string(),
                short_hash: short.to_string(),
                parents: parents.split_whitespace().map(str::to_string).collect(),
                author: author.to_string(),
                email: email.to_string(),
                date: date.parse().unwrap_or(0),
                refs: refs
                    .split(", ")
                    .filter(|r| !r.is_empty())
                    .map(str::to_string)
                    .collect(),
                subject: subject.to_string(),
            })
        })
        .collect()
}

/// A page of `rev`'s history (HEAD when empty), newest first.
///
/// # Errors
///
/// No repository at the folder, or what `git` reports.
pub fn log(root: &Path, rev: &str, skip: u32, max: u32) -> Result<Vec<Commit>, GitError> {
    let top = require_repo(root)?;
    if rev.is_empty() && !has_head(&top) {
        return Ok(Vec::new());
    }
    let rev = if rev.is_empty() { "HEAD" } else { rev };
    let raw = run(git(&top)
        .args(["log", LOG_FORMAT])
        .arg(format!("--skip={skip}"))
        .arg(format!("--max-count={max}"))
        .args([rev, "--"]))?;
    Ok(parse_log(&raw))
}

/// Parses `git diff --name-status -z` (also `diff-tree`'s): `M\0path\0`,
/// `R100\0old\0new\0`.
fn parse_name_status(raw: &str) -> Vec<FileChange> {
    let mut files = Vec::new();
    let mut fields = raw.split('\0').filter(|f| !f.is_empty());
    while let Some(code) = fields.next() {
        let Some(kind) = code.chars().next().and_then(ChangeKind::from_code) else {
            continue;
        };
        let Some(first) = fields.next() else { break };
        if matches!(kind, ChangeKind::Renamed | ChangeKind::Copied) {
            let Some(second) = fields.next() else { break };
            files.push(FileChange::new(second, Some(first), kind));
        } else {
            files.push(FileChange::new(first, None, kind));
        }
    }
    files
}

/// The files a commit changed, compared with its first parent.
///
/// # Errors
///
/// No repository at the folder, or what `git` reports.
pub fn commit_files(root: &Path, hash: &str) -> Result<Vec<FileChange>, GitError> {
    let top = require_repo(root)?;
    let parents = run(git(&top).args(["rev-list", "--parents", "-n", "1", hash]))?;
    let raw = match parents.split_whitespace().nth(1) {
        Some(parent) => {
            run(git(&top).args(["diff", "--name-status", "-z", "-M", parent, hash, "--"]))?
        }
        None => run(git(&top).args([
            "diff-tree",
            "--root",
            "--no-commit-id",
            "-r",
            "-z",
            "--name-status",
            "-M",
            hash,
        ]))?,
    };
    let mut files = parse_name_status(&raw);
    for file in &mut files {
        file.absolute_path = absolute(&top, &file.path);
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_name_status() {
        let raw = "M\0src/a.rs\0R087\0old.rs\0new.rs\0A\0added file.txt\0D\0gone.rs\0";
        assert_eq!(
            parse_name_status(raw),
            [
                FileChange::new("src/a.rs", None, ChangeKind::Modified),
                FileChange::new("new.rs", Some("old.rs"), ChangeKind::Renamed),
                FileChange::new("added file.txt", None, ChangeKind::Added),
                FileChange::new("gone.rs", None, ChangeKind::Deleted),
            ]
        );
    }

    #[test]
    fn parses_log_records() {
        let raw = "abc123\x1fabc\x1fp1 p2\x1fAda\x1fada@example.com\x1f1700000000\x1fHEAD -> main, origin/main\x1fMerge it\x1e\n\
                   def456\x1fdef\x1f\x1fBob\x1fbob@example.com\x1f1600000000\x1f\x1fInitial commit\x1e\n";
        let log = parse_log(raw);
        assert_eq!(log.len(), 2);
        assert_eq!(log[0].parents, ["p1", "p2"]);
        assert_eq!(log[0].refs, ["HEAD -> main", "origin/main"]);
        assert_eq!(log[0].subject, "Merge it");
        assert!(log[1].parents.is_empty() && log[1].refs.is_empty());
        assert_eq!(log[1].date, 1_600_000_000);
    }
}
