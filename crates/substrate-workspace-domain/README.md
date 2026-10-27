# substrate-workspace-domain

The workspace's vocabulary: the folders the user opened (`Roots`), why a path
is not in them (`WorkspaceError`), a folder's entries (`DirectoryEntry`,
`EntryKind`) and how a product's own file kinds are read as text (`TextView`,
`TextViews`). No I/O, no serde.

**Layer and context:** domain · workspace

- For: what every context that reads files shares: the containment rule and the
  text views.
- Not for: resolving paths (`substrate-workspace-app`) or touching the file
  system (`substrate-workspace-fs`).

## Usage

```rust
use std::path::Path;

use substrate_workspace_domain::Roots;

let mut roots = Roots::default();
roots.admit(Path::new("/work/app"));
assert!(roots.covers(Path::new("/work/app/src/main.rs")));
assert!(!roots.covers(Path::new("/work/other")));
```

## Depends on / used by

Depends on [`substrate-message`](../substrate-message/README.md). Used by
[`substrate-workspace-app`](../substrate-workspace-app/README.md),
[`substrate-workspace-fs`](../substrate-workspace-fs/README.md), the search and
source-control adapters (`TextViews`), the plugin and products.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
