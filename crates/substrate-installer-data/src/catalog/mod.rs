//! Reading the catalog from Substrate Data text.

mod dto;
mod locations;
mod newer_revision;
mod parse_catalog;
mod read;
mod scope;

pub use newer_revision::newer_revision;
pub use parse_catalog::parse_catalog;
