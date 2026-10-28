//! Reading a catalog file.

use substrate_installer_domain::channel::Catalog;

use super::read::read;

/// Reads a catalog from Substrate Data text and checks it.
///
/// # Errors
///
/// [`CatalogFileError`](crate::CatalogFileError): the text is not of the
/// catalog's shape or a value breaks a rule (with its line, column, path and
/// code), or the entries do not make a catalog (an id twice, a reference to
/// nothing, a requirement cycle).
pub fn parse_catalog(text: &str) -> Result<Catalog, crate::CatalogFileError> {
    let (file, doc) = read(text)?;
    file.into_catalog(text, &doc)
}
