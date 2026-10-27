# substrate-host-contract

The contract between Substrate's host and its UI: command lists, event names,
the message catalog, and the TypeScript module the UI imports, generated from
them.

**Layer and context:** interface · host-ui-contract

- For: the one list of host commands by domain (permissions, handler
  registration and the UI's copy all come from it) and the event names.
- Not for: what a command does; that is the plugin's job
  (`tauri-plugin-substrate`) over the core's modules.
- Not for: message templates themselves; they live in the module that says
  them and use `substrate-message`.

## Usage

```rust
use substrate_host_contract::commands::{SUBSTRATE, names};

assert!(names(SUBSTRATE).contains(&"git_status"));
```

The UI's generated copy is written by the `substrate-messages` binary:

```sh
cargo run -p substrate-host-contract --all-features --bin substrate-messages
```

## Features

Each feature (`dap`, `fs`, `git`, `lsp`, `project`, `search`, `state`,
`terminal`) adds that module's messages to the catalog; `full` turns all on and
is what the binary needs.

## Depends on / used by

Depends on [`substrate-message`](../substrate-message/README.md) and
[`substrate-platform`](../../README.md) (for the module catalogs). Used by
[`tauri-plugin-substrate`](../../tauri-plugin/README.md), whose build script
reads the command list.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
