//! Fakes of the ports.

mod download_script;
mod fake_detector;
mod fake_downloader;
mod fake_files;
mod fake_layout;
mod fake_products;
mod fake_release_source;
mod fake_store;
mod fake_tools;
mod fake_unpacker;
mod unpack_script;

pub(crate) use download_script::DownloadScript;
pub(crate) use fake_detector::FakeDetector;
pub(crate) use fake_downloader::FakeDownloader;
pub(crate) use fake_files::FakeFiles;
pub(crate) use fake_layout::FakeLayout;
pub(crate) use fake_products::FakeProducts;
pub(crate) use fake_release_source::FakeReleaseSource;
pub(crate) use fake_store::FakeStore;
pub(crate) use fake_tools::FakeTools;
pub(crate) use fake_unpacker::FakeUnpacker;
pub(crate) use unpack_script::UnpackScript;
