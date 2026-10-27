# substrate-lsp-toml

Reads a product's list of languages and their language servers from TOML
(`parse_languages`) into `substrate-lsp-domain`'s `Languages`. The TOML shape
(`[language.<id>]` with `extensions`, `file_names` and a `server` table) lives
here and nowhere else.

**Layer and context:** infra · language-server

- For: the file format of the product's language list.
- Not for: the languages' vocabulary (`substrate-lsp-domain`) or running the
  servers (`substrate-lsp-app`).

## Usage

```rust
use substrate_lsp_toml::parse_languages;

let languages = parse_languages(
    r#"
    [language.rust]
    extensions = ["rs"]
    server = { command = "rust-analyzer" }
    "#,
)?;
assert!(languages.find("rust").is_some());
# Ok::<(), substrate_lsp_domain::LspError>(())
```

## Depends on / used by

Depends on [`substrate-lsp-domain`](../substrate-lsp-domain/README.md). Used by
products, which hand the parsed `Languages` to `tauri-plugin-substrate`.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
