//! An adapter, for the UI.

use serde::Serialize;
use substrate_dap_domain::Adapter;

/// An adapter as the UI picks one by: its id and the languages it debugs.
#[derive(Serialize)]
pub(crate) struct AdapterEntry {
    /// Its id.
    id: String,
    /// The ids of the languages it debugs.
    languages: Vec<String>,
}

impl From<&Adapter> for AdapterEntry {
    fn from(adapter: &Adapter) -> Self {
        Self {
            id: adapter.id.to_string(),
            languages: adapter.languages.clone(),
        }
    }
}
