//! Reading a release's compatibility manifest.

mod dto;
mod error;
mod parse_needs;

pub use error::NeedsError;
pub use parse_needs::parse_needs;
