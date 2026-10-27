//! A running language server: requests sent and matched to their responses
//! by id, notifications both ways.

use std::fmt;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::sync::atomic::Ordering;
use std::sync::{Arc, mpsc};
use std::time::Duration;

use serde_json::{Value, json};
use substrate_lock::Locked;
use substrate_lsp_domain::{LspError, Notification, ServerDefinition};
use substrate_run_app::LogSink;
use substrate_run_app::ports::ServiceLauncher;
use substrate_run_domain::Task;

use crate::connection::Connection;
use crate::envelope::envelope;

/// How long [`Client::request`] waits for an answer.
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// A connection to one language server. Cheap to clone; every clone speaks
/// to the same server. Requests block their caller (a backend command's
/// thread), never the reader.
#[derive(Clone)]
pub struct Client {
    pub(crate) connection: Arc<Connection>,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("name", &self.connection.name)
            .finish_non_exhaustive()
    }
}

impl Client {
    /// Starts `server` through `launcher` in `cwd` (usually the workspace
    /// root): its stdin and stdout carry the protocol, its stderr goes to
    /// `log`, and `on_notification` hears what it says unasked — on the
    /// reader's thread, so it should hand off rather than block.
    ///
    /// # Errors
    ///
    /// The server's process did not start.
    pub fn start(
        launcher: &dyn ServiceLauncher,
        server: &ServerDefinition,
        cwd: &Path,
        log: LogSink,
        on_notification: impl Fn(Notification) + Send + Sync + 'static,
    ) -> Result<Self, LspError> {
        let mut task = Task::new(&server.command).args(&server.args).cwd(cwd);
        task.envs = server.env.clone().into_iter().collect();
        let service = launcher
            .launch_service(&task)
            .map_err(|source| LspError::CantStart {
                server: server.name().to_string(),
                source,
            })?;
        std::thread::spawn(move || {
            for line in BufReader::new(service.stderr).lines().map_while(Result::ok) {
                log.info(line);
            }
        });
        let client = Self::connect(
            server.name(),
            service.stdout,
            service.stdin,
            on_notification,
        );
        *client.connection.child.locked() = Some(service.child);
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
    #[must_use]
    pub fn name(&self) -> &str {
        &self.connection.name
    }

    /// Whether the server's stream has ended (it exited or crashed).
    #[must_use]
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
        let sent = self.connection.send(&envelope(Some(id), method, params));
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
    /// The server has stopped, or the stream can't be written.
    pub fn notify(&self, method: &str, params: Value) -> Result<(), LspError> {
        self.connection.send(&envelope(None, method, params))
    }
}
