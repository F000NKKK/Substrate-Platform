//! What talks to remotes or runs long — streamed to the UI.

use std::path::Path;

use substrate_git_app::ports::Remotes;
use substrate_git_app::syncing;
use substrate_git_domain::remotes::Resolution;
use substrate_message::Message;
use substrate_platform::workspace::Workspace;
use tauri::{AppHandle, Runtime, State};

use super::wire::remotes::{IntegrationDto, RemoteOperationDto};
use super::{SourceControl, checked, output};
use crate::blocking;

/// Fetch / Pull / Push / Sync, streamed.
#[tauri::command]
pub async fn git_remote<R: Runtime>(
    app: AppHandle<R>,
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
    op: RemoteOperationDto,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    let source = source.inner().clone();
    blocking(move || {
        let out = output(&app);
        syncing::synchronize(&source.adapter, &source.lock, &root, op.into(), &out)
    })
    .await
}

/// Merge into Current / Rebase onto, streamed.
#[tauri::command]
pub async fn git_integrate<R: Runtime>(
    app: AppHandle<R>,
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
    op: IntegrationDto,
    branch: String,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    let source = source.inner().clone();
    blocking(move || {
        let out = output(&app);
        let how = (op.into(), branch.as_str());
        syncing::integrate(&source.adapter, &source.lock, &root, how, &out)
    })
    .await
}

/// Continues (`go_on`, the UI's flag) or aborts the
/// merge/rebase/cherry-pick/revert in progress, streamed.
#[tauri::command]
pub async fn git_operation<R: Runtime>(
    app: AppHandle<R>,
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
    go_on: bool,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    let resolution = if go_on {
        Resolution::Continue
    } else {
        Resolution::Abort
    };
    let source = source.inner().clone();
    blocking(move || {
        let out = output(&app);
        syncing::resolve(&source.adapter, &source.lock, &root, resolution, &out)
    })
    .await
}

/// Clone a Repository into `parent` — a folder the user picked in the native
/// dialog, so it joins the workspace the way Open Folder's does. The new
/// working tree's path, to open.
#[tauri::command]
pub async fn git_clone<R: Runtime>(
    app: AppHandle<R>,
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    url: String,
    parent: String,
    name: String,
) -> Result<String, Message> {
    let parent = workspace.allow(Path::new(&parent))?;
    let source = source.inner().clone();
    blocking(move || {
        source
            .adapter
            .clone_repository(&url, &parent, &name, &output(&app))
    })
    .await
}

/// Create Git Repository in the project folder.
#[tauri::command]
pub async fn git_init<R: Runtime>(
    app: AppHandle<R>,
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
    branch: String,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    let source = source.inner().clone();
    blocking(move || source.adapter.init(&root, &branch, &output(&app))).await
}
