# substrate-project-app

Opening and saving solutions and projects through the formats a product
registers. The ports are `SolutionFormat` and `ProjectFormat` (the product
implements them for `.sln`, `.slnx`, `.csproj`…); `Formats` holds them and opens
a file with the first format that reads it. Saving under another format's name
converts.

**Layer and context:** app · project-model

- For: choosing a format by file and surfacing its failures as `ProjectError`.
- Not for: any concrete format (Substrate ships none) or the model itself
  (`substrate-project-domain`).

## Usage

```rust
use std::path::Path;

use substrate_project_app::Formats;

let formats = Formats::default();
assert!(formats.load_solution(Path::new("App.sln")).is_err());
```

## Depends on / used by

Depends on [`substrate-project-domain`](../substrate-project-domain/README.md)
and [`substrate-message`](../substrate-message/README.md). Used by products.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
