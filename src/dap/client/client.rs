//! A running debug adapter: requests sent and matched to their responses by
//! sequence number, the adapter's events decoded and passed on, its own
//! requests answered, and the `disconnect` farewell.

use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::Stdio;
use std::sync::atomic::Ordering;
use std::sync::{Arc, mpsc};
use std::time::Duration;

use serde_json::{Value, json};

use super::connection::Connection;
use crate::dap::{AdapterDefinition, Command, DapError, Event};
use crate::run::LogSink;
use crate::sync::Locked;

/// How long [`Client::request`] waits for an answer.
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
/// How long `shutdown` gives the adapter to answer `disconnect` before it is killed.
const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(2);
/// How long the adapter has to exit after `disconnect`.
const EXIT_GRACE: Duration = Duration::from_millis(500);
const EXIT_POLL: Duration = Duration::from_millis(20);

/// A connection to one debug adapter. Cheap to clone; every clone speaks to
/// the same adapter. Requests block their caller (a backend command's
/// thread), never the reader — so several can wait at once, which `launch`
/// needs: many adapters answer it only after `configurationDone`.
#[derive(Clone)]
pub struct Client {
    pub(super) connection: Arc<Connection>,
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
    /// Starts `adapter` in `cwd` (usually the workspace root): its stdin and
    /// stdout carry the protocol, its stderr goes to `log`, and `on_event`
    /// hears what it says unasked — on the reader's thread, so it should
    /// hand off rather than block.
    ///
    /// # Errors
    ///
    /// The adapter's process did not start.
    pub fn start(
        adapter: &AdapterDefinition,
        cwd: &Path,
        log: LogSink,
        on_event: impl Fn(Event) + Send + Sync + 'static,
    ) -> Result<Self, DapError> {
        let cant_start = |source: std::io::Error| DapError::CantStart {
            adapter: adapter.name().to_string(),
            source,
        };
        let mut command = crate::process::command(&adapter.command);
        command
            .args(&adapter.args)
            .envs(&adapter.env)
            .current_dir(cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        // Its own process group, so stopping it takes down what it started
        // (the debuggee) — see `run::kill_tree`.
        #[cfg(unix)]
        std::os::unix::process::CommandExt::process_group(&mut command, 0);
        let mut child = command.spawn().map_err(cant_start)?;
        let (Some(stdin), Some(stdout), Some(stderr)) =
            (child.stdin.take(), child.stdout.take(), child.stderr.take())
        else {
            crate::run::kill_tree(&mut child);
            return Err(cant_start(std::io::Error::other("no pipes")));
        };
        std::thread::spawn(move || {
            for line in BufReader::new(stderr).lines().map_while(Result::ok) {
                log.info(line);
            }
        });
        let client = Self::connect(adapter.name(), stdout, stdin, on_event);
        *client.connection.child.locked() = Some(child);
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
        let mut request = json!({ "seq": seq, "type": "request", "command": command.as_str() });
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

    /// `disconnect`, terminating the debuggee; a started adapter that's
    /// still running after that is killed. Nothing can be sent afterwards.
    pub fn shutdown(&self) {
        if !self.is_stopped() {
            self.request_within(
                &Command::Disconnect,
                json!({ "terminateDebuggee": true }),
                SHUTDOWN_TIMEOUT,
            )
            .ok();
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
            crate::run::kill_tree(&mut child);
            child.wait().ok();
        }
    }
}
