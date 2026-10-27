//! A branch, on the wire.

use serde::Serialize;
use substrate_git_domain::branches::Branch;

/// A local or remote-tracking branch with its tip.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[expect(
    clippy::struct_excessive_bools,
    reason = "the UI reads remote, current and gone as three separate flags of a branch"
)]
pub(crate) struct BranchDto {
    name: String,
    remote: bool,
    current: bool,
    upstream: Option<String>,
    ahead: u32,
    behind: u32,
    gone: bool,
    hash: String,
    date: i64,
    subject: String,
}

impl From<Branch> for BranchDto {
    fn from(branch: Branch) -> Self {
        Self {
            name: branch.name,
            remote: branch.remote,
            current: branch.current,
            upstream: branch.upstream,
            ahead: branch.ahead,
            behind: branch.behind,
            gone: branch.gone,
            hash: branch.hash,
            date: branch.date,
            subject: branch.subject,
        }
    }
}
