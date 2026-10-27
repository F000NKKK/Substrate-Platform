//! What was committed, through `git`.

use std::path::Path;

use substrate_git_app::ports::History;
use substrate_git_domain::GitError;
use substrate_git_domain::commits::Commit;
use substrate_git_domain::status::{FileChange, absolute};

use crate::GitCli;
use crate::cli::{git, has_head, require_repo, run};
use crate::porcelain::{LOG_FORMAT, parse_log, parse_name_status};

impl History for GitCli {
    fn last_message(&self, root: &Path) -> Result<String, GitError> {
        let top = require_repo(root)?;
        run(git(&top).args(["log", "-1", "--format=%B"])).map(|m| m.trim_end().to_string())
    }

    fn log(&self, root: &Path, rev: &str, skip: u32, max: u32) -> Result<Vec<Commit>, GitError> {
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

    fn commit_files(&self, root: &Path, hash: &str) -> Result<Vec<FileChange>, GitError> {
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
}
