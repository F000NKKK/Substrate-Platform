//! Downloads that fail.

use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::downloads::{
    DownloadError, DownloadRequest, Downloader, TransferControl,
};
use substrate_installer_domain::releases::HttpUrl;

use super::support::{body, downloader, run, serve_ranged};
use crate::HttpError;
use crate::testing::{MockServer, Reply};

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
