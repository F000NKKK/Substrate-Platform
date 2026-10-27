//! A run against the remote.

/// What a run against the remote does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RemoteOperation {
    /// Fetch every remote.
    Fetch,
    /// Pull the current branch.
    Pull,
    /// Push the current branch.
    Push,
    /// Pull, then push.
    Sync,
}
