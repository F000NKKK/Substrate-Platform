//! A release's manifest file.

use std::collections::BTreeMap;

use serde::Deserialize;
use substrate_installer_domain::compat::Needs;
use substrate_installer_domain::ids::ComponentId;
use substrate_installer_domain::versions::VersionRequirement;

use crate::error::NeedsError;

/// `{"requires": {"component.id": ">=1.2"}}`. Other keys are ignored: a
/// product's manifest may carry more than the engine reads.
#[derive(Debug, Deserialize)]
pub(crate) struct NeedsDto {
    #[serde(default)]
    requires: BTreeMap<String, String>,
}

impl NeedsDto {
    /// The domain's needs.
    pub(crate) fn into_needs(self) -> Result<Needs, NeedsError> {
        let requires = self
            .requires
            .into_iter()
            .map(|(key, requirement)| {
                let id =
                    ComponentId::new(key.as_str()).map_err(|source| NeedsError::Component {
                        key: key.clone(),
                        source,
                    })?;
                let wanted = VersionRequirement::new(&requirement).map_err(|source| {
                    NeedsError::Requirement {
                        key,
                        requirement,
                        source,
                    }
                })?;
                Ok((id, wanted))
            })
            .collect::<Result<_, NeedsError>>()?;
        Ok(Needs { requires })
    }
}
