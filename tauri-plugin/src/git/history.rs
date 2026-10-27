//! The history and the branches.

use substrate_git_app::branching;
use substrate_git_app::ports::{Branches, History};
use substrate_git_domain::GitError;
use substrate_git_domain::branches::{BranchCheckout, BranchSource, DeleteMode};
use substrate_message::Message;
use substrate_workspace_app::Workspace;
use tauri::State;

use super::wire::branches::BranchDto;
use super::wire::commits::CommitDto;
use super::wire::status::FileChangeDto;
use super::{SourceControl, checked};
use crate::blocking;

#[tauri::command]
pub(crate) async fn git_log(
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
    rev: String,
    skip: u32,
    max: u32,
) -> Result<Vec<CommitDto>, Message> {
    let root = checked(&workspace, &root)?;
    let source = source.inner().clone();
    blocking(move || {
        let log = source.adapter.log(&root, &rev, skip, max)?;
        Ok::<_, GitError>(log.into_iter().map(CommitDto::from).collect())
    })
    .await
}

#[tauri::command]
pub(crate) async fn git_commit_files(
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
    hash: String,
) -> Result<Vec<FileChangeDto>, Message> {
    let root = checked(&workspace, &root)?;
    let source = source.inner().clone();
    blocking(move || {
        let files = source.adapter.commit_files(&root, &hash)?;
        Ok::<_, GitError>(files.into_iter().map(FileChangeDto::from).collect())
    })
    .await
}

#[tauri::command]
pub(crate) async fn git_branches(
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
) -> Result<Vec<BranchDto>, Message> {
    let root = checked(&workspace, &root)?;
    let source = source.inner().clone();
    blocking(move || {
        let branches = source.adapter.list(&root)?;
        Ok::<_, GitError>(branches.into_iter().map(BranchDto::from).collect())
    })
    .await
}

/// Checks out `name`; `remote` (the UI's flag) when it's a remote-tracking branch.
#[tauri::command]
pub(crate) async fn git_switch(
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
    name: String,
    remote: bool,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    let kind = if remote {
        BranchSource::Remote
    } else {
        BranchSource::Local
    };
    let source = source.inner().clone();
    blocking(move || branching::switch(&source.adapter, &source.lock, &root, &name, kind)).await
}

/// New Branch from `start`; `checkout` (the UI's flag) switches to it.
#[tauri::command]
#[expect(
    clippy::too_many_arguments,
    reason = "Tauri injects state as separate parameters; the UI sends the rest"
)]
pub(crate) async fn git_branch_create(
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
    name: String,
    start: String,
    checkout: bool,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    let checkout = if checkout {
        BranchCheckout::Switch
    } else {
        BranchCheckout::Stay
    };
    let source = source.inner().clone();
    blocking(move || {
        let names = (name.as_str(), start.as_str());
        branching::create(&source.adapter, &source.lock, &root, names, checkout)
    })
    .await
}

/// Deletes a local branch; `force` (the UI's flag) even when it isn't merged.
#[tauri::command]
pub(crate) async fn git_branch_delete(
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
    name: String,
    force: bool,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    let mode = if force {
        DeleteMode::Force
    } else {
        DeleteMode::Merged
    };
    let source = source.inner().clone();
    blocking(move || branching::delete(&source.adapter, &source.lock, &root, &name, mode)).await
}
