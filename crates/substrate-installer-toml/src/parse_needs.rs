//! Reading a release's manifest.

use substrate_installer_domain::compat::Needs;

use crate::error::NeedsError;
use crate::needs_dto::NeedsDto;

/// Reads a release's compatibility manifest, `{"requires": {"id": ">=x"}}`,
/// into what the release needs of the other components. A manifest without
/// `requires` needs nothing. JSON has no spans: a wrong id or requirement is
/// named by its key, a wrong shape by line and column.
///
/// # Errors
///
/// [`NeedsError`]: not JSON of that shape, a key that is not a component
/// id, or a value that is not a version requirement.
pub fn parse_needs(json: &str) -> Result<Needs, NeedsError> {
    let dto: NeedsDto = serde_json::from_str(json).map_err(|source| NeedsError::Syntax {
        line: source.line(),
        column: source.column(),
        source,
    })?;
    dto.into_needs()
}
