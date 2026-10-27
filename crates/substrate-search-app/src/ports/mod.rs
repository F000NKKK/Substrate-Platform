//! What a search needs from the outside: files to walk, text to read and write.

mod decoding;
mod document_source;
mod file_walker;

pub use decoding::Decoding;
pub use document_source::DocumentSource;
pub use file_walker::FileWalker;
