//! The port that downloads `rustup-init`.

use std::path::Path;

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::Cause;
use substrate_installer_domain::releases::HttpUrl;

/// Downloads `rustup-init` for the bootstrap. An HTTP adapter implements it;
/// this crate does no network I/O of its own.
pub trait RustupInitFetcher: Send + Sync {
    /// Downloads `url` to the file `destination`, checking `cancel` between
    /// chunks.
    ///
    /// # Errors
    ///
    /// The adapter's failure; a cancelled download fails and the runner
    /// reports the cancel.
    fn fetch(&self, url: &HttpUrl, destination: &Path, cancel: &CancelToken) -> Result<(), Cause>;
}
