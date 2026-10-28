//! Retaining bundles atomically and registering their desktop entries.

use std::path::{Path, PathBuf};

use substrate_installer_app::ports::placement::{InstallLayout, ProductError};
use substrate_installer_app::ports::tools::ToolControl;
use substrate_installer_domain::channel::Item;
use substrate_installer_domain::platforms::Os;

use super::{RunningBundle, RunningProduct};
use crate::copy_tree::copy_tree;
use crate::products::HostProducts;
use crate::products::product_facts::ProductFacts;
use crate::products::product_failure::io_error;
use crate::promote::{promote, remove_any};

impl HostProducts {
    /// Retains a running bundle under the home and registers its launcher.
    /// Repeated calls from the retained copy repair its launcher without
    /// replacing files. Windows setup already places and registers the program.
    ///
    /// # Errors
    ///
    /// [`ProductError`] on cancellation, a bundle for another platform, or
    /// failure to copy the complete bundle or register its launcher.
    pub fn place_running(
        &self,
        request: &RunningProduct<'_>,
        control: &ToolControl<'_>,
    ) -> Result<PathBuf, ProductError> {
        control.cancel.check()?;
        let facts = ProductFacts::of(Item::Installer(request.installer))
            .ok_or(ProductError::UnsupportedPackage)?;
        let folder = self.layout().product_dir(facts.id, request.version);
        match (self.os(), request.bundle) {
            (Os::Linux, RunningBundle::AppImage(source)) => {
                let program = self.retain_bundle(source, &folder, control)?;
                self.register_program(&facts, &folder, &program)?;
                Ok(program)
            }
            (Os::Macos, RunningBundle::Application(source)) => {
                let application = self.retain_bundle(source, &folder, control)?;
                self.link_application(&application)?;
                Ok(application)
            }
            (Os::Windows, RunningBundle::InstalledProgram(program)) => {
                std::fs::canonicalize(program).map_err(|source| io_error(program, source))
            }
            _ => Err(ProductError::UnsupportedPackage),
        }
    }

    fn retain_bundle(
        &self,
        source: &Path,
        folder: &Path,
        control: &ToolControl<'_>,
    ) -> Result<PathBuf, ProductError> {
        let source = std::fs::canonicalize(source).map_err(|error| io_error(source, error))?;
        let name = source.file_name().ok_or(ProductError::UnsupportedPackage)?;
        let destination = folder.join(name);
        if std::fs::canonicalize(&destination).is_ok_and(|path| path == source) {
            return Ok(destination);
        }
        let staging = self.layout().staging(folder);
        remove_any(&staging).map_err(|error| io_error(&staging, error))?;
        std::fs::create_dir_all(&staging).map_err(|error| io_error(&staging, error))?;
        copy_bundle(&source, &staging.join(name)).map_err(|error| io_error(&source, error))?;
        control.cancel.check()?;
        promote(&staging, folder).map_err(|error| io_error(folder, error))?;
        Ok(destination)
    }
}

fn copy_bundle(source: &Path, destination: &Path) -> std::io::Result<()> {
    if source.is_dir() {
        copy_tree(source, destination)
    } else {
        std::fs::copy(source, destination)?;
        make_runnable(destination)
    }
}

#[cfg(unix)]
fn make_runnable(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))
}

#[cfg(not(unix))]
fn make_runnable(_path: &Path) -> std::io::Result<()> {
    Ok(())
}
