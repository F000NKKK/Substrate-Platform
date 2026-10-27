//! Whether a new branch is checked out.

/// Whether a new branch is checked out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BranchCheckout {
    /// The current branch stays checked out.
    Stay,
    /// The new branch is checked out.
    Switch,
}
