//! What a search looks for, compiled: the [`Matcher`] of a query, and the
//! [`FileMask`] of the files it covers.

mod file_mask;
mod matcher;
mod replacement_mode;

pub use file_mask::FileMask;
pub use matcher::Matcher;
pub use replacement_mode::ReplacementMode;

#[cfg(test)]
mod tests;
