//! What releases need of each other, and checking a selection against it.
//!
//! A release may carry a manifest (its file name is the catalog's, not the
//! engine's) saying what it needs of the other components. Reading the
//! manifest is an adapter's job; [`Needs`] is what it yields.

mod check;
mod chosen;
mod conflict;
mod needs;

pub use check::check;
pub use chosen::Chosen;
pub use conflict::Conflict;
pub use needs::Needs;
