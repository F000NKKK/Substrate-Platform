//! Removing a program on Windows.

use std::path::Path;

use substrate_installer_app::ports::placement::ProductError;
use substrate_installer_app::ports::tools::ToolControl;

use super::setup_command::uninstall_command;
use super::setup_run::run_setup;

/// What an NSIS setup leaves in the folder to remove itself.
const UNINSTALLER: &str = "uninstall.exe";

/// Runs the product's own uninstaller silently, when its folder has one.
pub(in crate::products) fn uninstall_on_windows(
    folder: &Path,
    control: &ToolControl<'_>,
) -> Result<(), ProductError> {
    let uninstaller = folder.join(UNINSTALLER);
    if uninstaller.is_file() {
        run_setup(&uninstall_command(&uninstaller), control)?;
    }
    Ok(())
}
