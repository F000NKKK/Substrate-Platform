//! A typed command as a command line.

use substrate_installer_app::ports::tools::ToolCommand;
use substrate_installer_domain::sources::{RustTarget, Toolchain};

use crate::invocation::Invocation;
use crate::unsupported_command::UnsupportedCommand;

/// Renders `command` once; rustup's and cargo's argument spelling lives here
/// and nowhere else.
pub(crate) fn render(command: &ToolCommand) -> Result<Invocation, UnsupportedCommand> {
    let arguments = match command {
        ToolCommand::InstallToolchain { toolchain } => words(&[
            "toolchain",
            "install",
            toolchain.as_str(),
            "--profile",
            "minimal",
        ]),
        ToolCommand::UninstallToolchain { toolchain } => {
            words(&["toolchain", "uninstall", toolchain.as_str()])
        }
        ToolCommand::AddTargets { toolchain, targets } => target_words("add", toolchain, targets),
        ToolCommand::RemoveTargets { toolchain, targets } => {
            target_words("remove", toolchain, targets)
        }
        ToolCommand::InstallCrate { name, version } => {
            let spec = format!("{}@{version}", name.as_str());
            words(&["install", "--locked", &spec])
        }
        ToolCommand::UninstallCrate { name } => words(&["uninstall", name.as_str()]),
        _ => return Err(UnsupportedCommand),
    };
    Ok(Invocation {
        program: command.program(),
        arguments,
    })
}

fn words(list: &[&str]) -> Vec<String> {
    list.iter().map(|word| (*word).to_owned()).collect()
}

fn target_words(verb: &str, toolchain: &Toolchain, targets: &[RustTarget]) -> Vec<String> {
    let mut arguments = words(&["target", verb, "--toolchain", toolchain.as_str()]);
    arguments.extend(targets.iter().map(|target| target.as_str().to_owned()));
    arguments
}

#[cfg(test)]
mod tests {
    use substrate_installer_domain::sources::CrateName;
    use substrate_installer_domain::versions::Version;

    use super::*;

    fn line(command: &ToolCommand) -> String {
        let invocation = render(command).unwrap();
        format!("{} {}", invocation.program, invocation.arguments.join(" "))
    }

    fn toolchain() -> Toolchain {
        Toolchain::new("1.99").unwrap()
    }

    fn targets() -> Vec<RustTarget> {
        ["a-b-c", "d-e-f"]
            .map(|t| RustTarget::new(t).unwrap())
            .to_vec()
    }

    #[test]
    fn a_toolchain_is_installed_minimal_and_uninstalled() {
        let install = ToolCommand::InstallToolchain {
            toolchain: toolchain(),
        };
        assert_eq!(
            line(&install),
            "rustup toolchain install 1.99 --profile minimal"
        );
        let uninstall = ToolCommand::UninstallToolchain {
            toolchain: toolchain(),
        };
        assert_eq!(line(&uninstall), "rustup toolchain uninstall 1.99");
    }

    #[test]
    fn targets_are_added_and_removed_for_a_toolchain() {
        let add = ToolCommand::AddTargets {
            toolchain: toolchain(),
            targets: targets(),
        };
        assert_eq!(line(&add), "rustup target add --toolchain 1.99 a-b-c d-e-f");
        let remove = ToolCommand::RemoveTargets {
            toolchain: toolchain(),
            targets: targets(),
        };
        assert_eq!(
            line(&remove),
            "rustup target remove --toolchain 1.99 a-b-c d-e-f"
        );
    }

    #[test]
    fn a_crate_is_installed_locked_at_its_version() {
        let name = CrateName::new("cargo-zigbuild").unwrap();
        let version = Version::new("v0.20.1").unwrap();
        let install = ToolCommand::InstallCrate {
            name: name.clone(),
            version,
        };
        assert_eq!(
            line(&install),
            "cargo install --locked cargo-zigbuild@0.20.1"
        );
        assert_eq!(
            line(&ToolCommand::UninstallCrate { name }),
            "cargo uninstall cargo-zigbuild"
        );
    }
}
