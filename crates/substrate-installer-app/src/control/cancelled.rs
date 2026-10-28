//! The user stopped the operation.

/// The user cancelled the operation: why a step stopped, as opposed to
/// something failing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("cancelled")]
pub struct Cancelled;
