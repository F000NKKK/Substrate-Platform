//! What a search covers: the open documents or the whole project.

use serde::Deserialize;

/// What a search covers.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Scope {
    /// The documents passed in (Current document / All open documents).
    Documents,
    /// Every file under the root, open documents' buffers standing in for their files.
    Project,
}
