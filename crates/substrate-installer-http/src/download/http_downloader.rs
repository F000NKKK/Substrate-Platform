//! The [`Downloader`] over HTTP.

use std::fs;
use std::io::Write;

use substrate_installer_app::ports::downloads::{
    DownloadError, DownloadRequest, Downloader, TransferControl,
};

use super::part_file::part_path;
use super::stop::Stop;
use super::transfer::Transfer;
use crate::{HttpClient, HttpError};

/// Downloads files. The bytes go to `<destination>.part`, which a later try
/// goes on from (a `Range` request) after a pause, a dropped connection or a
/// crash; once whole and verified against the request's digest it is renamed
/// to the destination, so the destination is never half a file. A cancelled
/// or damaged download deletes the partial file. A pause closes the
/// connection and waits at the cancel token; going on asks for the rest.
#[derive(Debug, Clone)]
pub struct HttpDownloader {
    client: HttpClient,
}

impl HttpDownloader {
    /// A downloader that asks through `client`.
    #[must_use]
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    fn run(
        &self,
        request: &DownloadRequest<'_>,
        control: &TransferControl<'_>,
        tee: Option<&mut dyn Write>,
    ) -> Result<(), DownloadError> {
        let part = part_path(request.destination);
        let transfer = Transfer::open(&self.client, request, control, tee, &part)
            .and_then(Transfer::run)
            .and_then(|()| finish(&part, request));
        transfer.map_err(|stop| {
            if stop.discards_partial() {
                drop(fs::remove_file(&part));
            }
            stop.into_error(request.url)
        })
    }
}

impl Downloader for HttpDownloader {
    fn download(
        &self,
        request: &DownloadRequest<'_>,
        control: &TransferControl<'_>,
    ) -> Result<(), DownloadError> {
        self.run(request, control, None)
    }

    fn stream(
        &self,
        request: &DownloadRequest<'_>,
        control: &TransferControl<'_>,
        tee: &mut dyn Write,
    ) -> Result<(), DownloadError> {
        self.run(request, control, Some(tee))
    }
}

/// The whole file takes the destination's place.
fn finish(part: &std::path::Path, request: &DownloadRequest<'_>) -> Result<(), Stop> {
    fs::rename(part, request.destination).map_err(|source| {
        Stop::Failed(HttpError::Io {
            path: request.destination.to_path_buf(),
            source,
        })
    })
}
