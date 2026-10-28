//! Installing a platform's programs: one folder per platform with the work
//! only that platform does, and the pure plans that decide what to do.

mod host_products;
mod launcher_dirs;
mod linux;
mod macos;
mod package_kind;
mod product_facts;
mod product_failure;
mod windows;

pub use host_products::HostProducts;
pub use launcher_dirs::{DesktopCaches, LauncherDirs};
pub use linux::{LauncherEntry, LauncherPlan, plan_launcher};
pub use package_kind::PackageKind;
pub use product_failure::ProductFailure;
