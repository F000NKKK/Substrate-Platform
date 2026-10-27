# substrate-run-process

Starting real processes for `substrate-run-app`: `SystemLauncher` implements
its `ProcessLauncher` port with the system's own environment, a process group
of its own per task and piped output, and `kill_tree` takes a process down with
everything it started. The only crate of the run context with `unsafe` (one
`kill(2)` call).

**Layer and context:** infra · run

- For: the real launcher, and `kill_tree` for other modules that start
  long-running children (language servers, debug adapters).
- Not for: the use cases (`substrate-run-app`).

## Usage

```rust
use substrate_run_app::{LogSink, spawn};
use substrate_run_domain::Task;
use substrate_run_process::SystemLauncher;

let handle = spawn(&SystemLauncher, &Task::new("true"), LogSink::new())?;
assert!(handle.wait().success);
# Ok::<(), std::io::Error>(())
```

## Depends on / used by

Depends on [`substrate-run-app`](../substrate-run-app/README.md),
[`substrate-run-domain`](../substrate-run-domain/README.md) and
[`substrate-process`](../substrate-process/README.md). Used by the platform's
`dap` and `lsp` modules and by products.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
