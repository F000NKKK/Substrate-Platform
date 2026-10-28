//! Windows: a setup program run silently.

mod install;
mod setup_command;
mod setup_run;
mod uninstall;

pub(super) use uninstall::uninstall_on_windows;
