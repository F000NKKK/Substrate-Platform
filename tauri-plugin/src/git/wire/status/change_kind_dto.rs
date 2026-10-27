//! How a file changed, on the wire.

use serde::{Deserialize, Serialize};
use substrate_git_domain::status::ChangeKind;

/// How a file changed.
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ChangeKindDto {
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    TypeChanged,
    Untracked,
    Conflicted,
}

impl From<ChangeKind> for ChangeKindDto {
    fn from(kind: ChangeKind) -> Self {
        match kind {
            ChangeKind::Modified => Self::Modified,
            ChangeKind::Added => Self::Added,
            ChangeKind::Deleted => Self::Deleted,
            ChangeKind::Renamed => Self::Renamed,
            ChangeKind::Copied => Self::Copied,
            ChangeKind::TypeChanged => Self::TypeChanged,
            ChangeKind::Untracked => Self::Untracked,
            ChangeKind::Conflicted => Self::Conflicted,
        }
    }
}

impl From<ChangeKindDto> for ChangeKind {
    fn from(kind: ChangeKindDto) -> Self {
        match kind {
            ChangeKindDto::Modified => Self::Modified,
            ChangeKindDto::Added => Self::Added,
            ChangeKindDto::Deleted => Self::Deleted,
            ChangeKindDto::Renamed => Self::Renamed,
            ChangeKindDto::Copied => Self::Copied,
            ChangeKindDto::TypeChanged => Self::TypeChanged,
            ChangeKindDto::Untracked => Self::Untracked,
            ChangeKindDto::Conflicted => Self::Conflicted,
        }
    }
}
