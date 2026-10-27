//! Source Control's commands: each checks its folder against the
//! [`Workspace`] and runs [`substrate_platform::git`] off the UI thread; long
//! runs stream to the UI as [`events::GIT_OUTPUT`]. Feature `git`.

pub(crate) mod changes;
mod checked;
pub(crate) mod history;
mod output;
pub(crate) mod remote;
pub(crate) mod stash;

use checked::checked;
use output::output;
