//! Why a validated text is refused.

use super::TextKind;

/// Why a text was refused.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum TextError {
    /// The text is empty.
    #[error("the {kind} is empty")]
    Empty {
        /// What was being made.
        kind: TextKind,
    },
    /// The text has a space or a control character where none may be.
    #[error("the {kind} `{text}` must not contain whitespace or control characters")]
    Whitespace {
        /// What was being made.
        kind: TextKind,
        /// The refused text.
        text: String,
    },
    /// The text is not shaped the way its kind is.
    #[error("the {kind} `{text}` is malformed")]
    Malformed {
        /// What was being made.
        kind: TextKind,
        /// The refused text.
        text: String,
    },
}
