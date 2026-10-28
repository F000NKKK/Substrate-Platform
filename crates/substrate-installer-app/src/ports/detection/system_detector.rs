//! The port that finds what was there before.

use substrate_installer_domain::channel::Catalog;
use substrate_installer_domain::inventory::Found;

/// Finds what was installed on the machine before the installer, or by
/// someone else: rustup's toolchains, `cargo install`ed tools, the system's
/// JDKs, a tool on `PATH`. Read-only. What it cannot find is not reported.
pub trait SystemDetector: Send + Sync {
    /// The components of `catalog` found on the system, each as
    /// [`Origin::System`](substrate_installer_domain::inventory::Origin::System).
    fn detect(&self, catalog: &Catalog) -> Vec<Found>;
}
