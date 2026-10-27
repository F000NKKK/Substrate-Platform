//! Opening links and files in the user's default app — through Substrate's
//! process helper, so what a packaged app starts gets the system's own
//! environment (Tauri's opener passes an `AppImage`'s on, and they fail).
#![expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value: `State` and deserialized parameters cannot be borrowed"
)]

use std::path::Path;

use substrate_message::Message;
use substrate_process as process;
use substrate_workspace_app::Workspace;
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
