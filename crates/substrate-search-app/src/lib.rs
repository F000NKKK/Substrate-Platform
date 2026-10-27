//! Find and Replace in Files' use cases. A search runs over a project (a
//! walk that skips build output, VCS metadata and binaries) or over just the
//! open documents, whose *buffer* text the frontend passes in — so unsaved
//! edits are what's searched, not what's on disk. Results come file by file;
//! `keep_going` stops a search that's been replaced. Walking and reading
//! files are the [`ports`]; the use cases are [`find`], [`replace`] and
//! [`files_containing`].
#![doc = include_str!("../README.md")]

mod containing;
mod coverage;
mod find;
mod plan;
pub mod ports;
mod replace;
mod replace_request;
mod search;
mod source;

pub use containing::files_containing;
pub use find::find;
pub use replace::replace;
pub use replace_request::ReplaceRequest;
pub use search::{DEFAULT_MAX_LINES, Search};
