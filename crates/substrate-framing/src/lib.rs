//! The framing the Language Server and Debug Adapter protocols share: a
//! `Content-Length` header, a blank line, then that many bytes of JSON.
#![doc = include_str!("../README.md")]

mod read_frame;
mod write_frame;

pub use read_frame::read_frame;
pub use write_frame::write_frame;
