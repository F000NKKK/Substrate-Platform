//! Fake tools and the runner every test of the suite drives.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::tools::{ToolCommand, ToolControl, ToolError, ToolRunner};
use substrate_installer_domain::platforms::{Arch, Os, PlatformTag};
use substrate_installer_domain::sources::Toolchain;
use substrate_installer_process::{CargoBin, ProcessToolRunner};

pub(crate) const LINUX: PlatformTag = PlatformTag::new(Os::Linux, Arch::X86_64);

pub(crate) fn script(dir: &Path, name: &str, body: &str) {
    let path = dir.join(name);
    fs::write(&path, format!("#!/bin/sh\n{body}\n")).expect("script is written");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).expect("script is executable");
}

/// A fake tool that prints its name and arguments on stdout, and an error line on stderr.
pub(crate) fn echoing(dir: &Path, name: &str) {
    script(dir, name, "echo \"$0 $*\" | sed 's|.*/||'\necho oops >&2");
}

pub(crate) fn toolchain() -> Toolchain {
    Toolchain::new("1.99").expect("a toolchain")
}

/// Runs `command`, telling the lines. A script written a moment ago can be
/// "text file busy" while another test's fork still holds its descriptor, so
/// that one error is retried.
pub(crate) fn run(
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

pub(crate) fn runner(dir: &Path) -> ProcessToolRunner {
    ProcessToolRunner::new(CargoBin::new(dir), LINUX)
}
