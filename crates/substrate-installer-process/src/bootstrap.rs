//! Fetching rustup when the machine has none.

use std::path::{Path, PathBuf};

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::Cause;
use substrate_installer_app::ports::tools::{ToolControl, ToolError, ToolProgram};
use substrate_installer_domain::platforms::PlatformTag;
use substrate_installer_domain::releases::HttpUrl;
use substrate_installer_domain::texts::TextError;

use crate::cargo_bin::CargoBin;
use crate::executable::executable;
use crate::running::execute;
use crate::rustup_host::{rustup_host, rustup_init_url};

/// What `rustup-init` is run with: rustup alone, no toolchain (the toolchain
/// is its own step of the plan), minimal profile.
const INIT_ARGUMENTS: [&str; 5] = ["-y", "--default-toolchain", "none", "--profile", "minimal"];

/// Downloads `rustup-init` for the bootstrap. An HTTP adapter implements it;
/// this crate does no network I/O of its own.
pub trait RustupInitFetcher: Send + Sync {
    /// Downloads `url` to the file `destination`, checking `cancel` between
    /// chunks.
    ///
    /// # Errors
    ///
    /// The adapter's failure; a cancelled download fails and the runner
    /// reports the cancel.
    fn fetch(&self, url: &HttpUrl, destination: &Path, cancel: &CancelToken) -> Result<(), Cause>;
}

/// Why rustup could not be fetched; the `cause` of a
/// [`ToolError::Unavailable`] for rustup.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum BootstrapError {
    /// The machine has no rustup and the runner was given no way to fetch it.
    #[error("rustup is not installed and no bootstrap is configured")]
    NotConfigured,
    /// Rust publishes no `rustup-init` for the platform.
    #[error("rustup has no installer for {platform}")]
    UnsupportedPlatform {
        /// The platform.
        platform: PlatformTag,
    },
    /// The link of `rustup-init` is not one.
    #[error("the rustup-init link is not valid")]
    Link(#[from] TextError),
    /// The download failed.
    #[error("rustup-init could not be downloaded")]
    Fetch(#[source] Cause),
    /// The scratch folder or the downloaded file could not be prepared.
    #[error("rustup-init could not be prepared")]
    Prepare(#[source] std::io::Error),
}

/// How to get rustup onto a machine that has none: a fetcher for
/// `rustup-init` and a scratch folder to put it in. The step runs `rustup-init
/// -y --default-toolchain none --profile minimal`, then deletes the file.
pub struct RustupBootstrap {
    fetcher: Box<dyn RustupInitFetcher>,
    scratch: PathBuf,
}

impl std::fmt::Debug for RustupBootstrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RustupBootstrap")
            .field("scratch", &self.scratch)
            .finish_non_exhaustive()
    }
}

impl RustupBootstrap {
    /// A bootstrap that downloads through `fetcher` into `scratch`.
    #[must_use]
    pub fn new(fetcher: Box<dyn RustupInitFetcher>, scratch: impl Into<PathBuf>) -> Self {
        Self {
            fetcher,
            scratch: scratch.into(),
        }
    }

    /// Installs rustup into `bin` for `platform`; returns where it is.
    pub(crate) fn install(
        &self,
        platform: PlatformTag,
        bin: &CargoBin,
        control: &ToolControl<'_>,
    ) -> Result<PathBuf, ToolError> {
        let init = self.download(platform, control)?;
        let arguments = INIT_ARGUMENTS.map(str::to_owned);
        let ran = execute(ToolProgram::Rustup, &init, &arguments, control);
        std::fs::remove_file(&init).ok();
        ran?;
        Ok(bin
            .locate(ToolProgram::Rustup)
            .unwrap_or_else(|| bin.rustup_path()))
    }

    /// Downloads `rustup-init` and makes it runnable; returns its path.
    fn download(
        &self,
        platform: PlatformTag,
        control: &ToolControl<'_>,
    ) -> Result<PathBuf, ToolError> {
        let unavailable = |cause: BootstrapError| ToolError::Unavailable {
            program: ToolProgram::Rustup,
            cause: Box::new(cause),
        };
        let host = rustup_host(platform)
            .ok_or_else(|| unavailable(BootstrapError::UnsupportedPlatform { platform }))?;
        let url = rustup_init_url(platform, host).map_err(|e| unavailable(e.into()))?;
        std::fs::create_dir_all(&self.scratch)
            .map_err(|e| unavailable(BootstrapError::Prepare(e)))?;
        let init = self.scratch.join(executable("rustup-init"));
        if let Err(cause) = self.fetcher.fetch(&url, &init, control.cancel) {
            return Err(if control.cancel.is_cancelled() {
                ToolError::Cancelled
            } else {
                unavailable(BootstrapError::Fetch(cause))
            });
        }
        make_executable(&init).map_err(|e| unavailable(BootstrapError::Prepare(e)))?;
        Ok(init)
    }
}

#[cfg(unix)]
fn make_executable(file: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(file, std::fs::Permissions::from_mode(0o755))
}

#[cfg(not(unix))]
fn make_executable(_file: &Path) -> std::io::Result<()> {
    Ok(())
}
