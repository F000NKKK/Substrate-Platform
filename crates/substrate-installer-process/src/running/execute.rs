//! Running a tool to its end and judging how it ended.

use std::path::Path;

use substrate_installer_app::ports::tools::{ToolControl, ToolError, ToolProgram};

use super::ended::Ended;
use super::supervise::supervise;

/// Runs `program` (found at `path`) with `arguments` to a successful end.
///
/// # Errors
///
/// [`ToolError::Cancelled`] once the token is cancelled,
/// [`ToolError::Unavailable`] when the program cannot be started, and
/// [`ToolError::Failed`] with its exit code when it fails.
pub(crate) fn execute(
    program: ToolProgram,
    path: &Path,
    arguments: &[String],
    control: &ToolControl<'_>,
) -> Result<(), ToolError> {
    match supervise(path, arguments, control) {
        Ok(Ended::Exited(status)) if status.success() => Ok(()),
        Ok(Ended::Exited(status)) => Err(ToolError::Failed {
            program,
            code: status.code(),
        }),
        Ok(Ended::Cancelled) => Err(ToolError::Cancelled),
        Err(error) => Err(ToolError::Unavailable {
            program,
            cause: Box::new(error),
        }),
    }
}
