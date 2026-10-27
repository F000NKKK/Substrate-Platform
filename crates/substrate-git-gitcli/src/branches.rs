//! Branches, through `git`.

use std::path::Path;

use substrate_git_app::ports::Branches;
use substrate_git_domain::GitError;
use substrate_git_domain::branches::{Branch, BranchCheckout, BranchSource, DeleteMode};

use crate::GitCli;
use crate::cli::{git, require_repo, run};
use crate::porcelain::{BRANCH_FORMAT, parse_branches};

impl Branches for GitCli {
    fn list(&self, root: &Path) -> Result<Vec<Branch>, GitError> {
        let top = require_repo(root)?;
        Ok(parse_branches(&run(git(&top).args([
            "for-each-ref",
            BRANCH_FORMAT,
            "refs/heads",
            "refs/remotes",
        ]))?))
    }

    fn switch(&self, root: &Path, name: &str, source: BranchSource) -> Result<(), GitError> {
        let top = require_repo(root)?;
        let mut command = git(&top);
        command.arg("switch");
        if source == BranchSource::Remote {
            command.arg("--track");
        }
        run(command.arg(name)).map(drop)
    }

    fn create(
        &self,
        root: &Path,
        name: &str,
        start: &str,
        checkout: BranchCheckout,
    ) -> Result<(), GitError> {
        let top = require_repo(root)?;
        let mut command = git(&top);
        match checkout {
            BranchCheckout::Switch => command.args(["switch", "-c", name]),
            BranchCheckout::Stay => command.args(["branch", name]),
        };
        if !start.is_empty() {
            command.arg(start);
        }
        run(&mut command).map(drop)
    }

    fn delete(&self, root: &Path, name: &str, mode: DeleteMode) -> Result<(), GitError> {
        let top = require_repo(root)?;
        let flag = match mode {
            DeleteMode::Merged => "-d",
            DeleteMode::Force => "-D",
        };
        run(git(&top).args(["branch", flag, name])).map(drop)
    }
}
