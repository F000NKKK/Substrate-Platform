//! A request the mock server got.

use std::collections::HashMap;

/// A request the server got.
#[derive(Debug, Clone)]
pub(crate) struct Request {
    pub(crate) path: String,
    /// Header names in lowercase.
    pub(crate) headers: HashMap<String, String>,
}
