# substrate-terminal-app

A terminal's use cases: `spawn_utf8` starts a shell session whose output
arrives as text, `detect_shells` lists the shells a machine has. The ports
(`PtyOpener`, `Session`, `ShellProbe`) are what a pty adapter implements.

**Layer and context:** app · terminal

- For: the session's life as a host drives it: start, write, resize, kill.
- Not for: the vocabulary (`substrate-terminal-domain`) or the real pty
  (`substrate-terminal-pty`).

## Usage

```rust
use substrate_terminal_app::detect_shells;
use substrate_terminal_app::ports::ShellProbe;

struct Machine;

impl ShellProbe for Machine {
    fn is_installed(&self, program: &str) -> bool {
        program == "bash"
    }

    fn default_shell(&self) -> String {
        "/bin/sh".to_string()
    }
}

let shells = detect_shells(&Machine);
assert!(!shells.is_empty());
```

## Depends on / used by

Depends on [`substrate-terminal-domain`](../substrate-terminal-domain/README.md)
and [`substrate-lock`](../substrate-lock/README.md). Implemented by
[`substrate-terminal-pty`](../substrate-terminal-pty/README.md); used by
[`tauri-plugin-substrate`](../../tauri-plugin/README.md).

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
