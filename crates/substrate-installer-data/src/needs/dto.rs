//! A release's manifest file.

use std::collections::BTreeMap;

use substrate_data::{Doc, Path, Schema};
use substrate_installer_domain::compat::Needs;
use substrate_installer_domain::ids::ComponentId;
use substrate_installer_domain::versions::VersionRequirement;

use super::NeedsError;

/// `{ requires: { "component.id": ">=1.2" } }`.
#[derive(Debug, Schema)]
pub(super) struct NeedsDto {
    #[data(default)]
    requires: BTreeMap<String, String>,
}

impl NeedsDto {
    /// The domain's needs; the document says where a wrong entry is.
    pub(super) fn into_needs(self, text: &str, doc: &Doc) -> Result<Needs, NeedsError> {
        let requires = self
            .requires
            .into_iter()
            .map(|(key, requirement)| {
                let at = Path::root().key("requires").key(&key);
                let location = doc.locate(text, &at).into();
                let id =
                    ComponentId::new(key.as_str()).map_err(|source| NeedsError::Component {
                        key: key.clone(),
                        location,
                        source,
                    })?;
                let wanted = VersionRequirement::new(&requirement).map_err(|source| {
                    NeedsError::Requirement {
                        key,
                        requirement,
                        location,
                        source,
                    }
                })?;
                Ok((id, wanted))
            })
            .collect::<Result<_, NeedsError>>()?;
        Ok(Needs { requires })
    }
}
