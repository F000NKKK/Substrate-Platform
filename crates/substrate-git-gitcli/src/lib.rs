//! Source control through the `git` executable rather than a library, so
//! the user's own configuration applies as is: identity, hooks,
//! `pull.rebase`, credential helpers, SSH keys. [`GitCli`] implements every
//! port of `substrate-git-app`.
//!
//! Git never gets a terminal here: `GIT_TERMINAL_PROMPT=0` makes a remote
//! that needs a password fail instead of waiting for input nobody can type
//! (a credential helper still answers), and `GIT_EDITOR=true` keeps
//! `rebase --continue` and friends from opening an editor. Fetch, pull,
//! push, merge and rebase stream what git prints to an
//! [`Output`](substrate_git_app::Output); everything else answers with
//! parsed, machine-readable output (`--porcelain=v2 -z` and friends), turned
//! into domain types in [`porcelain`] and nowhere else.
//!
//! Every function is blocking and takes the folder a project is in (`root`);
//! a host runs them off its UI thread.
#![doc = include_str!("../README.md")]

mod branches;
mod cli;
mod diff_text;
mod diffs;
mod git_cli;
mod history;
mod operation;
pub mod porcelain;
mod remotes;
mod settings;
mod stashes;
mod working_tree;

pub use cli::repo_root;
pub use git_cli::GitCli;
