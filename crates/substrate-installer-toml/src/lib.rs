//! Reads an installer's catalog file and a release's manifest into
//! `substrate-installer-domain`'s vocabulary. Read-only: nothing here writes
//! a catalog.
//!
//! [`parse_catalog`] reads the catalog as TOML:
//!
//! ```toml
//! revision = 3
//! manifest = "release.json"      # the asset that says what a release needs
//!
//! [installer]                    # the installer's own entry (id defaults to `installer`)
//! name = "Studio Installer"
//! program = "studio-installer"
//! source = { kind = "github", repo = "owner/installer", assets = { "linux-x86_64" = ".AppImage" } }
//!
//! [[group]]
//! id = "build"
//! name = "Build tools"
//!
//! [[product]]                    # id, name, description, workloads, program, icon?, source
//! [[workload]]                   # id, name, description, icon?, required, recommended?, optional?
//! [[component]]                  # id, name, description, group, requires?, source
//! ```
//!
//! A `source` is a table with `kind` = `github` (`repo`, `assets`,
//! `system`?), `git` (`repo`), `crate` (`name`), `rustup` (`toolchain`) or
//! `rust-targets` (`toolchain`, `targets`). Any other kind is refused by
//! name: the domain's `Source` has no variant for it.
//!
//! [`parse_needs`] reads a release's compatibility manifest,
//! `{"requires": {"component.id": ">=1.2"}}`. [`newer_revision`] reads a
//! catalog only when it is newer than the one a caller has.
#![doc = include_str!("../README.md")]

mod dto;
mod error;
mod lines;
mod needs_dto;
mod newer_revision;
mod parse_catalog;
mod parse_needs;
mod scope;

pub use error::{CatalogFileError, Location, NeedsError, Problem, Reason, Table};
pub use newer_revision::newer_revision;
pub use parse_catalog::parse_catalog;
pub use parse_needs::parse_needs;
