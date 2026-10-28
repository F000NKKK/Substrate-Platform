//! Why a platform tag is refused.

/// Why a platform tag is refused.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum PlatformError {
    /// The operating system is not one of [`Os`](super::Os).
    #[error("`{text}` is not an operating system (linux, macos, windows)")]
    UnknownOs {
        /// The refused text.
        text: String,
    },
    /// The architecture is not one of [`Arch`](super::Arch).
    #[error("`{text}` is not an architecture (x86_64, aarch64)")]
    UnknownArch {
        /// The refused text.
        text: String,
    },
    /// The tag is not `os-arch`.
    #[error("`{text}` is not a platform tag like `linux-x86_64`")]
    Malformed {
        /// The refused text.
        text: String,
    },
}
