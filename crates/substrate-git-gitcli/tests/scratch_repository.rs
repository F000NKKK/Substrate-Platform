//! End to end against a scratch repository — skipped where git isn't installed.

use std::path::Path;
use std::process::Command;

use substrate_git_app::ports::{Branches, History, Stashes, WorkingTree};
use substrate_git_domain::branches::{BranchCheckout, BranchSource};
use substrate_git_domain::commits::{CommitMode, CommitScope};
use substrate_git_domain::stashes::StashMode;
use substrate_git_domain::status::{ChangeKind, absolute};
use substrate_git_gitcli::GitCli;
use substrate_platform::text_view::TextViews;

fn git(dir: &Path, args: &[&str]) -> bool {
    Command::new("git")
        .current_dir(dir)
        .args(["-c", "user.name=Test", "-c", "user.email=test@example.com"])
        .args(args)
        .output()
        .is_ok_and(|out| out.status.success())
}

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("substrate-{name}-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).expect("scratch folder");
    dir
}

#[test]
fn status_of_a_scratch_repository() {
    let dir = scratch("git-status");
    if !git(&dir, &["init", "-q", "--initial-branch", "main"]) {
        return;
    }
    let cli = GitCli::new(TextViews::default());
    std::fs::write(dir.join("a.txt"), "one\n").expect("write");
    let status = cli.status(&dir).expect("status").expect("a repository");
    assert_eq!(
        (status.branch.as_deref(), status.head),
        (Some("main"), None)
    );
    let top = dir.canonicalize().expect("canonical");
    assert_eq!(status.unstaged.len(), 1);
    assert_eq!(status.unstaged[0].kind, ChangeKind::Untracked);
    assert_eq!(status.unstaged[0].absolute_path, absolute(&top, "a.txt"));

    cli.stage(&dir, &["a.txt".into()]).expect("stage");
    let staged = cli.status(&dir).expect("status").expect("repo").staged;
    assert_eq!((staged.len(), staged[0].kind), (1, ChangeKind::Added));

    let result = cli
        .commit(&dir, "first", CommitMode::New, CommitScope::Staged)
        .expect("commit");
    assert_eq!(result.subject, "first");
    assert_eq!(cli.last_message(&dir).expect("message"), "first");
    assert_eq!(cli.log(&dir, "", 0, 10).expect("log").len(), 1);

    cli.create(&dir, "topic", "", BranchCheckout::Stay)
        .expect("create");
    cli.switch(&dir, "topic", BranchSource::Local)
        .expect("switch");
    let branches = Branches::list(&cli, &dir).expect("branches");
    assert!(branches.iter().any(|b| b.name == "topic" && b.current));

    std::fs::write(dir.join("a.txt"), "two\n").expect("write");
    Stashes::push(&cli, &dir, "wip", StashMode::All).expect("stash");
    assert_eq!(Stashes::list(&cli, &dir).expect("stashes").len(), 1);
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_folder_outside_any_repository_has_no_status() {
    let dir = scratch("git-none");
    let cli = GitCli::new(TextViews::default());
    // Where git isn't installed this is an error, which is also not a status.
    assert!(!matches!(cli.status(&dir), Ok(Some(_))));
    std::fs::remove_dir_all(&dir).ok();
}
