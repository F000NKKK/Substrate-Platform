//! Linux: an `AppImage` or a Debian package's program, and its launcher entry.

mod desktop_refresh;
mod install;
mod launcher_plan;
mod unlist;

pub use launcher_plan::{LauncherEntry, LauncherPlan, plan_launcher};
