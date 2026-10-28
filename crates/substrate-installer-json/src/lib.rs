//! The installer's records of what is installed, kept as one versioned JSON
//! file: [`JsonInstalledStore`] implements the `InstalledStore` port. Reading
//! never fails on a bad file (it is an empty set, with a typed
//! [`StoreWarning`] to show); writing is atomic.
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod damage;
mod json_store;
mod record;
mod store_file;
mod store_warning;

pub use damage::Damage;
pub use json_store::{JsonInstalledStore, Loaded};
pub use record::RecordError;
pub use store_warning::StoreWarning;

#[cfg(test)]
mod tests;
