# substrate-run-toml

Reading a workflow file (TOML) into `substrate-run-domain`'s workflows. The
file format lives here, in private serde mirrors of the domain types; the
domain carries no serde.

**Layer and context:** infra · run

- For: `parse_workflow`, and the `FileError` of text that is not a workflow file.
- Not for: running workflows (`substrate-run-app`).

## Usage

```rust
let file = substrate_run_toml::parse_workflow(
    "[workflow.ci]\nsteps = [{ run = \"cargo\", args = [\"test\"] }]",
)?;
assert!(file.workflow.contains_key("ci"));
# Ok::<(), substrate_run_toml::FileError>(())
```

## Depends on / used by

Depends on [`substrate-run-domain`](../substrate-run-domain/README.md). Used by
products that keep workflows in a file.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
