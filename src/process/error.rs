//! Why opening a link, a file or a program failed.

/// Why opening a link, a file or a program failed — what the UI says is
/// [`substrate_message::Message::from`].
#[derive(Debug, thiserror::Error)]
pub enum ProcessError {
    /// Only web and mail links are opened.
    #[error("{url} isn't a web or mail link")]
    NotALink {
        /// What was asked to open.
        url: String,
    },
    /// The program did not start.
    #[error("couldn't start {program}: {source}")]
    CantStart {
        /// The program.
        program: String,
        /// Why.
        #[source]
        source: std::io::Error,
    },
}
