//! Installing a program on Windows.

use std::path::PathBuf;

use substrate_installer_app::ports::placement::ProductError;
use substrate_installer_app::ports::tools::ToolControl;

use super::setup_command::install_command;
use super::setup_run::run_setup;
use crate::products::host_products::{HostProducts, InstallJob};
use crate::products::package_kind::PackageKind;
use crate::products::product_failure::io_error;
use crate::promote::remove_any;

/// What the program's file ends with.
const PROGRAM_EXTENSION: &str = "exe";

impl HostProducts {
    /// Runs the setup silently into the product's folder; the path of the
    /// program it installs (the setup makes its own shortcuts).
    pub(in crate::products) fn install_on_windows(
        &self,
        job: &InstallJob<'_>,
        control: &ToolControl<'_>,
    ) -> Result<PathBuf, ProductError> {
        if job.kind != PackageKind::SetupExe {
            return Err(ProductError::UnsupportedPackage);
        }
        let folder = self.layout().product_dir(job.facts.id, job.request.version);
        remove_any(&folder).map_err(|source| io_error(&folder, source))?;
        std::fs::create_dir_all(&folder).map_err(|source| io_error(&folder, source))?;
        run_setup(&install_command(job.request.package, &folder), control)?;
        let mut program = folder.join(job.facts.program_file());
        program.set_extension(PROGRAM_EXTENSION);
        Ok(program)
    }
}
