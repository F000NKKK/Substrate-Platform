//! Why a solution or project could not be opened, saved or ordered.

use std::path::PathBuf;

use crate::message::Message;

/// Why a solution or project could not be opened, saved or ordered — what
/// the UI says is [`Message::from`].
#[derive(Debug, thiserror::Error)]
pub enum ProjectError {
    /// No registered format reads the file.
    #[error("no format this app knows reads {}", .path.display())]
    NoFormat {
        /// The file.
        path: PathBuf,
    },
    /// A project that depends on itself.
    #[error("{project} depends on itself through its build dependencies")]
    DependencyCycle {
        /// The project's name.
        project: String,
    },
    /// A dependency the solution lacks.
    #[error("{project} depends on {dependency}, which isn't in the solution")]
    UnknownDependency {
        /// The project's name.
        project: String,
        /// The missing project's id.
        dependency: String,
    },
    /// The format refused, in the product's own words.
    #[error("{0}")]
    Format(Message),
}
