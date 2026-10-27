# substrate-search-app

Find and Replace in Files' use cases: search and replace over a project or
open documents, and the ports for walking and reading files.

**Layer and context:** app · search

- For: `find`, `replace` and `files_containing`, which plan the files (open
  documents' buffers stand in for their files), match them and report file by
  file; the ports `FileWalker` and `DocumentSource`.
- Not for: the walk or the reading itself; that is `substrate-search-ignore`.
- Not for: the query and its matcher; they are `substrate-search-domain`.

## Usage

```rust
use substrate_search_app::DEFAULT_MAX_LINES;

assert!(DEFAULT_MAX_LINES > 0);
```

## Depends on / used by

Depends on [`substrate-search-domain`](../substrate-search-domain/README.md) and
[`substrate-message`](../substrate-message/README.md). Implemented by
[`substrate-search-ignore`](../substrate-search-ignore/README.md); used by
[`tauri-plugin-substrate`](../../tauri-plugin/README.md).

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
