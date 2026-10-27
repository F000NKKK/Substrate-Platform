//! The port that starts processes.

use substrate_run_domain::Task;

use super::Launched;

/// Starts a [`Task`]'s process with stdout and stderr piped. Implemented by
/// `substrate-run-process`; tests use a fake.
pub trait ProcessLauncher: Send + Sync {
    /// Starts `task`'s program in its own process group.
    ///
    /// # Errors
    ///
    /// The error of a program that can't be started.
    fn launch(&self, task: &Task) -> std::io::Result<Launched>;
}
