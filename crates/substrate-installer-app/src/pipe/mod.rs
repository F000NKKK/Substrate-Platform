//! A pipe between the download and the unpacking that reads it as it
//! arrives.

mod channel;
mod reader;
#[cfg(test)]
mod tests;
mod writer;

pub(crate) use channel::channel;
pub(crate) use reader::PipeReader;
pub(crate) use writer::PipeWriter;
