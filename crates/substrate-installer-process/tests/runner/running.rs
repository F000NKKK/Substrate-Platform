//! Installing and removing through the tool runner.

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::tools::{ToolCommand, ToolError, ToolProgram};
use substrate_installer_domain::sources::{CrateName, RustTarget};
use substrate_installer_domain::versions::Version;
use substrate_installer_process::BootstrapError;

use super::support::{echoing, run, runner, script, toolchain};

#[test]
fn a_toolchain_is_installed_and_its_lines_are_told() {
    let dir = tempfile::tempdir().unwrap();
    echoing(dir.path(), "rustup");
    let command = ToolCommand::InstallToolchain {
        toolchain: toolchain(),
    };
    let (result, mut lines) = run(&runner(dir.path()), &command, &CancelToken::new());
    assert_eq!(result.unwrap(), dir.path());
    lines.sort();
    assert_eq!(
        lines,
        ["oops", "rustup toolchain install 1.99 --profile minimal"]
    );
}

#[test]
fn targets_are_added_and_the_folder_is_returned() {
    let dir = tempfile::tempdir().unwrap();
    echoing(dir.path(), "rustup");
    let targets = vec![
        RustTarget::new("a-b").unwrap(),
        RustTarget::new("c-d").unwrap(),
    ];
    let command = ToolCommand::AddTargets {
        toolchain: toolchain(),
        targets,
    };
    let (result, lines) = run(&runner(dir.path()), &command, &CancelToken::new());
    assert_eq!(result.unwrap(), dir.path());
    assert!(lines.contains(&"rustup target add --toolchain 1.99 a-b c-d".to_owned()));
}

#[test]
fn a_crate_is_installed_locked_and_its_binary_is_returned() {
    let dir = tempfile::tempdir().unwrap();
    echoing(dir.path(), "cargo");
    let name = CrateName::new("studio-cli").unwrap();
    let command = ToolCommand::InstallCrate {
        name,
        version: Version::new("0.47.1").unwrap(),
    };
    let (result, lines) = run(&runner(dir.path()), &command, &CancelToken::new());
    assert_eq!(result.unwrap(), dir.path().join("studio-cli"));
    assert!(lines.contains(&"cargo install --locked studio-cli@0.47.1".to_owned()));
}

#[test]
fn a_failing_tool_reports_its_exit_code() {
    let dir = tempfile::tempdir().unwrap();
    script(dir.path(), "cargo", "echo broken >&2\nexit 101");
    let command = ToolCommand::UninstallCrate {
        name: CrateName::new("x").unwrap(),
    };
    let (result, lines) = run(&runner(dir.path()), &command, &CancelToken::new());
    assert!(matches!(
        result,
        Err(ToolError::Failed {
            program: ToolProgram::Cargo,
            code: Some(101)
        })
    ));
    assert_eq!(lines, ["broken"]);
}

#[test]
fn a_cancelled_token_runs_nothing() {
    let dir = tempfile::tempdir().unwrap();
    script(dir.path(), "cargo", "touch ran");
    let token = CancelToken::new();
    token.cancel();
    let command = ToolCommand::UninstallCrate {
        name: CrateName::new("x").unwrap(),
    };
    let (result, _) = run(&runner(dir.path()), &command, &token);
    assert!(matches!(result, Err(ToolError::Cancelled)));
    assert!(!dir.path().join("ran").exists());
}

#[test]
fn rustup_missing_without_a_bootstrap_is_unavailable() {
    let dir = tempfile::tempdir().unwrap();
    let command = ToolCommand::InstallToolchain {
        toolchain: toolchain(),
    };
    let (result, _) = run(&runner(dir.path()), &command, &CancelToken::new());
    let Err(ToolError::Unavailable { program, cause }) = result else {
        panic!("rustup should be unavailable");
    };
    assert_eq!(program, ToolProgram::Rustup);
    assert!(matches!(
        cause.downcast_ref::<BootstrapError>(),
        Some(BootstrapError::NotConfigured)
    ));
}
