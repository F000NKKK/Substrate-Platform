//! `[[product]]`.

use serde::Deserialize;
use substrate_installer_domain::channel::Product;
use substrate_installer_domain::ids::WorkloadId;
use toml::Spanned;

use super::source_dto::SourceDto;
use crate::error::CatalogFileError;
use crate::scope::Scope;

/// A `[[product]]` table.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ProductDto {
    pub(super) id: Spanned<String>,
    name: Spanned<String>,
    description: Spanned<String>,
    workloads: Vec<Spanned<String>>,
    program: Spanned<String>,
    icon: Option<Spanned<String>>,
    source: SourceDto,
}

impl ProductDto {
    /// The domain's product.
    pub(super) fn into_product(self, scope: &Scope<'_>) -> Result<Product, CatalogFileError> {
        Ok(Product {
            id: scope.parse("id", &self.id)?,
            name: scope.parse("name", &self.name)?,
            description: scope.parse("description", &self.description)?,
            workloads: scope.parse_all::<WorkloadId>("workloads", &self.workloads)?,
            program: scope.parse("program", &self.program)?,
            icon: scope.parse_opt("icon", self.icon.as_ref())?,
            source: self.source.into_source(scope)?,
        })
    }
}
