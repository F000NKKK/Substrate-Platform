//! A project file format.

use std::path::Path;

use super::Project;
use crate::message::Message;

/// A project file format: `.csproj`, `Cargo.toml`, a product's own.
pub trait ProjectFormat: Send + Sync {
    /// Its name, for messages: `csproj`.
    fn name(&self) -> &str;
    /// Whether `path` is a file of this format (by its name).
    fn reads(&self, path: &Path) -> bool;
    /// The project in `path`.
    ///
    /// # Errors
    ///
    /// A file that can't be read, or isn't one of this format.
    fn load(&self, path: &Path) -> Result<Project, Message>;
    /// Writes `project` to `path` in this format.
    ///
    /// # Errors
    ///
    /// A file that can't be written.
    fn save(&self, project: &Project, path: &Path) -> Result<(), Message>;
}
