//! Reading a release's manifest.

use substrate_data::schema::from_doc;
use substrate_data::{Diagnostic, Errors, text};
use substrate_installer_domain::compat::Needs;

use super::NeedsError;
use super::dto::NeedsDto;
use crate::problems::Problem;

/// Reads a release's compatibility manifest, Substrate Data text like
/// `{ requires: { "component.id": ">=x" } }`, into what the release needs of
/// the other components. A manifest without `requires` needs nothing. Other
/// keys are refused (`SD0005`), except notes (`comment`, `$…`, `_…`).
///
/// # Errors
///
/// [`NeedsError`]: not Substrate Data of that shape (with line, column, path
/// and code), a key that is not a component id, or a value that is not a
/// version requirement.
pub fn parse_needs(text: &str) -> Result<Needs, NeedsError> {
    let doc = text::parse(text).map_err(|error| invalid(Errors::from(error).0))?;
    let read = from_doc::<NeedsDto>(text, &doc).map_err(|errors| invalid(errors.0))?;
    read.value.into_needs(text, &doc)
}

fn invalid(diagnostics: Vec<Diagnostic>) -> NeedsError {
    NeedsError::Invalid {
        problems: diagnostics.into_iter().map(Problem::of).collect(),
    }
}
