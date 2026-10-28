//! The whole catalog file.

use serde::Deserialize;
use substrate_installer_domain::channel::{Catalog, CatalogParts, Installer};
use substrate_installer_domain::releases::AssetName;
use toml::Spanned;

use super::component_dto::ComponentDto;
use super::group_dto::GroupDto;
use super::installer_dto::InstallerDto;
use super::locations::Locations;
use super::product_dto::ProductDto;
use super::workload_dto::WorkloadDto;
use crate::error::{CatalogFileError, Table};
use crate::lines::Lines;
use crate::scope::Scope;

/// The catalog file: Yog Installer's `catalog.toml` plus `manifest`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CatalogDto {
    #[serde(default)]
    pub(crate) revision: u32,
    manifest: Option<Spanned<String>>,
    installer: Option<InstallerDto>,
    #[serde(default)]
    group: Vec<GroupDto>,
    #[serde(default)]
    product: Vec<ProductDto>,
    #[serde(default)]
    workload: Vec<WorkloadDto>,
    #[serde(default)]
    component: Vec<ComponentDto>,
}

impl CatalogDto {
    /// The checked catalog; an error of the domain's check points at the
    /// line of the entry it names.
    pub(crate) fn into_catalog(self, lines: &Lines) -> Result<Catalog, CatalogFileError> {
        let locations = self.locations(lines);
        let parts = self.into_parts(lines)?;
        Catalog::new(parts).map_err(|source| CatalogFileError::Rejected {
            location: locations.of(&source),
            source,
        })
    }

    fn into_parts(self, lines: &Lines) -> Result<CatalogParts, CatalogFileError> {
        let top = Scope::new(lines, Table::Catalog, None);
        Ok(CatalogParts {
            revision: self.revision,
            manifest: top.parse_opt::<AssetName>("manifest", self.manifest.as_ref())?,
            installer: self
                .installer
                .map(|dto| installer(dto, lines))
                .transpose()?,
            groups: each(
                self.group,
                Table::Group,
                lines,
                |d| &d.id,
                GroupDto::into_group,
            )?,
            products: each(
                self.product,
                Table::Product,
                lines,
                |d| &d.id,
                ProductDto::into_product,
            )?,
            workloads: each(
                self.workload,
                Table::Workload,
                lines,
                |d| &d.id,
                WorkloadDto::into_workload,
            )?,
            components: each(
                self.component,
                Table::Component,
                lines,
                |d| &d.id,
                ComponentDto::into_component,
            )?,
        })
    }

    /// The line of every entry's id.
    fn locations(&self, lines: &Lines) -> Locations {
        let mut found = Locations::default();
        let ids = self
            .group
            .iter()
            .map(|d| &d.id)
            .chain(self.product.iter().map(|d| &d.id))
            .chain(self.workload.iter().map(|d| &d.id))
            .chain(self.component.iter().map(|d| &d.id))
            .chain(self.installer.iter().filter_map(|d| d.id.as_ref()));
        for id in ids {
            found.insert(id.get_ref(), lines.locate(id.span().start));
        }
        found
    }
}

fn installer(dto: InstallerDto, lines: &Lines) -> Result<Installer, CatalogFileError> {
    let id = dto.id_text().to_owned();
    dto.into_installer(&Scope::new(lines, Table::Installer, Some(&id)))
}

/// Turns every entry of `table`, each read in the scope of its own id.
fn each<D, T>(
    entries: Vec<D>,
    table: Table,
    lines: &Lines,
    id: fn(&D) -> &Spanned<String>,
    convert: fn(D, &Scope<'_>) -> Result<T, CatalogFileError>,
) -> Result<Vec<T>, CatalogFileError> {
    entries
        .into_iter()
        .map(|entry| {
            let text = id(&entry).get_ref().clone();
            convert(entry, &Scope::new(lines, table, Some(&text)))
        })
        .collect()
}
