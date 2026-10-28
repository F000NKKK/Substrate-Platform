//! What a running tool reports to and obeys.

use crate::control::CancelToken;

/// What a running tool reports to and obeys: the operation's token and
/// where each line the tool prints goes.
#[derive(Clone, Copy)]
pub struct ToolControl<'a> {
    /// Pauses and cancels the tool. An adapter pauses and kills the tool's
    /// whole process group, so what it starts goes with it.
    pub cancel: &'a CancelToken,
    /// Told every line the tool prints (stdout and stderr), as it prints it.
    pub output: &'a dyn Fn(&str),
}

impl std::fmt::Debug for ToolControl<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToolControl")
            .field("cancel", &self.cancel)
            .finish_non_exhaustive()
    }
}
