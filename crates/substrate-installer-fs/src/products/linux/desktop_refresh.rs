//! Telling the desktop its launcher entries changed.

use std::path::Path;
use std::process::{Command, Stdio};

use crate::products::desktop_caches::DesktopCaches;

/// Tools that read a desktop's entries again, with whether they take the
/// entries' folder: the freedesktop database, KDE Plasma 6, KDE Plasma 5.
/// Without it a desktop may start the old program of an entry.
const TOOLS: [(&str, bool); 3] = [
    ("update-desktop-database", true),
    ("kbuildsycoca6", false),
    ("kbuildsycoca5", false),
];

/// Runs the tools this desktop has over `entries`; the ones it has not, and
/// their failures, are passed over: an entry works without the cache.
pub(super) fn refresh(caches: DesktopCaches, entries: &Path) {
    if caches == DesktopCaches::Leave {
        return;
    }
    for (tool, takes_folder) in TOOLS {
        let mut command = Command::new(tool);
        if takes_folder {
            command.arg(entries);
        }
        command
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .ok();
    }
}
