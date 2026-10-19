//! What the IDE remembers: a project's state in the product's private folder
//! of it ([`StateOptions::project_dir`]), the app's in its config folder.
//! The UI owns the keys of both. Feature `state`.

use serde_json::Value;
use std::path::{Path, PathBuf};
use substrate_platform::message::Message;
use substrate_platform::state::{APP_STATE_FILE, WORKSPACE_FILE, private_dir, read_json, write_json};
use substrate_platform::workspace::Workspace;
use tauri::{AppHandle, Manager, Runtime, State};

type Res<T> = Result<T, Message>;

/// Where a product keeps its own data in a project.
#[derive(Clone, Debug)]
pub struct StateOptions {
    /// The product's private folder of a project (`.yog-idle`, like Visual Studio's `.vs`).
    pub project_dir: String,
}

/// The product's private folder of `project_root`, once that's in the workspace.
fn project_dir(ws: &Workspace, options: &StateOptions, project_root: &str) -> Res<PathBuf> {
    let root = Path::new(project_root);
    ws.check(root)?;
    Ok(root.join(&options.project_dir))
}

fn app_state_file<R: Runtime>(app: &AppHandle<R>) -> Res<PathBuf> {
    app.path().app_config_dir().map(|dir| dir.join(APP_STATE_FILE)).map_err(Message::verbatim)
}

/// The project's remembered state (open files, layout, expanded folders…).
#[tauri::command]
pub fn workspace_load(ws: State<'_, Workspace>, options: State<'_, StateOptions>, project_root: String) -> Res<Value> {
    Ok(read_json(&project_dir(&ws, &options, &project_root)?.join(WORKSPACE_FILE)))
}

#[tauri::command]
pub fn workspace_save(ws: State<'_, Workspace>, options: State<'_, StateOptions>, project_root: String, value: Value) -> Res<()> {
    let dir = project_dir(&ws, &options, &project_root)?;
    private_dir(&dir)?;
    write_json(&dir.join(WORKSPACE_FILE), &value)
}

/// What isn't tied to a project (recent projects, the last one opened, the
/// layout with none open).
#[tauri::command]
pub fn app_state_load<R: Runtime>(app: AppHandle<R>) -> Res<Value> {
    Ok(read_json(&app_state_file(&app)?))
}

#[tauri::command]
pub fn app_state_save<R: Runtime>(app: AppHandle<R>, value: Value) -> Res<()> {
    write_json(&app_state_file(&app)?, &value)
}
