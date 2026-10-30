//! Folders and files, as commands.
#![expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value: `State` and deserialized parameters cannot be borrowed"
)]

use std::path::{Path, PathBuf};

use substrate_host_contract::events;
use substrate_message::Message;
use substrate_workspace_app::Workspace;
use substrate_workspace_fs::{DEFAULT_QUIET, DirectoryWatcher, FsError, copy_new, dir, read_text};
use tauri::{AppHandle, Emitter, Manager, Runtime, State};

use super::entry::Entry;
use super::watcher::Watcher;

/// `path`, once it's known to be in the workspace.
fn inside(workspace: &Workspace, path: &str) -> Result<PathBuf, Message> {
    let path = PathBuf::from(path);
    workspace.check(&path)?;
    Ok(path)
}

#[tauri::command]
pub(crate) fn dir_list(
    workspace: State<'_, Workspace>,
    path: String,
) -> Result<Vec<Entry>, Message> {
    let path = inside(&workspace, &path)?;
    let all = dir::list(&path).map_err(|e| FsError::io(&path, e))?;
    Ok(all.into_iter().map(Entry::from).collect())
}

#[tauri::command]
pub(crate) fn dir_create_file(
    workspace: State<'_, Workspace>,
    path: String,
) -> Result<(), Message> {
    let path = inside(&workspace, &path)?;
    Ok(dir::create_file(&path).map_err(|e| FsError::io(&path, e))?)
}

#[tauri::command]
pub(crate) fn dir_create_dir(workspace: State<'_, Workspace>, path: String) -> Result<(), Message> {
    let path = inside(&workspace, &path)?;
    Ok(dir::create(&path).map_err(|e| FsError::io(&path, e))?)
}

#[tauri::command]
pub(crate) fn dir_rename(
    workspace: State<'_, Workspace>,
    from: String,
    to: String,
) -> Result<(), Message> {
    let (from, to) = (inside(&workspace, &from)?, inside(&workspace, &to)?);
    Ok(dir::rename(&from, &to).map_err(|e| FsError::io(&from, e))?)
}

#[tauri::command]
pub(crate) fn dir_remove(workspace: State<'_, Workspace>, path: String) -> Result<(), Message> {
    let path = inside(&workspace, &path)?;
    Ok(dir::remove(&path).map_err(|e| FsError::io(&path, e))?)
}

/// Copies a file or a folder to `to`, which mustn't exist: the source may be
/// anywhere, the destination must be in the workspace.
#[tauri::command]
pub(crate) fn dir_copy(
    workspace: State<'_, Workspace>,
    from: String,
    to: String,
) -> Result<(), Message> {
    let to = inside(&workspace, &to)?;
    Ok(copy_new(Path::new(&from), &to)?)
}

#[tauri::command]
pub(crate) fn file_read(path: String) -> Result<String, Message> {
    Ok(read_text(Path::new(&path))?)
}

#[tauri::command]
pub(crate) fn file_write(
    workspace: State<'_, Workspace>,
    path: String,
    contents: String,
) -> Result<(), Message> {
    let path = inside(&workspace, &path)?;
    Ok(dir::write_file(&path, &contents).map_err(|e| FsError::io(&path, e))?)
}

/// A folder or file the user picked (a native dialog) joins the workspace.
#[tauri::command]
pub(crate) async fn allow_path<R: Runtime>(app: AppHandle<R>, path: String) -> Result<(), Message> {
    crate::blocking::blocking(move || {
        let path = app.state::<Workspace>().allow(Path::new(&path))?;
        let scope = app.asset_protocol_scope();
        if path.is_dir() {
            scope.allow_directory(&path, true)
        } else {
            scope.allow_file(&path)
        }
        .map_err(Message::verbatim)?;
        Ok::<_, Message>(())
    })
    .await
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use substrate_workspace_fs::DiskPaths;

    use super::*;

    #[test]
    fn a_picked_image_is_allowed_without_admitting_its_siblings() {
        let directory = tempfile::tempdir().unwrap();
        let image = directory.path().join("старое превью.png");
        let sibling = directory.path().join("private.png");
        std::fs::write(&image, b"existing image bytes").unwrap();
        std::fs::write(&sibling, b"unpicked image bytes").unwrap();
        let app = tauri::test::mock_app();
        app.manage(Workspace::new(Arc::new(DiskPaths)));
        assert!(!app.asset_protocol_scope().is_allowed(&image));
        tauri::async_runtime::block_on(allow_path(
            app.handle().clone(),
            image.to_string_lossy().into_owned(),
        ))
        .unwrap();
        assert!(app.asset_protocol_scope().is_allowed(&image));
        assert!(!app.asset_protocol_scope().is_allowed(&sibling));
        assert!(app.state::<Workspace>().check(&image).is_ok());
        assert!(app.state::<Workspace>().check(&sibling).is_err());
    }
}

/// Watches exactly `folders` for changes made outside the IDE; each burst
/// comes back as one [`events::FILES_CHANGED`], spelled like `folders`.
#[tauri::command]
pub(crate) fn fs_watch<R: Runtime>(
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
