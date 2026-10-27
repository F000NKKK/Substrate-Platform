//! How a workflow run ended.

/// How a whole workflow run ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// Every step succeeded.
    Succeeded,
    /// A step failed (or the workflow couldn't run at all) — see the log.
    Failed,
    /// [`super::Handle::cancel`] stopped it.
    Cancelled,
}

impl Outcome {
    /// Whether every step succeeded.
    #[must_use]
    pub fn is_success(self) -> bool {
        self == Outcome::Succeeded
    }
}
