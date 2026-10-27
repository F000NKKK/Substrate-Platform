//! Stashes and the global Git settings.

use substrate_platform::git::WriteLock;
use substrate_platform::git::config::{self, GlobalConfig};
use substrate_platform::git::stash::{self, StashEntry, StashMode, StashOperation};
use substrate_platform::message::Message;
use substrate_platform::workspace::Workspace;
use tauri::State;

use super::checked;
use crate::blocking;

#[tauri::command]
pub(crate) async fn git_stash_list(
    workspace: State<'_, Workspace>,
    root: String,
) -> Result<Vec<StashEntry>, Message> {
    let root = checked(&workspace, &root)?;
    blocking(move || stash::list(&root)).await
}

#[tauri::command]
pub(crate) async fn git_stash_push(
    workspace: State<'_, Workspace>,
    lock: State<'_, WriteLock>,
    root: String,
    message: String,
    mode: StashMode,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    let lock = lock.inner().clone();
    blocking(move || stash::push(&lock, &root, &message, mode)).await
}

#[tauri::command]
pub(crate) async fn git_stash_op(
    workspace: State<'_, Workspace>,
    lock: State<'_, WriteLock>,
    root: String,
    index: u32,
    op: StashOperation,
) -> Result<(), Message> {
    let root = checked(&workspace, &root)?;
    let lock = lock.inner().clone();
    blocking(move || stash::perform(&lock, &root, index, op)).await
}

#[tauri::command]
pub(crate) async fn git_config_get() -> Result<GlobalConfig, Message> {
    blocking(|| Ok::<_, Message>(config::get())).await
}

#[tauri::command]
pub(crate) async fn git_config_set(key: String, value: String) -> Result<(), Message> {
    blocking(move || config::set(&key, &value)).await
}
