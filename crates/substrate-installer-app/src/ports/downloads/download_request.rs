//! What to download.

use std::path::Path;

use substrate_installer_domain::releases::{HttpUrl, Sha256};

/// What to download, and where it goes.
#[derive(Debug, Clone, Copy)]
pub struct DownloadRequest<'a> {
    /// Where from.
    pub url: &'a HttpUrl,
    /// The file it is written to; its folders are made. A partial file left
    /// there by an earlier try may be resumed.
    pub destination: &'a Path,
    /// The digest the file must have, where the release says.
    pub sha256: Option<&'a Sha256>,
}
