//! What's changed in a working tree and the index.

mod change_kind;
mod discard_item;
mod file_change;
mod operation;
mod repo_path;
mod repo_status;

pub use change_kind::ChangeKind;
pub use discard_item::DiscardItem;
pub use file_change::FileChange;
pub use operation::Operation;
pub use repo_path::absolute;
#[expect(
    clippy::module_name_repetitions,
    reason = "`RepoStatus` is the whole repository's status; `Status` alone would read as one file's"
)]
pub use repo_status::RepoStatus;
