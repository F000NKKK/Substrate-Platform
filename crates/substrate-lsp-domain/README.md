# substrate-lsp-domain

Language servers' vocabulary: the languages a product knows and the server each
is served by (`Languages`, `Language`, `ServerDefinition`), what a server says
unasked (`Notification`), the `file://` URIs it names documents by (`uri`) and
why talking to a server fails (`LspError`). Params and results stay JSON
values. No I/O, no serde derives.

**Layer and context:** domain · language-server

- For: describing languages and servers, and the words the user hears.
- Not for: speaking the protocol (`substrate-lsp-app`) or reading the
  product's language list (`substrate-lsp-toml`).

## Usage

```rust
use std::path::Path;

use substrate_lsp_domain::{Language, Languages, ServerDefinition};

let languages = Languages::new(vec![Language {
    id: "rust".into(),
    extensions: vec!["rs".into()],
    file_names: Vec::new(),
    server: Some(ServerDefinition::new("rust-analyzer")),
}]);
assert_eq!(languages.for_path(Path::new("src/main.rs")).map(|l| l.id.as_str()), Some("rust"));
```

## Depends on / used by

Depends on [`substrate-message`](../substrate-message/README.md). Used by
[`substrate-lsp-app`](../substrate-lsp-app/README.md),
[`substrate-lsp-toml`](../substrate-lsp-toml/README.md), the plugin and products.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
