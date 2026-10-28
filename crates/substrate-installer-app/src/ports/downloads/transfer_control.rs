//! What a transfer reports to and obeys.

use super::Progress;
use crate::control::CancelToken;

/// What a transfer reports to and obeys: the operation's token, which it
/// checks between chunks, and where it tells its progress.
#[derive(Clone, Copy)]
pub struct TransferControl<'a> {
    /// Checked between chunks: pauses and cancels the transfer.
    pub cancel: &'a CancelToken,
    /// Told after every chunk.
    pub progress: &'a dyn Fn(Progress),
}

impl std::fmt::Debug for TransferControl<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TransferControl")
            .field("cancel", &self.cancel)
            .finish_non_exhaustive()
    }
}
