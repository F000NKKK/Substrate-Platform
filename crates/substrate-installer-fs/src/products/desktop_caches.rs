//! Whether the desktop is told of changes.

/// Whether the desktop's caches of launcher entries are told about a change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopCaches {
    /// `update-desktop-database` and KDE's `kbuildsycoca` are run when
    /// present, so a desktop does not start a removed program.
    Refresh,
    /// The caches are left as they are.
    Leave,
}
