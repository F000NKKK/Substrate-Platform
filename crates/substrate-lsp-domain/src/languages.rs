//! The languages a product knows.

use std::path::Path;

use crate::Language;

/// A product's languages, in its list's order of ids. A language without a
/// server is still one: the editor highlights it, nothing more.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Languages {
    languages: Vec<Language>,
}

impl Languages {
    /// These languages, in this order.
    #[must_use]
    pub fn new(languages: Vec<Language>) -> Self {
        Self { languages }
    }

    /// Every language, in the list's order.
    #[must_use]
    pub fn all(&self) -> &[Language] {
        &self.languages
    }

    /// The language with this id.
    #[must_use]
    pub fn find(&self, id: &str) -> Option<&Language> {
        self.languages.iter().find(|l| l.id == id)
    }

    /// The language `path` is in — a file name match before an extension.
    #[must_use]
    pub fn for_path(&self, path: &Path) -> Option<&Language> {
        let name = path.file_name().and_then(|n| n.to_str())?;
        self.languages
            .iter()
            .find(|l| l.file_names.iter().any(|f| f == name))
            .or_else(|| self.languages.iter().find(|l| l.contains(path)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ServerDefinition;

    fn language(id: &str, extensions: &[&str], file_names: &[&str]) -> Language {
        Language {
            id: id.into(),
            extensions: extensions.iter().map(ToString::to_string).collect(),
            file_names: file_names.iter().map(ToString::to_string).collect(),
            server: None,
        }
    }

    fn languages() -> Languages {
        Languages::new(vec![
            Language {
                server: Some(ServerDefinition::new("rust-analyzer")),
                ..language("rust", &["rs"], &[])
            },
            language("toml", &["toml"], &["Cargo.lock"]),
        ])
    }

    #[test]
    fn languages_are_found_by_file_name_then_extension() {
        let languages = languages();
        let id = |p: &str| languages.for_path(Path::new(p)).map(|l| l.id.as_str());
        assert_eq!(id("src/MAIN.RS"), Some("rust"));
        assert_eq!(id("Cargo.toml"), Some("toml"));
        assert_eq!(id("Cargo.lock"), Some("toml"));
        assert_eq!(id("notes.txt"), None);
        assert_eq!(id("/"), None);
    }

    #[test]
    fn a_language_is_found_by_id() {
        let languages = languages();
        assert!(languages.find("rust").is_some_and(|l| l.server.is_some()));
        assert!(languages.find("toml").is_some_and(|l| l.server.is_none()));
        assert!(languages.find("cobol").is_none());
        assert_eq!(languages.all().len(), 2);
    }
}
