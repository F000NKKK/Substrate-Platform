//! The catalog file's shape: Substrate Data schemas private to this crate,
//! mirroring Yog Installer's `catalog.toml`. Each turns itself into the
//! domain's value, naming the line of whatever is wrong.

mod catalog_dto;
mod component_dto;
mod group_dto;
mod installer_dto;
mod product_dto;
mod source_dto;
mod system_dto;
mod workload_dto;

pub(super) use catalog_dto::CatalogDto;
