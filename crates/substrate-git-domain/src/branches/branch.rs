//! A branch.

/// A local or remote-tracking branch with its tip.
#[derive(Clone, Debug, PartialEq, Eq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "remote, current and gone are independent facts git reports about a branch"
)]
pub struct Branch {
    /// `main`, `origin/main`.
    pub name: String,
    /// A remote-tracking branch.
    pub remote: bool,
    /// The one checked out.
    pub current: bool,
    /// The branch it tracks.
    pub upstream: Option<String>,
    /// Commits the upstream lacks.
    pub ahead: u32,
    /// Commits only the upstream has.
    pub behind: u32,
    /// The upstream was deleted on the remote.
    pub gone: bool,
    /// The tip commit's abbreviated hash.
    pub hash: String,
    /// The tip commit's date, unix seconds.
    pub date: i64,
    /// The tip commit's subject.
    pub subject: String,
}
