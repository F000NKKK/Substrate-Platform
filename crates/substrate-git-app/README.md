# substrate-git-app

Source control's use cases: the ports a git adapter implements, and the write
operations that never run at the same time.

**Layer and context:** app · source-control

- For: the ports (`WorkingTree`, `History`, `Diffs`, `Branches`, `Stashes`,
  `Remotes`, `Settings`), the `WriteLock` and the operations that take it
  (`changes`, `branching`, `stashing`, `syncing`). A read goes straight to its
  port.
- Not for: running git; that is `substrate-git-gitcli`.
- Not for: the vocabulary (status, branches, errors); that is
  `substrate-git-domain`.

## Usage

```rust
use substrate_git_app::WriteLock;

let lock = WriteLock::new();
let held = lock.hold();
drop(held);
```

## Depends on / used by

Depends on [`substrate-git-domain`](../substrate-git-domain/README.md) and
[`substrate-message`](../substrate-message/README.md). Implemented by
[`substrate-git-gitcli`](../substrate-git-gitcli/README.md); used by
[`tauri-plugin-substrate`](../../tauri-plugin/README.md).

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
