//! A tool the installer runs.

use std::fmt;

/// A program the installer runs to install what is not a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ToolProgram {
    /// Rust's toolchain installer.
    Rustup,
    /// Rust's package manager.
    Cargo,
}

impl fmt::Display for ToolProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Rustup => "rustup",
            Self::Cargo => "cargo",
        })
    }
}
