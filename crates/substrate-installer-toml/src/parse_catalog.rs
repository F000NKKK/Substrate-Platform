//! Reading a catalog file.

use substrate_installer_domain::channel::Catalog;

use crate::dto::CatalogDto;
use crate::error::CatalogFileError;
use crate::lines::Lines;

/// Reads a catalog from TOML and checks it.
///
/// # Errors
///
/// [`CatalogFileError`]: the text is not TOML of the catalog's shape, a
/// field is wrong (with its line and field), or the entries do not make a
/// catalog (an id twice, a reference to nothing, a requirement cycle).
pub fn parse_catalog(text: &str) -> Result<Catalog, CatalogFileError> {
    let lines = Lines::new(text);
    read(text, &lines)?.into_catalog(&lines)
}

/// The file's shape, before its values are checked.
pub(crate) fn read(text: &str, lines: &Lines) -> Result<CatalogDto, CatalogFileError> {
    toml::from_str(text).map_err(|source: toml::de::Error| CatalogFileError::Syntax {
        location: source.span().map(|span| lines.locate(span.start)),
        source,
    })
}
