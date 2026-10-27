//! What Replace All changed.

use substrate_message::Message;

use super::{ReplacedDocument, ReplacedFile};

/// What Replace All changed, and what it couldn't.
#[derive(Clone, Default, Debug)]
pub struct ReplaceSummary {
    /// Open documents' new text, for the UI to put in their buffers.
    pub documents: Vec<ReplacedDocument>,
    /// Files rewritten on disk (none of them open).
    pub files: Vec<ReplacedFile>,
    /// Replacements made, in both.
    pub total: usize,
    /// Files that couldn't be written.
    pub errors: Vec<Message>,
}
