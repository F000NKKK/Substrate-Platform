//! What Source Control's commands share.

use substrate_git_app::WriteLock;
use substrate_git_gitcli::GitCli;
use substrate_workspace_domain::TextViews;

/// What Source Control's commands share: the adapter that runs `git`, and
/// the lock that keeps two writes apart. Clones share both.
#[derive(Clone, Debug)]
pub(crate) struct SourceControl {
    pub(super) adapter: GitCli,
    pub(super) lock: WriteLock,
}

impl SourceControl {
    /// The adapter showing a product's binary formats through `views`, and a free lock.
    pub(crate) fn new(views: TextViews) -> Self {
        Self {
            adapter: GitCli::new(views),
            lock: WriteLock::new(),
        }
    }
}
