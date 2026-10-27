//! What an adapter says unasked.

use serde_json::Value;

use crate::{ChangeReason, OutputCategory, StopReason, ThreadId, VariablesReference};

/// What the adapter said without being asked.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// The adapter is ready for breakpoints: send them, then `configurationDone`.
    Initialized,
    /// The debuggee stopped.
    Stopped {
        /// Why.
        reason: StopReason,
        /// The thread that stopped — absent when all did.
        thread_id: Option<ThreadId>,
        /// Whether every thread stopped.
        all_threads_stopped: Option<bool>,
        /// A longer reason for the user.
        description: Option<String>,
        /// Extra detail (the exception's text).
        text: Option<String>,
        /// The breakpoints that were hit, by their ids.
        hit_breakpoint_ids: Vec<i64>,
    },
    /// The debuggee runs again.
    Continued {
        /// The thread that continued.
        thread_id: ThreadId,
        /// Whether every thread did.
        all_threads_continued: Option<bool>,
    },
    /// The debuggee or the adapter wrote something.
    Output {
        /// Where it belongs.
        category: OutputCategory,
        /// The text.
        output: String,
        /// Structured output to expand, if any.
        variables_reference: Option<VariablesReference>,
    },
    /// The debugging is over.
    Terminated {
        /// What to restart it with, if the adapter can.
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
