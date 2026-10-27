# substrate-project-toml

Reads a product's list of project standards from TOML (`parse_standards`) into
`substrate-project-domain`'s `ProjectStandard`s, in the file's order. The TOML
shape (`[[standard]]` with `id` and `detect_files`) lives here and nowhere else.

**Layer and context:** infra · project-model

- For: the file format of the product's standards.
- Not for: the vocabulary (`substrate-project-domain`) or project file formats.

## Usage

```rust
use substrate_project_toml::parse_standards;

let standards = parse_standards(
    r#"
    [[standard]]
    id = "rust"
    detect_files = ["Cargo.toml"]
    "#,
)?;
assert_eq!(standards[0].id, "rust");
# Ok::<(), substrate_project_domain::ProjectError>(())
```

## Depends on / used by

Depends on [`substrate-project-domain`](../substrate-project-domain/README.md).
Used by products.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
