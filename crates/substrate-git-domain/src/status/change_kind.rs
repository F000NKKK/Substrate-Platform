//! How a file changed.

/// How a file changed, as Git Changes shows it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChangeKind {
    /// Its contents changed.
    Modified,
    /// New to the index.
    Added,
    /// Removed.
    Deleted,
    /// Moved from its original path.
    Renamed,
    /// Copied from its original path.
    Copied,
    /// A file became a link, or the other way round.
    TypeChanged,
    /// Not tracked yet.
    Untracked,
    /// Both sides of a merge changed it.
    Conflicted,
}

impl ChangeKind {
    /// The kind of a porcelain status letter (`M`, `A`, …).
    #[must_use]
    pub fn from_code(code: char) -> Option<Self> {
        Some(match code {
            'M' => Self::Modified,
            'A' => Self::Added,
            'D' => Self::Deleted,
            'R' => Self::Renamed,
            'C' => Self::Copied,
            'T' => Self::TypeChanged,
            'U' => Self::Conflicted,
            _ => return None,
        })
    }
}
