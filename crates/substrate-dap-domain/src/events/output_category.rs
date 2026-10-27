//! Where an `output` event's text belongs.

/// Where an `output` event's text belongs.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum OutputCategory {
    /// The debug console — the default.
    #[default]
    Console,
    /// Something the user should see at once.
    Important,
    /// The debuggee's standard output.
    Stdout,
    /// The debuggee's standard error.
    Stderr,
    /// Telemetry, not for the user.
    Telemetry,
    /// A category this client doesn't know, as the adapter said it.
    Other(String),
}
