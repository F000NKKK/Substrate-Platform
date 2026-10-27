//! A product's machine-local folder in a project.

use std::path::Path;

use crate::StateError;

/// What keeps a private folder out of version control, from inside it.
pub(crate) const GITIGNORE: &str = ".gitignore";
/// What the ignore file holds.
pub(crate) const IGNORE_EVERYTHING: &str = "*\n";

/// Makes `dir` — a product's machine-local folder in a project — and keeps
/// it out of version control from inside, so the project's own
/// `.gitignore` is left alone.
///
/// # Errors
///
/// The folder or its ignore file can't be written.
pub fn private_dir(dir: &Path) -> Result<(), StateError> {
    std::fs::create_dir_all(dir).map_err(|e| StateError::io(dir, e))?;
    let ignore = dir.join(GITIGNORE);
    if ignore.exists() {
        return Ok(());
    }
    std::fs::write(&ignore, IGNORE_EVERYTHING).map_err(|e| StateError::io(&ignore, e))
}
