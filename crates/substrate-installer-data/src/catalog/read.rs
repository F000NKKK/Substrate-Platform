//! Reading the catalog's shape.

use substrate_data::schema::from_doc;
use substrate_data::{Diagnostic, Doc, Errors, text};

use super::dto::CatalogDto;
use crate::problems::{CatalogFileError, Problem};

/// Parses `text` and reads it as the catalog's shape; the document comes
/// back for the positions of what is checked later.
pub(super) fn read(text: &str) -> Result<(CatalogDto, Doc), CatalogFileError> {
    let doc = text::parse(text).map_err(|error| invalid(Errors::from(error).0))?;
    let read = from_doc::<CatalogDto>(text, &doc).map_err(|errors| invalid(errors.0))?;
    Ok((read.value, doc))
}

fn invalid(diagnostics: Vec<Diagnostic>) -> CatalogFileError {
    CatalogFileError::Invalid {
        problems: diagnostics.into_iter().map(Problem::of).collect(),
    }
}
