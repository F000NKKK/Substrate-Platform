//! An installer on the real file system: [`HomeLayout`] says where things go
//! under one home folder, [`DiskFiles`] moves a staging folder into place and
//! removes, and [`HostProducts`] installs a platform's programs (an
//! `AppImage` or Debian package with a launcher entry on Linux, a silent
//! setup on Windows, an application bundle on macOS).
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod component_root;
mod copy_tree;
mod disk_files;
mod home_layout;
mod path_segment;
mod products;
mod promote;

pub use disk_files::DiskFiles;
pub use home_layout::HomeLayout;
pub use products::{
    DesktopCaches, HostProducts, LauncherDirs, LauncherEntry, LauncherPlan, PackageKind,
    ProductFailure, RunningBundle, RunningPresentation, RunningProduct, plan_launcher,
};

#[cfg(test)]
mod tests;
