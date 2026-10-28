//! One installed thing as the file spells it.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use substrate_installer_domain::ids::{ComponentId, IdError, ItemId, ProductId};
use substrate_installer_domain::inventory::Installed;
use substrate_installer_domain::versions::{Version, VersionError};

/// Whether an id names a product or a component: ids share one namespace in
/// a catalog, but the store has no catalog to tell them apart with.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Kind {
    Product,
    Component,
}

/// A record of the file.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Record {
    kind: Kind,
    id: String,
    version: String,
    path: PathBuf,
}

/// Why a record of the file is not a valid installed thing.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RecordError {
    /// Its id is not a valid id.
    #[error("the id is not valid")]
    Id(#[from] IdError),
    /// Its version is not a valid version.
    #[error("the version is not valid")]
    Version(#[from] VersionError),
}

impl Record {
    pub(crate) fn of(installed: &Installed) -> Self {
        let kind = match installed.id {
            ItemId::Product(_) => Kind::Product,
            ItemId::Component(_) => Kind::Component,
        };
        Self {
            kind,
            id: installed.id.as_str().to_owned(),
            version: installed.version.as_str().to_owned(),
            path: installed.path.clone(),
        }
    }

    pub(crate) fn into_installed(self) -> Result<Installed, RecordError> {
        let id = match self.kind {
            Kind::Product => ItemId::Product(ProductId::new(self.id)?),
            Kind::Component => ItemId::Component(ComponentId::new(self.id)?),
        };
        Ok(Installed {
            id,
            version: Version::new(&self.version)?,
            path: self.path,
        })
    }
}
