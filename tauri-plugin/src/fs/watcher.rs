//! The folder watcher the plugin shares.

use std::sync::OnceLock;

use substrate_workspace_fs::DirectoryWatcher;

/// The watcher behind `fs_watch`, made on first use.
#[derive(Default)]
pub(crate) struct Watcher(pub(super) OnceLock<DirectoryWatcher>);
