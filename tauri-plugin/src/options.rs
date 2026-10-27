//! What a product tells the plugin about itself.

/// What a product tells the plugin about itself.
#[derive(Default)]
pub struct Options {
    /// What a search keeps out of the walk, and where generated code lives.
    #[cfg(feature = "search")]
    pub search: substrate_platform::search::Layout,
    /// The languages the product knows, and their servers.
    #[cfg(feature = "lsp")]
    pub languages: substrate_platform::lsp::Languages,
    /// Where the product keeps its own data in a project.
    #[cfg(feature = "state")]
    pub state: Option<crate::state::StateOptions>,
}
