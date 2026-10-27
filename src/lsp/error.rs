//! Why talking to a language server failed.

/// Why talking to a language server failed — what the UI says is
/// [`crate::message::Message::from`].
#[derive(Debug, thiserror::Error)]
pub enum LspError {
    /// The product's list of languages is not valid TOML of the shape.
    #[error("the list of languages is invalid: {0}")]
    InvalidList(#[source] toml::de::Error),
    /// The language has no server.
    #[error("there's no language server for {language}")]
    NoServer {
        /// The language's id.
        language: String,
    },
    /// The server's process did not start.
    #[error("couldn't start the language server {server}: {source}")]
    CantStart {
        /// The server's name.
        server: String,
        /// Why.
        #[source]
        source: std::io::Error,
    },
    /// Writing to the server failed.
    #[error("the language server {server} failed: {source}")]
    Failed {
        /// The server's name.
        server: String,
        /// Why.
        #[source]
        source: std::io::Error,
    },
    /// The server's stream has ended.
    #[error("the language server {server} has stopped")]
    Stopped {
        /// The server's name.
        server: String,
    },
    /// The server did not answer in time.
    #[error("{server} didn't answer {method} in time")]
    TimedOut {
        /// The server's name.
        server: String,
        /// The request's method.
        method: String,
    },
    /// The server answered a request with an error.
    #[error("{server} couldn't do {method}: {reason}")]
    Rejected {
        /// The server's name.
        server: String,
        /// The request's method.
        method: String,
        /// The server's words.
        reason: String,
    },
}

impl LspError {
    /// This error as the answer to the request `method`.
    pub(super) fn answering(self, method: &str) -> Self {
        match self {
            Self::Rejected { server, reason, .. } => Self::Rejected {
                server,
                method: method.to_string(),
                reason,
            },
            other => other,
        }
    }
}
