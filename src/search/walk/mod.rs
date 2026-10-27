//! The project's files as a search walks them: .gitignore-aware, build
//! output and binaries skipped ([`Layout`]); generated code on request; text
//! read safely ([`read_text`]).

mod decoding;
mod entry;
mod generated;
mod layout;
mod project;

pub use decoding::{Decoding, read_text};
pub use entry::{WalkEntry, WalkEntryKind, project_entries};
pub use generated::{Generated, generated_files};
pub use layout::Layout;
pub use project::{files_containing, project_files, relative};
