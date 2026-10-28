//! Downloads that go on from a partial file.

use std::fs;

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::downloads::{
    DownloadError, DownloadRequest, Downloader, TransferControl,
};
use substrate_installer_domain::releases::HttpUrl;

use super::support::{body, digest, downloader, part_of, serve_ranged};
use crate::HttpError;
use crate::testing::{MockServer, Reply};

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
