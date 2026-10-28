//! The file's whole shape.

use serde::{Deserialize, Serialize};
use substrate_installer_domain::inventory::InstalledSet;

use crate::damage::Damage;
use crate::record::Record;

/// The format this crate reads and writes. A file with another number is
/// from another version of the installer and is not read.
pub(crate) const FORMAT_VERSION: u32 = 1;

/// What the file holds.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct StoreFile {
    version: u32,
    installed: Vec<Record>,
}

/// The file's number, read before anything else of it, so a file of another
/// format is told apart from a damaged one.
#[derive(Debug, Deserialize)]
pub(crate) struct Header {
    pub(crate) version: u64,
}

impl StoreFile {
    pub(crate) fn of(set: &InstalledSet) -> Self {
        Self {
            version: FORMAT_VERSION,
            installed: set.records().iter().map(Record::of).collect(),
        }
    }

    /// The set the file holds; the first bad record damages the file.
    pub(crate) fn into_set(self) -> Result<InstalledSet, Damage> {
        let mut set = InstalledSet::default();
        for (index, record) in self.installed.into_iter().enumerate() {
            let installed = record
                .into_installed()
                .map_err(|source| Damage::Record { index, source })?;
            set.put(installed);
        }
        Ok(set)
    }
}
