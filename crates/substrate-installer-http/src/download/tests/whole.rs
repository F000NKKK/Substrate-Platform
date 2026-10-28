//! Downloads that complete or are stopped.

use std::fs;

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::downloads::{
    DownloadError, DownloadRequest, Downloader, Progress, TransferControl,
};
use substrate_installer_domain::releases::HttpUrl;

use super::support::{SIZE, body, digest, downloader, part_of, run, serve_ranged};
use crate::testing::MockServer;

#[test]
fn a_download_is_written_verified_and_reported() {
    let data = body();
    let server = MockServer::start(serve_ranged(data.clone()));
    let dir = tempfile::tempdir().unwrap();
    let run = run(
        &server.url("/f"),
        dir.path(),
        Some(&digest(&data)),
        &CancelToken::new(),
    );
    run.result.unwrap();
    assert_eq!(fs::read(&run.destination).unwrap(), data);
    assert!(!part_of(&run.destination).exists());
    let last = run.progress.last().unwrap();
    assert_eq!((last.done, last.total), (SIZE as u64, Some(SIZE as u64)));
}

#[test]
fn a_wrong_digest_deletes_everything() {
    let server = MockServer::start(serve_ranged(body()));
    let dir = tempfile::tempdir().unwrap();
    let wrong = digest(b"something else");
    let run = run(
        &server.url("/f"),
        dir.path(),
        Some(&wrong),
        &CancelToken::new(),
    );
    match run.result {
        Err(DownloadError::ChecksumMismatch { expected, actual }) => {
            assert_eq!((expected, actual), (wrong, digest(&body())));
        }
        other => panic!("{other:?}"),
    }
    assert!(!run.destination.exists() && !part_of(&run.destination).exists());
}

#[test]
fn a_cancelled_download_stops_and_leaves_no_file() {
    let server = MockServer::start(serve_ranged(body()));
    let dir = tempfile::tempdir().unwrap();
    let token = CancelToken::new();
    // The progress callback cancels after the first chunk.
    let cancel = token.clone();
    let url = HttpUrl::new(server.url("/f")).unwrap();
    let destination = dir.path().join("file.bin");
    let report = move |_: Progress| cancel.cancel();
    let control = TransferControl {
        cancel: &token,
        progress: &report,
    };
    let request = DownloadRequest {
        url: &url,
        destination: &destination,
        sha256: None,
    };
    let result = downloader().download(&request, &control);
    assert!(
        matches!(result, Err(DownloadError::Cancelled)),
        "{result:?}"
    );
    assert!(!destination.exists() && !part_of(&destination).exists());
}
