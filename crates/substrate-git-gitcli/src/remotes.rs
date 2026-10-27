//! Remotes and the long runs, through `git`.

use std::path::Path;

use substrate_git_app::Output;
use substrate_git_app::ports::Remotes;
use substrate_git_domain::GitError;
use substrate_git_domain::remotes::{
    Integration, RemoteOperation, Resolution, clone_directory_name,
};
use substrate_git_domain::status::Operation;

use crate::cli::{
    current_branch, git, has_upstream, repo_of, require_repo, run, stream, stream_git,
};
use crate::{GitCli, operation};

/// The remote a branch pushes to when nothing else says: git's convention.
const ORIGIN: &str = "origin";

/// Where a branch without an upstream gets pushed: its own `pushRemote`,
/// the repository's `remote.pushDefault`, `origin`, else the only remote.
fn push_remote(top: &Path, branch: &str) -> Result<String, GitError> {
    let config = |key: &str| {
        run(git(top).args(["config", "--get", key]))
            .ok()
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
    };
    if let Some(remote) =
        config(&format!("branch.{branch}.pushRemote")).or_else(|| config("remote.pushDefault"))
    {
        return Ok(remote);
    }
    let remotes = run(git(top).arg("remote"))?;
    let remotes: Vec<&str> = remotes
        .lines()
        .map(str::trim)
        .filter(|r| !r.is_empty())
        .collect();
    if remotes.contains(&ORIGIN) {
        return Ok(ORIGIN.to_string());
    }
    match remotes[..] {
        [only] => Ok(only.to_string()),
        [] => Err(GitError::NoPushRemote),
        _ => Err(GitError::NoOrigin),
    }
}

fn push(out: Output<'_>, top: &Path) -> Result<(), GitError> {
    if has_upstream(top) {
        return stream_git(out, top, &["push", "--progress"]);
    }
    // Push and create the upstream, as Visual Studio does for a new branch.
    let branch = current_branch(top).ok_or(GitError::DetachedPush)?;
    let remote = push_remote(top, &branch)?;
    stream_git(
        out,
        top,
        &["push", "--progress", "--set-upstream", &remote, &branch],
    )
}

fn pull(out: Output<'_>, top: &Path) -> Result<(), GitError> {
    if !has_upstream(top) {
        return Err(GitError::NoUpstream);
    }
    stream_git(out, top, &["pull", "--progress"])
}

impl Remotes for GitCli {
    fn synchronize(
        &self,
        root: &Path,
        operation: RemoteOperation,
        out: Output<'_>,
    ) -> Result<(), GitError> {
        let top = require_repo(root)?;
        match operation {
            RemoteOperation::Fetch => stream_git(out, &top, &["fetch", "--all", "--progress"]),
            RemoteOperation::Pull => pull(out, &top),
            RemoteOperation::Push => push(out, &top),
            RemoteOperation::Sync => pull(out, &top).and_then(|()| push(out, &top)),
        }
    }

    fn integrate(
        &self,
        root: &Path,
        integration: Integration,
        branch: &str,
        out: Output<'_>,
    ) -> Result<(), GitError> {
        let top = require_repo(root)?;
        match integration {
            Integration::Merge => stream_git(out, &top, &["merge", "--no-edit", branch]),
            Integration::Rebase => stream_git(out, &top, &["rebase", branch]),
        }
    }

    fn resolve(
        &self,
        root: &Path,
        resolution: Resolution,
        out: Output<'_>,
    ) -> Result<(), GitError> {
        let Some((top, git_dir)) = repo_of(root)? else {
            return Err(GitError::NotARepository);
        };
        let waiting = operation::in_progress(&git_dir).ok_or(GitError::NothingInProgress)?;
        // A merge is continued by committing it.
        if resolution == Resolution::Continue && waiting == Operation::Merge {
            return stream_git(out, &top, &["commit", "--no-edit"]);
        }
        let flag = match resolution {
            Resolution::Continue => "--continue",
            Resolution::Abort => "--abort",
        };
        stream_git(out, &top, &[operation::command(waiting), flag])
    }

    fn clone_repository(
        &self,
        url: &str,
        parent: &Path,
        name: &str,
        out: Output<'_>,
    ) -> Result<String, GitError> {
        let name = if name.trim().is_empty() {
            clone_directory_name(url)
        } else {
            name.trim().to_string()
        };
        let dest = parent.join(&name);
        if dest.exists()
            && std::fs::read_dir(&dest)
                .map(|mut d| d.next().is_some())
                .unwrap_or(true)
        {
            return Err(GitError::CloneTargetNotEmpty { path: dest });
        }
        let dest_text = dest.to_string_lossy().into_owned();
        stream(
            out,
            git(parent).args(["clone", "--progress", "--", url, &dest_text]),
            &format!("clone {url} {dest_text}"),
        )?;
        Ok(dest_text)
    }

    fn init(&self, root: &Path, branch: &str, out: Output<'_>) -> Result<(), GitError> {
        let mut args = vec!["init"];
        if !branch.trim().is_empty() {
            args.extend(["--initial-branch", branch.trim()]);
        }
        stream_git(out, root, &args)
    }
}
