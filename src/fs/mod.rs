//! The file system as an IDE browses it: listing and changing folders and
//! files ([`dir`]) and watching them ([`watch`]). Feature `fs`.

mod copy;
pub mod dir;
mod error;
mod error_message;
pub mod messages;
mod read_text;
pub mod watch;

pub use copy::copy_new;
pub use dir::{DirectoryEntry, EntryKind};
pub use error::FsError;
pub use read_text::read_text;
pub use watch::DirectoryWatcher;
