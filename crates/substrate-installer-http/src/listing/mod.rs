//! Listing releases: GitHub's, crates.io's and a toolchain's own.

mod crate_versions;
mod github_asset_dto;
mod github_release_dto;
mod github_releases;
mod http_release_source;
mod source_archive;

pub use http_release_source::HttpReleaseSource;

#[cfg(test)]
mod tests;
