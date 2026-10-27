//! Starting other programs with the system's own environment. A program
//! packaged as an AppImage runs with variables its runtime set for it —
//! `LD_LIBRARY_PATH` into its mount, its GTK and GIO modules, `APPDIR`… —
//! and a program it starts that inherits them loads the AppImage's
//! libraries instead of its own, and fails (`version 'LIBSYSTEMD_254' not
//! found`). Everything a product starts goes through here — programs
//! ([`command`], [`start`]), and links and files opened with the user's
//! default app ([`open_link`], [`open_path`], [`reveal`]).

mod environment;
mod messages;
mod open;

pub use environment::{command, host_environment};
pub use messages::{ALL, CANT_START, NOT_A_LINK};
pub use open::{open_link, open_path, reveal, start};
