//! What a search or a replace covers.

use super::{Layout, OpenDocument, Scope};

/// What a search or a replace covers, before its mask is applied.
pub(crate) struct Coverage<'a> {
    /// The project root — required for `Project` scope, used for relative paths otherwise.
    pub(crate) root: Option<&'a str>,
    /// Open documents only, or the whole project.
    pub(crate) scope: Scope,
    /// Open documents: their buffers stand in for their files.
    pub(crate) documents: &'a [OpenDocument],
    /// What the product keeps out of the walk.
    pub(crate) layout: &'a Layout,
}
