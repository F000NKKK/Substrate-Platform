//! The system's own environment, for a started program.

use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use std::process::Command;

/// What an `AppImage`'s runtime tells the program inside about itself.
const APPIMAGE_VARS: [&str; 4] = ["APPDIR", "APPIMAGE", "ARGV0", "OWD"];

/// How a started program's environment differs from this process's: each
/// variable an `AppImage` changed, with the value the system had (`None`:
/// unset). Lists (`XDG_DATA_DIRS`, `LD_LIBRARY_PATH`…) only lose the
/// entries into the `AppImage` — the runtime put them in front of the
/// system's own, which a GTK program can't start without. Empty outside an
/// `AppImage`.
#[must_use]
pub fn host_environment() -> Vec<(OsString, Option<OsString>)> {
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
#[must_use]
pub fn command(program: impl AsRef<OsStr>) -> Command {
    let mut invocation = Command::new(program);
    for (key, value) in host_environment() {
        match value {
            Some(value) => invocation.env(key, value),
            None => invocation.env_remove(key),
        };
    }
    invocation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outside_an_appimage_nothing_changes() {
        if std::env::var_os("APPDIR").is_none() {
            assert!(host_environment().is_empty());
        }
    }
}
