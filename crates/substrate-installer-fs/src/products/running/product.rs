//! The identity and version of a running installer bundle.

use substrate_installer_domain::channel::Installer;
use substrate_installer_domain::versions::Version;

use super::RunningBundle;

/// A running installer to place and register with the desktop.
#[derive(Debug, Clone, Copy)]
pub struct RunningProduct<'a> {
    /// Its channel entry.
    pub installer: &'a Installer,
    /// Its actual running version.
    pub version: &'a Version,
    /// The complete local distribution, never an `AppImage`'s mounted executable.
    pub bundle: RunningBundle<'a>,
}
