//! A repository's state, on the wire.

use serde::Serialize;
use substrate_git_domain::status::RepoStatus;

use super::{FileChangeDto, OperationDto};

/// A repository's state for Git Changes.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RepoStatusDto {
    repo_root: String,
    branch: Option<String>,
    head: Option<String>,
    upstream: Option<String>,
    ahead: u32,
    behind: u32,
    staged: Vec<FileChangeDto>,
    unstaged: Vec<FileChangeDto>,
    conflicted: Vec<FileChangeDto>,
    stashes: u32,
    operation: Option<OperationDto>,
}

fn changes(files: Vec<substrate_git_domain::status::FileChange>) -> Vec<FileChangeDto> {
    files.into_iter().map(FileChangeDto::from).collect()
}

impl From<RepoStatus> for RepoStatusDto {
    fn from(status: RepoStatus) -> Self {
        Self {
            repo_root: status.repo_root,
            branch: status.branch,
            head: status.head,
            upstream: status.upstream,
            ahead: status.ahead,
            behind: status.behind,
            staged: changes(status.staged),
            unstaged: changes(status.unstaged),
            conflicted: changes(status.conflicted),
            stashes: status.stashes,
            operation: status.operation.map(OperationDto::from),
        }
    }
}
