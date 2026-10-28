//! When downloads happen.

/// When downloads happen, as Visual Studio Installer offers it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Mode {
    /// Each package installs as soon as it is downloaded.
    #[default]
    InstallWhileDownloading,
    /// Everything downloads first, then installs: nothing changes if a
    /// download fails.
    DownloadAllThenInstall,
}
