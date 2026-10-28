//! The port that installs products.

use std::path::PathBuf;

use substrate_installer_domain::channel::Item;
use substrate_installer_domain::versions::Version;

use super::{ProductError, ProductInstall, ProductRemoval};
use crate::ports::tools::ToolControl;

/// Installs and removes the programs of a platform: a setup run silently, an
/// `AppImage` made runnable, a Debian package's files unpacked, an application
/// bundle linked among the applications, and the launcher entry that goes
/// with each. Everything about them is the platform's, so it is one port.
/// Blocking.
pub trait ProductInstaller: Send + Sync {
    /// Installs the downloaded package and returns the path of the program
    /// that starts it, to record.
    ///
    /// # Errors
    ///
    /// [`ProductError`]: cancelled, a package this platform does not install,
    /// or a failure.
    fn install(
        &self,
        request: &ProductInstall<'_>,
        control: &ToolControl<'_>,
    ) -> Result<PathBuf, ProductError>;

    /// Removes an installed version.
    ///
    /// # Errors
    ///
    /// As [`install`](Self::install).
    fn remove(
        &self,
        request: &ProductRemoval<'_>,
        control: &ToolControl<'_>,
    ) -> Result<(), ProductError>;

    /// Removes every installed version of `item` but `keep` (one an older
    /// update left behind).
    ///
    /// # Errors
    ///
    /// [`ProductError::Failed`] when a version cannot be removed.
    fn forget_other_versions(&self, item: Item<'_>, keep: &Version) -> Result<(), ProductError>;
}
