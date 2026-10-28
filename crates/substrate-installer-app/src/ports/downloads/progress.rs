//! How far a download has got.

/// How far a transfer has got.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Progress {
    /// Bytes so far.
    pub done: u64,
    /// Bytes in all, where the server says.
    pub total: Option<u64>,
}
