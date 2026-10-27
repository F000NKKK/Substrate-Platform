//! The workspace on the real file system: listing and changing folders and
//! files ([`dir`]), copying ([`copy_new`]), reading text ([`read_text`]),
//! watching folders for changes made outside the IDE
//! ([`DirectoryWatcher`]) and resolving paths for the workspace
//! ([`DiskPaths`]). Carries no notion of "is this path allowed": that
//! boundary (an IDE touches only files inside an opened project) is the
//! workspace's, one layer up.
#![doc = include_str!("../README.md")]

mod copy;
pub mod dir;
mod directory_watcher;
mod disk_paths;
mod fs_error;
mod fs_error_message;
pub mod messages;
mod read_text;

pub use copy::copy_new;
pub use directory_watcher::{DEFAULT_QUIET, DirectoryWatcher};
pub use disk_paths::DiskPaths;
pub use fs_error::FsError;
pub use read_text::read_text;
