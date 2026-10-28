# substrate-installer-toml

Reads an installer's catalog file and a release's compatibility manifest into
`substrate-installer-domain`'s vocabulary. The catalog's TOML shape
(`[installer]`, `[[group]]`, `[[product]]`, `[[workload]]`, `[[component]]`,
`revision`, `manifest` and the `source` tables) and the manifest's JSON shape
live here and nowhere else. Read-only: it writes nothing.

**Layer and context:** infra · installer

- For: `parse_catalog` (a file into a checked `Catalog`), `newer_revision`
  (the catalog only when its `revision` is above the one a caller has) and
  `parse_needs` (a release's `{"requires": {"id": ">=x"}}` into `Needs`).
- Not for: finding or downloading the files (the downloader and the release
  source), or what a catalog means (`substrate-installer-domain`).

Every problem is a typed error that says where it is. A wrong field is a
`CatalogFileError::Invalid` with the `Location` (line and column), the
`Table`, the entry's id, the field (`source.repo`) and a `Reason`; text that is
not TOML of the shape is a `Syntax` error with the parser's line; entries that
do not make a catalog (an id twice, a reference to nothing, a requirement
cycle) are `Rejected`, pointing at the line of the entry the domain names. A
`source` kind the domain has no source for (`adoptium`, `zig`) is refused by
name, never skipped. JSON has no spans, so a manifest's wrong key is named by
the key and its wrong shape by line and column.

## Usage

```rust
use substrate_installer_toml::{CatalogFileError, parse_catalog, parse_needs};

let catalog = parse_catalog(
    r#"
    revision = 3
    manifest = "release.json"

    [[group]]
    id = "build"
    name = "Build tools"

    [[component]]
    id = "rust.toolchain"
    name = "Rust toolchain"
    description = "rustc and cargo."
    group = "build"
    source = { kind = "rustup", toolchain = "1.99" }
    "#,
)?;
assert_eq!(catalog.revision(), 3);
assert_eq!(catalog.components().len(), 1);

let bad = parse_catalog("[[group]]\nid = \"Build\"\nname = \"B\"\n").unwrap_err();
assert!(matches!(bad, CatalogFileError::Invalid(problem) if problem.location.line == 2));

let needs = parse_needs(r#"{"requires": {"yog.cli": ">=0.47.0"}}"#)?;
assert_eq!(needs.requires.len(), 1);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Depends on / used by

Depends on [`substrate-installer-domain`](../substrate-installer-domain/README.md).
Used by products, which read their built-in catalog and the one a release
brings, and hand the `Catalog` to `substrate-installer-app`.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
