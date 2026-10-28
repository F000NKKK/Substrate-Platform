# substrate-installer-process

The installer context's process adapter: it implements `ToolRunner` and
`SystemDetector` of `substrate-installer-app` over real `rustup` and `cargo`
processes.

- `ProcessToolRunner` renders a typed `ToolCommand` as one command line
  (`rustup toolchain install <tc> --profile minimal`, `rustup target add
  --toolchain <tc> <targets>`, `cargo install --locked <name>@<version>`, the
  uninstalls), runs it with the system's own environment
  (`substrate-process`) as the leader of a process group of its own, tells
  every line it prints (stdout and stderr), pauses it with `SIGSTOP` to the
  group (Unix; elsewhere a pause waits for the step to end) and, once the
  token is cancelled, kills the whole tree (`substrate_run_process::kill_tree`).
  It returns cargo's `bin` folder, or the binary `cargo install` put there.
- **Bootstrap.** A rustup command on a machine without rustup first runs a
  documented step: a `RustupBootstrap` downloads `rustup-init` through a
  `RustupInitFetcher` (an HTTP adapter implements it; this crate does no
  network I/O), runs it with `-y --default-toolchain none --profile minimal`
  (the toolchain is its own step of the plan) and deletes it. Without a
  bootstrap the command fails with `ToolError::Unavailable` whose cause is
  `BootstrapError::NotConfigured`; an unsupported platform, a failed download
  and a failed preparation are the other `BootstrapError`s.
- `ProcessSystemDetector` finds what the catalog's sources imply is already
  on the machine, as `Origin::System`: a `Rustup` toolchain (`rustup toolchain
  list`), `RustTargets` (`rustup target list --installed --toolchain`) and
  `CratesIo` crates (`cargo install --list`). Each list is asked at most once;
  GitHub and tag sources are not found (the installer's records cover them).
  Parsing the lists is pure and tested on fixtures.
- `CargoBin` is cargo's `bin` folder (`$CARGO_HOME/bin`, else `~/.cargo/bin`);
  `host_platform` is the machine's `PlatformTag`.

**Layer and context:** infra · installer

- For: running rustup and cargo for an installer, and looking for them.
- Not for: downloading, unpacking or placing files (the other adapters), or
  deciding what to install (`substrate-installer-app`).

## Usage

```rust
use substrate_installer_process::{CargoBin, ProcessSystemDetector, ProcessToolRunner, host_platform};

if let (Some(bin), Some(platform)) = (CargoBin::from_environment(), host_platform()) {
    let _runner = ProcessToolRunner::new(bin.clone(), platform);
    let _detector = ProcessSystemDetector::new(bin);
}
```

## Depends on / used by

Depends on [`substrate-installer-app`](../substrate-installer-app/README.md) and
`-domain`, [`substrate-process`](../substrate-process/README.md) and
[`substrate-run-process`](../substrate-run-process/README.md) (`kill_tree`).
Used by products, which hand the runner and the detector to the installer.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
