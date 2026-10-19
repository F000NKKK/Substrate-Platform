//! Opening links and files in the user's default app — through Substrate's
//! process helper, so what a packaged app starts gets the system's own
//! environment (Tauri's opener passes an AppImage's on, and they fail).

use std::path::Path;
use substrate_platform::message::Message;
use substrate_platform::process;
use substrate_platform::workspace::Workspace;
use tauri::State;

/// Opens a web or mail link.
#[tauri::command]
pub fn open_url(url: String) -> Result<(), Message> {
    process::open_link(&url)
}

/// Opens a file or folder of the workspace.
#[tauri::command]
pub fn open_path(ws: State<'_, Workspace>, path: String) -> Result<(), Message> {
    ws.check(Path::new(&path))?;
    process::open_path(Path::new(&path))
}

/// Shows a file or folder of the workspace in the file manager.
#[tauri::command]
pub fn reveal_path(ws: State<'_, Workspace>, path: String) -> Result<(), Message> {
    ws.check(Path::new(&path))?;
    process::reveal(Path::new(&path))
}
