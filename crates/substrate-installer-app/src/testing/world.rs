//! A world of fakes to apply plans in.

use std::path::PathBuf;

use substrate_installer_domain::channel::Catalog;
use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::inventory::{Installed, InstalledSet};
use substrate_installer_domain::planning::Plan;
use substrate_installer_domain::platforms::Host;
use substrate_installer_domain::versions::Version;

use super::{
    CollectingSink, FakeDownloader, FakeFiles, FakeLayout, FakeProducts, FakeStore, FakeTools,
    FakeUnpacker, Journal, host, sample_catalog,
};
use crate::control::CancelToken;
use crate::installing::{Adapters, ApplyError, ApplyOptions, Installer};

/// Every port faked, sharing one journal.
#[derive(Debug)]
pub(crate) struct World {
    pub(crate) catalog: Catalog,
    pub(crate) host: Host,
    pub(crate) journal: Journal,
    pub(crate) layout: FakeLayout,
    pub(crate) files: FakeFiles,
    pub(crate) downloader: FakeDownloader,
    pub(crate) unpacker: FakeUnpacker,
    pub(crate) store: FakeStore,
    pub(crate) tools: FakeTools,
    pub(crate) products: FakeProducts,
    pub(crate) sink: CollectingSink,
}

impl World {
    pub(crate) fn new() -> Self {
        Self::with_sink(CollectingSink::default())
    }

    pub(crate) fn with_sink(sink: CollectingSink) -> Self {
        let journal = Journal::default();
        Self {
            catalog: sample_catalog(),
            host: host(),
            layout: FakeLayout,
            files: FakeFiles::new(journal.clone()),
            downloader: FakeDownloader::new(journal.clone()),
            unpacker: FakeUnpacker::new(journal.clone()),
            store: FakeStore::new(journal.clone()),
            tools: FakeTools::new(journal.clone()),
            products: FakeProducts {
                journal: journal.clone(),
            },
            journal,
            sink,
        }
    }

    /// Starts with these records installed.
    pub(crate) fn with_installed(self, records: Vec<Installed>) -> Self {
        *self.store.set.lock().unwrap() = InstalledSet::new(records);
        self
    }

    pub(crate) fn installer(&self) -> Installer<'_> {
        let adapters = Adapters {
            layout: &self.layout,
            files: &self.files,
            downloader: &self.downloader,
            unpacker: &self.unpacker,
            store: &self.store,
            tools: &self.tools,
            products: &self.products,
            sink: &self.sink,
        };
        Installer::new(adapters, &self.catalog, &self.host)
    }

    /// Applies `plan` with a fresh token.
    pub(crate) fn apply(&self, plan: &Plan, options: ApplyOptions) -> Result<(), ApplyError> {
        self.installer().apply(plan, options, &CancelToken::new())
    }
}

/// A record of `id` at `version`, installed at `path`.
pub(crate) fn record(id: ItemId, version: &str, path: &str) -> Installed {
    Installed {
        id,
        version: Version::new(version).unwrap(),
        path: PathBuf::from(path),
    }
}
