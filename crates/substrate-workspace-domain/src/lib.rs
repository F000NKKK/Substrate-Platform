//! The workspace's vocabulary: the folders the user opened ([`Roots`]), why a
//! path is not in them ([`WorkspaceError`]), a folder's entries
//! ([`DirectoryEntry`], [`EntryKind`]) and how a product's own file kinds are
//! read as text ([`TextViews`]). No I/O: resolving paths is a port of
//! `substrate-workspace-app`, the file system is `substrate-workspace-fs`.
#![doc = include_str!("../README.md")]

mod directory_entry;
mod entry_kind;
pub mod messages;
mod roots;
mod text_view;
mod text_views;
mod workspace_error;
mod workspace_error_message;

pub use directory_entry::DirectoryEntry;
pub use entry_kind::EntryKind;
pub use roots::Roots;
pub use text_view::TextView;
pub use text_views::TextViews;
pub use workspace_error::WorkspaceError;
