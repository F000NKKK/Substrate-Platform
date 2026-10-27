//! The adapters a product knows, from its TOML list:
//!
//! ```toml
//! [adapter.lldb]
//! languages = ["rust", "c", "cpp"]
//! command = "lldb-dap"
//!
//! [adapter.python]
//! languages = ["python"]
//! command = "python3"
//! args = ["-m", "debugpy.adapter"]
//! launch_defaults = { console = "internalConsole", justMyCode = true }
//! ```
//!
//! The key is the adapter's id; `languages` are the ids `lsp` knows them by.

use super::adapter_file::AdapterFile;
use crate::dap::{Adapter, AdapterId, DapError};

/// A product's debug adapters, in the order of their ids.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Adapters {
    adapters: Vec<Adapter>,
}

impl Adapters {
    /// Parses the list of adapters from TOML.
    ///
    /// # Errors
    ///
    /// The text is not TOML of that shape.
    pub fn parse(text: &str) -> Result<Self, DapError> {
        let file: AdapterFile = toml::from_str(text).map_err(DapError::InvalidList)?;
        let adapters = file
            .adapter
            .into_iter()
            .map(|(id, mut entry)| Adapter {
                id: AdapterId::new(id),
                languages: std::mem::take(&mut entry.languages),
                definition: entry.definition(),
            })
            .collect();
        Ok(Self { adapters })
    }

    /// Every adapter.
    #[must_use]
    pub fn all(&self) -> &[Adapter] {
        &self.adapters
    }

    /// The adapter `id`.
    #[must_use]
    pub fn get(&self, id: &AdapterId) -> Option<&Adapter> {
        self.adapters.iter().find(|a| &a.id == id)
    }

    /// The first adapter that debugs `language`.
    #[must_use]
    pub fn for_language(&self, language: &str) -> Option<&Adapter> {
        self.adapters.iter().find(|a| a.debugs(language))
    }
}
