//! How a supervised program ended.

use std::process::ExitStatus;

/// How a supervised program ended.
#[derive(Debug)]
pub(super) enum Ended {
    /// It ran to its end.
    Exited(ExitStatus),
    /// The operation was cancelled and the program's tree killed.
    Cancelled,
}
