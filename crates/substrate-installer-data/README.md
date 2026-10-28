# substrate-installer-data

The installer's own files as [Substrate Data](https://crates.io/crates/substrate-data):
no TOML and no JSON.

**Layer and context:** infra · installer

- `parse_catalog` reads the catalog from Substrate Data **text** (JSON with
  comments, unquoted keys and trailing commas) into a checked `Catalog`:
  `installer`, `groups`, `products`, `workloads`, `components`, `revision`,
  `manifest`, and `source` values tagged by `kind` (`github`, `git`, `crate`,
  `rustup`, `rust-targets`). `newer_revision` reads it only when its
  `revision` is above the one a caller has.
- `parse_needs` reads a release's compatibility manifest, text like
  `{ requires: { "component.id": ">=1.2" } }`, into `Needs`.
- `BinaryInstalledStore` implements the `InstalledStore` port with one versioned
  Substrate Data **binary** file of kind/id/version/path records, written to a
  sibling file and renamed over the real one. A missing file is an empty set; a
  damaged one, or one of another format, is an empty set with a typed
  `StoreWarning` (`load` returns it, `on_warning` is told it by `read`).
- Every problem in a text file is a `Problem`: line and column, the value's
  path (`components[2].source.repo`) and the Substrate Data code (`SD0001` text
  that does not parse, `SD0005` unknown field, `SD0006` missing field,
  `SD0000` a value that breaks a rule of the vocabulary). A catalog that is
  well formed but not a catalog (an id twice, a reference to nothing, a
  requirement cycle) is `Rejected`, pointing at the line of the entry. A source
  `kind` the domain has no variant for (`adoptium`, `zig`) is refused at its
  line.
- Not for: finding or downloading the files, or what a catalog means
  (`substrate-installer-domain`).

## Usage

```rust
use substrate_installer_data::{CatalogFileError, parse_catalog, parse_needs};

let catalog = parse_catalog(
    r#"{
        revision: 3,
        manifest: "release.data",
        groups: [{ id: "build", name: "Build tools" }],
        components: [{
            id: "rust.toolchain",     // comments are fine
            name: "Rust toolchain",
            description: "rustc and cargo.",
            group: "build",
            source: { kind: "rustup", toolchain: "1.99" },
        }],
    }"#,
)?;
assert_eq!(catalog.revision(), 3);
assert_eq!(catalog.components().len(), 1);

let bad = parse_catalog("{ groups: [{ id: \"Build\", name: \"B\" }] }").unwrap_err();
assert!(matches!(&bad, CatalogFileError::Invalid { problems } if problems[0].location.map(|at| at.line) == Some(1)));

let needs = parse_needs(r#"{ requires: { "yog.cli": ">=0.47.0" } }"#)?;
assert_eq!(needs.requires.len(), 1);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Depends on / used by

Depends on [`substrate-installer-app`](../substrate-installer-app/README.md),
[`substrate-installer-domain`](../substrate-installer-domain/README.md),
[`substrate-lock`](../substrate-lock/README.md) and `substrate-data`. Used by
products' composition roots.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
