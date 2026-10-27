//! Terminals, as commands.
#![expect(
    clippy::needless_pass_by_value,
    reason = "Tauri hands a command its arguments by value: `State` and deserialized parameters cannot be borrowed"
)]

use std::path::PathBuf;

use substrate_host_contract::events;
use substrate_message::Message;
use substrate_terminal_app::{detect_shells, spawn_utf8};
use substrate_terminal_domain::{SessionOptions, TerminalError, TerminalSize};
use substrate_terminal_pty::{PortablePty, SystemShells};
use substrate_workspace_app::Workspace;
use tauri::{AppHandle, Emitter, Runtime, State};

use super::exit::Exit;
use super::output::Output;
use super::shell_entry::ShellEntry;
use super::{SpawnRequest, Terminals};

/// The shells installed here, for "New Terminal".
#[tauri::command]
pub(crate) fn terminal_shells() -> Vec<ShellEntry> {
    detect_shells(&SystemShells)
        .into_iter()
        .map(ShellEntry::from)
        .collect()
}

#[tauri::command]
pub(crate) fn terminal_spawn<R: Runtime>(
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
    let session = spawn_utf8(
        &PortablePty,
        SessionOptions {
            shell: spec.shell,
            cwd,
            ..SessionOptions::default()
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
    )?;
    terminals.sessions().insert(spec.id, session);
    Ok(())
}

#[tauri::command]
pub(crate) fn terminal_write(
    terminals: State<'_, Terminals>,
    id: String,
    data: String,
) -> Result<(), Message> {
    match terminals.sessions().get_mut(&id) {
        Some(session) => Ok(session
            .write(data.as_bytes())
            .map_err(TerminalError::from)?),
        None => Ok(()),
    }
}

#[tauri::command]
pub(crate) fn terminal_resize(
    terminals: State<'_, Terminals>,
    id: String,
    cols: u16,
    rows: u16,
) -> Result<(), Message> {
    match terminals.sessions().get(&id) {
        Some(session) => Ok(session.resize(TerminalSize { cols, rows })?),
        None => Ok(()),
    }
}

#[tauri::command]
pub(crate) fn terminal_kill(terminals: State<'_, Terminals>, id: String) {
    if let Some(mut session) = terminals.sessions().remove(&id) {
        session.kill().ok();
    }
}
