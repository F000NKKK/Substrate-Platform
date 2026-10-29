//! Desktop metadata for a locally retained installer.

use std::path::Path;

use substrate_installer_app::ports::placement::ProductError;

use super::desktop_refresh::refresh;
use super::launcher_plan::entry_path;
use super::{LauncherEntry, plan_launcher};
use crate::products::product_failure::io_error;
use crate::{HostProducts, RunningPresentation, RunningProduct};

impl HostProducts {
    pub(in crate::products) fn register_running_presentation(
        &self,
        request: &RunningProduct<'_>,
        presentation: &RunningPresentation<'_>,
        program: &Path,
    ) -> Result<(), ProductError> {
        let entries = &self.launchers().desktop_entries;
        // `place_running` just wrote this legacy entry. Only remove that exact file.
        let legacy = entry_path(entries, &request.installer.id);
        let previous = std::fs::read_to_string(&legacy).map_err(|e| io_error(&legacy, e))?;
        let folder = self
            .layout()
            .product_dir(&request.installer.id, request.version);
        let icon = folder.join("icon.png");
        std::fs::write(&icon, presentation.icon).map_err(|e| io_error(&icon, e))?;
        let plan = plan_launcher(
            entries,
            &LauncherEntry {
                id: presentation.desktop_id,
                name: &request.installer.name,
                comment: Some(presentation.description),
                program,
                icon: Some(&icon),
            },
        );
        std::fs::write(&plan.path, plan.text).map_err(|e| io_error(&plan.path, e))?;
        if legacy != plan.path {
            let current = std::fs::read_to_string(&legacy).map_err(|e| io_error(&legacy, e))?;
            if current == previous {
                std::fs::remove_file(&legacy).map_err(|e| io_error(&legacy, e))?;
            }
        }
        refresh(self.launchers().caches, entries);
        Ok(())
    }
}
