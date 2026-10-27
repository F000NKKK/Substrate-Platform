//! What a search covers: the open documents or the whole project.

/// What a search covers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    /// The documents passed in (Current document / All open documents).
    Documents,
    /// Every file under the root, open documents' buffers standing in for their files.
    Project,
}
