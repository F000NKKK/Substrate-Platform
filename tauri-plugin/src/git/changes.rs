//! Git Changes: the status, staging, committing and the diffs.

use std::path::Path;

use substrate_git_app::changes;
use substrate_git_app::ports::{Diffs, History, WorkingTree};
use substrate_git_domain::commits::{CommitMode, CommitScope};
use substrate_git_domain::status::DiscardItem;
use substrate_message::Message;
use substrate_platform::workspace::Workspace;
use tauri::State;

use super::wire::commits::CommitResultDto;
use super::wire::diffs::{DiffSideDto, DiffTextsDto};
use super::wire::status::{DiscardItemDto, RepoStatusDto};
use super::{SourceControl, checked};
use crate::blocking;

/// The repository's state for Git Changes and the status bar — `None` when
/// `root` isn't in a repository.
#[tauri::command]
pub(crate) async fn git_status(
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
) -> Result<Option<RepoStatusDto>, Message> {
    let root = checked(&workspace, &root)?;
    let source = source.inner().clone();
    blocking(move || {
        let status = source.adapter.status(&root)?;
        Ok::<_, substrate_git_domain::GitError>(status.map(RepoStatusDto::from))
    })
    .await
}

#[tauri::command]
pub(crate) async fn git_stage(
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
    paths: Vec<String>,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    let source = source.inner().clone();
    blocking(move || changes::stage(&source.adapter, &source.lock, &root, &paths)).await
}

#[tauri::command]
pub(crate) async fn git_unstage(
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
    paths: Vec<String>,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    let source = source.inner().clone();
    blocking(move || changes::unstage(&source.adapter, &source.lock, &root, &paths)).await
}

#[tauri::command]
pub(crate) async fn git_discard(
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
    items: Vec<DiscardItemDto>,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    let source = source.inner().clone();
    let items: Vec<DiscardItem> = items.into_iter().map(DiscardItem::from).collect();
    blocking(move || changes::discard(&source.adapter, &source.lock, &root, &items)).await
}

/// Commit (`stage_all`: Commit All), or amend the last commit (`amend`).
/// The UI sends both as flags.
#[tauri::command]
#[expect(
    clippy::too_many_arguments,
    reason = "Tauri injects state as separate parameters; the UI sends the rest"
)]
pub(crate) async fn git_commit(
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
    message: String,
    amend: bool,
    stage_all: bool,
) -> Result<CommitResultDto, Message> {
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
    let source = source.inner().clone();
    blocking(move || {
        let made = changes::commit(
            &source.adapter,
            &source.lock,
            &root,
            &message,
            (mode, scope),
        )?;
        Ok::<_, substrate_git_domain::GitError>(CommitResultDto::from(made))
    })
    .await
}

#[tauri::command]
pub(crate) async fn git_last_message(
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
) -> Result<String, Message> {
    let root = checked(&workspace, &root)?;
    let source = source.inner().clone();
    blocking(move || source.adapter.last_message(&root)).await
}

#[tauri::command]
pub(crate) async fn git_diff_texts(
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
    left: DiffSideDto,
    right: DiffSideDto,
) -> Result<DiffTextsDto, Message> {
    let root = checked(&workspace, &root)?;
    let source = source.inner().clone();
    blocking(move || {
        let texts = source.adapter.texts(&root, &left.into(), &right.into())?;
        Ok::<_, substrate_git_domain::GitError>(DiffTextsDto::from(texts))
    })
    .await
}

#[tauri::command]
pub(crate) async fn git_head_text(
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
    path: String,
) -> Result<Option<String>, Message> {
    let root = checked(&workspace, &root)?;
    let source = source.inner().clone();
    blocking(move || source.adapter.head_text(&root, Path::new(&path))).await
}
