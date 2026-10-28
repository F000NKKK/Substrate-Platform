//! Where the desktop lists what can be started.

use std::path::{Path, PathBuf};

/// Where Linux desktops look for a user's launcher entries, under the user's home.
const LINUX_APPLICATIONS: [&str; 3] = [".local", "share", "applications"];
/// Where macOS keeps a user's applications, under the user's home.
const MAC_APPLICATIONS: &str = "Applications";

/// Whether the desktop's caches of launcher entries are told about a change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopCaches {
    /// `update-desktop-database` and KDE's `kbuildsycoca` are run when
    /// present, so a desktop does not start a removed program.
    Refresh,
    /// The caches are left as they are.
    Leave,
}

/// The folders where a program is listed so the user can start it.
#[derive(Debug, Clone)]
pub struct LauncherDirs {
    /// Linux: the folder of `.desktop` entries.
    pub desktop_entries: PathBuf,
    /// macOS: the folder application bundles are linked into.
    pub applications: PathBuf,
    /// Linux: whether the desktop's caches are refreshed after a change.
    pub caches: DesktopCaches,
}

impl LauncherDirs {
    /// The standard places under the user's home folder
    /// (`~/.local/share/applications`, `~/Applications`), with the caches refreshed.
    #[must_use]
    pub fn under_user_home(user_home: &Path) -> Self {
        Self {
            desktop_entries: LINUX_APPLICATIONS
                .iter()
                .fold(user_home.to_path_buf(), |path, part| path.join(part)),
            applications: user_home.join(MAC_APPLICATIONS),
            caches: DesktopCaches::Refresh,
        }
    }
}
