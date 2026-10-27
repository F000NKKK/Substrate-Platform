//! One standard as a TOML table.

use serde::Deserialize;
use substrate_project_domain::ProjectStandard;

/// One `[[standard]]` table.
#[derive(Deserialize)]
pub(crate) struct StandardDto {
    id: String,
    detect_files: Vec<String>,
}

impl StandardDto {
    /// The standard this table describes.
    pub(crate) fn into_standard(self) -> ProjectStandard {
        ProjectStandard {
            id: self.id,
            detect_files: self.detect_files,
        }
    }
}
