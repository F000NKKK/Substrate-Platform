//! The installer's own files, as Substrate Data.
//!
//! - [`parse_catalog`] reads the catalog from Substrate Data **text** (JSON
//!   with comments, unquoted keys and trailing commas): `revision`,
//!   `manifest`, `installer`, `groups`, `products`, `workloads` and
//!   `components`, each `source` tagged by `kind` (`github`, `git`, `crate`,
//!   `rustup`, `rust-targets`; any other kind is refused at its line).
//! - [`parse_needs`] reads a release's compatibility manifest, also text:
//!   `{ requires: { "component.id": ">=1.2" } }`. [`newer_revision`] reads a
//!   catalog only when it is newer than the one a caller has.
//! - [`BinaryInstalledStore`] keeps the records of what is installed in one
//!   versioned Substrate Data **binary** file, written atomically.
//!
//! Every problem in a text file is a [`Problem`]: line and column, the
//! value's path, and the Substrate Data code (`SD0006` …).
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod catalog;
mod needs;
mod problems;
mod store;

pub use catalog::{newer_revision, parse_catalog};
pub use needs::{NeedsError, parse_needs};
pub use problems::{CatalogFileError, Location, Problem, Reason};
pub use store::{BinaryInstalledStore, Damage, Loaded, RecordError, StoreWarning};
