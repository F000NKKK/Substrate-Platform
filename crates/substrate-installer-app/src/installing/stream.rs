//! Installing an archive while it downloads.

use std::path::PathBuf;
use std::{io, thread};

use substrate_installer_domain::channel::Item;
use substrate_installer_domain::events::Event;
use substrate_installer_domain::ids::ItemId;
use substrate_installer_domain::releases::Release;

use super::ApplyError;
use super::archive_target::ArchiveTarget;
use super::run::Run;
use crate::pipe;
use crate::ports::downloads::{DownloadError, DownloadRequest, TransferControl};
use crate::ports::unpacking::{Strip, UnpackError, UnpackRequest};

impl<'r> Run<'r> {
    /// Downloads `item`'s archive and unpacks it at once, as the bytes
    /// arrive; where it went.
    pub(super) fn stream_in(
        &self,
        item: Item<'r>,
        (release, target): (&'r Release, &ArchiveTarget<'r>),
        installing: &Event,
    ) -> Result<PathBuf, ApplyError> {
        let id = item.id();
        let adapters = self.installer.adapters;
        adapters.files.remove(&target.staging)?;
        self.emit(installing.clone());
        let asset = target.asset;
        let cache = adapters
            .layout
            .cache_file(&id, &release.version, &asset.name);
        let request = DownloadRequest {
            url: &asset.url,
            destination: &cache,
            sha256: asset.sha256.as_ref(),
        };
        let unpack = UnpackRequest {
            kind: target.kind,
            destination: &target.staging,
            strip: Strip::DropTopFolder,
        };
        self.download_and_unpack(&id, &request, &unpack)?;
        let path = self.promote(target)?;
        self.discard_cache(&id, &cache);
        Ok(path)
    }

    /// The download feeds a pipe; the unpacker reads the pipe on a thread of
    /// its own.
    fn download_and_unpack(
        &self,
        id: &ItemId,
        request: &DownloadRequest<'_>,
        unpack: &UnpackRequest<'_>,
    ) -> Result<(), ApplyError> {
        let adapters = self.installer.adapters;
        let (mut writer, mut reader) = pipe::channel();
        let progress = self.progress(id);
        let control = TransferControl {
            cancel: self.cancel,
            progress: &progress,
        };
        thread::scope(|scope| {
            let unpacking = scope.spawn(move || {
                let result = adapters
                    .unpacker
                    .unpack_stream(&mut reader, unpack, self.cancel);
                result.and_then(|()| drain(&mut reader))
            });
            let downloaded = adapters.downloader.stream(request, &control, &mut writer);
            drop(writer);
            let unpacked = unpacking.join().map_err(|_| ApplyError::WorkerStopped)?;
            combine(downloaded, unpacked)
        })
    }
}

/// Reads what the unpacker left (an archive's trailer), so the download is
/// not cut short by a closed pipe.
fn drain(reader: &mut impl io::Read) -> Result<(), UnpackError> {
    io::copy(reader, &mut io::sink())
        .map(|_| ())
        .map_err(|error| UnpackError::Failed(Box::new(error)))
}

/// The error of a download and an unpacking that ran side by side: the
/// download's, unless it only failed because the unpacking stopped first.
fn combine(
    downloaded: Result<(), DownloadError>,
    unpacked: Result<(), UnpackError>,
) -> Result<(), ApplyError> {
    match (downloaded, unpacked) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(DownloadError::ConsumerStopped(_)) | Ok(()), Err(error)) => Err(error.into()),
        (Err(error), _) => Err(error.into()),
    }
}
