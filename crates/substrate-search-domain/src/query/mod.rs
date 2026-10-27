//! What to look for, as the product states it.

mod case_sensitivity;
mod generated_code;
mod pattern_syntax;
mod search_query;
mod word_match;

pub use case_sensitivity::CaseSensitivity;
pub use generated_code::GeneratedCode;
pub use pattern_syntax::PatternSyntax;
#[expect(
    clippy::module_name_repetitions,
    reason = "`SearchQuery` is the one request the UI sends; its parts live beside it in `query`"
)]
pub use search_query::SearchQuery;
pub use word_match::WordMatch;
