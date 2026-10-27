//! What talks to remotes or runs long — streamed to the UI.

use super::{checked, output};
use crate::blocking;
use std::path::Path;
use substrate_message::Message;
use substrate_platform::git::WriteLock;
use substrate_platform::git::remote::{self, Integration, RemoteOperation, Resolution};
use substrate_platform::workspace::Workspace;
use tauri::{AppHandle, Runtime, State};

/// Fetch / Pull / Push / Sync, streamed.
#[tauri::command]
pub async fn git_remote<R: Runtime>(
    app: AppHandle<R>,
    workspace: State<'_, Workspace>,
    lock: State<'_, WriteLock>,
    root: String,
    op: RemoteOperation,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    let lock = lock.inner().clone();
    blocking(move || remote::synchronize(&lock, &root, op, &output(&app))).await
}

/// Merge into Current / Rebase onto, streamed.
#[tauri::command]
#[expect(
    clippy::too_many_arguments,
    reason = "Tauri injects state as separate parameters; the UI sends the rest"
)]
pub async fn git_integrate<R: Runtime>(
    app: AppHandle<R>,
    workspace: State<'_, Workspace>,
    lock: State<'_, WriteLock>,
    root: String,
    op: Integration,
    branch: String,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    let lock = lock.inner().clone();
    blocking(move || remote::integrate(&lock, &root, op, &branch, &output(&app))).await
}

/// Continues (`go_on`, the UI's flag) or aborts the
/// merge/rebase/cherry-pick/revert in progress, streamed.
#[tauri::command]
pub async fn git_operation<R: Runtime>(
    app: AppHandle<R>,
    workspace: State<'_, Workspace>,
    lock: State<'_, WriteLock>,
    root: String,
    go_on: bool,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    let resolution = if go_on {
        Resolution::Continue
    } else {
        Resolution::Abort
    };
    let lock = lock.inner().clone();
    blocking(move || remote::resolve(&lock, &root, resolution, &output(&app))).await
}

/// Clone a Repository into `parent` — a folder the user picked in the native
/// dialog, so it joins the workspace the way Open Folder's does. The new
/// working tree's path, to open.
#[tauri::command]
pub async fn git_clone<R: Runtime>(
    app: AppHandle<R>,
    workspace: State<'_, Workspace>,
    url: String,
    parent: String,
    name: String,
) -> Result<String, Message> {
    let parent = workspace.allow(Path::new(&parent))?;
    blocking(move || remote::clone(&url, &parent, &name, &output(&app))).await
}

/// Create Git Repository in the project folder.
#[tauri::command]
pub async fn git_init<R: Runtime>(
    app: AppHandle<R>,
    workspace: State<'_, Workspace>,
    root: String,
    branch: String,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    blocking(move || remote::init(&root, &branch, &output(&app))).await
}
