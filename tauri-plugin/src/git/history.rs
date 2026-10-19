//! The history and the branches.

use super::{Res, checked};
use crate::blocking;
use substrate_platform::workspace::Workspace;
use tauri::State;
use substrate_platform::git::branch::{self, BranchInfo};
use substrate_platform::git::history::{self, CommitInfo};
use substrate_platform::git::status::FileChange;

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
