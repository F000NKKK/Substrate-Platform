//! The port that keeps the records.

use substrate_installer_domain::inventory::InstalledSet;

use super::StoreError;

/// Keeps what the installer installed (a file such as `components.ydata`).
/// Only the installer writes it; everything else reads it.
pub trait InstalledStore: Send + Sync {
    /// Everything installed. A store nothing was written to yet is an empty
    /// set, not an error; one written by a version that cannot be read is
    /// empty too (nothing known installed until it is written again).
    ///
    /// # Errors
    ///
    /// [`StoreError::Read`] when the store exists and cannot be read.
    fn read(&self) -> Result<InstalledSet, StoreError>;

    /// Replaces the records with `set`, whole and atomically.
    ///
    /// # Errors
    ///
    /// [`StoreError::Write`] when the store cannot be written.
    fn write(&self, set: &InstalledSet) -> Result<(), StoreError>;
}
