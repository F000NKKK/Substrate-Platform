//! Stashes, through `git`.

use std::path::Path;

use substrate_git_app::ports::Stashes;
use substrate_git_domain::GitError;
use substrate_git_domain::stashes::{StashEntry, StashMode, StashOperation};

use crate::GitCli;
use crate::cli::{git, require_repo, run};
use crate::porcelain::{STASH_FORMAT, parse_stashes};

impl Stashes for GitCli {
    fn list(&self, root: &Path) -> Result<Vec<StashEntry>, GitError> {
        let top = require_repo(root)?;
        Ok(parse_stashes(&run(git(&top).args([
            "stash",
            "list",
            STASH_FORMAT,
        ]))?))
    }

    fn push(&self, root: &Path, message: &str, mode: StashMode) -> Result<(), GitError> {
        let top = require_repo(root)?;
        let mut command = git(&top);
        command.args(["stash", "push", "--quiet"]);
        match mode {
            StashMode::All => command.arg("--include-untracked"),
            StashMode::KeepIndex => command.args(["--include-untracked", "--keep-index"]),
            StashMode::Staged => command.arg("--staged"),
        };
        if !message.trim().is_empty() {
            command.args(["--message", message.trim()]);
        }
        run(&mut command).map(drop)
    }

    fn perform(&self, root: &Path, index: u32, operation: StashOperation) -> Result<(), GitError> {
        let top = require_repo(root)?;
        let verb = match operation {
            StashOperation::Apply => "apply",
            StashOperation::Pop => "pop",
            StashOperation::Drop => "drop",
        };
        run(git(&top).args(["stash", verb, "--quiet", &format!("stash@{{{index}}}")])).map(drop)
    }
}
