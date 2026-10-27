//! Running `git`: set up for a GUI host, its failures made actionable, its
//! long runs streamed; and finding the repository a folder is in.

mod command;
mod failure;
mod repo;
mod run;
mod stream;

pub(crate) use command::git;
pub use repo::repo_root;
pub(crate) use repo::{current_branch, has_head, has_upstream, repo_of, require_repo};
pub(crate) use run::{run, run_with_input};
pub(crate) use stream::{stream, stream_git};
