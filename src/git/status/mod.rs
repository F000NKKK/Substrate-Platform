//! What's changed — status, staging, undo, commit.

mod change_kind;
mod commit;
mod commit_mode;
mod commit_result;
mod commit_scope;
mod discard;
mod discard_item;
mod file_change;
mod operation;
mod porcelain;
mod repo_status;
mod staging;

pub use change_kind::ChangeKind;
pub use commit::{commit, last_message};
pub use commit_mode::CommitMode;
pub use commit_result::CommitResult;
pub use commit_scope::CommitScope;
pub use discard::discard;
pub use discard_item::DiscardItem;
pub use file_change::FileChange;
pub use operation::Operation;
pub use porcelain::{parse, read};
pub use repo_status::RepoStatus;
pub use staging::{stage, unstage};

#[cfg(test)]
mod tests;
