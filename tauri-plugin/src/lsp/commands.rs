//! Language servers, as commands.

use std::path::{Path, PathBuf};

use serde_json::Value;
use substrate_host_contract::events;
use substrate_message::Message;
use substrate_platform::lsp::{Client, Languages, LspError, ServerDefinition};
use substrate_platform::run::LogSink;
use substrate_platform::workspace::Workspace;
use tauri::{AppHandle, Emitter, Runtime, State};

use super::Servers;
use super::language_entry::LanguageEntry;
use super::notification_event::NotificationEvent;
use super::output_event::OutputEvent;
use super::server::Server;
use crate::blocking;

/// The product's languages.
#[tauri::command]
pub fn lsp_languages(languages: State<'_, Languages>) -> Vec<LanguageEntry> {
    languages
        .all()
        .iter()
        .map(|l| LanguageEntry {
            id: l.id.clone(),
            extensions: l.extensions.clone(),
            file_names: l.file_names.clone(),
            has_server: l.server.is_some(),
        })
        .collect()
}

/// Starts `language`'s server for the workspace folder `root` (once: a
/// running one is reused) and resolves with its capabilities.
#[tauri::command]
#[expect(
    clippy::too_many_arguments,
    reason = "Tauri command: the parameters are the UI's arguments and injected state"
)]
pub async fn lsp_start<R: Runtime>(
    app: AppHandle<R>,
    workspace: State<'_, Workspace>,
    languages: State<'_, Languages>,
    servers: State<'_, Servers>,
    root: String,
    language: String,
) -> Result<Value, Message> {
    workspace.check(Path::new(&root))?;
    let key = (root.clone(), language.clone());
    if let Some(server) = servers.running().get(&key)
        && !server.client.is_stopped()
    {
        return Ok(server.capabilities.clone());
    }
    let definition = languages
        .get(&language)
        .and_then(|l| l.server.clone())
        .ok_or_else(|| LspError::NoServer {
            language: language.clone(),
        })?;
    let started = blocking(move || launch(&app, &definition, &root, &language)).await?;
    let capabilities = started.capabilities.clone();
    // Another start may have won the race while this one initialized.
    let replaced = servers.running().insert(key, started);
    if let Some(old) = replaced {
        old.client.shutdown();
    }
    Ok(capabilities)
}

/// Starts and initializes `definition`'s server for `root`, its log and its
/// notifications going to the UI.
fn launch<R: Runtime>(
    app: &AppHandle<R>,
    definition: &ServerDefinition,
    root: &str,
    language: &str,
) -> Result<Server, LspError> {
    let log = LogSink::new();
    let (log_app, log_root, log_language) = (app.clone(), root.to_string(), language.to_string());
    log.on_push(move |line| {
        let event = OutputEvent {
            root: log_root.clone(),
            language: log_language.clone(),
            line: line.text.clone(),
        };
        // The UI window is gone: nobody to tell.
        log_app.emit(events::LSP_OUTPUT, event).ok();
    });
    let dir = PathBuf::from(root);
    let (note_app, note_root, note_language) =
        (app.clone(), root.to_string(), language.to_string());
    let client = Client::start(definition, &dir, log, move |notification| {
        let event = NotificationEvent {
            root: note_root.clone(),
            language: note_language.clone(),
            method: notification.method,
            params: notification.params,
        };
        // The UI window is gone: nobody to tell.
        note_app.emit(events::LSP_NOTIFICATION, event).ok();
    })?;
    match client.initialize(&dir, definition.initialization_options()) {
        Ok(capabilities) => Ok(Server {
            client,
            capabilities,
        }),
        Err(error) => {
            client.shutdown();
            Err(error)
        }
    }
}

/// `method` with `params` to the server: its result.
#[tauri::command]
pub async fn lsp_request(
    servers: State<'_, Servers>,
    root: String,
    language: String,
    method: String,
    params: Value,
) -> Result<Value, Message> {
    let client = servers.client(&root, &language)?;
    blocking(move || client.request(&method, params)).await
}

/// `method` with `params` to the server, no answer expected.
#[tauri::command]
pub fn lsp_notify(
    servers: State<'_, Servers>,
    root: String,
    language: String,
    method: String,
    params: Value,
) -> Result<(), Message> {
    Ok(servers.client(&root, &language)?.notify(&method, params)?)
}

/// The editor opened `path` (in the workspace) with `text` at `version`.
#[tauri::command]
#[expect(
    clippy::too_many_arguments,
    reason = "Tauri command: the parameters are the UI's arguments and injected state"
)]
pub fn lsp_did_open(
    workspace: State<'_, Workspace>,
    servers: State<'_, Servers>,
    root: String,
    language: String,
    path: String,
    version: i64,
    text: String,
) -> Result<(), Message> {
    let path = PathBuf::from(path);
    workspace.check(&path)?;
    Ok(servers
        .client(&root, &language)?
        .did_open(&path, &language, version, &text)?)
}

/// The editor changed `path` to `text` at `version`.
#[tauri::command]
pub fn lsp_did_change(
    servers: State<'_, Servers>,
    root: String,
    language: String,
    path: String,
    version: i64,
    text: String,
) -> Result<(), Message> {
    Ok(servers
        .client(&root, &language)?
        .did_change(Path::new(&path), version, &text)?)
}

/// The editor saved `path`.
#[tauri::command]
pub fn lsp_did_save(
    servers: State<'_, Servers>,
    root: String,
    language: String,
    path: String,
) -> Result<(), Message> {
    Ok(servers
        .client(&root, &language)?
        .did_save(Path::new(&path))?)
}

/// The editor closed `path`.
#[tauri::command]
pub fn lsp_did_close(
    servers: State<'_, Servers>,
    root: String,
    language: String,
    path: String,
) -> Result<(), Message> {
    Ok(servers
        .client(&root, &language)?
        .did_close(Path::new(&path))?)
}

/// Stops `language`'s server for `root`, if it runs.
#[tauri::command]
pub async fn lsp_stop(
    servers: State<'_, Servers>,
    root: String,
    language: String,
) -> Result<(), Message> {
    let server = servers.running().remove(&(root, language));
    if let Some(server) = server {
        blocking(move || {
            server.client.shutdown();
            Ok::<_, Message>(())
        })
        .await?;
    }
    Ok(())
}
