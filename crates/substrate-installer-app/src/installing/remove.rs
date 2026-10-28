//! Removing, and replacing an older version.

use std::path::Path;

use substrate_installer_domain::channel::Item;
use substrate_installer_domain::events::Event;
use substrate_installer_domain::inventory::InstalledSet;
use substrate_installer_domain::versions::Version;

use super::ApplyError;
use super::run::Run;
use crate::ports::placement::{ProductRemoval, RemovalMode};
use crate::ports::tools::ToolCommand;

impl<'r> Run<'r> {
    /// Removes `item` at `version` and forgets its record.
    pub(super) fn remove_change(
        &self,
        item: Item<'r>,
        version: &Version,
        set: &mut InstalledSet,
    ) -> Result<(), ApplyError> {
        let id = item.id();
        let path = set
            .get(&id, version)
            .map(|record| record.path.clone())
            .unwrap_or_default();
        self.remove_files(item, version, &path, RemovalMode::Uninstall)?;
        set.remove(&id, version);
        self.emit(Event::Removed { item: id });
        Ok(())
    }

    /// After an update `from` to `to`: the old version's files go (unless the
    /// new one is in the same place, or the tool replaced it in place), and
    /// its record; a product's every other version too.
    pub(super) fn replace_old(
        &self,
        item: Item<'r>,
        set: &mut InstalledSet,
        (from, to): (&Version, &Version),
        new_path: &Path,
    ) -> Result<(), ApplyError> {
        let id = item.id();
        let old = set.get(&id, from).map(|record| record.path.clone());
        if let Some(old) = old.filter(|old| old != new_path && !item.source().replaces_in_place()) {
            self.remove_files(item, from, &old, RemovalMode::Replaced)?;
        }
        set.remove(&id, from);
        if item.is_program() {
            self.installer
                .adapters
                .products
                .forget_other_versions(item, to)?;
        }
        Ok(())
    }

    /// Removes what `item` at `version` put on the machine (`path`: where it
    /// was recorded).
    fn remove_files(
        &self,
        item: Item<'r>,
        version: &Version,
        path: &Path,
        mode: RemovalMode,
    ) -> Result<(), ApplyError> {
        let id = item.id();
        let adapters = self.installer.adapters;
        if item.is_program() {
            let request = ProductRemoval {
                item,
                version,
                program: path,
                mode,
            };
            return Ok(
                self.with_output(&id, |control| adapters.products.remove(&request, control))?
            );
        }
        if let Some(command) = ToolCommand::uninstall(item.source()) {
            self.with_output(&id, |control| adapters.tools.run(&command, control))?;
            return Ok(());
        }
        let folder = adapters.layout.destination(item, version);
        match folder.as_deref().unwrap_or(path) {
            place if place.as_os_str().is_empty() => Ok(()),
            place => Ok(adapters.files.remove(place)?),
        }
    }
}
