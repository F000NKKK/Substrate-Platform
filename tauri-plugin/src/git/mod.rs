//! Source Control's commands: each checks its folder against the
//! [`Workspace`](substrate_workspace_app::Workspace) and runs the
//! `substrate-git-*` crates off the UI thread; long runs stream to the UI as
//! [`events::GIT_OUTPUT`](substrate_host_contract::events::GIT_OUTPUT).
//! Feature `git`.

pub(crate) mod changes;
mod checked;
pub(crate) mod history;
mod output;
pub(crate) mod remote;
mod source_control;
pub(crate) mod stash;
pub(crate) mod wire;

use checked::checked;
use output::output;
pub(crate) use source_control::SourceControl;
