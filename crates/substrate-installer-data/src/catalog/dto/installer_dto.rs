//! The installer's own entry.

use substrate_data::Schema;
use substrate_installer_domain::channel::Installer;
use substrate_installer_domain::ids::ProductId;

use super::source_dto::SourceDto;
use crate::catalog::scope::Scope;
use crate::problems::CatalogFileError;

/// The id the installer's entry has when the file gives none.
pub(super) const DEFAULT_ID: &str = "installer";

/// The `installer` entry.
#[derive(Debug, Schema)]
pub(super) struct InstallerDto {
    pub(super) id: Option<String>,
    name: String,
    program: String,
    source: SourceDto,
}

impl InstallerDto {
    /// The id as the file spells it, or the default.
    pub(super) fn id_text(&self) -> &str {
        self.id.as_deref().unwrap_or(DEFAULT_ID)
    }

    /// The domain's installer entry.
    pub(super) fn into_installer(self, scope: &Scope<'_>) -> Result<Installer, CatalogFileError> {
        Ok(Installer {
            id: scope.parse::<ProductId>("id", self.id_text())?,
            name: scope.parse("name", &self.name)?,
            program: scope.parse("program", &self.program)?,
            source: self.source.into_source(&scope.key("source"))?,
        })
    }
}
