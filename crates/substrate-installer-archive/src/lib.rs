//! The unpacker an installer's downloads go through: zip, tar.gz, tar.xz and
//! a Debian package's files, read from a file or from a stream while it
//! arrives, never written outside the folder they go into.
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod archive_failure;
mod archive_unpacker;
mod unpack_deb;
mod unpack_tar;
mod unpack_zip_file;
mod unpack_zip_stream;
mod writing;
mod zip_entry;

pub use archive_failure::ArchiveFailure;
pub use archive_unpacker::ArchiveUnpacker;

#[cfg(test)]
mod tests;
