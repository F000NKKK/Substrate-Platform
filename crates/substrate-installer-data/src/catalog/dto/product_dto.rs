//! A product.

use substrate_data::Schema;
use substrate_installer_domain::channel::Product;
use substrate_installer_domain::ids::WorkloadId;

use super::source_dto::SourceDto;
use crate::catalog::scope::Scope;
use crate::problems::CatalogFileError;

/// An entry of `products`.
#[derive(Debug, Schema)]
pub(super) struct ProductDto {
    pub(super) id: String,
    name: String,
    description: String,
    workloads: Vec<String>,
    program: String,
    icon: Option<String>,
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
            source: self.source.into_source(&scope.key("source"))?,
        })
    }
}
