//! What talks to remotes or runs long — streamed to the UI.

use super::{Res, checked, output};
use crate::blocking;
use std::path::Path;
use substrate_platform::git::remote::{self, BranchOp, RemoteOp};
use substrate_platform::workspace::Workspace;
use tauri::{AppHandle, Runtime, State};

/// Fetch / Pull / Push / Sync, streamed.
#[tauri::command]
pub async fn git_remote<R: Runtime>(app: AppHandle<R>, ws: State<'_, Workspace>, root: String, op: RemoteOp) -> Res<()> {
    let root = checked(&ws, &root)?;
    blocking(move || remote::remote(&root, op, &output(&app))).await
}

/// Merge into Current / Rebase onto, streamed.
#[tauri::command]
pub async fn git_integrate<R: Runtime>(app: AppHandle<R>, ws: State<'_, Workspace>, root: String, op: BranchOp, branch: String) -> Res<()> {
    let root = checked(&ws, &root)?;
    blocking(move || remote::integrate(&root, op, &branch, &output(&app))).await
}

/// Continues (`go_on`) or aborts the merge/rebase/cherry-pick/revert in progress, streamed.
#[tauri::command]
pub async fn git_operation<R: Runtime>(app: AppHandle<R>, ws: State<'_, Workspace>, root: String, go_on: bool) -> Res<()> {
    let root = checked(&ws, &root)?;
    blocking(move || remote::operation(&root, go_on, &output(&app))).await
}

/// Clone a Repository into `parent` — a folder the user picked in the native
/// dialog, so it joins the workspace the way Open Folder's does. The new
/// working tree's path, to open.
#[tauri::command]
pub async fn git_clone<R: Runtime>(app: AppHandle<R>, ws: State<'_, Workspace>, url: String, parent: String, name: String) -> Res<String> {
    let parent = ws.allow(Path::new(&parent))?;
    blocking(move || remote::clone(&url, &parent, &name, &output(&app))).await
}

/// Create Git Repository in the project folder.
#[tauri::command]
pub async fn git_init<R: Runtime>(app: AppHandle<R>, ws: State<'_, Workspace>, root: String, branch: String) -> Res<()> {
    let root = checked(&ws, &root)?;
    blocking(move || remote::init(&root, &branch, &output(&app))).await
}
