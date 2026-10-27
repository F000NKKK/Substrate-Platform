//! Where the branch to switch to is.

/// Where the branch to switch to is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BranchSource {
    /// A local branch, checked out as it is.
    Local,
    /// A remote-tracking branch (`origin/feature`): a local branch tracking it is made.
    Remote,
}
