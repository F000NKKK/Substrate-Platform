//! Opening and saving solutions and projects: a product registers the
//! formats it reads ([`ports::SolutionFormat`], [`ports::ProjectFormat`]) in
//! [`Formats`], which opens a file with the format that reads it. Saving
//! under another format's name converts.
#![doc = include_str!("../README.md")]

mod formats;
pub mod ports;

pub use formats::Formats;
