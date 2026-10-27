//! What Replace All changed.

use serde::Serialize;

use super::{ReplacedDocument, ReplacedFile};
use crate::message::Message;

/// What Replace All changed, and what it couldn't.
#[derive(Clone, Default, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
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
