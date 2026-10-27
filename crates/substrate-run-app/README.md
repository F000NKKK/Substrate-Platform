# substrate-run-app

Running tools' use cases: `spawn` a task with its output streamed line by line
into a `LogSink`, `workflow::run` a named flow step by step (stopping at the
first failure, cancellable), and the port that starts processes
(`ports::ProcessLauncher`). Handles (`TaskHandle`, `workflow::Handle`) poll, wait and
cancel.

**Layer and context:** app · run

- For: orchestrating a run over any `ProcessLauncher`; tested with a fake one.
- Not for: starting real processes (`substrate-run-process`) or reading
  workflow files (`substrate-run-toml`).

## Usage

```rust
use substrate_run_app::LogSink;
use substrate_run_domain::Level;

let sink = LogSink::new();
sink.push(Level::Info, "hello");
assert_eq!(sink.drain().len(), 1);
```

## Depends on / used by

Depends on [`substrate-run-domain`](../substrate-run-domain/README.md). Used by
[`substrate-run-process`](../substrate-run-process/README.md) (which implements
the port), the platform's `dap` and `lsp` modules, and products.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
