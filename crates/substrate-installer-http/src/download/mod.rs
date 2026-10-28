//! Downloading a file: resumed, verified, cancellable, streamed.

mod http_downloader;
mod part_file;
mod stop;
mod transfer;

pub use http_downloader::HttpDownloader;

#[cfg(test)]
mod tests;
