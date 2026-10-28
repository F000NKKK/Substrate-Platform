//! A downloader that writes nothing and tells what it was asked.

use std::io::Write;

use substrate_installer_domain::releases::Sha256;

use super::DownloadScript;
use crate::ports::downloads::{
    DownloadError, DownloadRequest, Downloader, Progress, TransferControl,
};
use crate::testing::{Call, Journal};

/// What the fake downloads: bytes of `payload`.
#[derive(Debug)]
pub(crate) struct FakeDownloader {
    pub(crate) journal: Journal,
    pub(crate) script: DownloadScript,
    pub(crate) payload: Vec<u8>,
}

impl FakeDownloader {
    pub(crate) fn new(journal: Journal) -> Self {
        Self {
            journal,
            script: DownloadScript::Works,
            payload: b"archive-bytes".to_vec(),
        }
    }

    fn fail_by_script(
        &self,
        request: &DownloadRequest<'_>,
        control: &TransferControl<'_>,
    ) -> Result<(), DownloadError> {
        match self.script {
            DownloadScript::Works => Ok(()),
            DownloadScript::Fails => Err(DownloadError::Failed {
                url: request.url.clone(),
                cause: "no network".into(),
            }),
            DownloadScript::WrongDigest => Err(DownloadError::ChecksumMismatch {
                expected: request
                    .sha256
                    .copied()
                    .unwrap_or(Sha256::from_bytes([0; 32])),
                actual: Sha256::from_bytes([9; 32]),
            }),
            DownloadScript::CancelsOnArrival => {
                control.cancel.cancel();
                Ok(control.cancel.check()?)
            }
        }
    }
}

impl Downloader for FakeDownloader {
    fn download(
        &self,
        request: &DownloadRequest<'_>,
        control: &TransferControl<'_>,
    ) -> Result<(), DownloadError> {
        self.journal.record(Call::Download {
            url: request.url.to_string(),
            destination: request.destination.to_path_buf(),
        });
        self.fail_by_script(request, control)?;
        let total = self.payload.len() as u64;
        (control.progress)(Progress {
            done: total,
            total: Some(total),
        });
        Ok(())
    }

    fn stream(
        &self,
        request: &DownloadRequest<'_>,
        control: &TransferControl<'_>,
        tee: &mut dyn Write,
    ) -> Result<(), DownloadError> {
        self.journal.record(Call::Stream {
            url: request.url.to_string(),
            destination: request.destination.to_path_buf(),
        });
        self.fail_by_script(request, control)?;
        let (first, second) = self.payload.split_at(self.payload.len() / 2);
        for (chunk, done) in [(first, first.len()), (second, self.payload.len())] {
            tee.write_all(chunk)
                .map_err(DownloadError::ConsumerStopped)?;
            (control.progress)(Progress {
                done: done as u64,
                total: Some(self.payload.len() as u64),
            });
        }
        Ok(())
    }
}
