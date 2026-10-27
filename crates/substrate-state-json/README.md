# substrate-state-json

What an IDE remembers between runs, as JSON files: `read_json` (never fails: a
missing or corrupt file is an empty object), `write_json` (atomic: a sibling file
renamed over the real one), `private_dir` (a product's machine-local folder in a
project, kept out of version control from inside) and the names of the files
(`WORKSPACE_FILE`, `APP_STATE_FILE`).

**Layer and context:** infra · ide-state

- For: persisting small JSON objects to disk.
- Not for: deciding what to remember: that is the product's.

## Usage

```rust
use substrate_state_json::{read_json, write_json};

let file = std::env::temp_dir().join(format!("state-json-doc-{}.json", std::process::id()));
write_json(&file, &serde_json::json!({ "open": ["a.rs"] }))?;
assert_eq!(read_json(&file)["open"][0], "a.rs");
std::fs::remove_file(&file)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Depends on / used by

Depends on [`substrate-message`](../substrate-message/README.md). Used by
`tauri-plugin-substrate` (feature `state`) and products.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
