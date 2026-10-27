//! Starting other programs with the system's own environment. A program
//! packaged as an AppImage runs with variables its runtime set for it —
//! `LD_LIBRARY_PATH` into its mount, its GTK and GIO modules, `APPDIR`… —
//! and a program it starts that inherits them loads the AppImage's
//! libraries instead of its own, and fails (`version 'LIBSYSTEMD_254' not
//! found`). Everything a product starts goes through here — programs
//! ([`command`], [`start`]), and links and files opened with the user's
//! default app ([`open_link`], [`open_path`], [`reveal`]).

mod open;

pub use open::{open_link, open_path, reveal, start};

use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use std::process::Command;

/// What an AppImage's runtime tells the program inside about itself.
const APPIMAGE_VARS: [&str; 4] = ["APPDIR", "APPIMAGE", "ARGV0", "OWD"];

crate::messages! {
    process {
        NOT_A_LINK = "{url} isn't a web or mail link.",
        CANT_START = "Couldn't start {program}: {error}",
    }
}

/// How a started program's environment differs from this process's: each
/// variable an AppImage changed, with the value the system had (`None`:
/// unset). Lists (`XDG_DATA_DIRS`, `LD_LIBRARY_PATH`…) only lose the
/// entries into the AppImage — the runtime put them in front of the
/// system's own, which a GTK program can't start without. Empty outside an
/// AppImage.
pub fn host_env() -> Vec<(OsString, Option<OsString>)> {
    let Some(appdir) = std::env::var_os("APPDIR").filter(|d| !d.is_empty()) else {
        return Vec::new();
    };
    let appdir = PathBuf::from(appdir);
    let mut out: Vec<(OsString, Option<OsString>)> = std::env::vars_os()
        .filter(|(_, v)| std::env::split_paths(v).any(|p| p.starts_with(&appdir)))
        .map(|(k, v)| {
            let kept: Vec<PathBuf> = std::env::split_paths(&v)
                .filter(|p| !p.as_os_str().is_empty() && !p.starts_with(&appdir))
                .collect();
            let value = if kept.is_empty() {
                None
            } else {
                std::env::join_paths(kept).ok()
            };
            (k, value)
        })
        .collect();
    out.extend(APPIMAGE_VARS.map(|k| (OsString::from(k), None)));
    out
}

/// A command for `program` with the system's own environment.
pub fn command(program: impl AsRef<OsStr>) -> Command {
    let mut cmd = Command::new(program);
    for (key, value) in host_env() {
        match value {
            Some(value) => cmd.env(key, value),
            None => cmd.env_remove(key),
        };
    }
    cmd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outside_an_appimage_nothing_changes() {
        if std::env::var_os("APPDIR").is_none() {
            assert!(host_env().is_empty());
        }
    }
}
