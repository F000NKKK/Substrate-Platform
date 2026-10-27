//! Language servers: one per workspace root and language, started on the
//! UI's first `lsp_start` and spoken to through the [`Languages`] the
//! product gives the plugin. What a server says unasked goes to the UI as
//! [`events::LSP_NOTIFICATION`], its own log as [`events::LSP_OUTPUT`].
//! Feature `lsp`.

use crate::blocking;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};
use substrate_platform::events;
use substrate_platform::lsp::messages::{NO_SERVER, STOPPED};
use substrate_platform::lsp::{Client, Languages};
use substrate_platform::message::Message;
use substrate_platform::run::LogSink;
use substrate_platform::sync::Locked;
use substrate_platform::workspace::Workspace;
use tauri::{AppHandle, Emitter, Runtime, State};

/// A running server and what it said it can do.
struct Server {
    client: Client,
    capabilities: Value,
}

/// The running servers, by (workspace root, language id).
#[derive(Default)]
pub struct Servers(Mutex<HashMap<(String, String), Server>>);

impl Servers {
    fn running(&self) -> MutexGuard<'_, HashMap<(String, String), Server>> {
        self.0.locked()
    }

    fn client(&self, root: &str, language: &str) -> Result<Client, Message> {
        self.running()
            .get(&(root.to_string(), language.to_string()))
            .map(|s| s.client.clone())
            .ok_or_else(|| Message::new(STOPPED).arg("server", language))
    }

    /// Stops every server — the app is closing.
    pub(crate) fn stop_all(&self) {
        let servers: Vec<Server> = self.running().drain().map(|(_, s)| s).collect();
        for server in servers {
            server.client.shutdown();
        }
    }
}

/// A language as the UI picks a file's by: its id, its files, whether a
/// server serves it.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageEntry {
    id: String,
    extensions: Vec<String>,
    file_names: Vec<String>,
    has_server: bool,
}

#[derive(Clone, Serialize)]
struct NotificationEvent {
    root: String,
    language: String,
    method: String,
    params: Value,
}

#[derive(Clone, Serialize)]
struct OutputEvent {
    root: String,
    language: String,
    line: String,
}

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
    let def = languages
        .get(&language)
        .and_then(|l| l.server.clone())
        .ok_or_else(|| Message::new(NO_SERVER).arg("language", &language))?;
    let (out_app, out_root, out_language) = (app.clone(), root.clone(), language.clone());
    let (note_root, note_language) = (root.clone(), language.clone());
    let started = blocking(move || {
        let log = LogSink::new();
        log.on_push(move |line| {
            let _ = out_app.emit(
                events::LSP_OUTPUT,
                OutputEvent {
                    root: out_root.clone(),
                    language: out_language.clone(),
                    line: line.text.clone(),
                },
            );
        });
        let dir = PathBuf::from(&note_root);
        let client = Client::start(&def, &dir, log, move |n| {
            let _ = app.emit(
                events::LSP_NOTIFICATION,
                NotificationEvent {
                    root: note_root.clone(),
                    language: note_language.clone(),
                    method: n.method,
                    params: n.params,
                },
            );
        })?;
        match client.initialize(&dir, def.initialization_options()) {
            Ok(capabilities) => Ok(Server {
                client,
                capabilities,
            }),
            Err(e) => {
                client.shutdown();
                Err(e)
            }
        }
    })
    .await?;
    let capabilities = started.capabilities.clone();
    // Another start may have won the race while this one initialized.
    let replaced = servers.running().insert(key, started);
    if let Some(old) = replaced {
        old.client.shutdown();
    }
    Ok(capabilities)
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

#[tauri::command]
pub fn lsp_notify(
    servers: State<'_, Servers>,
    root: String,
    language: String,
    method: String,
    params: Value,
) -> Result<(), Message> {
    servers.client(&root, &language)?.notify(&method, params)
}

/// The editor opened `path` (in the workspace) with `text` at `version`.
#[tauri::command]
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
    servers
        .client(&root, &language)?
        .did_open(&path, &language, version, &text)
}

#[tauri::command]
pub fn lsp_did_change(
    servers: State<'_, Servers>,
    root: String,
    language: String,
    path: String,
    version: i64,
    text: String,
) -> Result<(), Message> {
    servers
        .client(&root, &language)?
        .did_change(Path::new(&path), version, &text)
}

#[tauri::command]
pub fn lsp_did_save(
    servers: State<'_, Servers>,
    root: String,
    language: String,
    path: String,
) -> Result<(), Message> {
    servers.client(&root, &language)?.did_save(Path::new(&path))
}

#[tauri::command]
pub fn lsp_did_close(
    servers: State<'_, Servers>,
    root: String,
    language: String,
    path: String,
) -> Result<(), Message> {
    servers
        .client(&root, &language)?
        .did_close(Path::new(&path))
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
            Ok(())
        })
        .await?;
    }
    Ok(())
}
