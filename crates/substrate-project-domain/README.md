# substrate-project-domain

The project model: solutions and projects as an IDE models them, whatever builds
them (`Solution`, `Project`, `ProjectEntry`, `Configuration`, `Section`…), the
kinds of project a folder holds (`ProjectStandard`) and why opening or ordering
them fails (`ProjectError`). Sections and ordered properties keep what the model
does not know, so a file goes back as it was read. No I/O, no serde derives.

**Layer and context:** domain · project-model

- For: describing solutions and projects, build order, and the words the user hears.
- Not for: reading and writing `.sln`/`.csproj` (the product's formats, behind
  `substrate-project-app`'s ports) or reading the list of standards
  (`substrate-project-toml`).

## Usage

```rust
use substrate_project_domain::ProjectStandard;

let standards = [ProjectStandard {
    id: "rust".into(),
    detect_files: vec!["Cargo.toml".into()],
}];
let files = vec!["Cargo.toml".to_string()];
assert_eq!(ProjectStandard::detect(&files, &standards).as_deref(), Some("rust"));
```

## Depends on / used by

Depends on [`substrate-message`](../substrate-message/README.md). Used by
[`substrate-project-app`](../substrate-project-app/README.md),
[`substrate-project-toml`](../substrate-project-toml/README.md) and products.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
