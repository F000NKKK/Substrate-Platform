//! Source Control's commands: each checks its folder against the
//! [`Workspace`] and runs [`substrate_platform::git`] off the UI thread; long
//! runs stream to the UI as [`events::GIT_OUTPUT`]. Feature `git`.

pub(crate) mod changes;
pub(crate) mod history;
pub(crate) mod remote;
pub(crate) mod stash;

use std::path::{Path, PathBuf};
use substrate_platform::events;
use substrate_platform::message::Message;
use substrate_platform::workspace::Workspace;
use tauri::{AppHandle, Emitter, Runtime};

/// `root`, once it's known to be in the workspace.
fn checked(workspace: &Workspace, root: &str) -> Result<PathBuf, Message> {
    workspace.check(Path::new(root))?;
    Ok(PathBuf::from(root))
}

/// A long run's lines, to the UI.
fn output<R: Runtime>(app: &AppHandle<R>) -> impl Fn(Message) + Sync + '_ {
    move |line| {
        let _ = app.emit(events::GIT_OUTPUT, line);
    }
}
