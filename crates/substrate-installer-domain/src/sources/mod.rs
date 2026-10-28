//! Sources: where a component's versions are published, and so how it is
//! installed. A closed set of kinds, each with typed fields.

mod crate_name;
mod repo_slug;
mod rust_target;
mod source;
mod source_kind;
mod system_package;
mod toolchain;

pub use crate_name::CrateName;
pub use repo_slug::RepoSlug;
pub use rust_target::RustTarget;
pub use source::Source;
pub use source_kind::SourceKind;
pub use system_package::SystemPackage;
pub use toolchain::Toolchain;
