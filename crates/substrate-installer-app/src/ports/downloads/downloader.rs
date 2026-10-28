//! The port that downloads.

use std::io::Write;

use super::{DownloadError, DownloadRequest, TransferControl};

/// Downloads a file over the network. Blocking. An adapter resumes a partial
/// file where the server allows, checks `control.cancel` between chunks (which
/// is also where a pause waits), tells `control.progress`, and verifies the
/// digest of the whole file when the request carries one.
pub trait Downloader: Send + Sync {
    /// Downloads `request` to its destination.
    ///
    /// # Errors
    ///
    /// [`DownloadError`]: cancelled, a digest that does not match, or a
    /// failure of the transfer.
    fn download(
        &self,
        request: &DownloadRequest<'_>,
        control: &TransferControl<'_>,
    ) -> Result<(), DownloadError>;

    /// Downloads like [`download`](Self::download), and writes every byte to
    /// `tee` as it arrives, from the start of the file (a resumed download
    /// feeds `tee` the bytes it already has first). For whoever installs
    /// while the download goes on.
    ///
    /// # Errors
    ///
    /// As [`download`](Self::download), and
    /// [`DownloadError::ConsumerStopped`] when writing to `tee` fails.
    fn stream(
        &self,
        request: &DownloadRequest<'_>,
        control: &TransferControl<'_>,
        tee: &mut dyn Write,
    ) -> Result<(), DownloadError>;
}
