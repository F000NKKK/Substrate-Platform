//! Starting other programs with the system's own environment. A program
//! packaged as an AppImage runs with variables its runtime set for it —
//! `LD_LIBRARY_PATH` into its mount, its GTK and GIO modules, `APPDIR`… —
//! and a program it starts that inherits them loads the AppImage's
//! libraries instead of its own, and fails (`version 'LIBSYSTEMD_254' not
//! found`). Everything a product starts goes through here.

use std::ffi::{OsStr, OsString};
use std::process::Command;

/// What an AppImage's runtime tells the program inside about itself.
const APPIMAGE_VARS: [&str; 4] = ["APPDIR", "APPIMAGE", "ARGV0", "OWD"];

/// The variables a started program mustn't inherit: the AppImage's own, and
/// every one pointing into its mount. None outside an AppImage.
pub fn foreign_vars() -> Vec<OsString> {
    let Some(appdir) = std::env::var_os("APPDIR").filter(|d| !d.is_empty()) else {
        return Vec::new();
    };
    let appdir = appdir.to_string_lossy().into_owned();
    let mut out: Vec<OsString> = std::env::vars_os()
        .filter(|(_, v)| v.to_string_lossy().contains(&appdir))
        .map(|(k, _)| k)
        .collect();
    out.extend(APPIMAGE_VARS.map(OsString::from));
    out
}

/// A command for `program` with the system's own environment.
pub fn command(program: impl AsRef<OsStr>) -> Command {
    let mut cmd = Command::new(program);
    for key in foreign_vars() {
        cmd.env_remove(key);
    }
    cmd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outside_an_appimage_nothing_is_removed() {
        if std::env::var_os("APPDIR").is_none() {
            assert!(foreign_vars().is_empty());
        }
    }
}
