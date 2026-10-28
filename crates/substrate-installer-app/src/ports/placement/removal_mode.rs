//! Why a version of a product goes.

/// Why a version of a product goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemovalMode {
    /// The product is uninstalled: its files, its launcher entry and its links.
    Uninstall,
    /// An update replaced this version: only its files go; the launcher
    /// entry and links are the new version's now.
    Replaced,
}
