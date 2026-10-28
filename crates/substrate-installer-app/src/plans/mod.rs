//! Planning: turning what a user wants and what is published into a
//! [`Plan`](substrate_installer_domain::planning::Plan).

mod attach;
mod conflicts;
mod for_program;
mod for_removal;
mod for_selection;
mod situation;
mod survey;
#[cfg(test)]
mod tests;

pub use conflicts::conflicts;
pub use for_program::for_program;
pub use for_removal::for_removal;
pub use for_selection::for_selection;
pub use situation::Situation;
pub use survey::survey;
