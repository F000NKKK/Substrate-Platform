//! Branches: what they are, where one comes from, how one goes.

mod branch;
mod branch_checkout;
mod branch_source;
mod delete_mode;

pub use branch::Branch;
pub use branch_checkout::BranchCheckout;
pub use branch_source::BranchSource;
pub use delete_mode::DeleteMode;
