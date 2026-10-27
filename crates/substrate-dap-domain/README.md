# substrate-dap-domain

Debug adapters' vocabulary: the adapters a product knows (`Adapters`,
`Adapter`, `AdapterDefinition`), the ids the protocol numbers things by
(`ThreadId`, `FrameId`, `VariablesReference`, `Seq`), the requests (`Command`,
`EvaluateContext`, `Debuggee`), what an adapter says unasked (`Event`) and why
talking to one fails (`DapError`). Arguments and results stay JSON values. No
I/O, no serde derives.

**Layer and context:** domain · debug-adapter

- For: describing adapters and the debug session's vocabulary.
- Not for: speaking the protocol (`substrate-dap-app`) or reading the
  product's adapter list (`substrate-dap-toml`).

## Usage

```rust
use substrate_dap_domain::{Command, ThreadId};

assert_eq!(Command::from("stackTrace"), Command::StackTrace);
assert_eq!(Command::StepOut.to_string(), "stepOut");
assert_eq!(ThreadId(3).to_string(), "3");
```

## Depends on / used by

Depends on [`substrate-message`](../substrate-message/README.md). Used by
[`substrate-dap-app`](../substrate-dap-app/README.md),
[`substrate-dap-toml`](../substrate-dap-toml/README.md), the plugin and products.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
