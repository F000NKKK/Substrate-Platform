//! Status parsing's tests.

use super::*;
use crate::git::run::{absolute, git, run};

#[test]
fn parses_branch_headers() {
    let raw = "# branch.oid 1234abcd\0# branch.head main\0# branch.upstream origin/main\0# branch.ab +2 -3\0# stash 4\0";
    let status = parse(raw);
    assert_eq!(status.head.as_deref(), Some("1234abcd"));
    assert_eq!(status.branch.as_deref(), Some("main"));
    assert_eq!(status.upstream.as_deref(), Some("origin/main"));
    assert_eq!((status.ahead, status.behind, status.stashes), (2, 3, 4));
}

#[test]
fn parses_initial_and_detached() {
    let status = parse("# branch.oid (initial)\0# branch.head (detached)\0");
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
    let status = parse(raw);
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
    let status = parse(raw);
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
    let status = parse("u UU N... 100644 100644 100644 100644 a b c conflicted file.rs\0");
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

/// End to end against a scratch repository — skipped where git isn't installed.
#[test]
fn status_of_a_scratch_repository() {
    let dir = std::env::temp_dir().join(format!("substrate-git-test-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    if run(git(&dir).args(["init", "-q", "--initial-branch", "main"])).is_err() {
        return;
    }
    std::fs::write(dir.join("a.txt"), "one\n").unwrap();
    let s = read(&dir).unwrap().expect("a repository");
    assert_eq!((s.branch.as_deref(), s.head), (Some("main"), None));
    assert_eq!(
        s.unstaged,
        [FileChange {
            absolute_path: absolute(&dir.canonicalize().unwrap(), "a.txt"),
            ..FileChange::new("a.txt", None, ChangeKind::Untracked)
        }]
    );
    stage(&dir, &["a.txt".into()]).unwrap();
    let staged = read(&dir).unwrap().unwrap().staged;
    assert_eq!((staged.len(), staged[0].kind), (1, ChangeKind::Added));
    std::fs::remove_dir_all(&dir).ok();
    assert!(read(&std::env::temp_dir()).is_ok());
}
