//! A run against the remote, on the wire.

use serde::Deserialize;
use substrate_git_domain::remotes::RemoteOperation;

/// What a run against the remote does.
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum RemoteOperationDto {
    Fetch,
    Pull,
    Push,
    Sync,
}

impl From<RemoteOperationDto> for RemoteOperation {
    fn from(operation: RemoteOperationDto) -> Self {
        match operation {
            RemoteOperationDto::Fetch => Self::Fetch,
            RemoteOperationDto::Pull => Self::Pull,
            RemoteOperationDto::Push => Self::Push,
            RemoteOperationDto::Sync => Self::Sync,
        }
    }
}
