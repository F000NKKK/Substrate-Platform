# Contributing to Substrate-Platform

Thanks for wanting to help with Substrate Platform, the generic IDE core. Bug reports, ideas, documentation
fixes and code are all welcome.

## Reporting a bug

Open an [issue](https://github.com/F000NKKK/Substrate-Platform/issues/new/choose) with:

- what you did, what you expected and what happened instead;
- your operating system, Minecraft version and loader (Fabric, NeoForge or Forge);
- the log or a screenshot, if there is one.

Search the existing issues first — someone may have reported it already.
Security problems are not reported in issues: see [SECURITY.md](SECURITY.md).

## Suggesting a feature

Open an issue describing the problem you want solved before writing a lot of
code, so we can agree on the approach first.

## Making a change

1. Fork the repository and create a branch from `master`.
2. Make the change, with tests where it makes sense.
3. Check that everything builds and the tests pass:

   ```bash
   cargo test
   (cd ui && npm install && npm test)
   ```

4. Open a pull request that explains what changed and why.

Substrate stays generic: no Yog-specific formats or logic, only infrastructure any IDE could use.

## Style

- Code reads like the code around it: same naming, idioms and comment density.
- Rust is formatted with `cargo fmt`; TypeScript follows the existing formatting.
- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/):
  `feat(scope): …`, `fix(scope): …`, `docs: …`, `build(deps): …`, `chore: …`.
- The project is in active development: an outdated API is replaced, not kept
  alongside the new one.

## License

The project is dual-licensed (AGPL-3.0 and a [commercial license](COMMERCIAL-LICENSE.md)).
By contributing you agree that your contribution is licensed under the
[AGPL-3.0](LICENSE) and may also be distributed under the commercial license.

## Code of conduct

Everyone taking part is expected to follow the [Code of Conduct](CODE_OF_CONDUCT.md).
