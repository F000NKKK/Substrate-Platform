//! Both sides of a diff, on the wire.

use serde::Serialize;
use substrate_git_domain::diffs::DiffTexts;

/// Both sides of a file's diff.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiffTextsDto {
    left: Option<String>,
    right: Option<String>,
    binary: bool,
}

impl From<DiffTexts> for DiffTextsDto {
    fn from(texts: DiffTexts) -> Self {
        Self {
            left: texts.left,
            right: texts.right,
            binary: texts.binary,
        }
    }
}
