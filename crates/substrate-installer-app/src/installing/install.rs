//! Installing one change.

use std::path::{Path, PathBuf};

use substrate_installer_domain::channel::Item;
use substrate_installer_domain::events::Event;
use substrate_installer_domain::planning::Mode;
use substrate_installer_domain::releases::Release;
use substrate_installer_domain::versions::Version;

use super::ApplyError;
use super::archive_target::ArchiveTarget;
use super::run::Run;
use crate::ports::placement::ProductInstall;
use crate::ports::tools::ToolCommand;
use crate::ports::unpacking::{Strip, UnpackRequest};

impl<'r> Run<'r> {
    /// Installs `item` at `version`; where it went, to record.
    pub(super) fn install_change(
        &mut self,
        item: Item<'r>,
        version: &Version,
        installing: &Event,
    ) -> Result<PathBuf, ApplyError> {
        let id = item.id();
        let plan = self.plan;
        let release = plan
            .releases
            .get(&id)
            .ok_or_else(|| ApplyError::NoRelease {
                id: id.clone(),
                version: version.clone(),
            })?;
        let target = self.archive_target(item, release);
        let ahead = self.downloaded.remove(&id);
        if let (Some(target), None, Mode::InstallWhileDownloading) =
            (&target, &ahead, self.options.mode)
        {
            return self.stream_in(item, (release, target), installing);
        }
        let package = match ahead {
            Some(path) => Some(path),
            None => self.download_asset(item, release)?,
        };
        self.emit(installing.clone());
        let path = self.install_package(item, release, target.as_ref(), package.as_deref())?;
        if let Some(package) = &package {
            self.discard_cache(&id, package);
        }
        Ok(path)
    }

    /// Installs what was downloaded (`package`, none for a tool's).
    fn install_package(
        &self,
        item: Item<'r>,
        release: &'r Release,
        target: Option<&ArchiveTarget<'r>>,
        package: Option<&Path>,
    ) -> Result<PathBuf, ApplyError> {
        if item.is_program() {
            return self.install_program(item, release, package);
        }
        if let Some(target) = target {
            return self.unpack_into_place(item, target, package);
        }
        match ToolCommand::install(item.source(), &release.version) {
            Some(command) => self.run_tool(item, &command),
            None => Err(self.not_installable(item, release)),
        }
    }

    fn install_program(
        &self,
        item: Item<'r>,
        release: &'r Release,
        package: Option<&Path>,
    ) -> Result<PathBuf, ApplyError> {
        let id = item.id();
        let package = package.ok_or_else(|| ApplyError::NothingDownloaded { id: id.clone() })?;
        let asset = self
            .asset_of(item, release)
            .ok_or_else(|| ApplyError::NoAsset {
                id: id.clone(),
                version: release.version.clone(),
            })?;
        let request = ProductInstall {
            item,
            version: &release.version,
            package,
            asset: &asset.name,
        };
        let products = self.installer.adapters.products;
        Ok(self.with_output(&id, |control| products.install(&request, control))?)
    }

    fn unpack_into_place(
        &self,
        item: Item<'r>,
        target: &ArchiveTarget<'r>,
        package: Option<&Path>,
    ) -> Result<PathBuf, ApplyError> {
        let id = item.id();
        let package = package.ok_or(ApplyError::NothingDownloaded { id })?;
        let adapters = self.installer.adapters;
        adapters.files.remove(&target.staging)?;
        let request = UnpackRequest {
            kind: target.kind,
            destination: &target.staging,
            strip: Strip::DropTopFolder,
        };
        adapters
            .unpacker
            .unpack_file(package, &request, self.cancel)?;
        self.promote(target)
    }

    /// Moves the unpacked staging folder into place; where it is now.
    pub(super) fn promote(&self, target: &ArchiveTarget<'_>) -> Result<PathBuf, ApplyError> {
        let files = self.installer.adapters.files;
        Ok(files.promote(&target.staging, &target.destination)?)
    }

    fn run_tool(&self, item: Item<'r>, command: &ToolCommand) -> Result<PathBuf, ApplyError> {
        let tools = self.installer.adapters.tools;
        Ok(self.with_output(&item.id(), |control| tools.run(command, control))?)
    }

    /// Why `item`'s `release` has nothing to install from.
    fn not_installable(&self, item: Item<'r>, release: &'r Release) -> ApplyError {
        let id = item.id();
        match self.asset_of(item, release) {
            Some(asset)
                if self
                    .installer
                    .adapters
                    .layout
                    .destination(item, &release.version)
                    .is_some() =>
            {
                ApplyError::NotAnArchive {
                    id,
                    asset: asset.name.clone(),
                }
            }
            Some(_) => ApplyError::NoDestination { id },
            None if item.source().kind().installs_from_files() => ApplyError::NoAsset {
                id,
                version: release.version.clone(),
            },
            None => ApplyError::UnsupportedSource {
                id,
                kind: item.source().kind(),
            },
        }
    }
}
