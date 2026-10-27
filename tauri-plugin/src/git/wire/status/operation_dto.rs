//! An operation waiting to be finished, on the wire.

use serde::Serialize;
use substrate_git_domain::status::Operation;

/// A merge, rebase, cherry-pick or revert waiting to be finished.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum OperationDto {
    Rebase,
    Merge,
    CherryPick,
    Revert,
}

impl From<Operation> for OperationDto {
    fn from(operation: Operation) -> Self {
        match operation {
            Operation::Rebase => Self::Rebase,
            Operation::Merge => Self::Merge,
            Operation::CherryPick => Self::CherryPick,
            Operation::Revert => Self::Revert,
        }
    }
}
