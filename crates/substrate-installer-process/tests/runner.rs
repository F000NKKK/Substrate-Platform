//! The tool runner over real processes: shell scripts stand in for rustup and
//! cargo, in a folder of their own.
#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::Cause;
use substrate_installer_app::ports::tools::{
    ToolCommand, ToolControl, ToolError, ToolProgram, ToolRunner,
};
use substrate_installer_domain::platforms::{Arch, Os, PlatformTag};
use substrate_installer_domain::releases::HttpUrl;
use substrate_installer_domain::sources::{CrateName, RustTarget, Toolchain};
use substrate_installer_domain::versions::Version;
use substrate_installer_process::{
    BootstrapError, CargoBin, ProcessToolRunner, RustupBootstrap, RustupInitFetcher,
};

const LINUX: PlatformTag = PlatformTag::new(Os::Linux, Arch::X86_64);

fn script(dir: &Path, name: &str, body: &str) {
    let path = dir.join(name);
    fs::write(&path, format!("#!/bin/sh\n{body}\n")).expect("script is written");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).expect("script is executable");
}

/// A fake tool that prints its name and arguments on stdout, and an error line on stderr.
fn echoing(dir: &Path, name: &str) {
    script(dir, name, "echo \"$0 $*\" | sed 's|.*/||'\necho oops >&2");
}

fn toolchain() -> Toolchain {
    Toolchain::new("1.99").expect("a toolchain")
}

/// Runs `command`, telling the lines. A script written a moment ago can be
/// "text file busy" while another test's fork still holds its descriptor, so
/// that one error is retried.
fn run(
    runner: &ProcessToolRunner,
    command: &ToolCommand,
    cancel: &CancelToken,
) -> (Result<PathBuf, ToolError>, Vec<String>) {
    let mut attempts = 0;
    loop {
        let lines = Mutex::new(Vec::new());
        let output = |line: &str| lines.lock().expect("lock").push(line.to_owned());
        let result = runner.run(
            command,
            &ToolControl {
                cancel,
                output: &output,
            },
        );
        attempts += 1;
        if attempts < 10 && text_file_busy(&result) {
            std::thread::sleep(Duration::from_millis(30));
            continue;
        }
        return (result, lines.into_inner().expect("lock"));
    }
}

fn text_file_busy(result: &Result<PathBuf, ToolError>) -> bool {
    let Err(ToolError::Unavailable { cause, .. }) = result else {
        return false;
    };
    cause
        .downcast_ref::<std::io::Error>()
        .is_some_and(|error| error.kind() == std::io::ErrorKind::ExecutableFileBusy)
}

fn runner(dir: &Path) -> ProcessToolRunner {
    ProcessToolRunner::new(CargoBin::new(dir), LINUX)
}

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

/// Writes a `rustup-init` that installs a `rustup` next to itself and prints
/// its arguments; remembers the links it was asked for.
struct ScriptFetcher {
    links: Arc<Mutex<Vec<String>>>,
    fail: bool,
}

impl ScriptFetcher {
    fn new(fail: bool) -> (Self, Arc<Mutex<Vec<String>>>) {
        let links = Arc::new(Mutex::new(Vec::new()));
        (
            Self {
                links: Arc::clone(&links),
                fail,
            },
            links,
        )
    }
}

impl RustupInitFetcher for ScriptFetcher {
    fn fetch(&self, url: &HttpUrl, destination: &Path, _cancel: &CancelToken) -> Result<(), Cause> {
        self.links.lock().expect("lock").push(url.to_string());
        if self.fail {
            return Err("no network".into());
        }
        let bin = destination.parent().expect("a folder").join("bin");
        let body = format!(
            "mkdir -p {bin}\necho \"init $*\"\nprintf '#!/bin/sh\\necho rustup \"$@\"\\n' > {bin}/rustup\nchmod +x {bin}/rustup",
            bin = bin.display()
        );
        fs::write(destination, format!("#!/bin/sh\n{body}\n")).map_err(Cause::from)
    }
}

