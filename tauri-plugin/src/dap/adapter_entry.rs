//! An adapter, for the UI.

use serde::Serialize;
use substrate_platform::dap::{Adapter, AdapterId};

/// An adapter as the UI picks one by: its id and the languages it debugs.
#[derive(Serialize)]
pub struct AdapterEntry {
    /// Its id.
    id: AdapterId,
    /// The ids of the languages it debugs.
    languages: Vec<String>,
}

impl From<&Adapter> for AdapterEntry {
    fn from(adapter: &Adapter) -> Self {
        Self {
            id: adapter.id.clone(),
            languages: adapter.languages.clone(),
        }
    }
}
