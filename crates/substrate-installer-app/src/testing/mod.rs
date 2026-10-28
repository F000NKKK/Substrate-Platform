//! In-memory fakes of every port, and a world of them, for the crate's tests.

mod call;
mod catalog;
mod collecting_sink;
mod fakes;
mod journal;
mod world;

pub(crate) use call::Call;
pub(crate) use catalog::{
    component, host, install_of, plan_of, program, release, sample_catalog, tags, version,
};
pub(crate) use collecting_sink::CollectingSink;
pub(crate) use fakes::{
    DownloadScript, FakeDetector, FakeDownloader, FakeFiles, FakeLayout, FakeProducts,
    FakeReleaseSource, FakeStore, FakeTools, FakeUnpacker, UnpackScript,
};
pub(crate) use journal::Journal;
pub(crate) use world::{World, record};
