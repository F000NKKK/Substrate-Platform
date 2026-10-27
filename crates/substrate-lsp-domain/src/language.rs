//! One language of the product's list.

use std::path::Path;

use crate::ServerDefinition;

/// One language: the files that are in it and its server, if any.
#[derive(Debug, Clone, PartialEq)]
pub struct Language {
    /// Its id, as the list names it (`languageId` in the protocol).
    pub id: String,
    /// Extensions without the dot, compared ignoring case.
    pub extensions: Vec<String>,
    /// Whole file names (`Dockerfile`, `Cargo.lock`), compared exactly.
    pub file_names: Vec<String>,
    /// How to start its server, if it has one.
    pub server: Option<ServerDefinition>,
}

impl Language {
    /// Whether `path` is a file of this language.
    #[must_use]
    pub fn contains(&self, path: &Path) -> bool {
        let name = path.file_name().and_then(|n| n.to_str());
        if name.is_some_and(|n| self.file_names.iter().any(|f| f == n)) {
            return true;
        }
        path.extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| self.extensions.iter().any(|x| x.eq_ignore_ascii_case(e)))
    }
}
