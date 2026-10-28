//! What a read found.

use substrate_installer_domain::inventory::InstalledSet;

use crate::store_warning::StoreWarning;

/// What was read: the records, and why they are none when the file was passed over.
#[derive(Debug)]
pub struct Loaded {
    /// The records; empty when the file is missing or was passed over.
    pub set: InstalledSet,
    /// Why the file was passed over; none when it was read or is not there.
    pub warning: Option<StoreWarning>,
}
