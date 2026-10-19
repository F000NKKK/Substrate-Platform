//! Source Control's commands: each checks its folder against the
//! [`Workspace`] and runs [`substrate_platform::git`] off the UI thread; long
//! runs stream to the UI as [`events::GIT_OUTPUT`].

use crate::blocking;
use std::path::{Path, PathBuf};
use substrate_platform::events;
use substrate_platform::git::branch::{self, BranchInfo};
use substrate_platform::git::config::{self, GlobalConfig};
use substrate_platform::git::diff::{self, DiffSide, DiffTexts};
use substrate_platform::git::history::{self, CommitInfo};
use substrate_platform::git::remote::{self, BranchOp, RemoteOp};
use substrate_platform::git::stash::{self, StashInfo, StashMode, StashOp};
use substrate_platform::git::status::{self, CommitResult, DiscardItem, FileChange, RepoStatus};
use substrate_platform::message::Message;
use substrate_platform::workspace::Workspace;
use tauri::{AppHandle, Emitter, Runtime, State};

type Res<T> = Result<T, Message>;

/// `root`, once it's known to be in the workspace.
fn checked(ws: &Workspace, root: &str) -> Res<PathBuf> {
    ws.check(Path::new(root))?;
    Ok(PathBuf::from(root))
}

/// A long run's lines, to the UI.
fn output<R: Runtime>(app: &AppHandle<R>) -> impl Fn(Message) + Sync + '_ {
    move |line| {
        let _ = app.emit(events::GIT_OUTPUT, line);
    }
}

/// The repository's state for Git Changes and the status bar — `None` when
/// `root` isn't in a repository.
#[tauri::command]
pub async fn git_status(ws: State<'_, Workspace>, root: String) -> Res<Option<RepoStatus>> {
    let root = checked(&ws, &root)?;
    blocking(move || status::status(&root)).await
}

#[tauri::command]
pub async fn git_stage(ws: State<'_, Workspace>, root: String, paths: Vec<String>) -> Res<()> {
    let root = checked(&ws, &root)?;
    blocking(move || status::stage(&root, &paths)).await
}

#[tauri::command]
pub async fn git_unstage(ws: State<'_, Workspace>, root: String, paths: Vec<String>) -> Res<()> {
    let root = checked(&ws, &root)?;
    blocking(move || status::unstage(&root, &paths)).await
}

#[tauri::command]
pub async fn git_discard(ws: State<'_, Workspace>, root: String, items: Vec<DiscardItem>) -> Res<()> {
    let root = checked(&ws, &root)?;
    blocking(move || status::discard(&root, items)).await
}

#[tauri::command]
pub async fn git_commit(ws: State<'_, Workspace>, root: String, message: String, amend: bool, stage_all: bool) -> Res<CommitResult> {
    let root = checked(&ws, &root)?;
    blocking(move || status::commit(&root, &message, amend, stage_all)).await
}

#[tauri::command]
pub async fn git_last_message(ws: State<'_, Workspace>, root: String) -> Res<String> {
    let root = checked(&ws, &root)?;
    blocking(move || status::last_message(&root)).await
}

#[tauri::command]
pub async fn git_diff_texts(ws: State<'_, Workspace>, root: String, left: DiffSide, right: DiffSide) -> Res<DiffTexts> {
    let root = checked(&ws, &root)?;
    blocking(move || diff::texts(&root, &left, &right)).await
}

#[tauri::command]
pub async fn git_head_text(ws: State<'_, Workspace>, root: String, path: String) -> Res<Option<String>> {
    let root = checked(&ws, &root)?;
    blocking(move || diff::head_text(&root, Path::new(&path))).await
}

#[tauri::command]
pub async fn git_log(ws: State<'_, Workspace>, root: String, rev: String, skip: u32, max: u32) -> Res<Vec<CommitInfo>> {
    let root = checked(&ws, &root)?;
    blocking(move || history::log(&root, &rev, skip, max)).await
}

#[tauri::command]
pub async fn git_commit_files(ws: State<'_, Workspace>, root: String, hash: String) -> Res<Vec<FileChange>> {
    let root = checked(&ws, &root)?;
    blocking(move || history::commit_files(&root, &hash)).await
}

#[tauri::command]
pub async fn git_branches(ws: State<'_, Workspace>, root: String) -> Res<Vec<BranchInfo>> {
    let root = checked(&ws, &root)?;
    blocking(move || branch::list(&root)).await
}

#[tauri::command]
pub async fn git_switch(ws: State<'_, Workspace>, root: String, name: String, remote: bool) -> Res<()> {
    let root = checked(&ws, &root)?;
    blocking(move || branch::switch(&root, &name, remote)).await
}

#[tauri::command]
pub async fn git_branch_create(ws: State<'_, Workspace>, root: String, name: String, start: String, checkout: bool) -> Res<()> {
    let root = checked(&ws, &root)?;
    blocking(move || branch::create(&root, &name, &start, checkout)).await
}

#[tauri::command]
pub async fn git_branch_delete(ws: State<'_, Workspace>, root: String, name: String, force: bool) -> Res<()> {
    let root = checked(&ws, &root)?;
    blocking(move || branch::delete(&root, &name, force)).await
}

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

#[tauri::command]
pub async fn git_stash_list(ws: State<'_, Workspace>, root: String) -> Res<Vec<StashInfo>> {
    let root = checked(&ws, &root)?;
    blocking(move || stash::list(&root)).await
}

#[tauri::command]
pub async fn git_stash_push(ws: State<'_, Workspace>, root: String, message: String, mode: StashMode) -> Res<()> {
    let root = checked(&ws, &root)?;
    blocking(move || stash::push(&root, &message, mode)).await
}

#[tauri::command]
pub async fn git_stash_op(ws: State<'_, Workspace>, root: String, index: u32, op: StashOp) -> Res<()> {
    let root = checked(&ws, &root)?;
    blocking(move || stash::op(&root, index, op)).await
}

#[tauri::command]
pub async fn git_config_get() -> Res<GlobalConfig> {
    blocking(|| Ok(config::get())).await
}

#[tauri::command]
pub async fn git_config_set(key: String, value: String) -> Res<()> {
    blocking(move || config::set(&key, &value)).await
}
