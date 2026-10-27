//! The project a search walks: its layout, the files and folders it lists,
//! and what a search covers.

mod generated;
mod layout;
mod open_document;
mod relative_path;
mod scope;
mod walk_entry;
mod walk_entry_kind;

pub use generated::Generated;
pub use layout::Layout;
pub use open_document::OpenDocument;
pub use relative_path::relative;
pub use scope::Scope;
pub use walk_entry::WalkEntry;
pub use walk_entry_kind::WalkEntryKind;
