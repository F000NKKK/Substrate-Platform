//! The platform's product installer.

use std::path::PathBuf;
use std::sync::Arc;

use substrate_installer_app::ports::placement::{
    ProductError, ProductInstall, ProductInstaller, ProductRemoval, RemovalMode,
};
use substrate_installer_app::ports::tools::ToolControl;
use substrate_installer_app::ports::unpacking::Unpacker;
use substrate_installer_domain::channel::Item;
use substrate_installer_domain::platforms::Os;
use substrate_installer_domain::versions::Version;

use super::install_job::InstallJob;
use super::launcher_dirs::LauncherDirs;
use super::package_kind::PackageKind;
use super::product_facts::ProductFacts;
use super::product_failure::io_error;
use super::windows::uninstall_on_windows;
use crate::HomeLayout;
use crate::promote::remove_any;

/// Installs and removes programs the way the operating system does:
///
/// - Linux: an `AppImage` made runnable or a Debian package's files unpacked,
///   in the product's folder, with a `.desktop` launcher entry;
/// - Windows: the setup program run silently into the product's folder;
/// - macOS: the archive unpacked into the product's folder, and the `.app`
///   linked into the user's applications folder.
///
/// Each is in its own module; what to write is decided by pure plans
/// ([`plan_launcher`](crate::plan_launcher), the setup command, the
/// application link) that the tests check on any host. Archives are
/// unpacked through the [`Unpacker`] port it is given.
#[derive(Clone)]
pub struct HostProducts {
    os: Os,
    layout: HomeLayout,
    unpacker: Arc<dyn Unpacker>,
    launchers: LauncherDirs,
}

impl std::fmt::Debug for HostProducts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HostProducts")
            .field("os", &self.os)
            .field("layout", &self.layout)
            .field("launchers", &self.launchers)
            .finish_non_exhaustive()
    }
}

impl HostProducts {
    /// An installer for `os` (the running one in a product; any in a test).
    #[must_use]
    pub fn new(
        os: Os,
        layout: HomeLayout,
        unpacker: Arc<dyn Unpacker>,
        launchers: LauncherDirs,
    ) -> Self {
        Self {
            os,
            layout,
            unpacker,
            launchers,
        }
    }

    /// An installer for the operating system this program runs on; none on
    /// one the installer has no support for.
    #[must_use]
    pub fn for_host(
        layout: HomeLayout,
        unpacker: Arc<dyn Unpacker>,
        launchers: LauncherDirs,
    ) -> Option<Self> {
        let os = if cfg!(target_os = "linux") {
            Os::Linux
        } else if cfg!(target_os = "macos") {
            Os::Macos
        } else if cfg!(target_os = "windows") {
            Os::Windows
        } else {
            return None;
        };
        Some(Self::new(os, layout, unpacker, launchers))
    }

    pub(super) fn layout(&self) -> &HomeLayout {
        &self.layout
    }

    pub(super) fn os(&self) -> Os {
        self.os
    }

    pub(super) fn unpacker(&self) -> &dyn Unpacker {
        &*self.unpacker
    }

    pub(super) fn launchers(&self) -> &LauncherDirs {
        &self.launchers
    }
}

impl ProductInstaller for HostProducts {
    fn install(
        &self,
        request: &ProductInstall<'_>,
        control: &ToolControl<'_>,
    ) -> Result<PathBuf, ProductError> {
        control.cancel.check()?;
        let facts = ProductFacts::of(request.item).ok_or(ProductError::UnsupportedPackage)?;
        let kind =
            PackageKind::detect(self.os, request.asset).ok_or(ProductError::UnsupportedPackage)?;
        let job = InstallJob {
            request,
            facts,
            kind,
        };
        match self.os {
            Os::Linux => self.install_on_linux(&job, control),
            Os::Windows => self.install_on_windows(&job, control),
            Os::Macos => self.install_on_macos(&job, control),
            _ => Err(ProductError::UnsupportedPackage),
        }
    }

    fn remove(
        &self,
        request: &ProductRemoval<'_>,
        control: &ToolControl<'_>,
    ) -> Result<(), ProductError> {
        control.cancel.check()?;
        let facts = ProductFacts::of(request.item).ok_or(ProductError::UnsupportedPackage)?;
        let folder = self.layout.product_dir(facts.id, request.version);
        if request.mode == RemovalMode::Uninstall {
            match self.os {
                Os::Linux => self.unlist_on_linux(&facts),
                Os::Windows => uninstall_on_windows(&folder, control)?,
                Os::Macos => self.unlink_on_macos(request.program),
                _ => {}
            }
        }
        remove_any(&folder).map_err(|source| io_error(&folder, source))
    }

    fn forget_other_versions(&self, item: Item<'_>, keep: &Version) -> Result<(), ProductError> {
        let facts = ProductFacts::of(item).ok_or(ProductError::UnsupportedPackage)?;
        let kept = self.layout.product_dir(facts.id, keep);
        let versions = self.layout.product_versions(facts.id);
        let Ok(entries) = std::fs::read_dir(&versions) else {
            return Ok(());
        };
        for path in entries.flatten().map(|entry| entry.path()) {
            if path != kept {
                remove_any(&path).map_err(|source| io_error(&path, source))?;
            }
        }
        Ok(())
    }
}
