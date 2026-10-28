//! What a catalog's components look like on this machine.

use std::path::PathBuf;

use substrate_installer_app::ports::tools::ToolProgram;
use substrate_installer_domain::channel::{Catalog, Component};
use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::inventory::{Found, Origin};
use substrate_installer_domain::sources::{Source, SourceKind};
use substrate_installer_domain::versions::Version;

use super::crate_list::crate_version;
use super::installed_targets::has_every_target;
use super::probe::Probe;
use super::toolchain_list::has_toolchain;
use crate::cargo_bin::CargoBin;

/// One scan of the machine for a catalog. `rustup toolchain list` and `cargo
/// install --list` are asked at most once, and only when the catalog has a
/// source that needs them.
pub(super) struct Scan<'a> {
    probe: &'a dyn Probe,
    bin: &'a CargoBin,
    toolchains: Option<String>,
    crates: Option<String>,
}

impl<'a> Scan<'a> {
    /// Asks the machine what `catalog` needs asked.
    pub(super) fn new(probe: &'a dyn Probe, bin: &'a CargoBin, catalog: &Catalog) -> Self {
        let uses = |kind: SourceKind| catalog.components().iter().any(|c| c.source.kind() == kind);
        let mut scan = Self {
            probe,
            bin,
            toolchains: None,
            crates: None,
        };
        if uses(SourceKind::Rustup) {
            scan.toolchains = scan.ask(ToolProgram::Rustup, &["toolchain", "list"]);
        }
        if uses(SourceKind::CratesIo) {
            scan.crates = scan.ask(ToolProgram::Cargo, &["install", "--list"]);
        }
        scan
    }

    /// The component, when the machine has it.
    pub(super) fn find(&self, component: &Component) -> Option<Found> {
        let (version, path) = match &component.source {
            Source::Rustup { toolchain } => {
                let list = self.toolchains.as_deref()?;
                has_toolchain(list, toolchain).then_some(())?;
                (Version::new(toolchain.as_str()).ok()?, None)
            }
            Source::RustTargets { toolchain, targets } => {
                let list = self.ask(
                    ToolProgram::Rustup,
                    &[
                        "target",
                        "list",
                        "--installed",
                        "--toolchain",
                        toolchain.as_str(),
                    ],
                )?;
                has_every_target(&list, targets).then_some(())?;
                (Version::new(toolchain.as_str()).ok()?, None)
            }
            Source::CratesIo { name } => {
                let version = crate_version(self.crates.as_deref()?, name)?;
                (version, Some(self.bin.crate_binary(name)))
            }
            _ => return None,
        };
        Some(found(component, version, path))
    }

    fn ask(&self, program: ToolProgram, arguments: &[&str]) -> Option<String> {
        let path = self
            .bin
            .locate(program)
            .unwrap_or_else(|| PathBuf::from(program.to_string()));
        self.probe.output(&path, arguments)
    }
}

fn found(component: &Component, version: Version, path: Option<PathBuf>) -> Found {
    Found {
        id: ItemId::Component(component.id.clone()),
        version,
        path,
        origin: Origin::System,
    }
}
