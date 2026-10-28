//! Carrying out a plan: each change downloaded, installed and recorded,
//! with progress events, until it is done, fails or is cancelled.

mod adapters;
mod apply_error;
mod apply_options;
mod archive_target;
mod download_cache;
mod fetch;
mod install;
mod installer;
mod remove;
mod run;
mod stream;
#[cfg(test)]
mod tests;

pub use adapters::Adapters;
pub use apply_error::ApplyError;
pub use apply_options::ApplyOptions;
pub use download_cache::DownloadCache;
pub use installer::Installer;
