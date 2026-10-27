//! One adapter's connection: writing frames, and the reader that matches
//! responses to their requests.

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::{Mutex, mpsc};

use serde_json::{Value, json};
use substrate_dap_domain::{Command, DapError, Event, Seq};
use substrate_framing::{read_frame, write_frame};
use substrate_lock::Locked;
use substrate_run_app::ports::ChildProcess;

use crate::wire::decode_event;

/// What a request gets back.
pub(crate) type Reply = Result<Value, DapError>;
/// Who hears what the adapter says unasked.
pub(crate) type OnEvent = Box<dyn Fn(Event) + Send + Sync>;

/// One adapter's connection: what the client's clones and its reader share.
pub(crate) struct Connection {
    pub(crate) name: String,
    writer: Mutex<Box<dyn Write + Send>>,
    next_seq: AtomicI64,
    pub(crate) pending: Mutex<HashMap<Seq, mpsc::Sender<Reply>>>,
    pub(crate) stopped: AtomicBool,
    on_event: OnEvent,
    pub(crate) child: Mutex<Option<Box<dyn ChildProcess>>>,
}

impl Connection {
    /// A connection writing to `writer`, telling `on_event` what the adapter
    /// says unasked; no process attached yet.
    pub(crate) fn new(name: String, writer: Box<dyn Write + Send>, on_event: OnEvent) -> Self {
        Self {
            name,
            writer: Mutex::new(writer),
            next_seq: AtomicI64::new(Seq::FIRST.0),
            pending: Mutex::new(HashMap::new()),
            stopped: AtomicBool::new(false),
            on_event,
            child: Mutex::new(None),
        }
    }

    /// The number of the next message this side sends.
    pub(crate) fn next_seq(&self) -> Seq {
        Seq(self.next_seq.fetch_add(1, Ordering::Relaxed))
    }

    /// Sends one message, unless the stream has ended.
    pub(crate) fn send(&self, message: &Value) -> Result<(), DapError> {
        if self.stopped.load(Ordering::Acquire) {
            return Err(self.stopped_error());
        }
        write_frame(&mut *self.writer.locked(), message).map_err(|source| DapError::Failed {
            adapter: self.name.clone(),
            source,
        })
    }

    /// The error for an adapter no longer running.
    pub(crate) fn stopped_error(&self) -> DapError {
        DapError::Stopped {
            adapter: self.name.clone(),
        }
    }

    /// The stream ended: every request still waiting fails.
    pub(crate) fn stop(&self) {
        self.stopped.store(true, Ordering::Release);
        self.pending.locked().clear();
    }

    /// Reads frames until the stream ends: responses to their waiting
    /// request, events to the listener, the adapter's requests answered.
    pub(crate) fn read_loop(&self, mut input: impl BufRead) {
        while let Ok(Some(message)) = read_frame(&mut input) {
            match message.get("type").and_then(Value::as_str) {
                Some("response") => self.deliver(&message),
                Some("event") => (self.on_event)(Self::event(&message)),
                Some("request") => self.refuse(&message),
                _ => {}
            }
        }
        self.stop();
    }

    /// The event `message` is.
    fn event(message: &Value) -> Event {
        let name = message
            .get("event")
            .and_then(Value::as_str)
            .unwrap_or_default();
        decode_event(name, message.get("body").cloned().unwrap_or(Value::Null))
    }

    /// Answers the adapter's request: this client does none of them (a
    /// `runInTerminal` is only asked of a client that said it can).
    fn refuse(&self, request: &Value) {
        let command = request
            .get("command")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let reply = json!({
            "seq": self.next_seq().0,
            "type": "response",
            "request_seq": request.get("seq").cloned().unwrap_or(Value::Null),
            "success": false,
            "command": command,
            "message": format!("{command} isn't supported"),
        });
        // A failed send means the stream ended, which the read loop sees next.
        self.send(&reply).ok();
    }

    /// Hands the response `message` to the request waiting for it.
    fn deliver(&self, message: &Value) {
        let Some(request_seq) = message.get("request_seq").and_then(Value::as_i64) else {
            return;
        };
        let Some(waiting) = self.pending.locked().remove(&Seq(request_seq)) else {
            return;
        };
        let succeeded = message.get("success").and_then(Value::as_bool) == Some(true);
        let reply = if succeeded {
            Ok(message.get("body").cloned().unwrap_or(Value::Null))
        } else {
            Err(DapError::Rejected {
                adapter: self.name.clone(),
                command: Command::from(
                    message
                        .get("command")
                        .and_then(Value::as_str)
                        .unwrap_or_default(),
                ),
                reason: Self::reason(message),
            })
        };
        // The request gave up waiting (it timed out): nobody to tell.
        waiting.send(reply).ok();
    }

    /// The words of a failed response: its `message`, else the
    /// `body.error.format` some adapters put there.
    fn reason(message: &Value) -> String {
        message
            .get("message")
            .or_else(|| message.pointer("/body/error/format"))
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string()
    }
}
