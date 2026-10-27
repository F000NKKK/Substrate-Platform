//! What the pattern's text means.

/// What the pattern's text means.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PatternSyntax {
    /// The text as it is.
    #[default]
    Literal,
    /// A regular expression.
    Regex,
}
