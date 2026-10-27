//! One server's connection: writing frames, and the reader that matches
//! responses to their requests.

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::{Mutex, mpsc};

use serde_json::{Value, json};
use substrate_framing::{read_frame, write_frame};
use substrate_lock::Locked;
use substrate_lsp_domain::{LspError, Notification};
use substrate_run_app::ports::ChildProcess;

use crate::answer::answer;

/// What a request gets back.
pub(crate) type Reply = Result<Value, LspError>;
/// Who hears what the server says unasked.
pub(crate) type OnNotification = Box<dyn Fn(Notification) + Send + Sync>;

/// One server's connection: what the client's clones and its reader share.
pub(crate) struct Connection {
    pub(crate) name: String,
    pub(crate) writer: Mutex<Box<dyn Write + Send>>,
    pub(crate) next_id: AtomicI64,
    pub(crate) pending: Mutex<HashMap<i64, mpsc::Sender<Reply>>>,
    pub(crate) stopped: AtomicBool,
    pub(crate) on_notification: OnNotification,
    pub(crate) child: Mutex<Option<Box<dyn ChildProcess>>>,
}

impl Connection {
    /// A connection writing to `writer`, telling `on_notification` what the
    /// server says unasked; no process attached yet.
    pub(crate) fn new(
        name: String,
        writer: Box<dyn Write + Send>,
        on_notification: OnNotification,
    ) -> Self {
        Self {
            name,
            writer: Mutex::new(writer),
            next_id: AtomicI64::new(1),
            pending: Mutex::new(HashMap::new()),
            stopped: AtomicBool::new(false),
            on_notification,
            child: Mutex::new(None),
        }
    }

    /// Sends one message, unless the stream has ended.
    pub(crate) fn send(&self, message: &Value) -> Result<(), LspError> {
        if self.stopped.load(Ordering::Acquire) {
            return Err(self.stopped_error());
        }
        write_frame(&mut *self.writer.locked(), message).map_err(|source| LspError::Failed {
            server: self.name.clone(),
            source,
        })
    }

    /// The error for a server no longer running.
    pub(crate) fn stopped_error(&self) -> LspError {
        LspError::Stopped {
            server: self.name.clone(),
        }
    }

    /// The stream ended: every request still waiting fails.
    pub(crate) fn stop(&self) {
        self.stopped.store(true, Ordering::Release);
        self.pending.locked().clear();
    }

    /// Reads frames until the stream ends: responses to their waiting request,
    /// notifications to the listener, the server's requests answered.
    pub(crate) fn read_loop(&self, mut input: impl BufRead) {
        while let Ok(Some(message)) = read_frame(&mut input) {
            let method = message.get("method").and_then(Value::as_str);
            let params = || message.get("params").cloned().unwrap_or(Value::Null);
            match (method, message.get("id")) {
                (Some(method), Some(id)) => self.respond(id, method, &params()),
                (Some(method), None) => (self.on_notification)(Notification {
                    method: method.to_string(),
                    params: params(),
                }),
                (None, Some(id)) => self.deliver(id, &message),
                (None, None) => {}
            }
        }
        self.stop();
    }

    /// Answers the server's request `id`.
    fn respond(&self, id: &Value, method: &str, params: &Value) {
        let reply = match answer(method, params) {
            Ok(result) => json!({ "jsonrpc": "2.0", "id": id, "result": result }),
            Err((code, text)) => json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": { "code": code, "message": text },
            }),
        };
        // A failed send means the stream ended, which the read loop sees next.
        self.send(&reply).ok();
    }

    /// Hands the response `message` to the request `id` waiting for it.
    fn deliver(&self, id: &Value, message: &Value) {
        let Some(id) = id.as_i64() else { return };
        let Some(waiting) = self.pending.locked().remove(&id) else {
            return;
        };
        let reply = match message.get("error") {
            Some(error) => Err(LspError::Rejected {
                server: self.name.clone(),
                method: String::new(),
                reason: error
                    .get("message")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
            }),
            None => Ok(message.get("result").cloned().unwrap_or(Value::Null)),
        };
        // The request gave up waiting (it timed out): nobody to tell.
        waiting.send(reply).ok();
    }
}
