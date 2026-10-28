//! A command line.

use substrate_installer_app::ports::tools::ToolProgram;

/// The command line of a [`ToolCommand`]: the program and its arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Invocation {
    pub(crate) program: ToolProgram,
    pub(crate) arguments: Vec<String>,
}
