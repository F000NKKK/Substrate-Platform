//! The port that starts processes to talk to.

use substrate_run_domain::Task;

use super::Service;

/// Starts a [`Task`]'s program with all three standard streams piped, to be
/// talked to over them. Implemented by `substrate-run-process`; tests use a
/// fake.
pub trait ServiceLauncher: Send + Sync {
    /// Starts `task`'s program in its own process group, so stopping it takes
    /// down what it started too.
    ///
    /// # Errors
    ///
    /// The error of a program that can't be started, or whose streams can't be piped.
    fn launch_service(&self, task: &Task) -> std::io::Result<Service>;
}
