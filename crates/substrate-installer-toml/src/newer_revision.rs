//! Taking a catalog only when it is newer.

use substrate_installer_domain::channel::Catalog;

use crate::error::CatalogFileError;
use crate::lines::Lines;
use crate::parse_catalog::read;

/// Reads a catalog only when its `revision` is above `than`: an installer
/// takes the newest of the catalog it was built with and the one a release
/// brings, and a stale or equal one is not worth checking.
///
/// # Errors
///
/// [`CatalogFileError`] as [`parse_catalog`](crate::parse_catalog); an older
/// file is still refused when its shape is wrong, but not when its entries
/// are (they are never checked).
pub fn newer_revision(text: &str, than: u32) -> Result<Option<Catalog>, CatalogFileError> {
    let lines = Lines::new(text);
    let file = read(text, &lines)?;
    if file.revision <= than {
        return Ok(None);
    }
    file.into_catalog(&lines).map(Some)
}
