# substrate-process

Starting other programs with the system's own environment, and opening links
and files with the user's default app.

**Layer and context:** infra · process

- For: `command` (a `Command` without the variables an `AppImage`'s runtime
  set), `start`, `open_link`, `open_path`, `reveal`, and the messages of
  `ProcessError`.
- Not for: streaming a program's output into a log; that is
  `substrate-run-app` over `substrate-run-process`.

## Usage

```rust
let mut ls = substrate_process::command("ls");
ls.arg("-l");
```

## Depends on / used by

Depends on [`substrate-message`](../substrate-message/README.md). Used by
`substrate-run-process`, `substrate-git-gitcli`, the platform core and
[`tauri-plugin-substrate`](../../tauri-plugin/README.md).

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
