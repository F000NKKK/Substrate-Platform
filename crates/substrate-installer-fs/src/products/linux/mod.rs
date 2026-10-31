//! Linux: an `AppImage` or a Debian package's program, and its launcher entry.

mod desktop_refresh;
mod install;
mod launcher_entry;
mod launcher_plan;
mod package_icon;
mod presentation;
mod unlist;

pub use launcher_entry::LauncherEntry;
pub use launcher_plan::{LauncherPlan, plan_launcher};
