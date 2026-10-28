//! HTTP adapters of the installer ports, over `ureq` with rustls.
//!
//! - [`HttpReleaseSource`] implements `ReleaseSource`: GitHub releases (with
//!   each asset's digest, and the source archive of a tag), crates.io
//!   versions, and a toolchain's name as its one release; drafts and yanked
//!   versions are left out.
//! - [`HttpDownloader`] implements `Downloader`: resumable, verified against
//!   the release's SHA-256, cancellable and pausable between chunks, and
//!   streamable to a writer as the bytes arrive.
//! - [`HttpClient`] holds the agent, the optional token and the `ETag` cache
//!   of JSON answers; [`ClientConfig`] says how it is made. There is no global
//!   state: the composition root builds one and hands it to both adapters.
//!
//! Failures reach the ports as their `cause`: [`HttpError`] and
//! [`ReleaseReadError`]; [`is_rate_limited`] tells a rate limit from the rest.
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod cached_answer;
mod client_config;
mod download;
mod endpoints;
mod github_token;
mod http_client;
mod http_error;
mod listing;
mod rate_limit;
mod release_read_error;
mod response_cache;
#[cfg(test)]
mod testing;

pub use client_config::ClientConfig;
pub use download::HttpDownloader;
pub use endpoints::Endpoints;
pub use github_token::GithubToken;
pub use http_client::HttpClient;
pub use http_error::{HttpError, is_rate_limited};
pub use listing::HttpReleaseSource;
pub use release_read_error::ReleaseReadError;
