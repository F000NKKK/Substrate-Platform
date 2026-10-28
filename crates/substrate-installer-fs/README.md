# substrate-installer-fs

An installer on the real file system: where things go under a home folder, moving a staging folder into place, and installing a platform's programs with their launcher.

**Layer and context:** infra · installer

- For: `HomeLayout`, the `InstallLayout` port rooted at one home folder
  (`products/<id>/<version>`, `components/<id>/<version>`, `cache/…`,
  `staging/…`); `DiskFiles`, the `InstallFiles` port (a promotion keeps the old
  folder until the new one is in place, falls back to copy across disks, and
  records `Contents/Home` for a macOS bundle); `HostProducts`, the
  `ProductInstaller` port: on Linux an `AppImage` or a Debian package's files
  plus a `.desktop` entry, on Windows the setup run silently, on macOS the
  `.app` unpacked and linked into `~/Applications`.
- Not for: unpacking archives (`HostProducts` is given an `Unpacker`, such as
  [`substrate-installer-archive`](../substrate-installer-archive/README.md)),
  downloading, running rustup or cargo (`ToolRunner`), or keeping the records
  (`InstalledStore`).

Platform code is in a folder per platform under `products/`. What to write is
decided by pure plans that are tested on every host: `plan_launcher` (the
`.desktop` entry's path and text), the silent setup's command line and the
application link.

## Usage

```rust
use substrate_installer_app::ports::placement::{InstallFiles, InstallLayout};
use substrate_installer_fs::{DiskFiles, HomeLayout};

let home = tempfile::tempdir()?;
let layout = HomeLayout::new(home.path());
let destination = home.path().join("components/jdk/21");
let staging = layout.staging(&destination);
std::fs::create_dir_all(&staging)?;
std::fs::write(staging.join("release"), "21")?;

let placed = DiskFiles.promote(&staging, &destination)?;
assert_eq!(std::fs::read_to_string(placed.join("release"))?, "21");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Notes

- A version or file name with a `/` or `..` is made safe to be one folder
  name, so it cannot step out of its folder.
- A setup's output is not read (a silent setup prints none); cancelling kills
  the setup, pausing waits at the next look while it goes on.
- The desktop's caches (`update-desktop-database`, `kbuildsycoca`) are
  refreshed after a launcher change unless `DesktopCaches::Leave`.
- The product's icon is listed when `icon.png` is in its folder; fetching it
  is not done here.

## Depends on / used by

Depends on [`substrate-installer-app`](../substrate-installer-app/README.md)
and [`substrate-installer-domain`](../substrate-installer-domain/README.md).
Used by products' composition roots.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
