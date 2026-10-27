//! Source control's vocabulary: what a repository looks like to an IDE
//! (status, changes, branches, commits, stashes, diffs) and why an
//! operation fails ([`GitError`]). No I/O: the ports are in
//! `substrate-git-app`, the `git` executable behind them in
//! `substrate-git-gitcli`.
#![doc = include_str!("../README.md")]

pub mod branches;
pub mod commits;
pub mod diffs;
mod git_error;
mod git_error_message;
pub mod messages;
pub mod remotes;
pub mod settings;
pub mod stashes;
pub mod status;

pub use git_error::GitError;
