//! A running language server: requests sent and matched to their responses
//! by id, notifications both ways, the server's own requests answered, the
//! document lifecycle (`didOpen`/`didChange`/`didClose`) and the
//! `initialize`…`exit` handshake.

use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::Stdio;
use std::sync::atomic::Ordering;
use std::sync::{Arc, mpsc};
use std::time::Duration;

use serde_json::{Value, json};
use substrate_run_app::LogSink;

use super::connection::Connection;
use super::{LspError, Notification, ServerDefinition, capabilities, uri};
use crate::sync::Locked;

/// How long [`Client::request`] waits for an answer.
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
/// How long `shutdown` gives the server to answer before it is killed.
const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(2);
/// How long the server has to exit after `exit`.
const EXIT_GRACE: Duration = Duration::from_millis(500);
const EXIT_POLL: Duration = Duration::from_millis(20);

/// A connection to one language server. Cheap to clone; every clone speaks
/// to the same server. Requests block their caller (a backend command's
/// thread), never the reader.
#[derive(Clone)]
pub struct Client {
    connection: Arc<Connection>,
}

impl Client {
    /// Starts `server` in `cwd` (usually the workspace root): its stdin and
    /// stdout carry the protocol, its stderr goes to `log`, and
    /// `on_notification` hears what it says unasked — on the reader's
    /// thread, so it should hand off rather than block.
    ///
    /// # Errors
    ///
    /// The server has stopped, does not answer in time, or refuses.
    pub fn start(
        server: &ServerDefinition,
        cwd: &Path,
        log: LogSink,
        on_notification: impl Fn(Notification) + Send + Sync + 'static,
    ) -> Result<Self, LspError> {
        let cant_start = |source: std::io::Error| LspError::CantStart {
            server: server.name().to_string(),
            source,
        };
        let mut command = substrate_process::command(&server.command);
        command
            .args(&server.args)
            .envs(&server.env)
            .current_dir(cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        // Its own process group, so stopping it takes down what it started
        // (`cargo check` under rust-analyzer) — see `substrate_run_process::kill_tree`.
        #[cfg(unix)]
        std::os::unix::process::CommandExt::process_group(&mut command, 0);
        let mut child = command.spawn().map_err(cant_start)?;
        let (Some(stdin), Some(stdout), Some(stderr)) =
            (child.stdin.take(), child.stdout.take(), child.stderr.take())
        else {
            substrate_run_process::kill_tree(&mut child);
            return Err(cant_start(std::io::Error::other("no pipes")));
        };
        std::thread::spawn(move || {
            for line in BufReader::new(stderr).lines().map_while(Result::ok) {
                log.info(line);
            }
        });
        let client = Self::connect(server.name(), stdout, stdin, on_notification);
        *client.connection.child.locked() = Some(child);
        Ok(client)
    }

    /// Speaks the protocol over `input`/`output` — a started process's
    /// pipes, a socket. `name` is the server's, for the user.
    pub fn connect(
        name: impl Into<String>,
        input: impl Read + Send + 'static,
        output: impl Write + Send + 'static,
        on_notification: impl Fn(Notification) + Send + Sync + 'static,
    ) -> Self {
        let connection = Arc::new(Connection::new(
            name.into(),
            Box::new(output),
            Box::new(on_notification),
        ));
        let reader = Arc::clone(&connection);
        std::thread::spawn(move || reader.read_loop(BufReader::new(input)));
        Self { connection }
    }

    /// The server's name, for the user.
    pub fn name(&self) -> &str {
        &self.connection.name
    }

    /// Whether the server's stream has ended (it exited or crashed).
    pub fn is_stopped(&self) -> bool {
        self.connection.stopped.load(Ordering::Acquire)
    }

    /// Sends `method` and waits up to [`REQUEST_TIMEOUT`] for its result.
    ///
    /// # Errors
    ///
    /// The server has stopped, does not answer in time, or refuses.
    pub fn request(&self, method: &str, params: Value) -> Result<Value, LspError> {
        self.request_within(method, params, REQUEST_TIMEOUT)
    }

    /// [`Self::request`] with its own timeout. A request that times out is
    /// cancelled (`$/cancelRequest`).
    ///
    /// # Errors
    ///
    /// The server has stopped, does not answer in time, or refuses.
    pub fn request_within(
        &self,
        method: &str,
        params: Value,
        timeout: Duration,
    ) -> Result<Value, LspError> {
        let id = self.connection.next_id.fetch_add(1, Ordering::Relaxed);
        let (tx, rx) = mpsc::channel();
        self.connection.pending.locked().insert(id, tx);
        let sent = self.connection.send(&json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        }));
        if let Err(e) = sent {
            self.connection.pending.locked().remove(&id);
            return Err(e);
        }
        match rx.recv_timeout(timeout) {
            Ok(reply) => reply.map_err(|e| e.answering(method)),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                self.connection.pending.locked().remove(&id);
                self.notify("$/cancelRequest", json!({ "id": id })).ok();
                Err(LspError::TimedOut {
                    server: self.connection.name.clone(),
                    method: method.to_string(),
                })
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => Err(self.connection.stopped_error()),
        }
    }

    /// Sends a notification to the server; no answer is awaited.
    ///
    /// # Errors
    ///
    /// The server has stopped, does not answer in time, or refuses.
    pub fn notify(&self, method: &str, params: Value) -> Result<(), LspError> {
        self.connection.send(&json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
        }))
    }

    /// `initialize` for the workspace at `root`, then `initialized`: the
    /// server's capabilities (the result's `capabilities`).
    ///
    /// # Errors
    ///
    /// The server has stopped, does not answer in time, or refuses.
    pub fn initialize(
        &self,
        root: &Path,
        initialization_options: Option<Value>,
    ) -> Result<Value, LspError> {
        let root_uri = uri::from_path(root);
        let folder_name = root
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| root_uri.clone());
        let mut params = json!({
            "processId": std::process::id(),
            "clientInfo": { "name": "substrate-platform", "version": env!("CARGO_PKG_VERSION") },
            "rootUri": root_uri,
            "workspaceFolders": [{ "uri": root_uri, "name": folder_name }],
            "capabilities": capabilities::client(),
        });
        if let Some(options) = initialization_options {
            params["initializationOptions"] = options;
        }
        let result = self.request("initialize", params)?;
        self.notify("initialized", json!({}))?;
        Ok(result.get("capabilities").cloned().unwrap_or(Value::Null))
    }

    /// `textDocument/didOpen`: the editor holds `path` now, `text` is its
    /// content at `version`.
    ///
    /// # Errors
    ///
    /// The server has stopped, does not answer in time, or refuses.
    pub fn did_open(
        &self,
        path: &Path,
        language_id: &str,
        version: i64,
        text: &str,
    ) -> Result<(), LspError> {
        self.notify(
            "textDocument/didOpen",
            json!({ "textDocument": {
                "uri": uri::from_path(path),
                "languageId": language_id,
                "version": version,
                "text": text,
            }}),
        )
    }

    /// `textDocument/didChange` with the whole new text (full sync — every
    /// server accepts it).
    ///
    /// # Errors
    ///
    /// The server has stopped, does not answer in time, or refuses.
    pub fn did_change(&self, path: &Path, version: i64, text: &str) -> Result<(), LspError> {
        self.notify(
            "textDocument/didChange",
            json!({
                "textDocument": { "uri": uri::from_path(path), "version": version },
                "contentChanges": [{ "text": text }],
            }),
        )
    }

    /// `textDocument/didSave`: the editor saved `path`.
    ///
    /// # Errors
    ///
    /// The server has stopped, does not answer in time, or refuses.
    pub fn did_save(&self, path: &Path) -> Result<(), LspError> {
        self.notify(
            "textDocument/didSave",
            json!({ "textDocument": { "uri": uri::from_path(path) } }),
        )
    }

    /// `textDocument/didClose`: the editor closed `path`.
    ///
    /// # Errors
    ///
    /// The server has stopped, does not answer in time, or refuses.
    pub fn did_close(&self, path: &Path) -> Result<(), LspError> {
        self.notify(
            "textDocument/didClose",
            json!({ "textDocument": { "uri": uri::from_path(path) } }),
        )
    }

    /// `shutdown`, then `exit`; a started server that's still running after
    /// that is killed. Nothing can be sent afterwards.
    pub fn shutdown(&self) {
        if !self.is_stopped() {
            self.request_within("shutdown", Value::Null, SHUTDOWN_TIMEOUT)
                .ok();
            self.notify("exit", Value::Null).ok();
        }
        self.connection.stop();
        if let Some(mut child) = self.connection.child.locked().take() {
            let deadline = std::time::Instant::now() + EXIT_GRACE;
            while std::time::Instant::now() < deadline {
                if let Ok(Some(_)) = child.try_wait() {
                    return;
                }
                std::thread::sleep(EXIT_POLL);
            }
            substrate_run_process::kill_tree(&mut child);
            child.wait().ok();
        }
    }
}
