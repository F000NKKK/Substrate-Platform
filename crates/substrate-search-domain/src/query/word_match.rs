//! Whole words or any text.

/// Whether a match must stand as a word.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WordMatch {
    /// Any text.
    #[default]
    Anywhere,
    /// Only matches with no word character next to their word-character edges.
    WholeWord,
}
