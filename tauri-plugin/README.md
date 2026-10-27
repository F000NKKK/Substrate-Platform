# tauri-plugin-substrate

Substrate Platform's host side as a Tauri plugin: the workspace boundary,
opening links, Source Control, terminals, language servers and debug adapters
as the commands Substrate's UI calls, the same in every product.

**Layer and context:** interface · host-ui-contract

- For: the composition root. It builds the owned state (workspace, text views,
  write lock, registries), registers the commands and turns each module's
  typed error into a `Message` on the way out.
- Not for: what the commands do. That is the context crates (`substrate-*-app` and
  their adapters); this crate only calls them from Tauri's thread model.
- Not for: command and event names or permissions' source of truth; they live
  in `substrate-host-contract`.

## Usage

```rust
let builder = tauri::Builder::default().plugin(tauri_plugin_substrate::init());
# let _ = builder;
```

Grant `substrate:default` in the app's capabilities. `init_with(Options)` takes
what is the product's: its languages, its debug adapters, its text views.

## Features

`dap`, `fs`, `git`, `lsp`, `search`, `state`, `terminal`: each turns on the
core's module of that name and its commands.

## Depends on / used by

Depends on the context crates under [`crates/`](../crates),
[`substrate-message`](../crates/substrate-message/README.md) and
[`substrate-host-contract`](../crates/substrate-host-contract/README.md). Used
by the products.

## License

AGPL-3.0-only; see [LICENSE](../LICENSE).
