//! What the download tests share.

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use sha2::{Digest, Sha256 as Hasher};
use substrate_installer_app::control::CancelToken;
use substrate_installer_app::ports::downloads::{
    DownloadError, DownloadRequest, Downloader, Progress, TransferControl,
};
use substrate_installer_domain::releases::{HttpUrl, Sha256};

use crate::download::HttpDownloader;
use crate::testing::{Reply, Request};
use crate::{ClientConfig, HttpClient};

pub(super) const SIZE: usize = 300_000;

pub(super) fn body() -> Vec<u8> {
    (0..SIZE).map(|i| u8::try_from(i % 251).unwrap()).collect()
}

pub(super) fn digest(bytes: &[u8]) -> Sha256 {
    Sha256::from_bytes(Hasher::digest(bytes).into())
}

pub(super) fn downloader() -> HttpDownloader {
    HttpDownloader::new(HttpClient::new(ClientConfig::new("test")))
}

/// Serves `body`, honouring `Range: bytes=N-` with a 206.
pub(super) fn serve_ranged(server_body: Vec<u8>) -> impl Fn(&Request) -> Reply + Send {
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

pub(super) struct Run {
    pub(super) result: Result<(), DownloadError>,
    pub(super) destination: PathBuf,
    pub(super) progress: Vec<Progress>,
}

pub(super) fn run(url: &str, dir: &Path, sha256: Option<&Sha256>, cancel: &CancelToken) -> Run {
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

pub(super) fn part_of(destination: &Path) -> PathBuf {
    destination.with_file_name("file.bin.part")
}
