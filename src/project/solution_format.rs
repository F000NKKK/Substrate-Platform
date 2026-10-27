//! A solution file format.

use std::path::Path;

use super::Solution;
use crate::message::Message;

/// A solution file format: `.sln`, `.slnx`, a product's own.
pub trait SolutionFormat: Send + Sync {
    /// Its name, for messages: `sln`, `slnx`.
    fn name(&self) -> &str;
    /// Whether `path` is a file of this format (by its name).
    fn reads(&self, path: &Path) -> bool;
    /// The solution in `path`; its project paths resolved against `path`'s folder.
    ///
    /// # Errors
    ///
    /// A file that can't be read, or isn't one of this format.
    fn load(&self, path: &Path) -> Result<Solution, Message>;
    /// Writes `solution` to `path` in this format.
    ///
    /// # Errors
    ///
    /// A file that can't be written.
    fn save(&self, solution: &Solution, path: &Path) -> Result<(), Message>;
}
