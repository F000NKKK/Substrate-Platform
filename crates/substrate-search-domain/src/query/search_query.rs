//! What to look for.

use super::{CaseSensitivity, GeneratedCode, PatternSyntax, WordMatch};
use crate::pattern::ReplacementMode;

/// What to look for — shared by Find in Files, Replace in Files and Code Search's text mode.
#[derive(Clone, Debug, Default)]
pub struct SearchQuery {
    /// The text or (with `syntax`) the regular expression.
    pub pattern: String,
    /// Whether case matters.
    pub case: CaseSensitivity,
    /// Whether a match must stand as a word.
    pub word: WordMatch,
    /// What `pattern` means.
    pub syntax: PatternSyntax,
    /// File types mask, VS-style: `*.rs;*.toml`, `!*.lock` excludes; empty means all files.
    pub file_mask: String,
    /// Whether generated code is searched too.
    pub generated: GeneratedCode,
}

impl SearchQuery {
    /// How a replacement text is read for this query: groups expand in regex mode.
    #[must_use]
    pub fn replacement_mode(&self) -> ReplacementMode {
        match self.syntax {
            PatternSyntax::Regex => ReplacementMode::Expand,
            PatternSyntax::Literal => ReplacementMode::Literal,
        }
    }
}
