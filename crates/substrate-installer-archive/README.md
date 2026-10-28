# substrate-installer-archive

Unpacks an installer's downloads: zip, tar.gz, tar.xz and a Debian package's files, from a file or from a stream, never outside the folder.

**Layer and context:** infra · installer

- For: `ArchiveUnpacker`, the `Unpacker` port of
  [`substrate-installer-app`](../substrate-installer-app/README.md), in pure
  Rust (`lzma-rust2` for xz, no external tools). It drops the top folder on
  request, keeps Unix permission bits (without setuid and setgid) and
  symbolic links, looks at the `CancelToken` between entries and chunks, and
  refuses an entry with an absolute path or `..`, a link that points outside
  the folder, and anything that would be written through a link
  (`UnpackError::EntryEscapes`).
- Not for: downloading (`Downloader`), choosing the staging folder or moving
  it into place (`InstallLayout` and `InstallFiles`, in
  [`substrate-installer-fs`](../substrate-installer-fs/README.md)).

## Usage

```rust
use std::io::Write;

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::unpacking::{Strip, UnpackRequest, Unpacker};
use substrate_installer_archive::ArchiveUnpacker;
use substrate_installer_domain::releases::ArchiveKind;

// A tar.gz with one file, `top/bin/tool`.
let mut tar = tar::Builder::new(Vec::new());
let mut header = tar::Header::new_gnu();
header.set_size(2);
header.set_mode(0o755);
tar.append_data(&mut header, "top/bin/tool", &b"hi"[..])?;
let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
gz.write_all(&tar.into_inner()?)?;
let bytes = gz.finish()?;

let folder = tempfile::tempdir()?;
let request = UnpackRequest {
    kind: ArchiveKind::TarGz,
    destination: folder.path(),
    strip: Strip::DropTopFolder,
};
ArchiveUnpacker.unpack_stream(&mut &bytes[..], &request, &CancelToken::new())?;
assert_eq!(std::fs::read(folder.path().join("bin/tool"))?, b"hi");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Notes

- A zip keeps permissions and links at its end, so a zip that arrives as a
  stream is spooled to an unnamed file beside the destination, then read as a
  file; the other kinds are unpacked as the bytes arrive.
- On Windows, modes and links are left out.
- A failure is `UnpackError::Failed` whose source is an `ArchiveFailure`.

## Depends on / used by

Depends on [`substrate-installer-app`](../substrate-installer-app/README.md)
and [`substrate-installer-domain`](../substrate-installer-domain/README.md).
Used by products' composition roots.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
