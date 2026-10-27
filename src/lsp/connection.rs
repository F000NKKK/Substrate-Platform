//! One server's connection: writing frames, and the reader that matches
//! responses to their requests.

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::Child;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::{Mutex, mpsc};

use serde_json::{Value, json};

use super::answer::answer;
use super::{Notification, framing, messages};
use crate::message::Message;
use crate::sync::Locked;

/// What a request gets back.
pub(super) type Reply = Result<Value, Message>;
/// Who hears what the server says unasked.
pub(super) type OnNotification = Box<dyn Fn(Notification) + Send + Sync>;

/// One server's connection: what the client's clones and its reader share.
pub(super) struct Connection {
    pub(super) name: String,
    pub(super) writer: Mutex<Box<dyn Write + Send>>,
    pub(super) next_id: AtomicI64,
    pub(super) pending: Mutex<HashMap<i64, mpsc::Sender<Reply>>>,
    pub(super) stopped: AtomicBool,
    pub(super) on_notification: OnNotification,
    pub(super) child: Mutex<Option<Child>>,
}

impl Connection {
    /// A connection writing to `writer`, telling `on_notification` what the
    /// server says unasked; no process attached yet.
    pub(super) fn new(
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
    pub(super) fn send(&self, message: &Value) -> Result<(), Message> {
        if self.stopped.load(Ordering::Acquire) {
            return Err(self.stopped_message());
        }
        framing::write(&mut *self.writer.locked(), message).map_err(|e| self.failed(e))
    }

    /// The message for a failure to speak to the server.
    pub(super) fn failed(&self, error: impl ToString) -> Message {
        Message::new(messages::FAILED)
            .arg("server", &self.name)
            .arg("error", error.to_string())
    }

    /// The message for a server no longer running.
    pub(super) fn stopped_message(&self) -> Message {
        Message::new(messages::STOPPED).arg("server", &self.name)
    }

    /// The stream ended: every request still waiting fails.
    pub(super) fn stop(&self) {
        self.stopped.store(true, Ordering::Release);
        self.pending.locked().clear();
    }

    /// Reads frames until the stream ends: responses to their waiting request,
    /// notifications to the listener, the server's requests answered.
    pub(super) fn read_loop(&self, mut input: impl BufRead) {
        while let Ok(Some(message)) = framing::read(&mut input) {
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
        let _ = self.send(&reply);
    }

    /// Hands the response `message` to the request `id` waiting for it.
    fn deliver(&self, id: &Value, message: &Value) {
        let Some(id) = id.as_i64() else { return };
        let Some(waiting) = self.pending.locked().remove(&id) else {
            return;
        };
        let reply = match message.get("error") {
            Some(error) => Err(Message::new(messages::ERROR).arg("server", &self.name).arg(
                "error",
                error.get("message").and_then(Value::as_str).unwrap_or(""),
            )),
            None => Ok(message.get("result").cloned().unwrap_or(Value::Null)),
        };
        // The request gave up waiting (it timed out): nobody to tell.
        let _ = waiting.send(reply);
    }
}
