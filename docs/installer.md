# Installer

The `installer` context is the reusable engine of an installer like Visual
Studio Installer: products made of workloads, workloads made of components, a
live list of versions per component, a Modify window's checkboxes turned into
a plan, and a plan carried out with progress, pause and cancel. It is
extracted from Yog Installer's core and rebuilt on Craftbook's rules: ids,
platforms, versions, sources, errors and events are typed, not strings.

Phase 1 is two crates and no adapters:

| Crate | Layer | Holds |
|---|---|---|
| `substrate-installer-domain` | domain | the vocabulary and the rules; no I/O, no `async`, no serde |
| `substrate-installer-app` | app | the use cases and the ports (traits); tested with in-memory fakes of every port |
| `substrate-installer-http` | infra | `ReleaseSource` (GitHub releases and source archives, crates.io, toolchains) and `Downloader` (resumable, SHA-256 verified, cancellable, streamable) over `ureq`, with an `ETag` cache of JSON answers; the GitHub token is passed in |
| `substrate-installer-data` | infra | the installer's own files as Substrate Data, no TOML or JSON: the catalog from text (`installer`, `groups`, `products`, `workloads`, `components`, `revision`, `manifest`, `source` values tagged by `kind`: `github`, `git`, `crate`, `rustup`, `rust-targets`) into a checked `Catalog` (`parse_catalog`, `newer_revision`); a release's `{ requires: { id: ">=x" } }` into `Needs` (`parse_needs`); every problem is a typed error with its line, column, path and `SD…` code; and `InstalledStore` as one versioned binary file (`DataInstalledStore`, atomic write; a damaged or other-format file reads as an empty set with a typed `StoreWarning`) |
| `substrate-installer-process` | infra | `ToolRunner` over rustup and cargo (output streamed, paused with `SIGSTOP` to the process group, cancelled by killing the tree, rustup fetched by a documented bootstrap step) and `SystemDetector` (rustup toolchains and targets, `cargo install`ed crates the catalog's sources imply) |
| `substrate-installer-archive` | infra | `Unpacker` for zip, tar.gz, tar.xz (pure Rust) and a Debian package's files, from a file or a stream: drops the top folder, keeps Unix modes (without setuid/setgid) and links, checks the cancel token between entries and chunks, refuses `..`, absolute entries, links that point outside and writes through links (`EntryEscapes`); a zip stream is spooled to an unnamed file first, because a zip keeps modes and links at its end |
| `substrate-installer-fs` | infra | `InstallLayout` under one home folder (`products/<id>/<version>`, `components/<id>/<version>`, `cache/`, `staging/`), `InstallFiles` (the old folder is kept until the new one is in place, copy across disks, `Contents/Home` of a macOS bundle recorded) and `ProductInstaller` per platform (Linux `AppImage` or `.deb` plus a `.desktop` entry, Windows silent setup, macOS `.app` linked into `~/Applications`); archives go through an injected `Unpacker`, and the launcher, setup command and link are pure plans |

Adapters (GitHub and crates.io listing, an HTTP downloader, an unpacker, a
file store for the records, rustup and cargo, the platform's program
installer) are infrastructure crates added later. The catalog file format is an
adapter's too.

## Domain

| Module | What it holds |
|---|---|
| `ids` | `ComponentId`, `ProductId`, `WorkloadId`, `GroupId` (validated: lowercase ASCII, digits, `.`, `-`, `_`), `ItemId` (a product or a component) |
| `texts` | `Label`, `Description`, `IconRef`, `ProgramName` and the rules behind every validated text |
| `versions` | `Version` (lenient: `1.99`, `v0.6.1`, `21.0.5+11`; not semver is older and ordered as text), `VersionRequirement` (semver's `>=0.47.0, <0.48`) |
| `platforms` | `Os`, `Arch`, `PlatformTag` (`linux-x86_64`), `PlatformAssets`, `SystemLibrary`, `Host` |
| `releases` | `Release`, `Asset`, `Sha256`, `AssetName`, `AssetSuffix`, `HttpUrl`, `PublishedAt`, `ArchiveKind` |
| `sources` | `Source` (`non_exhaustive`: `GithubRelease`, `GitTag`, `CratesIo`, `Rustup`, `RustTargets`), `SystemPackage`, `SourceKind` and the names they use |
| `channel` | `Catalog` (checked: unique ids, dangling references named, requirement cycles refused), `Product`, `Workload`, `Component`, `Group`, `Installer` (the self entry), `Item`; the manifest file name is the catalog's |
| `choice` | `Selection` (workloads, unchecked, checked) and `Why` for every component it makes |
| `changes` | `Change` (install, update, remove), `dependency_order`, `derive`: managed things update and are removed, system-found ones count as installed and are left alone |
| `planning` | `Plan` (changes plus the release each takes), `Mode` |
| `compat` | `Needs`, `Chosen`, `Conflict`, `check` |
| `inventory` | `Installed`, `InstalledSet` (`newest`, `put`, `remove`), `Found`, `Origin` |
| `events` | `Event`, `Outcome` |

Adoptium and zig are deliberately not sources: `Source` is `non_exhaustive`, and
a product that needs another kind adds a variant with its port support.

## App

`listing` asks the sources what is published: `compatible` lists one item's
releases a host can install, `newest` asks the whole catalog side by side and
returns an `Available` (the newest release per item, and the sources that
failed). `plans` makes a `Plan`: `for_selection`, `for_program` (a product or the
installer), `for_removal`, `conflicts` against what releases need, and `survey`
(what the store records plus what the system detector finds). `installing::Installer`
applies a plan.

### Applying a plan

`Installer::apply(&plan, options, &cancel)` tells `Started`, then for each change
checks the cancel token, installs or removes, records the result in the store
(read once, written after each change) and tells `Installed` or `Removed`, then
tells `Finished` with an `Outcome` (completed, cancelled, failed). It stops at
the first error; what was done stays done and recorded.

- `InstallWhileDownloading`: a component whose download is an archive is
  unpacked into a staging folder while the bytes arrive (a pipe between
  `Downloader::stream` and `Unpacker::unpack_stream`, on a scoped thread), then
  the staging folder is promoted into place. Everything else is downloaded
  first, then installed.
- `DownloadAllThenInstall`: every file is downloaded before anything is
  installed; nothing changes if a download fails.
- An update installs the new version, then removes the old files unless the new
  one is in the same place or a tool replaces its own version (crates, rustup).
  A product loses its other versions too.
- Products and the installer entry are always downloaded and handed to the
  platform's `ProductInstaller`.
- Downloads leave the cache after install unless `DownloadCache::Keep`.

### Ports

All ports are blocking traits (`Send + Sync`), never `async`: the work runs on a
background thread. Each has an error enum of its own that keeps the adapter's
failure as its `source`.

```rust
trait ReleaseSource   { fn releases(&self, &Source) -> Result<Vec<Release>, ListingError>; }   // newest first
trait Downloader      {
    fn download(&self, &DownloadRequest, &TransferControl) -> Result<(), DownloadError>;
    fn stream(&self, &DownloadRequest, &TransferControl, tee: &mut dyn Write) -> Result<(), DownloadError>;
}                                                      // resume, sha256, cancel/pause between chunks
trait Unpacker        {
    fn unpack_file(&self, &Path, &UnpackRequest, &CancelToken) -> Result<(), UnpackError>;
    fn unpack_stream(&self, &mut dyn Read, &UnpackRequest, &CancelToken) -> Result<(), UnpackError>;
}                                                      // zip, tar.gz, tar.xz, deb; strip the top folder; never outside
trait InstalledStore  { fn read(&self) -> Result<InstalledSet, StoreError>; fn write(&self, &InstalledSet) -> Result<(), StoreError>; }
trait ToolRunner      { fn run(&self, &ToolCommand, &ToolControl) -> Result<PathBuf, ToolError>; }   // rustup, cargo
trait InstallLayout   {                                // pure: where things go
    fn destination(&self, Item, &Version) -> Option<PathBuf>;
    fn cache_file(&self, &ItemId, &Version, &AssetName) -> PathBuf;
    fn staging(&self, &Path) -> PathBuf;
}
trait InstallFiles    { fn promote(&self, staging: &Path, destination: &Path) -> Result<PathBuf, FilesError>; fn remove(&self, &Path) -> Result<(), FilesError>; }
trait ProductInstaller { fn install(..) / remove(..) / forget_other_versions(..) }   // setup, AppImage, deb, .app, launcher
trait SystemDetector  { fn detect(&self, &Catalog) -> Vec<Found>; }
trait EventSink       { fn emit(&self, Event); }       // also any Fn(Event)
```

`CancelToken` pauses and cancels: every step and every adapter calls
`check()` between chunks of work; it waits while paused and fails once
cancelled. `ToolCommand` is typed (`InstallToolchain`, `AddTargets`,
`InstallCrate`, ...) and the adapter renders the command line once; it also
fetches rustup first when a rustup command finds none. `ToolRunner` pauses and
kills the tool's whole process group.

## Yog Installer

Yog Installer is unchanged and keeps its own copy of this logic. This context
is the generic engine extracted from it; adopting it in Yog Installer (and
the catalog file reader, `components.ydata` store and the rest of the
adapters it needs) is later work.

## Open questions

- Reading a release's manifest (`Needs`) is an adapter's job (download the
  small asset named by `Catalog::manifest`, then `substrate-installer-data`'s
  `parse_needs`); `plans::conflicts` takes the parsed needs.
- A catalog file naming a source kind the domain has no variant for (Yog's
  `adoptium` and `zig`) is refused by `parse_catalog`, naming the kind and its
  line; those kinds come with their ports, or a product keeps them out of its
  catalog.
- Finding the `Host` (platform and system libraries) is an adapter's job too;
  the use cases take it as a value.
- Developer news (Yog Installer's recent releases of its own repos) is not in
  `listing`; it can be built from `compatible` by a product.
