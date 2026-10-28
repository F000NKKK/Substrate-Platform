//! Running one program to its end, supervised.

mod ended;
mod execute;
mod output;
mod process_group;
mod supervise;

pub(crate) use execute::execute;
