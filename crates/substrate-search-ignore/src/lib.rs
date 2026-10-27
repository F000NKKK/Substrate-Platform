//! Find and Replace in Files over the file system: [`IgnoreWalker`] walks a
//! project the way a developer expects (`.gitignore`d entries and the
//! layout's build output left out), [`DiskDocuments`] reads and writes text
//! safely (too large, binary or invalid files are no text) through the
//! product's text views.
#![doc = include_str!("../README.md")]

mod disk_documents;
mod generated_walk;
mod ignore_walker;
mod project_walk;

pub use disk_documents::DiskDocuments;
pub use ignore_walker::IgnoreWalker;
