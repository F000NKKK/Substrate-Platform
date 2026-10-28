//! One run of a plan.

use std::collections::BTreeMap;
use std::path::PathBuf;

use substrate_installer_domain::changes::Change;
use substrate_installer_domain::channel::Item;
use substrate_installer_domain::events::Event;
use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::inventory::{Installed, InstalledSet};
use substrate_installer_domain::planning::{Mode, Plan};
use substrate_installer_domain::versions::Version;

use super::{ApplyError, ApplyOptions, Installer};
use crate::control::CancelToken;
use crate::ports::tools::ToolControl;

/// The state of one run: what was downloaded ahead of the installing.
pub(super) struct Run<'r> {
    pub(super) installer: &'r Installer<'r>,
    pub(super) plan: &'r Plan,
    pub(super) options: ApplyOptions,
    pub(super) cancel: &'r CancelToken,
    /// Packages downloaded ahead (download-all mode), by what they install.
    pub(super) downloaded: BTreeMap<ItemId, PathBuf>,
}

impl<'r> Run<'r> {
    pub(super) fn new(
        installer: &'r Installer<'r>,
        plan: &'r Plan,
        options: ApplyOptions,
        cancel: &'r CancelToken,
    ) -> Self {
        Self {
            installer,
            plan,
            options,
            cancel,
            downloaded: BTreeMap::new(),
        }
    }

    /// Every change in order, each recorded as soon as it is made.
    pub(super) fn execute(mut self) -> Result<(), ApplyError> {
        if self.options.mode == Mode::DownloadAllThenInstall {
            self.download_all()?;
        }
        let store = self.installer.adapters.store;
        let mut set = store.read()?;
        let plan = self.plan;
        for (index, change) in plan.changes.iter().enumerate() {
            self.cancel.check()?;
            self.apply_change(&mut set, index, change)?;
            store.write(&set)?;
        }
        Ok(())
    }

    pub(super) fn emit(&self, event: Event) {
        self.installer.adapters.sink.emit(event);
    }

    pub(super) fn item(&self, id: &ItemId) -> Result<Item<'r>, ApplyError> {
        self.installer
            .catalog
            .item(id)
            .ok_or_else(|| ApplyError::NotInCatalog(id.clone()))
    }

    /// Runs `work` with a control that tells a tool's lines as output of `id`.
    pub(super) fn with_output<T>(
        &self,
        id: &ItemId,
        work: impl FnOnce(&ToolControl<'_>) -> T,
    ) -> T {
        let output = |line: &str| {
            self.emit(Event::Output {
                item: id.clone(),
                line: line.to_owned(),
            });
        };
        work(&ToolControl {
            cancel: self.cancel,
            output: &output,
        })
    }

    fn apply_change(
        &mut self,
        set: &mut InstalledSet,
        index: usize,
        change: &Change,
    ) -> Result<(), ApplyError> {
        let item = self.item(change.id())?;
        match change {
            Change::Remove { version, .. } => self.remove_change(item, version, set),
            Change::Install { version, .. } | Change::Update { to: version, .. } => {
                let installing = Event::Installing {
                    item: change.id().clone(),
                    index: index + 1,
                    of: self.plan.changes.len(),
                };
                let path = self.install_change(item, version, &installing)?;
                if let Change::Update { from, .. } = change {
                    self.replace_old(item, set, (from, version), &path)?;
                }
                self.record(set, item.id(), version, path);
                Ok(())
            }
        }
    }

    fn record(&self, set: &mut InstalledSet, id: ItemId, version: &Version, path: PathBuf) {
        set.put(Installed {
            id: id.clone(),
            version: version.clone(),
            path,
        });
        self.emit(Event::Installed {
            item: id,
            version: version.clone(),
        });
    }
}
