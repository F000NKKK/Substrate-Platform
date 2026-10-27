//! The port that says which shells the machine has.

/// What a machine says about its shells. Implemented by
/// `substrate-terminal-pty`; tests use a fake.
pub trait ShellProbe: Send + Sync {
    /// Whether `program` can be started from the search path.
    fn is_installed(&self, program: &str) -> bool;

    /// The user's own shell on this system.
    fn default_shell(&self) -> String;
}
