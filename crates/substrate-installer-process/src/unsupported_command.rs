//! A command with no command line.

/// A command this adapter has no command line for (the port's command enum
/// is `non_exhaustive`).
#[derive(Debug, thiserror::Error)]
#[error("this command has no command line in this adapter")]
pub(crate) struct UnsupportedCommand;
