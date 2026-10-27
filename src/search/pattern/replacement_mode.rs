//! How Replace in Files reads its replacement text.

/// How a replacement text is read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReplacementMode {
    /// As it is.
    Literal,
    /// With `$1`/`${name}` expanded from the match's groups (regex mode).
    Expand,
}
