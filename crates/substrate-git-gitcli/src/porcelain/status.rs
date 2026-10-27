//! `git status --porcelain=v2 -z` as a [`RepoStatus`].

use substrate_git_domain::status::{ChangeKind, FileChange, RepoStatus};

/// Parses `git status --porcelain=v2 --branch --show-stash -z`.
#[must_use]
pub fn parse_status(raw: &str) -> RepoStatus {
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
            [FileChange::new(
                "new name.rs",
                Some("old name.rs"),
                ChangeKind::Renamed
            )]
        );
        // The record after the rename's extra field still parses.
        assert_eq!(
            status.unstaged,
            [FileChange::new("after.rs", None, ChangeKind::Modified)]
        );
    }

    #[test]
    fn parses_conflicts() {
        let status =
            parse_status("u UU N... 100644 100644 100644 100644 a b c conflicted file.rs\0");
        assert_eq!(
            status.conflicted,
            [FileChange::new(
                "conflicted file.rs",
                None,
                ChangeKind::Conflicted
            )]
        );
        assert!(status.staged.is_empty() && status.unstaged.is_empty());
    }
}
