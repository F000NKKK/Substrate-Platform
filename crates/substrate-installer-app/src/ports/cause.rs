//! What an adapter's failure carries.

/// The error an adapter hit, kept as the source of the port's error.
pub type Cause = Box<dyn std::error::Error + Send + Sync>;
