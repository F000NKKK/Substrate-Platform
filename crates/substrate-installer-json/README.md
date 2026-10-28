# substrate-installer-json

The installer's records of what is installed, kept as one versioned JSON file written atomically.

**Layer and context:** infra · installer

- For: `JsonInstalledStore`, the `InstalledStore` port of
  [`substrate-installer-app`](../substrate-installer-app/README.md). The file
  is `{"version": 1, "installed": [{"kind", "id", "version", "path"}]}`
  (`kind` is `product` or `component`: the store has no catalog to tell ids
  apart with). A write goes to a sibling file renamed over the real one. A
  missing file is an empty set; a damaged one, or one of another format, is an
  empty set with a typed `StoreWarning` (`load` returns it, `on_warning` is
  told it by `read`), so nothing is known installed until the next write.
- Not for: deciding what is installed (`substrate-installer-app`), or the
  IDE's remembered state (`substrate-state-json`).

## Usage

```rust
use std::path::PathBuf;

use substrate_installer_app::ports::records::InstalledStore;
use substrate_installer_domain::ids::{ComponentId, ItemId};
use substrate_installer_domain::inventory::{Installed, InstalledSet};
use substrate_installer_domain::versions::Version;
use substrate_installer_json::JsonInstalledStore;

let folder = tempfile::tempdir()?;
let store = JsonInstalledStore::new(folder.path().join("installed.json"));
assert!(store.read()?.records().is_empty());

let jdk = Installed {
    id: ItemId::Component(ComponentId::new("jdk")?),
    version: Version::new("21.0.5")?,
    path: PathBuf::from("/home/u/.installer/components/jdk/21.0.5"),
};
store.write(&InstalledSet::new(vec![jdk.clone()]))?;
assert_eq!(store.read()?.records(), [jdk]);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Depends on / used by

Depends on [`substrate-installer-app`](../substrate-installer-app/README.md),
[`substrate-installer-domain`](../substrate-installer-domain/README.md) and
[`substrate-lock`](../substrate-lock/README.md). Used by products'
composition roots.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
