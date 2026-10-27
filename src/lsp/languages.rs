//! The languages a product knows, from its TOML list:
//!
//! ```toml
//! [language.rust]
//! extensions = ["rs"]
//! server = { command = "rust-analyzer" }
//!
//! [language.python]
//! extensions = ["py", "pyi"]
//! server = { command = "pyright-langserver", args = ["--stdio"] }
//!
//! [language.toml]
//! extensions = ["toml"]
//! file_names = ["Cargo.lock"]
//! ```
//!
//! The key is the language's id as the protocol names it (`languageId`). A
//! language without a server is still one: the editor highlights it, nothing
//! more.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;
use serde_json::Value;

use super::LspError;

/// How a language server is started.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ServerDefinition {
    /// The program to start.
    pub command: String,
    /// Its arguments.
    #[serde(default)]
    pub args: Vec<String>,
    /// Environment variables to set for it.
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    /// What `initialize` passes on to it as `initializationOptions`.
    #[serde(default)]
    pub initialization_options: Option<toml::Table>,
}

impl ServerDefinition {
    /// The server's name for the user: its program.
    pub fn name(&self) -> &str {
        Path::new(&self.command)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(&self.command)
    }

    /// The options as JSON, for [`super::Client::initialize`].
    pub fn initialization_options(&self) -> Option<Value> {
        self.initialization_options
            .as_ref()
            .and_then(|t| serde_json::to_value(t).ok())
    }
}

/// One language: the files that are in it and its server, if any.
#[derive(Debug, Clone, PartialEq)]
pub struct Language {
    /// Its id, as the list names it.
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

#[derive(Deserialize)]
struct LanguageFile {
    #[serde(default)]
    language: BTreeMap<String, LanguageEntry>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LanguageEntry {
    #[serde(default)]
    extensions: Vec<String>,
    #[serde(default)]
    file_names: Vec<String>,
    server: Option<ServerDefinition>,
}

/// A product's languages, in its list's order of ids.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Languages {
    languages: Vec<Language>,
}

impl Languages {
    /// Parses the list of languages from TOML.
    ///
    /// # Errors
    ///
    /// The server has stopped, does not answer in time, or refuses.
    pub fn parse(text: &str) -> Result<Self, LspError> {
        let file: LanguageFile = toml::from_str(text).map_err(LspError::InvalidList)?;
        let languages = file
            .language
            .into_iter()
            .map(|(id, entry)| Language {
                id,
                extensions: entry
                    .extensions
                    .into_iter()
                    .map(|e| e.trim_start_matches('.').to_string())
                    .collect(),
                file_names: entry.file_names,
                server: entry.server,
            })
            .collect();
        Ok(Self { languages })
    }

    /// Every language, in the list's order.
    #[must_use]
    pub fn all(&self) -> &[Language] {
        &self.languages
    }

    /// The language with this id.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&Language> {
        self.languages.iter().find(|l| l.id == id)
    }

    /// The language `path` is in — a file name match before an extension.
    pub fn for_path(&self, path: &Path) -> Option<&Language> {
        let name = path.file_name().and_then(|n| n.to_str())?;
        self.languages
            .iter()
            .find(|l| l.file_names.iter().any(|f| f == name))
            .or_else(|| self.languages.iter().find(|l| l.contains(path)))
    }
}
