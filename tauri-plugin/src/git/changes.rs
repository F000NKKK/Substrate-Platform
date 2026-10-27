//! Git Changes: the status, staging, committing and the diffs.

use super::checked;
use crate::blocking;
use std::path::Path;
use substrate_platform::git::diff::{self, DiffSide, DiffTexts};
use substrate_platform::git::status::{
    self, CommitMode, CommitResult, CommitScope, DiscardItem, RepoStatus,
};
use substrate_platform::message::Message;
use substrate_platform::workspace::Workspace;
use tauri::State;

/// The repository's state for Git Changes and the status bar — `None` when
/// `root` isn't in a repository.
#[tauri::command]
pub async fn git_status(
    workspace: State<'_, Workspace>,
    root: String,
) -> Result<Option<RepoStatus>, Message> {
    let root = checked(&workspace, &root)?;
    blocking(move || status::read(&root)).await
}

#[tauri::command]
pub async fn git_stage(
    workspace: State<'_, Workspace>,
    root: String,
    paths: Vec<String>,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    blocking(move || status::stage(&root, &paths)).await
}

#[tauri::command]
pub async fn git_unstage(
    workspace: State<'_, Workspace>,
    root: String,
    paths: Vec<String>,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    blocking(move || status::unstage(&root, &paths)).await
}

#[tauri::command]
pub async fn git_discard(
    workspace: State<'_, Workspace>,
    root: String,
    items: Vec<DiscardItem>,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    blocking(move || status::discard(&root, items)).await
}

/// Commit (`stage_all`: Commit All), or amend the last commit (`amend`).
/// The UI sends both as flags.
#[tauri::command]
pub async fn git_commit(
    workspace: State<'_, Workspace>,
    root: String,
    message: String,
    amend: bool,
    stage_all: bool,
) -> Result<CommitResult, Message> {
    let root = checked(&workspace, &root)?;
    let mode = if amend {
        CommitMode::Amend
    } else {
        CommitMode::New
    };
    let scope = if stage_all {
        CommitScope::All
    } else {
        CommitScope::Staged
    };
    blocking(move || status::commit(&root, &message, mode, scope)).await
}

#[tauri::command]
pub async fn git_last_message(
    workspace: State<'_, Workspace>,
    root: String,
) -> Result<String, Message> {
    let root = checked(&workspace, &root)?;
    blocking(move || status::last_message(&root)).await
}

#[tauri::command]
pub async fn git_diff_texts(
    workspace: State<'_, Workspace>,
    root: String,
    left: DiffSide,
    right: DiffSide,
) -> Result<DiffTexts, Message> {
    let root = checked(&workspace, &root)?;
    blocking(move || diff::texts(&root, &left, &right)).await
}

#[tauri::command]
pub async fn git_head_text(
    workspace: State<'_, Workspace>,
    root: String,
    path: String,
) -> Result<Option<String>, Message> {
    let root = checked(&workspace, &root)?;
    blocking(move || diff::head_text(&root, Path::new(&path))).await
}
