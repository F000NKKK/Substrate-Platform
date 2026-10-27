# substrate-run-domain

Running tools' vocabulary: the `Task` to run, how it ended (`TaskExit`), the
log's `Level` and `LogLine`, and named multi-step `workflow`s (`File`,
`Definition`, `Step`, `Outcome`) with their `${VAR}` substitution. Pure data and
rules: no processes, no threads, no serde.

**Layer and context:** domain · run

- For: describing what to run and what happened.
- Not for: starting processes or streaming their output (`substrate-run-app`
  over `substrate-run-process`) and reading workflow files
  (`substrate-run-toml`).

## Usage

```rust
use substrate_run_domain::Task;

let task = Task::new("cargo").arg("build").cwd("/work");
assert_eq!(task.command_line(), "$ cargo build");
```

## Depends on / used by

Depends on nothing in the workspace. Used by
[`substrate-run-app`](../substrate-run-app/README.md),
[`substrate-run-toml`](../substrate-run-toml/README.md),
[`substrate-run-process`](../substrate-run-process/README.md) and products.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
