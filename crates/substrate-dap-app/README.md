# substrate-dap-app

A running debug adapter spoken to over the Debug Adapter Protocol (`Client`):
requests matched to their responses by sequence number, the adapter's events
decoded and passed on, its own requests refused, the `disconnect` farewell. The
`wire` module holds the serde shape of an event (`EventDto`, `decode_event`),
the one the protocol and the UI share.

**Layer and context:** app · debug-adapter

- For: protocol plumbing only. Arguments and results stay JSON; the debug
  session that asked for a stack trace reads it. The adapter's process is
  started through the run context's `ServiceLauncher` port and its log is a
  `LogSink`.
- Not for: the adapters and their vocabulary (`substrate-dap-domain`) or
  reading the product's adapter list (`substrate-dap-toml`).

## Usage

```rust
use substrate_dap_app::Client;

// Over any pair of streams: here, a pipe that nothing answers on.
let (input, _adapter_end) = std::io::pipe()?;
let (_client_end, output) = std::io::pipe()?;
let client = Client::connect("fake-dap", input, output, |_event| {});
assert_eq!(client.name(), "fake-dap");
# Ok::<(), std::io::Error>(())
```

## Depends on / used by

Depends on [`substrate-dap-domain`](../substrate-dap-domain/README.md),
[`substrate-framing`](../substrate-framing/README.md),
[`substrate-lock`](../substrate-lock/README.md),
[`substrate-run-app`](../substrate-run-app/README.md) and
[`substrate-run-domain`](../substrate-run-domain/README.md). Used by
[`tauri-plugin-substrate`](../../tauri-plugin/README.md) and products.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
