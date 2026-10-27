//! An event, on the wire.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use substrate_dap_domain::{Event, ThreadId, VariablesReference};

use super::change_reason_dto::ChangeReasonDto;
use super::output_category_dto::OutputCategoryDto;
use super::stop_reason_dto::StopReasonDto;

/// What the adapter said without being asked, as the protocol has it and the
/// UI reads it: `{ "event": "stopped", "body": { … } }`, the body's keys
/// camelCase.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "event",
    content = "body",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum EventDto {
    /// The adapter is ready for breakpoints.
    Initialized,
    /// The debuggee stopped.
    Stopped {
        /// Why.
        reason: StopReasonDto,
        /// The thread that stopped — absent when all did.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        thread_id: Option<i64>,
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
        thread_id: i64,
        /// Whether every thread did.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        all_threads_continued: Option<bool>,
    },
    /// The debuggee or the adapter wrote something.
    Output {
        /// Where it belongs.
        #[serde(default)]
        category: OutputCategoryDto,
        /// The text.
        output: String,
        /// Structured output to expand, if any.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        variables_reference: Option<i64>,
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
        reason: ChangeReasonDto,
        /// The breakpoint, as the adapter describes it.
        breakpoint: Value,
    },
    /// A thread started or exited.
    Thread {
        /// Which of the two.
        reason: ChangeReasonDto,
        /// The thread.
        thread_id: i64,
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

impl From<EventDto> for Event {
    fn from(dto: EventDto) -> Self {
        match dto {
            EventDto::Initialized => Self::Initialized,
            EventDto::Stopped {
                reason,
                thread_id,
                all_threads_stopped,
                description,
                text,
                hit_breakpoint_ids,
            } => Self::Stopped {
                reason: reason.into(),
                thread_id: thread_id.map(ThreadId),
                all_threads_stopped,
                description,
                text,
                hit_breakpoint_ids,
            },
            EventDto::Continued {
                thread_id,
                all_threads_continued,
            } => Self::Continued {
                thread_id: ThreadId(thread_id),
                all_threads_continued,
            },
            EventDto::Output {
                category,
                output,
                variables_reference,
            } => Self::Output {
                category: category.into(),
                output,
                variables_reference: variables_reference.map(VariablesReference),
            },
            EventDto::Terminated { restart } => Self::Terminated { restart },
            EventDto::Exited { exit_code } => Self::Exited { exit_code },
            EventDto::Breakpoint { reason, breakpoint } => Self::Breakpoint {
                reason: reason.into(),
                breakpoint,
            },
            EventDto::Thread { reason, thread_id } => Self::Thread {
                reason: reason.into(),
                thread_id: ThreadId(thread_id),
            },
            EventDto::Other { name, body } => Self::Other { name, body },
        }
    }
}

impl From<Event> for EventDto {
    fn from(event: Event) -> Self {
        match event {
            Event::Initialized => Self::Initialized,
            Event::Stopped {
                reason,
                thread_id,
                all_threads_stopped,
                description,
                text,
                hit_breakpoint_ids,
            } => Self::Stopped {
                reason: reason.into(),
                thread_id: thread_id.map(|id| id.0),
                all_threads_stopped,
                description,
                text,
                hit_breakpoint_ids,
            },
            Event::Continued {
                thread_id,
                all_threads_continued,
            } => Self::Continued {
                thread_id: thread_id.0,
                all_threads_continued,
            },
            Event::Output {
                category,
                output,
                variables_reference,
            } => Self::Output {
                category: category.into(),
                output,
                variables_reference: variables_reference.map(|reference| reference.0),
            },
            Event::Terminated { restart } => Self::Terminated { restart },
            Event::Exited { exit_code } => Self::Exited { exit_code },
            Event::Breakpoint { reason, breakpoint } => Self::Breakpoint {
                reason: reason.into(),
                breakpoint,
            },
            Event::Thread { reason, thread_id } => Self::Thread {
                reason: reason.into(),
                thread_id: thread_id.0,
            },
            Event::Other { name, body } => Self::Other { name, body },
        }
    }
}
