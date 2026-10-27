# substrate-search-ignore

Find and Replace in Files over the file system: a `.gitignore`-aware walk of a
project, and text read and written through the product's text views.

**Layer and context:** infra · search

- For: `IgnoreWalker` (implements `FileWalker` with the `ignore` crate) and
  `DiskDocuments` (implements `DocumentSource`: size and binary checks, the
  product's text views).
- Not for: matching; the query, matcher and use cases are
  `substrate-search-domain` and `substrate-search-app`.

## Usage

```rust
use substrate_workspace_domain::TextViews;
use substrate_search_ignore::{DiskDocuments, IgnoreWalker};

let (_walker, _documents) = (IgnoreWalker, DiskDocuments::new(TextViews::default()));
```

## Depends on / used by

Depends on [`substrate-search-app`](../substrate-search-app/README.md),
[`substrate-search-domain`](../substrate-search-domain/README.md) and
[`substrate-workspace-domain`](../substrate-workspace-domain/README.md) (the text
views). Used by
[`tauri-plugin-substrate`](../../tauri-plugin/README.md).

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