#[test]
fn a_missing_rustup_is_bootstrapped_then_used() {
    let home = tempfile::tempdir().unwrap();
    let scratch = home.path().join("scratch");
    let (init_script, urls) = ScriptFetcher::new(false);
    let bootstrap = RustupBootstrap::new(Box::new(init_script), &scratch);
    // The init script puts rustup in `<scratch>/bin`, which is cargo's folder here.
    let tools =
        ProcessToolRunner::new(CargoBin::new(scratch.join("bin")), LINUX).with_bootstrap(bootstrap);
    let command = ToolCommand::InstallToolchain {
        toolchain: toolchain(),
    };
    let (result, lines) = run(&tools, &command, &CancelToken::new());
    assert_eq!(result.unwrap(), scratch.join("bin"));
    assert_eq!(
        lines,
        [
            "init -y --default-toolchain none --profile minimal",
            "rustup toolchain install 1.99 --profile minimal"
        ]
    );
    assert_eq!(
        *urls.lock().unwrap(),
        ["https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"]
    );
    assert!(
        !scratch.join("rustup-init").exists(),
        "rustup-init is deleted"
    );
}

#[test]
fn a_failed_download_makes_rustup_unavailable() {
    let dir = tempfile::tempdir().unwrap();
    let (fetcher, _) = ScriptFetcher::new(true);
    let bootstrap = RustupBootstrap::new(Box::new(fetcher), dir.path().join("scratch"));
    let tools = runner(dir.path()).with_bootstrap(bootstrap);
    let command = ToolCommand::InstallToolchain {
        toolchain: toolchain(),
    };
    let (result, _) = run(&tools, &command, &CancelToken::new());
    let Err(ToolError::Unavailable { cause, .. }) = result else {
        panic!("rustup should be unavailable");
    };
    assert!(matches!(
        cause.downcast_ref::<BootstrapError>(),
        Some(BootstrapError::Fetch(_))
    ));
}

#[test]
fn cancelling_kills_the_whole_tree() {
    let dir = tempfile::tempdir().unwrap();
    script(dir.path(), "cargo", "sleep 30 &\necho started\nwait");
    let token = CancelToken::new();
    let stopper = token.clone();
    let handle = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(300));
        stopper.cancel();
    });
    let started = Instant::now();
    let command = ToolCommand::UninstallCrate {
        name: CrateName::new("x").unwrap(),
    };
    let (result, lines) = run(&runner(dir.path()), &command, &token);
    handle.join().unwrap();
    assert!(matches!(result, Err(ToolError::Cancelled)));
    assert_eq!(lines, ["started"]);
    // The `sleep` holds the output pipes: the run only returns once it is dead.
    assert!(
        started.elapsed() < Duration::from_secs(10),
        "took {:?}",
        started.elapsed()
    );
}

#[test]
fn pausing_stops_the_tool_and_resuming_lets_it_go_on() {
    let dir = tempfile::tempdir().unwrap();
    script(
        dir.path(),
        "cargo",
        "i=0\nwhile [ $i -lt 40 ]; do echo tick; i=$((i+1)); sleep 0.05; done",
    );
    let token = CancelToken::new();
    let control = token.clone();
    let seen = Arc::new(Mutex::new(0_usize));
    let at_pause = Arc::clone(&seen);
    let pauser = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(400));
        control.pause();
        std::thread::sleep(Duration::from_millis(400));
        let before = *at_pause.lock().unwrap();
        std::thread::sleep(Duration::from_millis(600));
        let during = *at_pause.lock().unwrap();
        control.resume();
        (before, during)
    });
    let command = ToolCommand::UninstallCrate {
        name: CrateName::new("x").unwrap(),
    };
    let counter = Arc::clone(&seen);
    let output = move |_: &str| *counter.lock().unwrap() += 1;
    let result = runner(dir.path()).run(
        &command,
        &ToolControl {
            cancel: &token,
            output: &output,
        },
    );
    result.unwrap();
    let (before, during) = pauser.join().unwrap();
    assert_eq!(before, during, "no line while paused");
    assert_eq!(
        *seen.lock().unwrap(),
        40,
        "every line arrives after the resume"
    );
}
