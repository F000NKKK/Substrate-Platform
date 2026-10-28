# substrate-installer-app

An installer's use cases, the reusable engine of an installer like Visual
Studio Installer, over [`substrate-installer-domain`](../substrate-installer-domain/README.md):

- `listing`: ask the sources what is published, filtered for the host
  (`compatible` for one item's releases, `newest` for the whole catalog, side
  by side);
- `plans`: turn a `Selection` into a `Plan` (`for_selection`), plan a
  product's install or update (`for_program`) or an uninstall
  (`for_removal`), check a plan against what releases need (`conflicts`), and
  `survey` what is installed;
- `installing::Installer::apply`: carry a plan out, in `InstallWhileDownloading`
  or `DownloadAllThenInstall` mode, recording each change in the store,
  replacing older versions, removing, and telling the events;
- `CancelToken`: pause, resume and cancel from another thread;
- `ports`: every contact with the outside world as a blocking trait
  (`ReleaseSource`, `Downloader`, `Unpacker`, `InstalledStore`, `ToolRunner`,
  `InstallLayout`, `InstallFiles`, `ProductInstaller`, `SystemDetector`) and
  `EventSink` for the progress.

**Layer and context:** app · installer

- For: orchestrating an install over any adapters; tested with in-memory fakes
  of every port.
- Not for: talking to GitHub or crates.io, downloading, unpacking, running
  rustup or cargo, installing a platform's programs or keeping the records
  (adapter crates implement the ports).

## Usage

```rust
use substrate_installer_app::control::CancelToken;

let token = CancelToken::new();
token.pause();
token.cancel(); // a paused operation wakes up and stops
assert!(token.check().is_err());
```

## Depends on / used by

Depends on [`substrate-installer-domain`](../substrate-installer-domain/README.md).
Used by the installer adapter crates (which implement the ports) and by
products' composition roots.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
