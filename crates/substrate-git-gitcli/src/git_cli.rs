//! The adapter.

use substrate_platform::text_view::TextViews;

/// Source control through the `git` executable. Clones share the text views.
#[derive(Clone, Debug, Default)]
pub struct GitCli {
    pub(crate) views: TextViews,
}

impl GitCli {
    /// An adapter whose diffs show a product's binary formats as the text its
    /// `views` read them as.
    #[must_use]
    pub fn new(views: TextViews) -> Self {
        Self { views }
    }
}
