//! Why talking to a debug adapter failed.

use super::{AdapterId, Command};

/// Why talking to a debug adapter failed — what the UI says is
/// [`crate::message::Message::from`].
#[derive(Debug, thiserror::Error)]
pub enum DapError {
    /// The product's list of adapters is not valid TOML of the shape.
    #[error("the list of debug adapters is invalid: {0}")]
    InvalidList(#[source] toml::de::Error),
    /// The list has no such adapter.
    #[error("there's no debug adapter {adapter}")]
    NoAdapter {
        /// The adapter's id.
        adapter: AdapterId,
    },
    /// The adapter's process did not start.
    #[error("couldn't start the debug adapter {adapter}: {source}")]
    CantStart {
        /// The adapter's name.
        adapter: String,
        /// Why.
        #[source]
        source: std::io::Error,
    },
    /// Writing to the adapter failed.
    #[error("the debug adapter {adapter} failed: {source}")]
    Failed {
        /// The adapter's name.
        adapter: String,
        /// Why.
        #[source]
        source: std::io::Error,
    },
    /// The adapter's stream has ended.
    #[error("the debug adapter {adapter} has stopped")]
    Stopped {
        /// The adapter's name.
        adapter: String,
    },
    /// The adapter did not answer in time.
    #[error("{adapter} didn't answer {command} in time")]
    TimedOut {
        /// The adapter's name.
        adapter: String,
        /// The request's command.
        command: Command,
    },
    /// The adapter answered a request with an error.
    #[error("{adapter} couldn't do {command}: {reason}")]
    Rejected {
        /// The adapter's name.
        adapter: String,
        /// The request's command.
        command: Command,
        /// The adapter's words.
        reason: String,
    },
}
