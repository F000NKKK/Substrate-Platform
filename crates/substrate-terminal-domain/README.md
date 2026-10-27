# substrate-terminal-domain

A terminal's vocabulary: its `TerminalSize`, the `Shell`s a machine offers,
`SessionOptions` (what to start), `Utf8Chunker` (output decoded as it streams,
a character split across reads stays whole) and `TerminalError`. No I/O, no
serde.

**Layer and context:** domain · terminal

- For: describing a terminal session and decoding its output.
- Not for: opening a pty or finding installed shells
  (`substrate-terminal-app`'s ports, `substrate-terminal-pty`'s adapter).

## Usage

```rust
use substrate_terminal_domain::Utf8Chunker;

let bytes = "héllo".as_bytes();
let mut chunker = Utf8Chunker::new();
let mut text = chunker.push(&bytes[..2]);
text.push_str(&chunker.push(&bytes[2..]));
assert_eq!(text, "héllo");
```

## Depends on / used by

Depends on [`substrate-message`](../substrate-message/README.md). Used by
[`substrate-terminal-app`](../substrate-terminal-app/README.md),
[`substrate-terminal-pty`](../substrate-terminal-pty/README.md), the plugin and
products.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
