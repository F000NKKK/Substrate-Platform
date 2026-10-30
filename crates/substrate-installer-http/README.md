# substrate-installer-http

HTTP adapters of the installer ports (`substrate-installer-app`), over `ureq`
with rustls:

- `HttpReleaseSource` implements `ReleaseSource`: GitHub releases (each asset
  with its size and the SHA-256 GitHub publishes as `digest`; the tag's source
  archive for a `GitTag`), crates.io versions, and a toolchain's name as its
  one release (`Rustup`, `RustTargets`). Drafts and yanked versions are left
  out.
- `HttpDownloader` implements `Downloader`: writes to `<destination>.part`,
  goes on from it with a `Range` request after a pause, a dropped connection or
  a crash, verifies the SHA-256, then renames it into place. It reports
  progress, waits or stops at the `CancelToken` between chunks, and can feed a
  writer the bytes as they arrive (`stream`).
- `HttpClient` holds the agent, the optional GitHub token and an `ETag` cache of
  JSON answers in a directory; `ClientConfig` says how it is made. A repeated
  question costs a `304`, and a rate limit is answered from the cache.

**Layer and context:** infra · installer

- For: listing and downloading over HTTP; no global state, the composition root
  builds the client and hands it to both adapters.
- Not for: finding the GitHub token (`GITHUB_TOKEN`, `gh auth token`: the
  composition root does), unpacking, or installing.

GitHub release assets use their API download URL with `Accept: application/octet-stream`.
A configured token is attached only to the configured GitHub API base and is stripped on
redirects; storage/CDN requests and unrelated downloads receive no credentials. Older
release metadata without an API asset URL retains its browser download URL.

Failures reach the ports as their `cause` (`HttpError`, `ReleaseReadError`);
`is_rate_limited` tells a rate limit from the rest.

## Usage

```rust
use substrate_installer_http::{ClientConfig, HttpClient, HttpDownloader, HttpReleaseSource};

let client = HttpClient::new(ClientConfig::new("my-installer/1.0").with_cache_dir("/tmp/http"));
let releases = HttpReleaseSource::new(client.clone());
let downloader = HttpDownloader::new(client);
# let _ = (releases, downloader);
```

## Depends on / used by

Depends on [`substrate-installer-app`](../substrate-installer-app/README.md)
(the ports) and [`substrate-installer-domain`](../substrate-installer-domain/README.md).
Used by products' composition roots.

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
