# substrate-lsp-app

A running language server spoken to over the Language Server Protocol
(`Client`): requests matched to their responses by id, notifications both ways,
the server's own requests answered, the document lifecycle
(`didOpen`/`didChange`/`didClose`) and the `initialize`…`exit` handshake.

**Layer and context:** app · language-server

- For: protocol plumbing only. Params and results stay JSON; the editor that
  asked for a completion reads it. The server's process is started through the
  run context's `ServiceLauncher` port and its log is a `LogSink`.
- Not for: the languages and their servers (`substrate-lsp-domain`) or reading
  the product's list of them (`substrate-lsp-toml`).

## Usage

```rust
use substrate_lsp_app::Client;

// Over any pair of streams: here, a pipe that nothing answers on.
let (input, _server_end) = std::io::pipe()?;
let (_client_end, output) = std::io::pipe()?;
let client = Client::connect("fake-ls", input, output, |_notification| {});
assert_eq!(client.name(), "fake-ls");
# Ok::<(), std::io::Error>(())
```

## Depends on / used by

Depends on [`substrate-lsp-domain`](../substrate-lsp-domain/README.md),
[`substrate-framing`](../substrate-framing/README.md),
[`substrate-lock`](../substrate-lock/README.md),
[`substrate-run-app`](../substrate-run-app/README.md) and
[`substrate-run-domain`](../substrate-run-domain/README.md). Used by
[`tauri-plugin-substrate`](../../tauri-plugin/README.md) and products.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
