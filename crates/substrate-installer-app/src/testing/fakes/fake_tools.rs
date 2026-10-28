//! A tool runner that runs nothing.

use std::path::PathBuf;

use crate::ports::tools::{ToolCommand, ToolControl, ToolError, ToolRunner};
use crate::testing::{Call, Journal};

/// Records the command, prints one line, and puts everything in `/cargo/bin`.
#[derive(Debug)]
pub(crate) struct FakeTools {
    pub(crate) journal: Journal,
    pub(crate) failing: bool,
}

impl FakeTools {
    pub(crate) fn new(journal: Journal) -> Self {
        Self {
            journal,
            failing: false,
        }
    }
}

impl ToolRunner for FakeTools {
    fn run(&self, command: &ToolCommand, control: &ToolControl<'_>) -> Result<PathBuf, ToolError> {
        control.cancel.check()?;
        self.journal.record(Call::Tool(command.clone()));
        (control.output)("running");
        if self.failing {
            return Err(ToolError::Failed {
                program: command.program(),
                code: Some(101),
            });
        }
        Ok(PathBuf::from("/cargo/bin"))
    }
}
