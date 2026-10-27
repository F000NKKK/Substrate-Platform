# substrate-workspace-app

The workspace's use cases: the folders the user opened and the project shown
now (`Workspace`), checking that a path is inside them (`Workspace::check`,
`Boundary`), and the port the paths are resolved through (`PathResolver`).

**Layer and context:** app · workspace

- For: the one boundary every host command checks its paths against.
- Not for: the vocabulary (`substrate-workspace-domain`) or the file system
  behind the port (`substrate-workspace-fs`).

## Usage

```rust
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use substrate_workspace_app::Workspace;
use substrate_workspace_app::ports::PathResolver;

#[derive(Debug)]
struct Disk;

impl PathResolver for Disk {
    fn canonicalize(&self, path: &Path) -> io::Result<PathBuf> {
        path.canonicalize()
    }
}

let workspace = Workspace::new(Arc::new(Disk));
let here = std::env::temp_dir();
workspace.open(&here)?;
assert!(workspace.check(&here.join("not-yet-created.txt")).is_ok());
# Ok::<(), substrate_workspace_domain::WorkspaceError>(())
```

## Depends on / used by

Depends on [`substrate-workspace-domain`](../substrate-workspace-domain/README.md)
and [`substrate-lock`](../substrate-lock/README.md). Implemented by
[`substrate-workspace-fs`](../substrate-workspace-fs/README.md); used by
[`tauri-plugin-substrate`](../../tauri-plugin/README.md) and products.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
