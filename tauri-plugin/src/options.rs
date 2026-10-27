//! What a product tells the plugin about itself.

/// What a product tells the plugin about itself.
#[derive(Debug, Default)]
pub struct Options {
    /// What a search keeps out of the walk, and where generated code lives.
    #[cfg(feature = "search")]
    pub search: substrate_search_domain::project::Layout,
    /// The languages the product knows, and their servers.
    #[cfg(feature = "lsp")]
    pub languages: substrate_lsp_domain::Languages,
    /// The debug adapters the product knows.
    #[cfg(feature = "dap")]
    pub adapters: substrate_dap_domain::Adapters,
    /// How the product's own file kinds are read and written as text.
    pub text_views: substrate_workspace_domain::TextViews,
    /// Where the product keeps its own data in a project.
    #[cfg(feature = "state")]
    pub state: Option<crate::state::StateOptions>,
}
