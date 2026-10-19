//! Source control — the backend of Visual Studio's Git Changes and Git
//! Repository windows. Everything goes through the `git` executable rather
//! than a library, so the user's own configuration applies as is: identity,
//! hooks, `pull.rebase`, credential helpers, SSH keys.
//!
//! Git never gets a terminal here: `GIT_TERMINAL_PROMPT=0` makes a remote
//! that needs a password fail instead of waiting for input nobody can type
//! (a credential helper still answers), and `GIT_EDITOR=true` keeps
//! `rebase --continue` and friends from opening an editor. Fetch, pull,
//! push, merge and rebase stream what git prints to an [`Output`];
//! everything else answers with parsed, machine-readable output
//! (`--porcelain=v2 -z` and friends).
//!
//! Every function is blocking and takes the folder a project is in (`root`);
//! a host runs them off its UI thread.

pub mod branch;
pub mod config;
pub mod diff;
pub mod history;
pub mod messages;
pub mod remote;
mod run;
pub mod stash;
pub mod status;

pub use run::{Output, absolute, repo_root};
