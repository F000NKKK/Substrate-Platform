//! Why rustup could not be fetched.

use substrate_installer_app::ports::Cause;
use substrate_installer_domain::platforms::PlatformTag;
use substrate_installer_domain::texts::TextError;

/// Why rustup could not be fetched; the `cause` of a
/// [`ToolError::Unavailable`] for rustup.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum BootstrapError {
    /// The machine has no rustup and the runner was given no way to fetch it.
    #[error("rustup is not installed and no bootstrap is configured")]
    NotConfigured,
    /// Rust publishes no `rustup-init` for the platform.
    #[error("rustup has no installer for {platform}")]
    UnsupportedPlatform {
        /// The platform.
        platform: PlatformTag,
    },
    /// The link of `rustup-init` is not one.
    #[error("the rustup-init link is not valid")]
    Link(#[from] TextError),
    /// The download failed.
    #[error("rustup-init could not be downloaded")]
    Fetch(#[source] Cause),
    /// The scratch folder or the downloaded file could not be prepared.
    #[error("rustup-init could not be prepared")]
    Prepare(#[source] std::io::Error),
}
