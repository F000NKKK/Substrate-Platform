//! Why a git operation failed.

use std::path::PathBuf;

/// Why a git operation failed — what the UI says is [`crate::message::Message::from`].
#[derive(Debug, thiserror::Error)]
pub enum GitError {
    /// The `git` executable isn't there.
    #[error("git isn't installed, or isn't on PATH")]
    Missing,
    /// `git` is there but didn't start.
    #[error("couldn't start git: {0}")]
    CantStart(#[source] std::io::Error),
    /// The remote asked for credentials nobody can type.
    #[error("the remote asked for credentials, and this app can't prompt for them")]
    AuthFailed,
    /// Git doesn't know who commits.
    #[error("git doesn't know who you are yet")]
    NoIdentity,
    /// The folder isn't in a repository.
    #[error("this folder isn't in a git repository")]
    NotARepository,
    /// A push with no remote to go to.
    #[error("this repository has no remote to push to")]
    NoPushRemote,
    /// A push of a branch without upstream, with no `origin`.
    #[error("this branch has no upstream, and there's no \"origin\" remote to push it to")]
    NoOrigin,
    /// A push of a detached HEAD.
    #[error("a detached HEAD can't be pushed")]
    DetachedPush,
    /// A pull of a branch without upstream.
    #[error("the current branch has no upstream branch to pull from")]
    NoUpstream,
    /// Continue or abort with nothing to continue or abort.
    #[error("no merge or rebase is in progress")]
    NothingInProgress,
    /// A clone into a folder with something in it.
    #[error("{} already exists and isn't empty", .path.display())]
    CloneTargetNotEmpty {
        /// The folder.
        path: PathBuf,
    },
    /// A global setting this app doesn't set.
    #[error("{key} can't be set from here")]
    NotSettable {
        /// The setting.
        key: String,
    },
    /// Git's own words about what went wrong.
    #[error("{text}")]
    Failed {
        /// What git printed.
        text: String,
    },
    /// Talking to the process failed.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
