//! Why a tool did not do what it was asked.

use super::ToolProgram;
use crate::control::Cancelled;
use crate::ports::Cause;

/// Why a tool did not do what it was asked.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ToolError {
    /// The operation was cancelled; the tool was stopped.
    #[error("cancelled")]
    Cancelled,
    /// The tool could not be started or fetched.
    #[error("{program} could not be started")]
    Unavailable {
        /// The program.
        program: ToolProgram,
        /// What went wrong.
        #[source]
        cause: Cause,
    },
    /// The tool ran and failed.
    #[error("{program} failed{}", exit_text(*.code))]
    Failed {
        /// The program.
        program: ToolProgram,
        /// Its exit code, none when a signal ended it.
        code: Option<i32>,
    },
}

fn exit_text(code: Option<i32>) -> String {
    code.map_or_else(String::new, |code| format!(" with exit code {code}"))
}

impl From<Cancelled> for ToolError {
    fn from(_: Cancelled) -> Self {
        Self::Cancelled
    }
}
