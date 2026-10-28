//! The tool runner over real processes.

use std::path::PathBuf;

use substrate_installer_app::ports::tools::{
    ToolCommand, ToolControl, ToolError, ToolProgram, ToolRunner,
};
use substrate_installer_domain::platforms::PlatformTag;

use crate::bootstrap::{BootstrapError, RustupBootstrap};
use crate::cargo_bin::CargoBin;
use crate::render::{Invocation, UnsupportedCommand, render};
use crate::running::execute;

/// Runs rustup and cargo as real processes: one command line per
/// [`ToolCommand`], every output line told to the control, a pause stopping
/// the tool's process group and a cancel killing its tree. Returns cargo's
/// `bin` folder, or for a crate the binary `cargo install` put there. A
/// rustup command on a machine without rustup runs the [`RustupBootstrap`]
/// first, or fails with [`BootstrapError::NotConfigured`].
#[derive(Debug)]
pub struct ProcessToolRunner {
    bin: CargoBin,
    platform: PlatformTag,
    bootstrap: Option<RustupBootstrap>,
}

impl ProcessToolRunner {
    /// A runner over the tools in `bin`, on a machine of `platform` (which
    /// picks the `rustup-init` to fetch).
    #[must_use]
    pub fn new(bin: CargoBin, platform: PlatformTag) -> Self {
        Self {
            bin,
            platform,
            bootstrap: None,
        }
    }

    /// The same runner, fetching rustup with `bootstrap` when it is missing.
    #[must_use]
    pub fn with_bootstrap(mut self, bootstrap: RustupBootstrap) -> Self {
        self.bootstrap = Some(bootstrap);
        self
    }

    /// Where the program of `invocation` is; rustup is bootstrapped when
    /// missing. cargo falls back to its bare name, so starting it fails with
    /// the system's own "not found".
    fn program_path(
        &self,
        invocation: &Invocation,
        control: &ToolControl<'_>,
    ) -> Result<PathBuf, ToolError> {
        let program = invocation.program;
        if let Some(found) = self.bin.locate(program) {
            return Ok(found);
        }
        match (program, &self.bootstrap) {
            (ToolProgram::Rustup, Some(bootstrap)) => {
                bootstrap.install(self.platform, &self.bin, control)
            }
            (ToolProgram::Rustup, None) => Err(ToolError::Unavailable {
                program,
                cause: Box::new(BootstrapError::NotConfigured),
            }),
            _ => Ok(PathBuf::from(program.to_string())),
        }
    }

    /// What a successful `command` reports: the binary of an installed crate,
    /// else cargo's folder.
    fn outcome(&self, command: &ToolCommand) -> PathBuf {
        match command {
            ToolCommand::InstallCrate { name, .. } => self.bin.crate_binary(name),
            _ => self.bin.dir().to_path_buf(),
        }
    }
}

impl ToolRunner for ProcessToolRunner {
    fn run(&self, command: &ToolCommand, control: &ToolControl<'_>) -> Result<PathBuf, ToolError> {
        control.cancel.check()?;
        let invocation =
            render(command).map_err(|error: UnsupportedCommand| ToolError::Unavailable {
                program: command.program(),
                cause: Box::new(error),
            })?;
        let path = self.program_path(&invocation, control)?;
        execute(invocation.program, &path, &invocation.arguments, control)?;
        Ok(self.outcome(command))
    }
}
