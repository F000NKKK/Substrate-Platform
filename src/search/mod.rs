//! Find in Files / Replace in Files — Visual Studio's "Find and Replace"
//! window, backend half. A search runs over a project (a .gitignore-aware
//! walk that skips build output, VCS metadata and binaries) or over just the
//! open documents, whose *buffer* text the frontend passes in — so unsaved
//! edits are what's searched, not what's on disk. Results come file by file;
//! `keep_going` stops a search that's been replaced. What a product keeps out
//! of the walk, and where its generated code lives, is its [`Layout`].
//! Feature `search`.

mod coverage;
mod error;
mod error_message;
mod find;
pub mod messages;
mod open_document;
mod pattern;
mod plan;
mod query;
mod replace;
mod replace_request;
mod results;
mod scope;
mod source;
mod walk;

pub use error::SearchError;
pub use find::{DEFAULT_MAX_LINES, Search, find};
pub use open_document::OpenDocument;
pub use pattern::{FileMask, Matcher, ReplacementMode};
pub use query::SearchQuery;
pub use replace::replace;
pub use replace_request::ReplaceRequest;
pub use results::{
    FileResult, FindSummary, LineHit, ReplaceSummary, ReplacedDocument, ReplacedFile, line_hits,
};
pub use scope::Scope;
pub use walk::{
    Decoding, Generated, Layout, WalkEntry, WalkEntryKind, files_containing, generated_files,
    project_entries, project_files, read_text, relative,
};

#[cfg(test)]
mod tests;
