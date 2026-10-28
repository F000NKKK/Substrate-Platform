//! One installed thing as the file spells it.

use std::path::PathBuf;

use substrate_data::Schema;
use substrate_installer_domain::ids::{ComponentId, IdError, ItemId, ProductId};
use substrate_installer_domain::inventory::Installed;
use substrate_installer_domain::versions::{Version, VersionError};

use super::not_text_path::NotTextPath;

/// Whether an id names a product or a component: ids share one namespace in
/// a catalog, but the store has no catalog to tell them apart with.
#[derive(Debug, Clone, Copy, Schema)]
#[data(rename_all = "camelCase")]
enum Kind {
    Product,
    Component,
}

/// A record of the file.
#[derive(Debug, Schema)]
pub(super) struct Record {
    kind: Kind,
    id: String,
    version: String,
    path: String,
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
    pub(super) fn of(installed: &Installed) -> Result<Self, NotTextPath> {
        let kind = match installed.id {
            ItemId::Product(_) => Kind::Product,
            ItemId::Component(_) => Kind::Component,
        };
        let path = installed
            .path
            .to_str()
            .ok_or_else(|| NotTextPath(installed.path.clone()))?;
        Ok(Self {
            kind,
            id: installed.id.as_str().to_owned(),
            version: installed.version.as_str().to_owned(),
            path: path.to_owned(),
        })
    }

    pub(super) fn into_installed(self) -> Result<Installed, RecordError> {
        let id = match self.kind {
            Kind::Product => ItemId::Product(ProductId::new(self.id)?),
            Kind::Component => ItemId::Component(ComponentId::new(self.id)?),
        };
        Ok(Installed {
            id,
            version: Version::new(&self.version)?,
            path: PathBuf::from(self.path),
        })
    }
}
