//! The catalog (the channel): what can be installed. Products made of
//! workloads, workloads made of components, components in groups, each with
//! the [`Source`](crate::sources::Source) it is published at. A [`Catalog`]
//! is checked when made: ids are unique, every reference names something
//! there, no component requires itself through others.

mod catalog;
mod catalog_error;
mod catalog_parts;
mod component;
mod entry_ref;
mod group;
mod installer;
mod item;
mod product;
#[cfg(test)]
mod tests;
mod validate;
mod workload;

pub use catalog::Catalog;
pub use catalog_error::CatalogError;
pub use catalog_parts::CatalogParts;
pub use component::Component;
pub use entry_ref::EntryRef;
pub use group::Group;
pub use installer::Installer;
pub use item::Item;
pub use product::Product;
pub use workload::Workload;
