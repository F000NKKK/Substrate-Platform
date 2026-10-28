//! The downloader against a local server.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use sha2::{Digest, Sha256 as Hasher};
use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::downloads::{
    DownloadError, DownloadRequest, Downloader, Progress, TransferControl,
};
use substrate_installer_domain::releases::{HttpUrl, Sha256};

use super::HttpDownloader;
use crate::testing::{MockServer, Reply, Request};
use crate::{ClientConfig, HttpClient, HttpError};

const SIZE: usize = 300_000;

fn body() -> Vec<u8> {
    (0..SIZE).map(|i| u8::try_from(i % 251).unwrap()).collect()
}

fn digest(bytes: &[u8]) -> Sha256 {
    Sha256::from_bytes(Hasher::digest(bytes).into())
}

fn downloader() -> HttpDownloader {
    HttpDownloader::new(HttpClient::new(ClientConfig::new("test")))
}

/// Serves `body`, honouring `Range: bytes=N-` with a 206.
fn serve_ranged(server_body: Vec<u8>) -> impl Fn(&Request) -> Reply + Send {
    move |request| match request.headers.get("range") {
        Some(range) => {
            let from: usize = range
                .trim_start_matches("bytes=")
                .trim_end_matches('-')
                .parse()
                .unwrap();
            Reply::status(206).body(server_body[from..].to_vec())
        }
        None => Reply::ok(server_body.clone()),
    }
}

struct Run {
    result: Result<(), DownloadError>,
    destination: PathBuf,
    progress: Vec<Progress>,
}

fn run(url: &str, dir: &Path, sha256: Option<&Sha256>, cancel: &CancelToken) -> Run {
    let url = HttpUrl::new(url).unwrap();
    let destination = dir.join("sub").join("file.bin");
    let seen = Mutex::new(Vec::new());
    let report = |progress: Progress| {
        seen.lock().unwrap().push(progress);
    };
    let control = TransferControl {
        cancel,
        progress: &report,
    };
    let request = DownloadRequest {
        url: &url,
        destination: &destination,
        sha256,
    };
    let result = downloader().download(&request, &control);
    Run {
        result,
        destination,
        progress: seen.into_inner().unwrap(),
    }
}

fn part_of(destination: &Path) -> PathBuf {
    destination.with_file_name("file.bin.part")
}

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

#[test]
fn a_stream_feeds_the_consumer_every_byte_and_a_resumed_one_from_the_start() {
    let data = body();
    let server = MockServer::start(serve_ranged(data.clone()));
    let dir = tempfile::tempdir().unwrap();
    let destination = dir.path().join("file.bin");
    fs::write(part_of(&destination), &data[..1000]).unwrap();
    let url = HttpUrl::new(server.url("/f")).unwrap();
    let cancel = CancelToken::new();
    let control = TransferControl {
        cancel: &cancel,
        progress: &|_| {},
    };
    let request = DownloadRequest {
        url: &url,
        destination: &destination,
        sha256: Some(&digest(&data)),
    };
    let mut tee = Vec::new();
    downloader().stream(&request, &control, &mut tee).unwrap();
    assert_eq!(tee, data);
    assert_eq!(fs::read(&destination).unwrap(), data);
    let requests = server.requests();
    assert_eq!(
        requests[0].headers.get("range").map(String::as_str),
        Some("bytes=1000-")
    );
}

#[test]
fn a_server_that_cannot_resume_restarts_a_file_but_not_a_stream() {
    let data = body();
    let whole = data.clone();
    let server = MockServer::start(move |_| Reply::ok(whole.clone()));
    let dir = tempfile::tempdir().unwrap();
    let destination = dir.path().join("file.bin");
    fs::write(part_of(&destination), &data[..10]).unwrap();
    let url = HttpUrl::new(server.url("/f")).unwrap();
    let cancel = CancelToken::new();
    let control = TransferControl {
        cancel: &cancel,
        progress: &|_| {},
    };
    let request = DownloadRequest {
        url: &url,
        destination: &destination,
        sha256: Some(&digest(&data)),
    };
    let mut tee = Vec::new();
    let streamed = downloader().stream(&request, &control, &mut tee);
    match streamed {
        Err(DownloadError::Failed { cause, .. }) => {
            assert!(matches!(
                cause.downcast_ref::<HttpError>(),
                Some(HttpError::ResumeUnsupported { .. })
            ));
        }
        other => panic!("{other:?}"),
    }
    assert!(!part_of(&destination).exists());
    fs::write(part_of(&destination), &data[..10]).unwrap();
    downloader().download(&request, &control).unwrap();
    assert_eq!(fs::read(&destination).unwrap(), data);
}

#[test]
fn a_consumer_that_stops_stops_the_download() {
    struct Broken;
    impl std::io::Write for Broken {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("closed"))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let server = MockServer::start(serve_ranged(body()));
    let dir = tempfile::tempdir().unwrap();
    let url = HttpUrl::new(server.url("/f")).unwrap();
    let cancel = CancelToken::new();
    let control = TransferControl {
        cancel: &cancel,
        progress: &|_| {},
    };
    let request = DownloadRequest {
        url: &url,
        destination: &dir.path().join("file.bin"),
        sha256: None,
    };
    let result = downloader().stream(&request, &control, &mut Broken);
    assert!(
        matches!(result, Err(DownloadError::ConsumerStopped(_))),
        "{result:?}"
    );
}

#[test]
fn a_refusal_is_a_failure_that_names_the_status() {
    let server = MockServer::start(|_| Reply::status(404));
    let dir = tempfile::tempdir().unwrap();
    let run = run(
        &server.url("/missing"),
        dir.path(),
        None,
        &CancelToken::new(),
    );
    match run.result {
        Err(DownloadError::Failed { cause, .. }) => assert!(matches!(
            cause.downcast_ref::<HttpError>(),
            Some(HttpError::Status { status: 404, .. })
        )),
        other => panic!("{other:?}"),
    }
    assert!(!run.destination.exists());
}
