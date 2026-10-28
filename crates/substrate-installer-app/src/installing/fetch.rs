//! Downloading a change's file.

use std::path::{Path, PathBuf};

use substrate_installer_domain::channel::Item;
use substrate_installer_domain::events::Event;
use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::releases::{Asset, Release};

use super::archive_target::ArchiveTarget;
use super::run::Run;
use super::{ApplyError, DownloadCache};
use crate::ports::downloads::{DownloadRequest, Progress, TransferControl};

impl<'r> Run<'r> {
    /// The file of `release` this host installs for `item`.
    pub(super) fn asset_of(&self, item: Item<'r>, release: &'r Release) -> Option<&'r Asset> {
        item.source().asset(release, self.installer.host)
    }

    /// Where `item`'s download unpacks, when it is an archive the layout has
    /// a folder for. A product or the installer is installed by the platform
    /// instead, whatever its package is.
    pub(super) fn archive_target(
        &self,
        item: Item<'r>,
        release: &'r Release,
    ) -> Option<ArchiveTarget<'r>> {
        if item.is_program() {
            return None;
        }
        let asset = self.asset_of(item, release)?;
        let kind = substrate_installer_domain::releases::ArchiveKind::of(&asset.name)?;
        let layout = self.installer.adapters.layout;
        let destination = layout.destination(item, &release.version)?;
        let staging = layout.staging(&destination);
        Some(ArchiveTarget {
            asset,
            kind,
            destination,
            staging,
        })
    }

    /// Tells the progress of `id`'s download as events.
    pub(super) fn progress<'a>(&'a self, id: &'a ItemId) -> impl Fn(Progress) + 'a {
        move |progress| {
            self.emit(Event::Downloading {
                item: id.clone(),
                done: progress.done,
                total: progress.total,
            });
        }
    }

    /// Downloads everything the plan installs, before anything is installed.
    pub(super) fn download_all(&mut self) -> Result<(), ApplyError> {
        let plan = self.plan;
        for change in plan.changes.iter().filter(|change| change.installs()) {
            let item = self.item(change.id())?;
            let Some(release) = plan.releases.get(change.id()) else {
                continue;
            };
            if let Some(path) = self.download_asset(item, release)? {
                self.downloaded.insert(change.id().clone(), path);
            }
        }
        Ok(())
    }

    /// Downloads `item`'s file of `release` into the cache; none when it has
    /// none (a tool's).
    pub(super) fn download_asset(
        &self,
        item: Item<'r>,
        release: &'r Release,
    ) -> Result<Option<PathBuf>, ApplyError> {
        let Some(asset) = self.asset_of(item, release) else {
            return Ok(None);
        };
        let id = item.id();
        let adapters = self.installer.adapters;
        let cache = adapters
            .layout
            .cache_file(&id, &release.version, &asset.name);
        let request = DownloadRequest {
            url: &asset.url,
            destination: &cache,
            sha256: asset.sha256.as_ref(),
        };
        let progress = self.progress(&id);
        let control = TransferControl {
            cancel: self.cancel,
            progress: &progress,
        };
        adapters.downloader.download(&request, &control)?;
        Ok(Some(cache))
    }

    /// Removes a download from the cache, unless it is to be kept. A file
    /// that cannot be removed is told as output, not an error: the package is
    /// installed.
    pub(super) fn discard_cache(&self, id: &ItemId, path: &Path) {
        if self.options.downloads == DownloadCache::Keep {
            return;
        }
        if let Err(error) = self.installer.adapters.files.remove(path) {
            self.emit(Event::Output {
                item: id.clone(),
                line: error.to_string(),
            });
        }
    }
}
