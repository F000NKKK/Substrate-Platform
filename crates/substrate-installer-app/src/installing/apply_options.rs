//! How a plan is carried out.

use substrate_installer_domain::planning::Mode;

use super::DownloadCache;

/// How a plan is carried out.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ApplyOptions {
    /// When downloads happen.
    pub mode: Mode,
    /// What happens to downloads once installed.
    pub downloads: DownloadCache,
}
