//! The catalog file's shape: serde mirrors of Yog Installer's
//! `catalog.toml`, private to this crate. Each turns itself into the
//! domain's value, naming the line of whatever is wrong.

mod assets;
mod catalog_dto;
mod component_dto;
mod file_kind;
mod group_dto;
mod installer_dto;
mod locations;
mod product_dto;
mod source_dto;
mod system_dto;
mod workload_dto;

pub(crate) use catalog_dto::CatalogDto;
