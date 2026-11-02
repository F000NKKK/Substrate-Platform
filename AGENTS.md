# Agent instructions

Follow [CONTRIBUTING.md](CONTRIBUTING.md) and the project policy in [CraftBook.toml](CraftBook.toml).

For Rust engineering decisions, consult the [Craftbook v2 handbook](https://github.com/F000NKKK/Craftbook) and relevant chapters. Apply its MUST/SHOULD/MAY/PROJECT distinction: optional architectural patterns and stylistic suggestions are not mandatory rules.

Validate locally with `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, tests, and `cargo craftcheck check` / `cargo craftcheck deps`. CI runs the [Craftbook GitHub Action](https://github.com/marketplace/actions/craftbook-policy-check).
