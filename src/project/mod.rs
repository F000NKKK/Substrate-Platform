//! Projects and solutions, the way an IDE models them whatever builds them.
//! The model holds what Visual Studio's `.sln`/`.slnx` and MSBuild's
//! `.csproj` hold, as well as Cargo's or a product's own files:
//! - [`Solution`]: solution folders, configurations and platforms, each
//!   project's configuration per solution configuration, build dependencies;
//! - [`Project`]: properties (per configuration too), target frameworks,
//!   items, references to projects, packages and files.
//!
//! Formats belong to the product: it implements [`SolutionFormat`] /
//! [`ProjectFormat`] and registers them in [`Formats`], which opens a file
//! with the format that reads it — saving under another format's name
//! converts. Substrate ships no format: how a `.sln`, `.slnx` or
//! `.csproj` is read and written is the product's business; the model only
//! has room for all they hold ([`Section`]s and ordered properties keep
//! what it doesn't know, so a file goes back as it was read). What kind of
//! project a folder holds is told by marker files ([`ProjectStandard`]).
//! Feature `project`.

mod entry;
mod error;
mod error_message;
mod format;
mod item;
pub mod messages;
mod model;
mod reference;
mod settings;
mod solution;
mod solution_folder;
mod standard;

pub use entry::ProjectEntry;
pub use error::ProjectError;
pub use format::{Formats, ProjectFormat, SolutionFormat};
pub use item::Item;
pub use model::Project;
pub use reference::Reference;
pub use settings::{Configuration, ConfigurationMapping, ConfiguredProperties, Section};
pub use solution::Solution;
pub use solution_folder::SolutionFolder;
pub use standard::ProjectStandard;
