//! Both sides of a diff, as text.

/// Both sides of a file's diff.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DiffTexts {
    /// The left side's text; `None` when the file isn't there.
    pub left: Option<String>,
    /// The right side's text; `None` when the file isn't there.
    pub right: Option<String>,
    /// Either side isn't text (or is too large to show).
    pub binary: bool,
}
