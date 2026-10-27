//! Projects and solutions, the way an IDE models them whatever builds them.
//! The model holds what Visual Studio's `.sln`/`.slnx` and `MSBuild`'s
//! `.csproj` hold, as well as Cargo's or a product's own files:
//! - [`Solution`]: solution folders, configurations and platforms, each
//!   project's configuration per solution configuration, build dependencies;
//! - [`Project`]: properties (per configuration too), target frameworks,
//!   items, references to projects, packages and files.
//!
//! How a `.sln`, `.slnx` or `.csproj` is read and written is the product's
//! business (`substrate-project-app` has the ports); the model only has
//! room for all they hold ([`Section`]s and ordered properties keep what it
//! doesn't know, so a file goes back as it was read). What kind of project
//! a folder holds is told by marker files ([`ProjectStandard`]).
#![doc = include_str!("../README.md")]

mod item;
pub mod messages;
mod project;
mod project_entry;
mod project_error;
mod project_error_message;
mod project_standard;
mod reference;
mod settings;
mod solution;
mod solution_folder;

pub use item::Item;
pub use project::Project;
pub use project_entry::ProjectEntry;
pub use project_error::ProjectError;
pub use project_standard::ProjectStandard;
pub use reference::Reference;
pub use settings::{Configuration, ConfigurationMapping, ConfiguredProperties, Section};
pub use solution::Solution;
pub use solution_folder::SolutionFolder;
