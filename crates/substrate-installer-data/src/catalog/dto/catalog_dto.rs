//! The whole catalog file.

use substrate_data::{Doc, Path, Schema};
use substrate_installer_domain::channel::{Catalog, CatalogParts, Installer};
use substrate_installer_domain::releases::AssetName;

use super::component_dto::ComponentDto;
use super::group_dto::GroupDto;
use super::installer_dto::InstallerDto;
use super::product_dto::ProductDto;
use super::workload_dto::WorkloadDto;
use crate::catalog::locations::Locations;
use crate::catalog::scope::Scope;
use crate::problems::CatalogFileError;

/// The catalog file.
#[derive(Debug, Schema)]
pub(in crate::catalog) struct CatalogDto {
    #[data(default)]
    pub(in crate::catalog) revision: u32,
    manifest: Option<String>,
    installer: Option<InstallerDto>,
    #[data(default)]
    groups: Vec<GroupDto>,
    #[data(default)]
    products: Vec<ProductDto>,
    #[data(default)]
    workloads: Vec<WorkloadDto>,
    #[data(default)]
    components: Vec<ComponentDto>,
}

impl CatalogDto {
    /// The checked catalog; an error of the domain's check points at the
    /// line of the entry it names.
    pub(in crate::catalog) fn into_catalog(
        self,
        text: &str,
        doc: &Doc,
    ) -> Result<Catalog, CatalogFileError> {
        let root = Scope::root(text, doc);
        let locations = self.locations();
        let parts = self.into_parts(&root)?;
        Catalog::new(parts).map_err(|source| {
            let path = locations.of(&source).cloned();
            CatalogFileError::Rejected {
                location: path.as_ref().map(|path| root.locate(path)),
                path,
                source,
            }
        })
    }

    fn into_parts(self, root: &Scope<'_>) -> Result<CatalogParts, CatalogFileError> {
        Ok(CatalogParts {
            revision: self.revision,
            manifest: root.parse_opt::<AssetName>("manifest", self.manifest.as_ref())?,
            installer: self
                .installer
                .map(|dto| installer(dto, &root.key("installer")))
                .transpose()?,
            groups: each(self.groups, &root.key("groups"), GroupDto::into_group)?,
            products: each(
                self.products,
                &root.key("products"),
                ProductDto::into_product,
            )?,
            workloads: each(
                self.workloads,
                &root.key("workloads"),
                WorkloadDto::into_workload,
            )?,
            components: each(
                self.components,
                &root.key("components"),
                ComponentDto::into_component,
            )?,
        })
    }

    /// The path of every entry's id.
    fn locations(&self) -> Locations {
        let mut found = Locations::default();
        let at = |list: &str, index: usize| Path::root().key(list).index(index).key("id");
        let lists: [(&str, Vec<&str>); 4] = [
            (
                "groups",
                self.groups.iter().map(|d| d.id.as_str()).collect(),
            ),
            (
                "products",
                self.products.iter().map(|d| d.id.as_str()).collect(),
            ),
            (
                "workloads",
                self.workloads.iter().map(|d| d.id.as_str()).collect(),
            ),
            (
                "components",
                self.components.iter().map(|d| d.id.as_str()).collect(),
            ),
        ];
        for (list, ids) in lists {
            for (index, id) in ids.into_iter().enumerate() {
                found.insert(id, at(list, index));
            }
        }
        if let Some(dto) = &self.installer {
            found.insert(dto.id_text(), Path::root().key("installer").key("id"));
        }
        found
    }
}

fn installer(dto: InstallerDto, scope: &Scope<'_>) -> Result<Installer, CatalogFileError> {
    dto.into_installer(scope)
}

/// Turns every entry of the list at `scope`, each read in the scope of its
/// own index.
fn each<D, T>(
    entries: Vec<D>,
    scope: &Scope<'_>,
    convert: fn(D, &Scope<'_>) -> Result<T, CatalogFileError>,
) -> Result<Vec<T>, CatalogFileError> {
    entries
        .into_iter()
        .enumerate()
        .map(|(index, entry)| convert(entry, &scope.index(index)))
        .collect()
}
