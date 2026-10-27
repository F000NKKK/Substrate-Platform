//! What to do with the operation in progress.

/// What to do with the operation in progress.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resolution {
    /// Go on with it (`--continue`; a merge is committed).
    Continue,
    /// Give it up (`--abort`).
    Abort,
}
