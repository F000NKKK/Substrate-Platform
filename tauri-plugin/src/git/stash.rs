//! Stashes and the global Git settings.
#![expect(
    clippy::unreachable,
    reason = "the expansion of `#[tauri::command]` contains `unreachable!`"
)]

use substrate_git_app::ports::{Settings, Stashes};
use substrate_git_app::stashing;
use substrate_git_domain::GitError;
use substrate_message::Message;
use substrate_workspace_app::Workspace;
use tauri::State;

use super::wire::settings::GlobalConfigDto;
use super::wire::stashes::{StashEntryDto, StashModeDto, StashOperationDto};
use super::{SourceControl, checked};
use crate::blocking;

#[tauri::command]
pub(crate) async fn git_stash_list(
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
) -> Result<Vec<StashEntryDto>, Message> {
    let root = checked(&workspace, &root)?;
    let source = source.inner().clone();
    blocking(move || {
        let stashes = Stashes::list(&source.adapter, &root)?;
        Ok::<_, GitError>(stashes.into_iter().map(StashEntryDto::from).collect())
    })
    .await
}

#[tauri::command]
pub(crate) async fn git_stash_push(
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
    message: String,
    mode: StashModeDto,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    let source = source.inner().clone();
    blocking(move || stashing::push(&source.adapter, &source.lock, &root, &message, mode.into()))
        .await
}

#[tauri::command]
pub(crate) async fn git_stash_op(
    workspace: State<'_, Workspace>,
    source: State<'_, SourceControl>,
    root: String,
    index: u32,
    op: StashOperationDto,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    let source = source.inner().clone();
    blocking(move || stashing::perform(&source.adapter, &source.lock, &root, index, op.into()))
        .await
}

#[tauri::command]
pub(crate) async fn git_config_get(
    source: State<'_, SourceControl>,
) -> Result<GlobalConfigDto, Message> {
    let source = source.inner().clone();
    blocking(move || Ok::<_, Message>(GlobalConfigDto::from(source.adapter.get()))).await
}

#[tauri::command]
pub(crate) async fn git_config_set(
    source: State<'_, SourceControl>,
    key: String,
    value: String,
) -> Result<(), Message> {
    let source = source.inner().clone();
    blocking(move || source.adapter.set(&key, &value)).await
}
