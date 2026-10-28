//! The port that lists releases.

use substrate_installer_domain::releases::Release;
use substrate_installer_domain::sources::Source;

use super::ListingError;

/// Lists the releases a [`Source`] has published. One adapter serves every
/// kind of source (GitHub's API, crates.io, a toolchain's name) and
/// answers [`ListingError::Unsupported`] for a kind it does not know.
/// Blocking; the use cases call it from several threads at once.
pub trait ReleaseSource: Send + Sync {
    /// `source`'s releases, newest first, drafts and yanked versions left
    /// out. A source that is not a list of versions (a toolchain) answers its
    /// one release.
    ///
    /// # Errors
    ///
    /// [`ListingError`] when the source cannot be asked or understood.
    fn releases(&self, source: &Source) -> Result<Vec<Release>, ListingError>;
}
