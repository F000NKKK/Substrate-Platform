//! What a product tells the plugin about itself.

/// What a product tells the plugin about itself.
#[derive(Debug, Default)]
pub struct Options {
    /// What a search keeps out of the walk, and where generated code lives.
    #[cfg(feature = "search")]
    pub search: substrate_platform::search::Layout,
    /// The languages the product knows, and their servers.
    #[cfg(feature = "lsp")]
    pub languages: substrate_platform::lsp::Languages,
    /// The debug adapters the product knows.
    #[cfg(feature = "dap")]
    pub adapters: substrate_platform::dap::Adapters,
    /// How the product's own file kinds are read and written as text.
    pub text_views: substrate_platform::text_view::TextViews,
    /// Where the product keeps its own data in a project.
    #[cfg(feature = "state")]
    pub state: Option<crate::state::StateOptions>,
}
