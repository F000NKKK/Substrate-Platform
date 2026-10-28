//! Listing releases: GitHub's, crates.io's and a toolchain's own.

mod crate_versions;
mod github_dto;
mod github_releases;
mod http_release_source;

pub use http_release_source::HttpReleaseSource;

#[cfg(test)]
mod tests;
