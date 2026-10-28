//! `[installer]`.

use serde::Deserialize;
use substrate_installer_domain::channel::Installer;
use substrate_installer_domain::ids::ProductId;
use toml::Spanned;

use super::source_dto::SourceDto;
use crate::error::CatalogFileError;
use crate::scope::Scope;

/// The id the installer's entry has when the file gives none.
pub(super) const DEFAULT_ID: &str = "installer";

/// The `[installer]` table.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct InstallerDto {
    pub(super) id: Option<Spanned<String>>,
    name: Spanned<String>,
    program: Spanned<String>,
    source: SourceDto,
}

impl InstallerDto {
    /// The id as the file spells it, or the default.
    pub(super) fn id_text(&self) -> &str {
        self.id.as_ref().map_or(DEFAULT_ID, |id| id.get_ref())
    }

    /// The domain's installer entry.
    pub(super) fn into_installer(self, scope: &Scope<'_>) -> Result<Installer, CatalogFileError> {
        let id = match &self.id {
            Some(id) => scope.parse::<ProductId>("id", id)?,
            None => ProductId::new(DEFAULT_ID)
                .map_err(|error| scope.fail(&self.name.span(), "id", error))?,
        };
        Ok(Installer {
            id,
            name: scope.parse("name", &self.name)?,
            program: scope.parse("program", &self.program)?,
            source: self.source.into_source(scope)?,
        })
    }
}
