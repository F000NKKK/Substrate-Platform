# Contributing to Substrate-Platform

Thanks for wanting to help with Substrate Platform, the generic IDE core. Bug reports, ideas, documentation
fixes and code are all welcome.

## Reporting a bug

Open an [issue](https://github.com/F000NKKK/Substrate-Platform/issues/new/choose) with:

- what you did, what you expected and what happened instead;
- your operating system and the Substrate-Platform version (or commit);
- the log or a screenshot, if there is one.

Search the existing issues first — someone may have reported it already.
Security problems are not reported in issues: see [SECURITY.md](SECURITY.md).

## Suggesting a feature

Open an issue describing the problem you want solved before writing a lot of
code, so we can agree on the approach first.

## Making a change

1. Fork the repository and create a branch from `master`.
2. Make the change. It is done when it meets Craftbook's definition of done
   ([§8.6](https://github.com/F000NKKK/craftbook/blob/main/rust/08-testing-and-quality.md#86-definition-of-done)):
   tests for the new behaviour, every bug fix with a regression test, public
   items documented, [`docs/glossary.md`](docs/glossary.md) and
   [`docs/context-map.md`](docs/context-map.md) current.
3. Check that the gates pass (they are what CI runs,
   [`.github/workflows/quality.yml`](.github/workflows/quality.yml)):

   ```bash
   cargo fmt --all --check
   cargo clippy --workspace --all-targets --all-features
   python3 <craftbook>/tools/craftcheck check . --baseline .craftcheck-baseline
   python3 <craftbook>/tools/craftcheck deps . --baseline .craftcheck-deps-baseline
   cargo test --workspace --all-features
   cargo llvm-cov --package substrate-platform --all-features --lib --summary-only
   (cd ui && npm install && npm test)
   ```

   Every module is a feature, so tests run with `--all-features`; plain
   `cargo test` skips most of them. `tauri-plugin` needs GTK and WebKitGTK
   on Linux (`libgtk-3-dev libwebkit2gtk-4.1-dev libsoup-3.0-dev
   libayatana-appindicator3-dev librsvg2-dev`).

   The lint set is at `warn` while the code is migrated to Craftbook
   (`adoption/substrate-platform.md` there), except `missing_docs`,
   `missing_debug_implementations`, `unreachable_pub` and
   `elided_lifetimes_in_paths`, which are `deny`: a product building Substrate
   by path sees its warnings, so these stay at zero in every feature
   combination. A change adds no new warning
   and no new `craftcheck` finding, and regenerates the baselines when it
   removes some (`--write-baseline`; they only shrink). Coverage only goes up.
4. Open a pull request that explains what changed and why.

Substrate stays generic: no Yog-specific formats or logic, only infrastructure any IDE could use.

## Style

- Rust follows [Craftbook](https://github.com/F000NKKK/craftbook), our
  engineering standard: files and modules, functions, naming, types and
  errors, docs, tests. Where it is silent, code reads like the code around it.
- Rust is formatted with `cargo fmt` (`rustfmt.toml`); TypeScript follows the existing formatting.
- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/):
  `feat(scope): …`, `fix(scope): …`, `docs: …`, `build(deps): …`, `chore: …`;
  the scope is the context (`source-control`, `search`, `run`, `terminal`, …).
- The project is in active development: an outdated API is replaced, not kept
  alongside the new one.

## License

The project is dual-licensed (AGPL-3.0 and a [commercial license](COMMERCIAL-LICENSE.md)).
By contributing you agree that your contribution is licensed under the
[AGPL-3.0](LICENSE) and may also be distributed under the commercial license.

## Code of conduct

Everyone taking part is expected to follow the [Code of Conduct](CODE_OF_CONDUCT.md).
