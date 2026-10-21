//! Git Changes: the status, staging, committing and the diffs.

use super::{Res, checked};
use crate::blocking;
use std::path::Path;
use substrate_platform::git::diff::{self, DiffSide, DiffTexts};
use substrate_platform::git::status::{self, CommitResult, DiscardItem, RepoStatus};
use substrate_platform::workspace::Workspace;
use tauri::State;

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
pub async fn git_discard(
    ws: State<'_, Workspace>,
    root: String,
    items: Vec<DiscardItem>,
) -> Res<()> {
    let root = checked(&ws, &root)?;
    blocking(move || status::discard(&root, items)).await
}

#[tauri::command]
pub async fn git_commit(
    ws: State<'_, Workspace>,
    root: String,
    message: String,
    amend: bool,
    stage_all: bool,
) -> Res<CommitResult> {
    let root = checked(&ws, &root)?;
    blocking(move || status::commit(&root, &message, amend, stage_all)).await
}

#[tauri::command]
pub async fn git_last_message(ws: State<'_, Workspace>, root: String) -> Res<String> {
    let root = checked(&ws, &root)?;
    blocking(move || status::last_message(&root)).await
}

#[tauri::command]
pub async fn git_diff_texts(
    ws: State<'_, Workspace>,
    root: String,
    left: DiffSide,
    right: DiffSide,
) -> Res<DiffTexts> {
    let root = checked(&ws, &root)?;
    blocking(move || diff::texts(&root, &left, &right)).await
}

#[tauri::command]
pub async fn git_head_text(
    ws: State<'_, Workspace>,
    root: String,
    path: String,
) -> Res<Option<String>> {
    let root = checked(&ws, &root)?;
    blocking(move || diff::head_text(&root, Path::new(&path))).await
}
