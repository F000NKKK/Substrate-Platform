//! Why a workflow file could not be read.

/// Why a workflow file could not be read.
#[derive(Debug, thiserror::Error)]
#[error("invalid workflow file: {0}")]
pub struct FileError(#[from] toml::de::Error);
