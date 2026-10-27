//! How a file's bytes become text.

/// How a file's bytes become text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decoding {
    /// Invalid UTF-8 becomes U+FFFD (a search only reads it).
    Lossy,
    /// Invalid UTF-8 is no text at all (a replace mustn't mangle it).
    Strict,
}
