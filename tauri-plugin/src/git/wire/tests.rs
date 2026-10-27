//! The wire format the UI relies on, written out.

use serde_json::json;
use substrate_git_domain::branches::Branch;
use substrate_git_domain::commits::CommitResult;
use substrate_git_domain::diffs::{DiffSide, DiffTexts, SideVersion};
use substrate_git_domain::status::{ChangeKind, DiscardItem, FileChange, Operation, RepoStatus};

use super::branches::BranchDto;
use super::commits::CommitResultDto;
use super::diffs::{DiffSideDto, DiffTextsDto};
use super::stashes::StashModeDto;
use super::status::{DiscardItemDto, RepoStatusDto};

#[test]
fn a_status_has_the_names_the_ui_reads() {
    let mut change = FileChange::new("b.rs", Some("a.rs"), ChangeKind::Renamed);
    change.absolute_path = "/w/b.rs".into();
    let status = RepoStatus {
        repo_root: "/w".into(),
        branch: Some("main".into()),
        staged: vec![change],
        operation: Some(Operation::CherryPick),
        ..RepoStatus::default()
    };
    let value = serde_json::to_value(RepoStatusDto::from(status)).expect("serializes");
    assert_eq!(
        value,
        json!({
            "repoRoot": "/w", "branch": "main", "head": null, "upstream": null,
            "ahead": 0, "behind": 0,
            "staged": [{ "path": "b.rs", "origPath": "a.rs", "kind": "renamed", "absPath": "/w/b.rs" }],
            "unstaged": [], "conflicted": [], "stashes": 0, "operation": "cherryPick",
        })
    );
}

#[test]
fn a_discard_item_reads_what_the_ui_sends() {
    let raw = json!({ "path": "n.rs", "origPath": "o.rs", "kind": "typeChanged", "staged": true });
    let item: DiscardItemDto = serde_json::from_value(raw).expect("deserializes");
    let item = DiscardItem::from(item);
    assert_eq!(item.kind, ChangeKind::TypeChanged);
    assert_eq!(item.original_path.as_deref(), Some("o.rs"));
    assert!(item.staged);
}

#[test]
fn modes_are_camel_case_words() {
    let mode: StashModeDto = serde_json::from_value(json!("keepIndex")).expect("deserializes");
    assert!(matches!(mode, StashModeDto::KeepIndex));
}

#[test]
fn small_results_keep_their_names() {
    let branch = Branch {
        name: "main".into(),
        remote: false,
        current: true,
        upstream: None,
        ahead: 1,
        behind: 2,
        gone: false,
        hash: "abc".into(),
        date: 7,
        subject: "Tip".into(),
    };
    let value = serde_json::to_value(BranchDto::from(branch)).expect("serializes");
    assert_eq!(value["current"], json!(true));
    assert_eq!(value["subject"], json!("Tip"));
    let made = CommitResult {
        hash: "abc".into(),
        subject: "s".into(),
    };
    let committed = serde_json::to_value(CommitResultDto::from(made)).expect("serializes");
    assert_eq!(committed, json!({ "hash": "abc", "subject": "s" }));
    let texts = DiffTexts {
        left: Some("a".into()),
        ..DiffTexts::default()
    };
    let diffed = serde_json::to_value(DiffTextsDto::from(texts)).expect("serializes");
    assert_eq!(
        diffed,
        json!({ "left": "a", "right": null, "binary": false })
    );
}

#[test]
fn a_diff_side_names_its_version_by_word() {
    let side = |rev: &str| -> DiffSide {
        let raw = json!({ "rev": rev, "path": "a.rs" });
        serde_json::from_value::<DiffSideDto>(raw)
            .expect("deserializes")
            .into()
    };
    assert_eq!(side("").version, SideVersion::Nothing);
    assert_eq!(side("INDEX").version, SideVersion::Index);
    assert_eq!(side("WORKTREE").version, SideVersion::Worktree);
    assert_eq!(
        side("abc123").version,
        SideVersion::Revision("abc123".into())
    );
}
