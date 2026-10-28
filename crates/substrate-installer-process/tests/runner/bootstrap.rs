//! Fetching rustup when it is missing.

use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::Cause;
use substrate_installer_app::ports::tools::{ToolCommand, ToolError};
use substrate_installer_domain::releases::HttpUrl;
use substrate_installer_process::{
    BootstrapError, CargoBin, ProcessToolRunner, RustupBootstrap, RustupInitFetcher,
};

use super::support::{LINUX, run, runner, toolchain};

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
