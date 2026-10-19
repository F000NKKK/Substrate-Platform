//! The file system as an IDE browses it: listing and changing folders and
//! files ([`dir`]) and watching them ([`watch`]). Feature `fs`.

pub mod dir;
pub mod messages;
pub mod watch;

pub use dir::{DirEntryInfo, EntryKind};
pub use watch::DirWatcher;
