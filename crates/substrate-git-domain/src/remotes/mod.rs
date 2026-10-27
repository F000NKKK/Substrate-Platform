//! Working with remotes and with what is in progress.

mod clone_name;
mod integration;
mod remote_operation;
mod resolution;

pub use clone_name::clone_directory_name;
pub use integration::Integration;
pub use remote_operation::RemoteOperation;
pub use resolution::Resolution;
