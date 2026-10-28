//! The port that runs tools.

use std::path::PathBuf;

use super::{ToolCommand, ToolControl, ToolError};

/// Runs rustup and cargo, line by line, paused and stopped with the
/// operation. Blocking. On Unix an adapter runs the tool in a process group
/// of its own, so a pause or a cancel reaches what the tool starts (cargo's
/// rustc).
pub trait ToolRunner: Send + Sync {
    /// Runs `command` to its end, telling `control.output` each line it
    /// prints. Returns where the tool put what it installed (cargo's bin
    /// folder, the installed binary); for a removal the tool's folder, which
    /// the caller ignores.
    ///
    /// # Errors
    ///
    /// [`ToolError`]: cancelled, the tool not available, or the tool failed.
    fn run(&self, command: &ToolCommand, control: &ToolControl<'_>) -> Result<PathBuf, ToolError>;
}
