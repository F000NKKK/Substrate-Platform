//! The adapters an installer works through.

use std::fmt;

use crate::event_sink::EventSink;
use crate::ports::downloads::Downloader;
use crate::ports::placement::{InstallFiles, InstallLayout, ProductInstaller};
use crate::ports::records::InstalledStore;
use crate::ports::tools::ToolRunner;
use crate::ports::unpacking::Unpacker;

/// The adapters an [`Installer`](super::Installer) works through: one
/// implementation of each port, and who hears the events. Built by the
/// composition root.
#[derive(Clone, Copy)]
pub struct Adapters<'a> {
    /// Where things go.
    pub layout: &'a dyn InstallLayout,
    /// Puts files in place.
    pub files: &'a dyn InstallFiles,
    /// Downloads.
    pub downloader: &'a dyn Downloader,
    /// Unpacks archives.
    pub unpacker: &'a dyn Unpacker,
    /// Keeps what is installed.
    pub store: &'a dyn InstalledStore,
    /// Runs rustup and cargo.
    pub tools: &'a dyn ToolRunner,
    /// Installs the platform's programs.
    pub products: &'a dyn ProductInstaller,
    /// Who hears the progress.
    pub sink: &'a dyn EventSink,
}

impl fmt::Debug for Adapters<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Adapters").finish_non_exhaustive()
    }
}
