//! The anticorruption layer for git's porcelain: its text, turned into
//! domain types here and nowhere else. Pure functions, tested on fixtures.

mod branches;
mod log;
mod name_status;
mod stashes;
mod status;

pub use branches::{BRANCH_FORMAT, parse_branches};
pub use log::{LOG_FORMAT, parse_log};
pub use name_status::parse_name_status;
pub use stashes::{STASH_FORMAT, parse_stashes};
pub use status::parse_status;
