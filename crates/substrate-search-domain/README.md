# substrate-search-domain

Find and Replace in Files' vocabulary: the query, its matcher and file mask,
line hits, the project layout and why a search fails.

**Layer and context:** domain · search

- For: `SearchQuery` (case, word and syntax are enums, not flags), `Matcher`,
  `FileMask`, `LineHit`, `Layout`, `Scope`, the result types and `SearchError`
  with the words the user reads.
- Not for: walking or reading files; `substrate-search-ignore` does, behind the
  ports of `substrate-search-app`.
- Depends on the regex and glob engines: pure computation, no I/O.

## Usage

```rust
use substrate_search_domain::pattern::Matcher;
use substrate_search_domain::query::SearchQuery;

let query = SearchQuery { pattern: "needle".into(), ..SearchQuery::default() };
let matcher = Matcher::new(&query).expect("a valid pattern");
assert_eq!(matcher.find_all("a Needle here", 10), [(2, 8)]);
```

## Depends on / used by

Depends on [`substrate-message`](../substrate-message/README.md). Used by
[`substrate-search-app`](../substrate-search-app/README.md),
[`substrate-search-ignore`](../substrate-search-ignore/README.md) and
[`tauri-plugin-substrate`](../../tauri-plugin/README.md).

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
