//! Folders and files as an IDE browses and edits them. Writes stay inside the
//! [`Workspace`]; reads may go anywhere (a debugger frame in a dependency, a
//! file the user picked). Changes made outside the IDE come back as
//! [`events::FILES_CHANGED`]. Feature `fs`.

use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use substrate_host_contract::events;
use substrate_message::Message;
use substrate_platform::fs::watch::DEFAULT_QUIET;
use substrate_platform::fs::{
    DirectoryEntry, DirectoryWatcher, EntryKind, FsError, copy_new, dir, read_text,
};
use substrate_platform::workspace::Workspace;
use tauri::{AppHandle, Emitter, Runtime, State};

/// The watcher behind `fs_watch`, made on first use.
#[derive(Default)]
pub struct Watcher(OnceLock<DirectoryWatcher>);

/// A folder's entry, as the UI's file tree takes it.
#[derive(Serialize)]
pub struct Entry {
    name: String,
    path: String,
    kind: EntryKind,
}

impl From<DirectoryEntry> for Entry {
    fn from(e: DirectoryEntry) -> Self {
        Entry {
            name: e.name,
            path: e.path.to_string_lossy().into_owned(),
            kind: e.kind,
        }
    }
}

/// `path`, once it's known to be in the workspace.
fn inside(workspace: &Workspace, path: &str) -> Result<PathBuf, Message> {
    let path = PathBuf::from(path);
    workspace.check(&path)?;
    Ok(path)
}

#[tauri::command]
pub fn dir_list(workspace: State<'_, Workspace>, path: String) -> Result<Vec<Entry>, Message> {
    let path = inside(&workspace, &path)?;
    let all = dir::list(&path).map_err(|e| FsError::io(&path, e))?;
    Ok(all.into_iter().map(Entry::from).collect())
}

#[tauri::command]
pub fn dir_create_file(workspace: State<'_, Workspace>, path: String) -> Result<(), Message> {
    let path = inside(&workspace, &path)?;
    Ok(dir::create_file(&path).map_err(|e| FsError::io(&path, e))?)
}

#[tauri::command]
pub fn dir_create_dir(workspace: State<'_, Workspace>, path: String) -> Result<(), Message> {
    let path = inside(&workspace, &path)?;
    Ok(dir::create(&path).map_err(|e| FsError::io(&path, e))?)
}

#[tauri::command]
pub fn dir_rename(
    workspace: State<'_, Workspace>,
    from: String,
    to: String,
) -> Result<(), Message> {
    let (from, to) = (inside(&workspace, &from)?, inside(&workspace, &to)?);
    Ok(dir::rename(&from, &to).map_err(|e| FsError::io(&from, e))?)
}

#[tauri::command]
pub fn dir_remove(workspace: State<'_, Workspace>, path: String) -> Result<(), Message> {
    let path = inside(&workspace, &path)?;
    Ok(dir::remove(&path).map_err(|e| FsError::io(&path, e))?)
}

/// Copies a file or a folder to `to`, which mustn't exist: the source may be
/// anywhere, the destination must be in the workspace.
#[tauri::command]
pub fn dir_copy(workspace: State<'_, Workspace>, from: String, to: String) -> Result<(), Message> {
    let to = inside(&workspace, &to)?;
    Ok(copy_new(Path::new(&from), &to)?)
}

#[tauri::command]
pub fn file_read(path: String) -> Result<String, Message> {
    Ok(read_text(Path::new(&path))?)
}

#[tauri::command]
pub fn file_write(
    workspace: State<'_, Workspace>,
    path: String,
    contents: String,
) -> Result<(), Message> {
    let path = inside(&workspace, &path)?;
    Ok(dir::write_file(&path, &contents).map_err(|e| FsError::io(&path, e))?)
}

/// A folder or file the user picked (a native dialog) joins the workspace.
#[tauri::command]
pub fn allow_path(workspace: State<'_, Workspace>, path: String) -> Result<(), Message> {
    workspace.allow(Path::new(&path))?;
    Ok(())
}

/// Watches exactly `folders` for changes made outside the IDE; each burst
/// comes back as one [`events::FILES_CHANGED`], spelled like `folders`.
#[tauri::command]
pub fn fs_watch<R: Runtime>(
    app: AppHandle<R>,
    watcher: State<'_, Watcher>,
    folders: Vec<String>,
) -> Result<(), Message> {
    if watcher.0.get().is_none() {
        let made = DirectoryWatcher::new(DEFAULT_QUIET, move |paths| {
            let paths: Vec<String> = paths
                .iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect();
            app.emit(events::FILES_CHANGED, paths).ok();
        })?;
        watcher.0.set(made).ok();
    }
    if let Some(w) = watcher.0.get() {
        w.set(folders.into_iter().map(PathBuf::from));
    }
    Ok(())
}
