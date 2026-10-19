//! Terminals: shell sessions on real ptys, each addressed by an id the UI
//! chooses; their output and exit go to the UI as
//! [`events::TERMINAL_OUTPUT`] / [`events::TERMINAL_EXIT`]. Feature `terminal`.

use substrate_platform::sync::Locked;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};
use substrate_platform::events;
use substrate_platform::message::Message;
use substrate_platform::terminal::messages::{CANT_START, FAILED};
use substrate_platform::terminal::{PtyOptions, PtySession, ShellInfo, detect_shells};
use substrate_platform::workspace::Workspace;
use tauri::{AppHandle, Emitter, Runtime, State};

/// The open terminals, by id.
#[derive(Default)]
pub struct Terminals(Mutex<HashMap<String, PtySession>>);

impl Terminals {
    fn sessions(&self) -> MutexGuard<'_, HashMap<String, PtySession>> {
        self.0.locked()
    }
}

/// What to start: the terminal's id, its size, the shell (none: the
/// system's) and the folder (none: the open project's).
#[derive(Deserialize)]
pub struct Spawn {
    id: String,
    cols: u16,
    rows: u16,
    shell: Option<String>,
    cwd: Option<String>,
}

#[derive(Clone, Serialize)]
struct Output {
    id: String,
    data: String,
}

#[derive(Clone, Serialize)]
struct Exit {
    id: String,
}

fn failed(template: &'static str, error: impl ToString) -> Message {
    Message::new(template).arg("error", error.to_string())
}

/// The shells installed here, for "New Terminal".
#[tauri::command]
pub fn terminal_shells() -> Vec<ShellInfo> {
    detect_shells()
}

#[tauri::command]
pub fn terminal_spawn<R: Runtime>(app: AppHandle<R>, terminals: State<'_, Terminals>, ws: State<'_, Workspace>, spec: Spawn) -> Result<(), Message> {
    // "Open in Terminal" on a folder of the workspace; otherwise the open project.
    let cwd = match spec.cwd {
        Some(dir) => {
            let dir = PathBuf::from(dir);
            ws.check(&dir)?;
            Some(dir)
        }
        None => ws.current(),
    };
    let (out_app, out_id) = (app.clone(), spec.id.clone());
    let (exit_app, exit_id) = (app, spec.id.clone());
    let session = PtySession::spawn_utf8(
        PtyOptions { shell: spec.shell, cwd, ..PtyOptions::default() },
        spec.cols,
        spec.rows,
        move |data| {
            let _ = out_app.emit(events::TERMINAL_OUTPUT, Output { id: out_id.clone(), data });
        },
        move || {
            let _ = exit_app.emit(events::TERMINAL_EXIT, Exit { id: exit_id.clone() });
        },
    )
    .map_err(|e| failed(CANT_START, e))?;
    terminals.sessions().insert(spec.id, session);
    Ok(())
}

#[tauri::command]
pub fn terminal_write(terminals: State<'_, Terminals>, id: String, data: String) -> Result<(), Message> {
    match terminals.sessions().get_mut(&id) {
        Some(session) => session.write(data.as_bytes()).map_err(|e| failed(FAILED, e)),
        None => Ok(()),
    }
}

#[tauri::command]
pub fn terminal_resize(terminals: State<'_, Terminals>, id: String, cols: u16, rows: u16) -> Result<(), Message> {
    match terminals.sessions().get(&id) {
        Some(session) => session.resize(cols, rows).map_err(|e| failed(FAILED, e)),
        None => Ok(()),
    }
}

#[tauri::command]
pub fn terminal_kill(terminals: State<'_, Terminals>, id: String) {
    if let Some(mut session) = terminals.sessions().remove(&id) {
        let _ = session.kill();
    }
}
