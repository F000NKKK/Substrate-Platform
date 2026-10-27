# substrate-dap-toml

Reads a product's list of debug adapters from TOML (`parse_adapters`) into
`substrate-dap-domain`'s `Adapters`. The TOML shape (`[adapter.<id>]` with
`languages`, `command`, `args`, `env` and `launch_defaults`) lives here and
nowhere else.

**Layer and context:** infra · debug-adapter

- For: the file format of the product's adapter list.
- Not for: the adapters' vocabulary (`substrate-dap-domain`) or running them
  (`substrate-dap-app`).

## Usage

```rust
use substrate_dap_toml::parse_adapters;

let adapters = parse_adapters(
    r#"
    [adapter.lldb]
    languages = ["rust"]
    command = "lldb-dap"
    "#,
)?;
assert_eq!(adapters.for_language("rust").map(|a| a.id.as_str()), Some("lldb"));
# Ok::<(), substrate_dap_domain::DapError>(())
```

## Depends on / used by

Depends on [`substrate-dap-domain`](../substrate-dap-domain/README.md). Used by
products, which hand the parsed `Adapters` to `tauri-plugin-substrate`.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
