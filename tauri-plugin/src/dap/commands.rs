//! Debug adapters, as commands.

use std::path::{Path, PathBuf};

use serde_json::Value;
use substrate_host_contract::events;
use substrate_message::Message;
use substrate_platform::dap::{AdapterDefinition, AdapterId, Adapters, Client, Command, DapError};
use substrate_platform::workspace::Workspace;
use substrate_run_app::LogSink;
use tauri::{AppHandle, Emitter, Runtime, State};

use super::Sessions;
use super::adapter_entry::AdapterEntry;
use super::debug_event::DebugEvent;
use super::output_event::OutputEvent;
use super::session::Session;
use crate::blocking;

/// The product's adapters.
#[tauri::command]
pub(crate) fn dap_adapters(adapters: State<'_, Adapters>) -> Vec<AdapterEntry> {
    adapters.all().iter().map(AdapterEntry::from).collect()
}

/// Starts `adapter` for the workspace folder `root` (once: a running one is
/// reused) and resolves with its capabilities. The session is not launched:
/// the UI sends `launch` (or `attach`) with `dap_request`.
#[tauri::command]
#[expect(
    clippy::too_many_arguments,
    reason = "Tauri command: the parameters are the UI's arguments and injected state"
)]
pub(crate) async fn dap_start<R: Runtime>(
    app: AppHandle<R>,
    workspace: State<'_, Workspace>,
    adapters: State<'_, Adapters>,
    sessions: State<'_, Sessions>,
    root: String,
    adapter: AdapterId,
) -> Result<Value, Message> {
    workspace.check(Path::new(&root))?;
    let key = (root.clone(), adapter.clone());
    if let Some(session) = sessions.running().get(&key)
        && !session.client.is_stopped()
    {
        return Ok(session.capabilities.clone());
    }
    let definition = adapters
        .get(&adapter)
        .map(|a| a.definition.clone())
        .ok_or_else(|| DapError::NoAdapter {
            adapter: adapter.clone(),
        })?;
    let started = blocking(move || launch(&app, definition, &root, &adapter)).await?;
    let capabilities = started.capabilities.clone();
    // Another start may have won the race while this one initialized.
    let replaced = sessions.running().insert(key, started);
    if let Some(old) = replaced {
        old.client.shutdown();
    }
    Ok(capabilities)
}

/// Starts and initializes `definition`'s adapter for `root`, its log and its
/// events going to the UI.
fn launch<R: Runtime>(
    app: &AppHandle<R>,
    definition: AdapterDefinition,
    root: &str,
    adapter: &AdapterId,
) -> Result<Session, DapError> {
    let log = LogSink::new();
    let (log_app, log_root, log_adapter) = (app.clone(), root.to_string(), adapter.clone());
    log.on_push(move |line| {
        let event = OutputEvent {
            root: log_root.clone(),
            adapter: log_adapter.clone(),
            line: line.text.clone(),
        };
        // The UI window is gone: nobody to tell.
        log_app.emit(events::DAP_OUTPUT, event).ok();
    });
    let (event_app, event_root, event_adapter) = (app.clone(), root.to_string(), adapter.clone());
    let client = Client::start(&definition, &PathBuf::from(root), log, move |event| {
        let event = DebugEvent {
            root: event_root.clone(),
            adapter: event_adapter.clone(),
            event,
        };
        // The UI window is gone: nobody to tell.
        event_app.emit(events::DAP_EVENT, event).ok();
    })?;
    match client.initialize(adapter) {
        Ok(capabilities) => Ok(Session {
            client,
            capabilities,
            definition,
        }),
        Err(error) => {
            client.shutdown();
            Err(error)
        }
    }
}

/// `command` with `arguments` to the adapter: its body. A `launch` starts
/// from the adapter's `launch_defaults`.
#[tauri::command]
pub(crate) async fn dap_request(
    sessions: State<'_, Sessions>,
    root: String,
    adapter: AdapterId,
    command: String,
    arguments: Value,
) -> Result<Value, Message> {
    let command = Command::from(command.as_str());
    let (client, arguments) = sessions.get(&root, &adapter, |session| {
        let arguments = match command {
            Command::Launch => session.definition.launch_arguments(&arguments),
            _ => arguments,
        };
        (session.client.clone(), arguments)
    })?;
    blocking(move || client.request(&command, arguments)).await
}

/// Stops `adapter`'s session for `root`, if it runs.
#[tauri::command]
pub(crate) async fn dap_stop(
    sessions: State<'_, Sessions>,
    root: String,
    adapter: AdapterId,
) -> Result<(), Message> {
    let session = sessions.running().remove(&(root, adapter));
    if let Some(session) = session {
        blocking(move || {
            session.client.shutdown();
            Ok::<_, Message>(())
        })
        .await?;
    }
    Ok(())
}
