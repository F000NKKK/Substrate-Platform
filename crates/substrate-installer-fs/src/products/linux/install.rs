//! Installing a program on Linux.

use std::path::{Path, PathBuf};

use substrate_installer_app::ports::placement::ProductError;
use substrate_installer_app::ports::tools::ToolControl;
use substrate_installer_app::ports::unpacking::{Strip, UnpackRequest};
use substrate_installer_domain::releases::ArchiveKind;

use super::desktop_refresh::refresh;
use super::launcher_entry::LauncherEntry;
use super::launcher_plan::plan_launcher;
use crate::products::host_products::HostProducts;
use crate::products::install_job::InstallJob;
use crate::products::package_kind::PackageKind;
use crate::products::product_failure::{ProductFailure, io_error, unpack_error};
use crate::promote::remove_any;

/// Where a Debian package keeps its programs.
const DEB_BIN: [&str; 2] = ["usr", "bin"];
/// The icon a product's folder may hold, listed in its launcher entry.
const ICON_FILE: &str = "icon.png";

impl HostProducts {
    /// Installs the package of `job` in the product's folder and lists the
    /// program among the desktop's applications; the program's path.
    pub(in crate::products) fn install_on_linux(
        &self,
        job: &InstallJob<'_>,
        control: &ToolControl<'_>,
    ) -> Result<PathBuf, ProductError> {
        let folder = self.layout().product_dir(job.facts.id, job.request.version);
        remove_any(&folder).map_err(|source| io_error(&folder, source))?;
        std::fs::create_dir_all(&folder).map_err(|source| io_error(&folder, source))?;
        let program = match job.kind {
            PackageKind::AppImage => copy_app_image(job, &folder)?,
            PackageKind::Deb => self.unpack_deb(job, &folder, control)?,
            _ => return Err(ProductError::UnsupportedPackage),
        };
        make_executable(&program)?;
        control.cancel.check()?;
        self.list_program(job, &folder, &program)?;
        Ok(program)
    }

    fn unpack_deb(
        &self,
        job: &InstallJob<'_>,
        folder: &Path,
        control: &ToolControl<'_>,
    ) -> Result<PathBuf, ProductError> {
        let request = UnpackRequest {
            kind: ArchiveKind::Deb,
            destination: folder,
            strip: Strip::KeepLayout,
        };
        self.unpacker()
            .unpack_file(job.request.package, &request, control.cancel)
            .map_err(unpack_error)?;
        let program = DEB_BIN
            .iter()
            .fold(folder.to_path_buf(), |path, part| path.join(part))
            .join(job.facts.program_file());
        if program.is_file() {
            Ok(program)
        } else {
            Err(ProductFailure::ProgramMissing { path: program }.into())
        }
    }

    /// Writes the program's launcher entry and tells the desktop.
    fn list_program(
        &self,
        job: &InstallJob<'_>,
        folder: &Path,
        program: &Path,
    ) -> Result<(), ProductError> {
        self.register_program(&job.facts, folder, program)
    }

    pub(in crate::products) fn register_program(
        &self,
        facts: &crate::products::product_facts::ProductFacts<'_>,
        folder: &Path,
        program: &Path,
    ) -> Result<(), ProductError> {
        let icon = folder.join(ICON_FILE);
        let entry = LauncherEntry {
            id: facts.id,
            name: facts.name,
            comment: facts.description,
            program,
            icon: icon.is_file().then_some(icon.as_path()),
        };
        let entries = &self.launchers().desktop_entries;
        let plan = plan_launcher(entries, &entry);
        std::fs::create_dir_all(entries).map_err(|source| io_error(entries, source))?;
        std::fs::write(&plan.path, plan.text).map_err(|source| io_error(&plan.path, source))?;
        refresh(self.launchers().caches, entries);
        Ok(())
    }
}

/// Copies the `AppImage` into the folder under the package's own name.
fn copy_app_image(job: &InstallJob<'_>, folder: &Path) -> Result<PathBuf, ProductError> {
    let name = Path::new(job.request.asset.as_str())
        .file_name()
        .ok_or(ProductError::UnsupportedPackage)?;
    let program = folder.join(name);
    std::fs::copy(job.request.package, &program).map_err(|source| io_error(&program, source))?;
    Ok(program)
}

/// Makes `path` runnable by its owner and readable by all.
#[cfg(unix)]
fn make_executable(path: &Path) -> Result<(), ProductError> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755))
        .map_err(|source| io_error(path, source))
}

/// Runnable is Unix's notion; elsewhere there is nothing to set.
#[cfg(not(unix))]
fn make_executable(_path: &Path) -> Result<(), ProductError> {
    Ok(())
}
