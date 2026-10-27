//! Terminals, as commands.

use std::path::PathBuf;

use substrate_host_contract::events;
use substrate_message::Message;
use substrate_platform::terminal::messages::{CANT_START, FAILED};
use substrate_platform::terminal::{PtyOptions, PtySession, Shell, TerminalSize, detect_shells};
use substrate_platform::workspace::Workspace;
use tauri::{AppHandle, Emitter, Runtime, State};

use super::exit::Exit;
use super::failure::failed;
use super::output::Output;
use super::{SpawnRequest, Terminals};

/// The shells installed here, for "New Terminal".
#[tauri::command]
pub fn terminal_shells() -> Vec<Shell> {
    detect_shells()
}

#[tauri::command]
pub fn terminal_spawn<R: Runtime>(
    app: AppHandle<R>,
    terminals: State<'_, Terminals>,
    workspace: State<'_, Workspace>,
    spec: SpawnRequest,
) -> Result<(), Message> {
    // "Open in Terminal" on a folder of the workspace; otherwise the open project.
    let cwd = match spec.cwd {
        Some(dir) => {
            let dir = PathBuf::from(dir);
            workspace.check(&dir)?;
            Some(dir)
        }
        None => workspace.current(),
    };
    let (out_app, out_id) = (app.clone(), spec.id.clone());
    let (exit_app, exit_id) = (app, spec.id.clone());
    let session = PtySession::spawn_utf8(
        PtyOptions {
            shell: spec.shell,
            cwd,
            ..PtyOptions::default()
        },
        TerminalSize {
            cols: spec.cols,
            rows: spec.rows,
        },
        move |data| {
            out_app
                .emit(
                    events::TERMINAL_OUTPUT,
                    Output {
                        id: out_id.clone(),
                        data,
                    },
                )
                .ok();
        },
        move || {
            exit_app
                .emit(
                    events::TERMINAL_EXIT,
                    Exit {
                        id: exit_id.clone(),
                    },
                )
                .ok();
        },
    )
    .map_err(|e| failed(CANT_START, e))?;
    terminals.sessions().insert(spec.id, session);
    Ok(())
}

#[tauri::command]
pub fn terminal_write(
    terminals: State<'_, Terminals>,
    id: String,
    data: String,
) -> Result<(), Message> {
    match terminals.sessions().get_mut(&id) {
        Some(session) => session
            .write(data.as_bytes())
            .map_err(|e| failed(FAILED, e)),
        None => Ok(()),
    }
}

#[tauri::command]
pub fn terminal_resize(
    terminals: State<'_, Terminals>,
    id: String,
    cols: u16,
    rows: u16,
) -> Result<(), Message> {
    match terminals.sessions().get(&id) {
        Some(session) => session
            .resize(TerminalSize { cols, rows })
            .map_err(|e| failed(FAILED, e)),
        None => Ok(()),
    }
}

#[tauri::command]
pub fn terminal_kill(terminals: State<'_, Terminals>, id: String) {
    if let Some(mut session) = terminals.sessions().remove(&id) {
        session.kill().ok();
    }
}
