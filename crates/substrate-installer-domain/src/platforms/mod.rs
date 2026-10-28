//! Platforms: an operating system and a processor architecture as the
//! release assets are tagged with them, and the host an installer runs on.

mod arch;
mod host;
mod os;
mod platform_assets;
mod platform_error;
mod platform_tag;
mod system_library;

pub use arch::Arch;
pub use host::Host;
pub use os::Os;
pub use platform_assets::PlatformAssets;
pub use platform_error::PlatformError;
pub use platform_tag::PlatformTag;
pub use system_library::SystemLibrary;
