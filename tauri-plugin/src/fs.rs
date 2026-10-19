//! Folders and files as an IDE browses and edits them. Writes stay inside the
//! [`Workspace`]; reads may go anywhere (a debugger frame in a dependency, a
//! file the user picked). Changes made outside the IDE come back as
//! [`events::FILES_CHANGED`]. Feature `fs`.

use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use substrate_platform::events;
use substrate_platform::fs::messages::{ALREADY_EXISTS, FAILED, INTO_ITSELF, NOT_A_FILE};
use substrate_platform::fs::watch::DEFAULT_QUIET;
use substrate_platform::fs::{DirEntryInfo, DirWatcher, EntryKind, dir};
use substrate_platform::message::Message;
use substrate_platform::workspace::Workspace;
use tauri::{AppHandle, Emitter, Runtime, State};

type Res<T> = Result<T, Message>;

/// The watcher behind `fs_watch`, made on first use.
#[derive(Default)]
pub struct Watcher(OnceLock<DirWatcher>);

/// A folder's entry, as the UI's file tree takes it.
#[derive(Serialize)]
pub struct Entry {
    name: String,
    path: String,
    kind: EntryKind,
}

impl From<DirEntryInfo> for Entry {
    fn from(e: DirEntryInfo) -> Self {
        Entry { name: e.name, path: e.path.to_string_lossy().into_owned(), kind: e.kind }
    }
}

fn failed(path: &Path, error: impl ToString) -> Message {
    Message::new(FAILED).arg("path", path.display()).arg("error", error.to_string())
}

/// `path`, once it's known to be in the workspace.
fn inside(ws: &Workspace, path: &str) -> Res<PathBuf> {
    let path = PathBuf::from(path);
    ws.check(&path)?;
    Ok(path)
}

#[tauri::command]
pub fn dir_list(ws: State<'_, Workspace>, path: String) -> Res<Vec<Entry>> {
    let path = inside(&ws, &path)?;
    dir::list_dir(&path).map(|all| all.into_iter().map(Entry::from).collect()).map_err(|e| failed(&path, e))
}

#[tauri::command]
pub fn dir_create_file(ws: State<'_, Workspace>, path: String) -> Res<()> {
    let path = inside(&ws, &path)?;
    dir::create_file(&path).map_err(|e| failed(&path, e))
}

#[tauri::command]
pub fn dir_create_dir(ws: State<'_, Workspace>, path: String) -> Res<()> {
    let path = inside(&ws, &path)?;
    dir::create_dir(&path).map_err(|e| failed(&path, e))
}

#[tauri::command]
pub fn dir_rename(ws: State<'_, Workspace>, from: String, to: String) -> Res<()> {
    let (from, to) = (inside(&ws, &from)?, inside(&ws, &to)?);
    dir::rename(&from, &to).map_err(|e| failed(&from, e))
}

#[tauri::command]
pub fn dir_remove(ws: State<'_, Workspace>, path: String) -> Res<()> {
    let path = inside(&ws, &path)?;
    dir::remove(&path).map_err(|e| failed(&path, e))
}

/// Copies a file or a folder to `to`, which mustn't exist: the source may be
/// anywhere, the destination must be in the workspace.
#[tauri::command]
pub fn dir_copy(ws: State<'_, Workspace>, from: String, to: String) -> Res<()> {
    let from = PathBuf::from(from);
    let to = inside(&ws, &to)?;
    if to.exists() {
        return Err(Message::new(ALREADY_EXISTS).arg("path", to.display()));
    }
    if to.starts_with(&from) {
        return Err(Message::new(INTO_ITSELF));
    }
    dir::copy(&from, &to).map_err(|e| failed(&to, e))
}

#[tauri::command]
pub fn file_read(path: String) -> Res<String> {
    let path = PathBuf::from(path);
    if !path.is_file() {
        return Err(Message::new(NOT_A_FILE).arg("path", path.display()));
    }
    dir::read_file(&path).map_err(|e| failed(&path, e))
}

#[tauri::command]
pub fn file_write(ws: State<'_, Workspace>, path: String, contents: String) -> Res<()> {
    let path = inside(&ws, &path)?;
    dir::write_file(&path, &contents).map_err(|e| failed(&path, e))
}

/// A folder or file the user picked (a native dialog) joins the workspace.
#[tauri::command]
pub fn allow_path(ws: State<'_, Workspace>, path: String) -> Res<()> {
    ws.allow(Path::new(&path)).map(|_| ())
}

/// Watches exactly `folders` for changes made outside the IDE; each burst
/// comes back as one [`events::FILES_CHANGED`], spelled like `folders`.
#[tauri::command]
pub fn fs_watch<R: Runtime>(app: AppHandle<R>, watcher: State<'_, Watcher>, folders: Vec<String>) -> Res<()> {
    if watcher.0.get().is_none() {
        let made = DirWatcher::new(DEFAULT_QUIET, move |paths| {
            let paths: Vec<String> = paths.iter().map(|p| p.to_string_lossy().into_owned()).collect();
            let _ = app.emit(events::FILES_CHANGED, paths);
        })
        .map_err(Message::verbatim)?;
        let _ = watcher.0.set(made);
    }
    if let Some(w) = watcher.0.get() {
        w.set(folders.into_iter().map(PathBuf::from));
    }
    Ok(())
}
