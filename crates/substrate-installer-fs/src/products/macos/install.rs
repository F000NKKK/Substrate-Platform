//! Installing a program on macOS.

use std::path::{Path, PathBuf};

use substrate_installer_app::ports::placement::ProductError;
use substrate_installer_app::ports::tools::ToolControl;
use substrate_installer_app::ports::unpacking::{Strip, UnpackRequest};

use super::application_link::application_link;
use super::find_application::find_application;
use crate::products::host_products::HostProducts;
use crate::products::install_job::InstallJob;
use crate::products::package_kind::PackageKind;
use crate::products::product_failure::{ProductFailure, io_error, unpack_error};
use crate::promote::remove_any;

impl HostProducts {
    /// Unpacks the package into the product's folder and links its `.app`
    /// among the user's applications; the `.app`'s path.
    pub(in crate::products) fn install_on_macos(
        &self,
        job: &InstallJob<'_>,
        control: &ToolControl<'_>,
    ) -> Result<PathBuf, ProductError> {
        let PackageKind::AppArchive(kind) = job.kind else {
            return Err(ProductError::UnsupportedPackage);
        };
        let folder = self.layout().product_dir(job.facts.id, job.request.version);
        remove_any(&folder).map_err(|source| io_error(&folder, source))?;
        let request = UnpackRequest {
            kind,
            destination: &folder,
            strip: Strip::KeepLayout,
        };
        self.unpacker()
            .unpack_file(job.request.package, &request, control.cancel)
            .map_err(unpack_error)?;
        let application = find_application(&folder, &job.facts.program_file())
            .ok_or(ProductFailure::NoApplication { folder })?;
        self.link_application(&application)?;
        Ok(application)
    }

    /// Links `application` into the applications folder, replacing a link
    /// an earlier version made.
    pub(in crate::products) fn link_application(
        &self,
        application: &Path,
    ) -> Result<(), ProductError> {
        let applications = &self.launchers().applications;
        let Some(link) = application_link(applications, application) else {
            return Ok(());
        };
        std::fs::create_dir_all(applications).map_err(|source| io_error(applications, source))?;
        remove_any(&link).map_err(|source| io_error(&link, source))?;
        make_link(application, &link).map_err(|source| io_error(&link, source))
    }
}

#[cfg(unix)]
fn make_link(application: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(application, link)
}

/// Links are Unix's here, and macOS is Unix.
#[cfg(not(unix))]
fn make_link(_application: &Path, _link: &Path) -> std::io::Result<()> {
    Ok(())
}
