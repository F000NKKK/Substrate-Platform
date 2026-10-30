//! Terminals, as commands.
#![expect(
    clippy::unreachable,
    reason = "the expansion of `#[tauri::command]` contains `unreachable!`"
)]

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use substrate_host_contract::events;
use substrate_lock::Locked;
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
use crate::blocking::blocking;

/// The shells installed here, for "New Terminal".
#[tauri::command]
pub(crate) async fn terminal_shells() -> Result<Vec<ShellEntry>, Message> {
    blocking(|| {
        Ok::<_, Message>(
            detect_shells(&SystemShells)
                .into_iter()
                .map(ShellEntry::from)
                .collect(),
        )
    })
    .await
}

#[tauri::command]
pub(crate) async fn terminal_spawn<R: Runtime>(
    app: AppHandle<R>,
    terminals: State<'_, Terminals>,
    workspace: State<'_, Workspace>,
    spec: SpawnRequest,
) -> Result<(), Message> {
    let cwd = directory(&workspace, spec.cwd)?;
    let terminals = terminals.inner().clone();
    blocking(move || {
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
        terminals
            .sessions()
            .insert(spec.id, Arc::new(Mutex::new(session)));
        Ok::<_, Message>(())
    })
    .await
}

#[tauri::command]
pub(crate) async fn terminal_write(
    terminals: State<'_, Terminals>,
    id: String,
    data: String,
) -> Result<(), Message> {
    let terminals = terminals.inner().clone();
    blocking(move || {
        let session = terminals.sessions().get(&id).cloned();
        match session {
            Some(session) => Ok::<_, Message>(
                session
                    .locked()
                    .write(data.as_bytes())
                    .map_err(TerminalError::from)?,
            ),
            None => Ok(()),
        }
    })
    .await
}

#[tauri::command]
pub(crate) async fn terminal_resize(
    terminals: State<'_, Terminals>,
    id: String,
    cols: u16,
    rows: u16,
) -> Result<(), Message> {
    let terminals = terminals.inner().clone();
    blocking(move || {
        let session = terminals.sessions().get(&id).cloned();
        match session {
            Some(session) => {
                Ok::<_, Message>(session.locked().resize(TerminalSize { cols, rows })?)
            }
            None => Ok::<_, Message>(()),
        }
    })
    .await
}

#[tauri::command]
pub(crate) async fn terminal_kill(
    terminals: State<'_, Terminals>,
    id: String,
) -> Result<(), Message> {
    let terminals = terminals.inner().clone();
    blocking(move || {
        let session = terminals.sessions().remove(&id);
        if let Some(session) = session {
            session.locked().kill().map_err(TerminalError::from)?;
        }
        Ok::<_, Message>(())
    })
    .await
}

fn directory(workspace: &Workspace, dir: Option<String>) -> Result<Option<PathBuf>, Message> {
    match dir {
        Some(dir) => {
            let dir = PathBuf::from(dir);
            workspace.check(&dir)?;
            Ok(Some(dir))
        }
        None => Ok(workspace.current()),
    }
}
