//! What an adapter says unasked.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::{ChangeReason, OutputCategory, StopReason, ThreadId, VariablesReference};

/// What the adapter said without being asked. On the wire to the UI it is
/// `{ "event": "stopped", "body": { … } }`, the body's keys camelCase as
/// the protocol has them.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "event",
    content = "body",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum Event {
    /// The adapter is ready for breakpoints: send them, then `configurationDone`.
    Initialized,
    /// The debuggee stopped.
    Stopped {
        /// Why.
        reason: StopReason,
        /// The thread that stopped — absent when all did.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        thread_id: Option<ThreadId>,
        /// Whether every thread stopped.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        all_threads_stopped: Option<bool>,
        /// A longer reason for the user.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Extra detail (the exception's text).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        /// The breakpoints that were hit, by their ids.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        hit_breakpoint_ids: Vec<i64>,
    },
    /// The debuggee runs again.
    Continued {
        /// The thread that continued.
        thread_id: ThreadId,
        /// Whether every thread did.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        all_threads_continued: Option<bool>,
    },
    /// The debuggee or the adapter wrote something.
    Output {
        /// Where it belongs.
        #[serde(default)]
        category: OutputCategory,
        /// The text.
        output: String,
        /// Structured output to expand, if any.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        variables_reference: Option<VariablesReference>,
    },
    /// The debugging is over.
    Terminated {
        /// What to restart it with, if the adapter can.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        restart: Option<Value>,
    },
    /// The debuggee exited.
    Exited {
        /// Its exit code.
        exit_code: i64,
    },
    /// A breakpoint changed.
    Breakpoint {
        /// How.
        reason: ChangeReason,
        /// The breakpoint, as the adapter describes it.
        breakpoint: Value,
    },
    /// A thread started or exited.
    Thread {
        /// Which of the two.
        reason: ChangeReason,
        /// The thread.
        thread_id: ThreadId,
    },
    /// An event this client doesn't know — or one whose body isn't what the
    /// protocol says — kept as it came.
    Other {
        /// The event's name.
        name: String,
        /// Its body, `null` if it had none.
        body: Value,
    },
}

impl Event {
    /// The event `name` with `body` as the adapter sent it. An event that
    /// isn't known, or whose body doesn't fit, is [`Event::Other`] — nothing
    /// the adapter says is lost.
    #[must_use]
    pub fn decode(name: &str, body: Value) -> Self {
        if name == "initialized" {
            return Self::Initialized;
        }
        // An event with no body is an empty one: most of its fields are optional.
        let sent = if body.is_null() {
            json!({})
        } else {
            body.clone()
        };
        let tagged = json!({ "event": name, "body": sent });
        match serde_json::from_value::<Self>(tagged) {
            Ok(event) if !matches!(event, Self::Other { .. } | Self::Initialized) => event,
            _ => Self::Other {
                name: name.to_string(),
                body,
            },
        }
    }
}
