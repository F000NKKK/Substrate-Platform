//! What is done with a stash, on the wire.

use serde::Deserialize;
use substrate_git_domain::stashes::StashOperation;

/// What is done with an existing stash.
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum StashOperationDto {
    Apply,
    Pop,
    Drop,
}

impl From<StashOperationDto> for StashOperation {
    fn from(operation: StashOperationDto) -> Self {
        match operation {
            StashOperationDto::Apply => Self::Apply,
            StashOperationDto::Pop => Self::Pop,
            StashOperationDto::Drop => Self::Drop,
        }
    }
}
