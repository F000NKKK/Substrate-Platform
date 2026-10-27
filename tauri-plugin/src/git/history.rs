//! The history and the branches.

use super::checked;
use crate::blocking;
use substrate_message::Message;
use substrate_platform::git::WriteLock;
use substrate_platform::git::branch::{self, Branch, BranchCheckout, BranchSource, DeleteMode};
use substrate_platform::git::history::{self, Commit};
use substrate_platform::git::status::FileChange;
use substrate_platform::workspace::Workspace;
use tauri::State;

#[tauri::command]
pub async fn git_log(
    workspace: State<'_, Workspace>,
    root: String,
    rev: String,
    skip: u32,
    max: u32,
) -> Result<Vec<Commit>, Message> {
    let root = checked(&workspace, &root)?;
    blocking(move || history::log(&root, &rev, skip, max)).await
}

#[tauri::command]
pub async fn git_commit_files(
    workspace: State<'_, Workspace>,
    root: String,
    hash: String,
) -> Result<Vec<FileChange>, Message> {
    let root = checked(&workspace, &root)?;
    blocking(move || history::commit_files(&root, &hash)).await
}

#[tauri::command]
pub async fn git_branches(
    workspace: State<'_, Workspace>,
    root: String,
) -> Result<Vec<Branch>, Message> {
    let root = checked(&workspace, &root)?;
    blocking(move || branch::list(&root)).await
}

/// Checks out `name`; `remote` (the UI's flag) when it's a remote-tracking branch.
#[tauri::command]
pub async fn git_switch(
    workspace: State<'_, Workspace>,
    lock: State<'_, WriteLock>,
    root: String,
    name: String,
    remote: bool,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    let source = if remote {
        BranchSource::Remote
    } else {
        BranchSource::Local
    };
    let lock = lock.inner().clone();
    blocking(move || branch::switch(&lock, &root, &name, source)).await
}

/// New Branch from `start`; `checkout` (the UI's flag) switches to it.
#[tauri::command]
#[expect(
    clippy::too_many_arguments,
    reason = "Tauri injects state as separate parameters; the UI sends the rest"
)]
pub async fn git_branch_create(
    workspace: State<'_, Workspace>,
    lock: State<'_, WriteLock>,
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
    let lock = lock.inner().clone();
    blocking(move || branch::create(&lock, &root, &name, &start, checkout)).await
}

/// Deletes a local branch; `force` (the UI's flag) even when it isn't merged.
#[tauri::command]
pub async fn git_branch_delete(
    workspace: State<'_, Workspace>,
    lock: State<'_, WriteLock>,
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
    let lock = lock.inner().clone();
    blocking(move || branch::delete(&lock, &root, &name, mode)).await
}
