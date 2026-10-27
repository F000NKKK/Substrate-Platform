//! Opening links and files in the user's default app — through Substrate's
//! process helper, so what a packaged app starts gets the system's own
//! environment (Tauri's opener passes an AppImage's on, and they fail).

use std::path::Path;

use substrate_message::Message;
use substrate_platform::workspace::Workspace;
use substrate_process as process;
use tauri::State;

/// Opens a web or mail link.
#[tauri::command]
pub(crate) fn open_url(url: String) -> Result<(), Message> {
    Ok(process::open_link(&url)?)
}

/// Opens a file or folder of the workspace.
#[tauri::command]
pub(crate) fn open_path(workspace: State<'_, Workspace>, path: String) -> Result<(), Message> {
    workspace.check(Path::new(&path))?;
    Ok(process::open_path(Path::new(&path))?)
}

/// Shows a file or folder of the workspace in the file manager.
#[tauri::command]
pub(crate) fn reveal_path(workspace: State<'_, Workspace>, path: String) -> Result<(), Message> {
    workspace.check(Path::new(&path))?;
    Ok(process::reveal(Path::new(&path))?)
}
