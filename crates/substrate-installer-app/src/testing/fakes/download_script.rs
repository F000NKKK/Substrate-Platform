//! How the fake downloader behaves.

/// How the fake downloader behaves.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum DownloadScript {
    #[default]
    Works,
    Fails,
    WrongDigest,
    CancelsOnArrival,
}
