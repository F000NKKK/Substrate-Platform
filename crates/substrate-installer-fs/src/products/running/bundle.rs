//! A running installer in its platform's distribution form.

use std::path::Path;

/// The local bundle to retain when an installer runs for the first time.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum RunningBundle<'a> {
    /// The original `AppImage` (`APPIMAGE`), outside its temporary mount.
    AppImage(&'a Path),
    /// The complete macOS `.app`, including resources and signatures.
    Application(&'a Path),
    /// A Windows program already installed by its native setup.
    InstalledProgram(&'a Path),
}
