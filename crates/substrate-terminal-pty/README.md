# substrate-terminal-pty

A terminal on a real, cross-platform pty (Windows, Linux, macOS) through
`portable-pty`: `PortablePty` starts the shell, `SystemShells` finds the
installed ones on the `PATH`, `default_shell` names the user's own.

**Layer and context:** infra · terminal

- For: the real implementations of `substrate-terminal-app`'s ports. The shell
  starts with the system's own environment, not an AppImage's.
- Not for: the session's use cases (`substrate-terminal-app`).

## Usage

```rust
use substrate_terminal_app::ports::ShellProbe;
use substrate_terminal_pty::SystemShells;

assert!(!SystemShells.default_shell().is_empty());
```

## Depends on / used by

Depends on [`substrate-terminal-app`](../substrate-terminal-app/README.md),
[`substrate-terminal-domain`](../substrate-terminal-domain/README.md) and
[`substrate-process`](../substrate-process/README.md). Used by
[`tauri-plugin-substrate`](../../tauri-plugin/README.md) and products.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
