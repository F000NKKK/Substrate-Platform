//! What to look for, on the wire.

use serde::Deserialize;
use substrate_search_domain::query::{
    CaseSensitivity, GeneratedCode, PatternSyntax, SearchQuery, WordMatch,
};

/// What to look for: the UI's checkboxes are flags.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[expect(
    clippy::struct_excessive_bools,
    reason = "the UI's wire shape: each option is a checkbox; the domain has enums"
)]
pub(crate) struct SearchQueryDto {
    pattern: String,
    #[serde(default)]
    match_case: bool,
    #[serde(default)]
    whole_word: bool,
    #[serde(default)]
    regex: bool,
    #[serde(default)]
    file_mask: String,
    #[serde(default)]
    include_generated: bool,
}

impl From<SearchQueryDto> for SearchQuery {
    fn from(query: SearchQueryDto) -> Self {
        Self {
            pattern: query.pattern,
            case: if query.match_case {
                CaseSensitivity::Sensitive
            } else {
                CaseSensitivity::Insensitive
            },
            word: if query.whole_word {
                WordMatch::WholeWord
            } else {
                WordMatch::Anywhere
            },
            syntax: if query.regex {
                PatternSyntax::Regex
            } else {
                PatternSyntax::Literal
            },
            file_mask: query.file_mask,
            generated: if query.include_generated {
                GeneratedCode::Include
            } else {
                GeneratedCode::Skip
            },
        }
    }
}
