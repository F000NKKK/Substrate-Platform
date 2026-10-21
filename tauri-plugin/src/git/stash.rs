//! Stashes and the global Git settings.

use super::{Res, checked};
use crate::blocking;
use substrate_platform::git::config::{self, GlobalConfig};
use substrate_platform::git::stash::{self, StashInfo, StashMode, StashOp};
use substrate_platform::workspace::Workspace;
use tauri::State;

#[tauri::command]
pub async fn git_stash_list(ws: State<'_, Workspace>, root: String) -> Res<Vec<StashInfo>> {
    let root = checked(&ws, &root)?;
    blocking(move || stash::list(&root)).await
}

#[tauri::command]
pub async fn git_stash_push(
    ws: State<'_, Workspace>,
    root: String,
    message: String,
    mode: StashMode,
) -> Res<()> {
    let root = checked(&ws, &root)?;
    blocking(move || stash::push(&root, &message, mode)).await
}

#[tauri::command]
pub async fn git_stash_op(
    ws: State<'_, Workspace>,
    root: String,
    index: u32,
    op: StashOp,
) -> Res<()> {
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
