//! What happens to a download once it is installed.

/// What happens to a downloaded package once it is installed.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DownloadCache {
    /// The file is removed from the cache.
    #[default]
    Discard,
    /// The file stays in the cache (to install again without downloading).
    Keep,
}
