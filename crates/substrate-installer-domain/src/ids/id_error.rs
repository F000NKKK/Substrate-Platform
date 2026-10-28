//! Why an id is refused.

use super::IdKind;

/// Why an id was refused.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum IdError {
    /// The id is empty.
    #[error("a {kind} id is empty")]
    Empty {
        /// The kind being made.
        kind: IdKind,
    },
    /// The id is longer than ids may be.
    #[error("the {kind} id `{text}` is longer than {max} characters")]
    TooLong {
        /// The kind being made.
        kind: IdKind,
        /// The refused text.
        text: String,
        /// The longest an id may be.
        max: usize,
    },
    /// The id has a character ids do not use.
    #[error(
        "the {kind} id `{text}` has `{found}` at {at}; ids use lowercase letters, digits, `.`, `-` and `_`"
    )]
    InvalidCharacter {
        /// The kind being made.
        kind: IdKind,
        /// The refused text.
        text: String,
        /// The character.
        found: char,
        /// Its position, in characters.
        at: usize,
    },
    /// The id does not start and end with a letter or a digit.
    #[error("the {kind} id `{text}` must start and end with a letter or a digit")]
    BadEdge {
        /// The kind being made.
        kind: IdKind,
        /// The refused text.
        text: String,
    },
}
