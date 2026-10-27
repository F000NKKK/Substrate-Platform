//! A language, for the UI.

use serde::Serialize;

/// A language as the UI picks a file's by: its id, its files, whether a
/// server serves it.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageEntry {
    /// Its id.
    pub(super) id: String,
    /// Its file extensions.
    pub(super) extensions: Vec<String>,
    /// Its file names.
    pub(super) file_names: Vec<String>,
    /// Whether a server serves it.
    pub(super) has_server: bool,
}
