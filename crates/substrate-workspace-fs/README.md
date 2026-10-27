# substrate-workspace-fs

The workspace on the real file system: listing and changing folders and files
(`dir`), `copy_new`, `read_text`, `DirectoryWatcher` for changes made outside
the IDE, and `DiskPaths`, the resolver the workspace checks paths with.

**Layer and context:** infra · workspace

- For: a file tree's listing, an editor's open and save, Copy and Paste, a
  watcher that coalesces a burst of changes into one batch.
- Not for: deciding whether a path may be touched (`substrate-workspace-app`'s
  `Workspace::check`, called by the host first).

## Usage

```rust
use substrate_workspace_fs::dir;

let folder = std::env::temp_dir().join("substrate-workspace-fs-readme");
std::fs::create_dir_all(&folder)?;
dir::create_file(&folder.join("a.txt"))?;
let names: Vec<_> = dir::list(&folder)?.into_iter().map(|entry| entry.name).collect();
assert_eq!(names, ["a.txt"]);
dir::remove(&folder)?;
# Ok::<(), std::io::Error>(())
```

## Depends on / used by

Depends on [`substrate-workspace-app`](../substrate-workspace-app/README.md),
[`substrate-workspace-domain`](../substrate-workspace-domain/README.md),
[`substrate-lock`](../substrate-lock/README.md) and
[`substrate-message`](../substrate-message/README.md). Used by
[`tauri-plugin-substrate`](../../tauri-plugin/README.md) and products.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
