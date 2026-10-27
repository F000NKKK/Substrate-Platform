//! One side of a diff, on the wire.

use serde::Deserialize;
use substrate_git_domain::diffs::{DiffSide, SideVersion};

/// What `rev` says for what's staged.
const INDEX: &str = "INDEX";
/// What `rev` says for the file on disk.
const WORKTREE: &str = "WORKTREE";

/// One side of a diff: a revision (or `INDEX`, `WORKTREE`, empty) and a path.
#[derive(Deserialize)]
pub(crate) struct DiffSideDto {
    rev: String,
    path: String,
}

/// The version `rev` names, parsed once.
fn parse_version(rev: String) -> SideVersion {
    match rev.as_str() {
        "" => SideVersion::Nothing,
        INDEX => SideVersion::Index,
        WORKTREE => SideVersion::Worktree,
        _ => SideVersion::Revision(rev),
    }
}

impl From<DiffSideDto> for DiffSide {
    fn from(side: DiffSideDto) -> Self {
        Self {
            version: parse_version(side.rev),
            path: side.path,
        }
    }
}
