//! Versions, ordered and matched leniently, and the requirements on them.

mod version;
mod version_error;
mod version_requirement;

pub use version::Version;
pub use version_error::VersionError;
pub use version_requirement::VersionRequirement;
