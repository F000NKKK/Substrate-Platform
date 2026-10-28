//! The file's whole shape.

use substrate_data::Schema;
use substrate_installer_domain::inventory::InstalledSet;

use super::Damage;
use super::not_text_path::NotTextPath;
use super::record::Record;

/// The format this crate reads and writes. A file with another number is
/// from another version of the installer and is not read.
pub(super) const FORMAT_VERSION: u32 = 1;

/// What the file holds.
#[derive(Debug, Schema)]
pub(super) struct Records {
    version: u32,
    installed: Vec<Record>,
}

impl Records {
    pub(super) fn of(set: &InstalledSet) -> Result<Self, NotTextPath> {
        Ok(Self {
            version: FORMAT_VERSION,
            installed: set
                .records()
                .iter()
                .map(Record::of)
                .collect::<Result<_, _>>()?,
        })
    }

    /// The set the file holds; the first bad record damages the file.
    pub(super) fn into_set(self) -> Result<InstalledSet, Damage> {
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
