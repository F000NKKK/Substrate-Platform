//! A project folder checked against the workspace.

use std::path::{Path, PathBuf};

use substrate_platform::message::Message;
use substrate_platform::workspace::Workspace;

/// `root`, once it's known to be in the workspace.
///
/// # Errors
///
/// `root` is not in the workspace.
pub(super) fn checked(workspace: &Workspace, root: &str) -> Result<PathBuf, Message> {
    workspace.check(Path::new(root))?;
    Ok(PathBuf::from(root))
}
