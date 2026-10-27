//! Why a pty session failed.

/// What the pty library reported.
type Source = Box<dyn std::error::Error + Send + Sync>;

/// Why a pty session failed.
#[derive(Debug, thiserror::Error)]
pub enum PtyError {
    /// The pty could not be opened.
    #[error("{0}")]
    Open(Source),
    /// The shell could not be started.
    #[error("{0}")]
    Spawn(Source),
    /// The pty's reader or writer could not be taken.
    #[error("{0}")]
    Stream(Source),
    /// The pty refused its new size.
    #[error("{0}")]
    Resize(Source),
}
