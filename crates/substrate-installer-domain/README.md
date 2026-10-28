# substrate-installer-domain

An installer's vocabulary, the reusable core of an installer like Visual
Studio Installer: the `Catalog` of products, workloads and components (with
the installer's own entry), the closed `Source` enum saying where each is
published, `Release`s and their `Asset`s, a user's `Selection` (with the
`Why` of every component), the `Change`s and `Plan` that make a machine match
it, compatibility `Needs` and their `check`, the `InstalledSet`, and the
progress `Event`s. Ids, platforms, versions and digests are typed and checked
where they are made. Pure data and rules: no files, no network, no processes,
no `async`, no serde.

**Layer and context:** domain · installer

- For: describing what can be installed, what a selection changes and what
  happened.
- Not for: listing releases, downloading, unpacking, running tools or storing
  what is installed (`substrate-installer-app`'s ports, implemented by
  adapter crates), or reading a catalog file (an adapter).

## Usage

```rust
use substrate_installer_domain::versions::{Version, VersionRequirement};

let newer = Version::new("0.47.1")?;
let older = Version::new("v0.47")?;
assert!(newer.compare(&older).is_gt());
assert!(VersionRequirement::new(">=0.47.0, <0.48")?.matches(&newer));
# Ok::<(), substrate_installer_domain::versions::VersionError>(())
```

## Depends on / used by

Depends on nothing in the workspace (`semver`, `thiserror`). Used by
[`substrate-installer-app`](../substrate-installer-app/README.md) and by
products' installer adapters.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
