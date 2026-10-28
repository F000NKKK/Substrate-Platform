//! What a tool is asked to do.

use substrate_installer_domain::sources::{CrateName, RustTarget, Source, Toolchain};
use substrate_installer_domain::versions::Version;

use super::ToolProgram;

/// What a tool is asked to do, typed. The adapter renders it as a command
/// line once, and fetches rustup first when a rustup command finds none.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ToolCommand {
    /// `rustup toolchain install <toolchain>` (minimal profile).
    InstallToolchain {
        /// The toolchain.
        toolchain: Toolchain,
    },
    /// `rustup toolchain uninstall <toolchain>`.
    UninstallToolchain {
        /// The toolchain.
        toolchain: Toolchain,
    },
    /// `rustup target add --toolchain <toolchain> <targets>`.
    AddTargets {
        /// The toolchain.
        toolchain: Toolchain,
        /// The targets.
        targets: Vec<RustTarget>,
    },
    /// `rustup target remove --toolchain <toolchain> <targets>`.
    RemoveTargets {
        /// The toolchain.
        toolchain: Toolchain,
        /// The targets.
        targets: Vec<RustTarget>,
    },
    /// `cargo install --locked <name>@<version>`.
    InstallCrate {
        /// The crate.
        name: CrateName,
        /// Its version.
        version: Version,
    },
    /// `cargo uninstall <name>`.
    UninstallCrate {
        /// The crate.
        name: CrateName,
    },
}

impl ToolCommand {
    /// The command that installs `source` at `version`; none for a source
    /// that is installed from a file.
    #[must_use]
    pub fn install(source: &Source, version: &Version) -> Option<Self> {
        match source {
            Source::CratesIo { name } => Some(Self::InstallCrate {
                name: name.clone(),
                version: version.clone(),
            }),
            Source::Rustup { toolchain } => Some(Self::InstallToolchain {
                toolchain: toolchain.clone(),
            }),
            Source::RustTargets { toolchain, targets } => Some(Self::AddTargets {
                toolchain: toolchain.clone(),
                targets: targets.clone(),
            }),
            _ => None,
        }
    }

    /// The command that removes `source`; none for a source that is
    /// installed from a file.
    #[must_use]
    pub fn uninstall(source: &Source) -> Option<Self> {
        match source {
            Source::CratesIo { name } => Some(Self::UninstallCrate { name: name.clone() }),
            Source::Rustup { toolchain } => Some(Self::UninstallToolchain {
                toolchain: toolchain.clone(),
            }),
            Source::RustTargets { toolchain, targets } => Some(Self::RemoveTargets {
                toolchain: toolchain.clone(),
                targets: targets.clone(),
            }),
            _ => None,
        }
    }

    /// The program that runs it.
    #[must_use]
    pub fn program(&self) -> ToolProgram {
        match self {
            Self::InstallCrate { .. } | Self::UninstallCrate { .. } => ToolProgram::Cargo,
            _ => ToolProgram::Rustup,
        }
    }
}

#[cfg(test)]
mod tests {
    use substrate_installer_domain::sources::RepoSlug;

    use super::*;

    fn toolchain() -> Toolchain {
        Toolchain::new("1.99").unwrap()
    }

    fn sources() -> [Source; 3] {
        [
            Source::CratesIo {
                name: CrateName::new("c").unwrap(),
            },
            Source::Rustup {
                toolchain: toolchain(),
            },
            Source::RustTargets {
                toolchain: toolchain(),
                targets: vec![RustTarget::new("t").unwrap()],
            },
        ]
    }

    #[test]
    fn tools_install_and_uninstall_by_their_programs() {
        let version = Version::new("1.2.3").unwrap();
        let programs: Vec<_> = sources()
            .iter()
            .map(|s| {
                let install = ToolCommand::install(s, &version).unwrap();
                let uninstall = ToolCommand::uninstall(s).unwrap();
                assert_eq!(install.program(), uninstall.program());
                install.program()
            })
            .collect();
        assert_eq!(
            programs,
            [ToolProgram::Cargo, ToolProgram::Rustup, ToolProgram::Rustup]
        );
        assert_eq!(ToolProgram::Cargo.to_string(), "cargo");
        assert_eq!(ToolProgram::Rustup.to_string(), "rustup");
    }

    #[test]
    fn a_crate_is_installed_at_its_version() {
        let version = Version::new("1.2.3").unwrap();
        let [krate, ..] = sources();
        assert_eq!(
            ToolCommand::install(&krate, &version),
            Some(ToolCommand::InstallCrate {
                name: CrateName::new("c").unwrap(),
                version
            })
        );
    }

    #[test]
    fn a_source_installed_from_files_has_no_command() {
        let git = Source::GitTag {
            repo: RepoSlug::new("o/r").unwrap(),
        };
        let version = Version::new("1").unwrap();
        assert_eq!(ToolCommand::install(&git, &version), None);
        assert_eq!(ToolCommand::uninstall(&git), None);
    }
}
