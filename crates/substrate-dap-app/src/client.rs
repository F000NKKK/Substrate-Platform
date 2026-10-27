//! A running debug adapter: requests sent and matched to their responses by
//! sequence number, the adapter's events decoded and passed on, its own
//! requests answered, and the `disconnect` farewell.

use std::io::{BufRead, BufReader, Read, Write};
use std::sync::atomic::Ordering;
use std::sync::{Arc, mpsc};
use std::time::Duration;

use serde_json::{Value, json};
use substrate_dap_domain::{AdapterDefinition, Command, DapError, Event};
use substrate_lock::Locked;
use substrate_run_app::LogSink;
use substrate_run_app::ports::ServiceLauncher;
use substrate_run_domain::Task;

use crate::connection::Connection;

/// How long [`Client::request`] waits for an answer.
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// A connection to one debug adapter. Cheap to clone; every clone speaks to
/// the same adapter. Requests block their caller (a backend command's
/// thread), never the reader — so several can wait at once, which `launch`
/// needs: many adapters answer it only after `configurationDone`.
#[derive(Clone)]
pub struct Client {
    pub(crate) connection: Arc<Connection>,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("adapter", &self.connection.name)
            .field("stopped", &self.is_stopped())
            .finish()
    }
}

impl Client {
    /// Starts `adapter` through `launcher` in `cwd` (usually the workspace
    /// root): its stdin and stdout carry the protocol, its stderr goes to
    /// `log`, and `on_event` hears what it says unasked — on the reader's
    /// thread, so it should hand off rather than block.
    ///
    /// # Errors
    ///
    /// The adapter's process did not start.
    pub fn start(
        launcher: &dyn ServiceLauncher,
        adapter: &AdapterDefinition,
        cwd: &std::path::Path,
        log: LogSink,
        on_event: impl Fn(Event) + Send + Sync + 'static,
    ) -> Result<Self, DapError> {
        let mut task = Task::new(&adapter.command).args(&adapter.args).cwd(cwd);
        task.envs = adapter.env.clone().into_iter().collect();
        let service = launcher
            .launch_service(&task)
            .map_err(|source| DapError::CantStart {
                adapter: adapter.name().to_string(),
                source,
            })?;
        std::thread::spawn(move || {
            for line in BufReader::new(service.stderr).lines().map_while(Result::ok) {
                log.info(line);
            }
        });
        let client = Self::connect(adapter.name(), service.stdout, service.stdin, on_event);
        *client.connection.child.locked() = Some(service.child);
        Ok(client)
    }

    /// Speaks the protocol over `input`/`output` — a started process's
    /// pipes, a socket. `name` is the adapter's, for the user.
    pub fn connect(
        name: impl Into<String>,
        input: impl Read + Send + 'static,
        output: impl Write + Send + 'static,
        on_event: impl Fn(Event) + Send + Sync + 'static,
    ) -> Self {
        let connection = Arc::new(Connection::new(
            name.into(),
            Box::new(output),
            Box::new(on_event),
        ));
        let reader = Arc::clone(&connection);
        std::thread::spawn(move || reader.read_loop(BufReader::new(input)));
        Self { connection }
    }

    /// The adapter's name, for the user.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.connection.name
    }

    /// Whether the adapter's stream has ended (it exited or crashed).
    #[must_use]
    pub fn is_stopped(&self) -> bool {
        self.connection.stopped.load(Ordering::Acquire)
    }

    /// Sends `command` and waits up to [`REQUEST_TIMEOUT`] for its body.
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn request(&self, command: &Command, arguments: Value) -> Result<Value, DapError> {
        self.request_within(command, arguments, REQUEST_TIMEOUT)
    }

    /// [`Self::request`] with its own timeout.
    ///
    /// # Errors
    ///
    /// The adapter has stopped, does not answer in time, or refuses.
    pub fn request_within(
        &self,
        command: &Command,
        arguments: Value,
        timeout: Duration,
    ) -> Result<Value, DapError> {
        let seq = self.connection.next_seq();
        let (tx, rx) = mpsc::channel();
        self.connection.pending.locked().insert(seq, tx);
        let mut request = json!({ "seq": seq.0, "type": "request", "command": command.as_str() });
        request["arguments"] = arguments;
        let sent = self.connection.send(&request);
        if let Err(e) = sent {
            self.connection.pending.locked().remove(&seq);
            return Err(e);
        }
        match rx.recv_timeout(timeout) {
            Ok(reply) => reply,
            Err(mpsc::RecvTimeoutError::Timeout) => {
                self.connection.pending.locked().remove(&seq);
                Err(DapError::TimedOut {
                    adapter: self.connection.name.clone(),
                    command: command.clone(),
                })
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => Err(self.connection.stopped_error()),
        }
    }
}
