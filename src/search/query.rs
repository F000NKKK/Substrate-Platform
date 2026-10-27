//! What to look for, as the UI sends it.

use serde::Deserialize;

/// What to look for — shared by Find in Files, Replace in Files and Code Search's text mode.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[expect(
    clippy::struct_excessive_bools,
    reason = "the UI's wire shape: each option is a checkbox; options become enums in wave 7"
)]
pub struct SearchQuery {
    /// The text or (with `regex`) the regular expression.
    pub pattern: String,
    /// Case-sensitive.
    #[serde(default)]
    pub match_case: bool,
    /// Only matches with no word character next to their word-character edges.
    #[serde(default)]
    pub whole_word: bool,
    /// `pattern` is a regular expression.
    #[serde(default)]
    pub regex: bool,
    /// File types mask, VS-style: `*.rs;*.toml`, `!*.lock` excludes; empty means all files.
    #[serde(default)]
    pub file_mask: String,
    /// Also search the generated code (the layout's `generated`) — skipped
    /// with the rest of the build output otherwise.
    #[serde(default)]
    pub include_generated: bool,
}
