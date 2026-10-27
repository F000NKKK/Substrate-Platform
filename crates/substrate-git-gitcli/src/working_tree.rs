//! The working tree and the index, through `git`.

use std::path::Path;

use substrate_git_app::ports::WorkingTree;
use substrate_git_domain::GitError;
use substrate_git_domain::commits::{CommitMode, CommitResult, CommitScope};
use substrate_git_domain::status::{ChangeKind, DiscardItem, RepoStatus};

use crate::cli::{git, has_head, repo_of, require_repo, run, run_with_input};
use crate::porcelain::parse_status;
use crate::{GitCli, operation};

impl WorkingTree for GitCli {
    fn status(&self, root: &Path) -> Result<Option<RepoStatus>, GitError> {
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
        status.set_absolute_paths(&top);
        status.repo_root = top.to_string_lossy().into_owned();
        status.operation = operation::in_progress(&git_dir);
        Ok(Some(status))
    }

    fn stage(&self, root: &Path, paths: &[String]) -> Result<(), GitError> {
        let top = require_repo(root)?;
        run(git(&top).args(["add", "--all", "--"]).args(paths)).map(drop)
    }

    fn unstage(&self, root: &Path, paths: &[String]) -> Result<(), GitError> {
        let top = require_repo(root)?;
        // Before the first commit there's no HEAD to reset to: files simply leave the index.
        if !has_head(&top) {
            let mut command = git(&top);
            command.args(["rm", "--cached", "-r", "-q", "--"]);
            if paths.is_empty() {
                command.arg(".");
            } else {
                command.args(paths);
            }
            return run(&mut command).map(drop);
        }
        if paths.is_empty() {
            run(git(&top).args(["reset", "-q"])).map(drop)
        } else {
            run(git(&top).args(["reset", "-q", "--"]).args(paths)).map(drop)
        }
    }

    fn discard(&self, root: &Path, items: &[DiscardItem]) -> Result<(), GitError> {
        let top = require_repo(root)?;
        let head = has_head(&top);
        let (mut untracked, mut worktree, mut added, mut to_head) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for item in items {
            let path = item.path.clone();
            match (item.staged, item.kind) {
                (false, ChangeKind::Untracked) => untracked.push(path),
                (false, _) => worktree.push(path),
                (true, ChangeKind::Added | ChangeKind::Copied) => added.push(path),
                (true, _) if !head => added.push(path),
                (true, ChangeKind::Renamed) => {
                    added.push(path);
                    to_head.extend(item.original_path.clone());
                }
                (true, _) => to_head.push(path),
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

    fn commit(
        &self,
        root: &Path,
        message: &str,
        mode: CommitMode,
        scope: CommitScope,
    ) -> Result<CommitResult, GitError> {
        let top = require_repo(root)?;
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
}
