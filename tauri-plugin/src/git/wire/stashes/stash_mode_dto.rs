//! What a stash takes, on the wire.

use serde::Deserialize;
use substrate_git_domain::stashes::StashMode;

/// What a new stash takes.
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum StashModeDto {
    All,
    KeepIndex,
    Staged,
}

impl From<StashModeDto> for StashMode {
    fn from(mode: StashModeDto) -> Self {
        match mode {
            StashModeDto::All => Self::All,
            StashModeDto::KeepIndex => Self::KeepIndex,
            StashModeDto::Staged => Self::Staged,
        }
    }
}
