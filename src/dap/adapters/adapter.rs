//! One adapter of the product's list.

use crate::dap::{AdapterDefinition, AdapterId};

/// One adapter: the languages it debugs and how it is started.
#[derive(Debug, Clone, PartialEq)]
pub struct Adapter {
    /// Its id.
    pub id: AdapterId,
    /// The ids of the languages it debugs (as `lsp` names them).
    pub languages: Vec<String>,
    /// How it is started.
    pub definition: AdapterDefinition,
}

impl Adapter {
    /// Whether it debugs `language`.
    #[must_use]
    pub fn debugs(&self, language: &str) -> bool {
        self.languages.iter().any(|l| l == language)
    }
}
