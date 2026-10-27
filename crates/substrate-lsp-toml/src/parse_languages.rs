//! Parsing the list of languages.

use substrate_lsp_domain::{Languages, LspError};

use crate::language_file_dto::LanguageFileDto;

/// Parses the list of languages from TOML, in the order of their ids.
///
/// # Errors
///
/// The text is not TOML of that shape.
pub fn parse_languages(text: &str) -> Result<Languages, LspError> {
    let file: LanguageFileDto = toml::from_str(text).map_err(|error| LspError::InvalidList {
        reason: error.message().to_string(),
    })?;
    Ok(Languages::new(
        file.language
            .into_iter()
            .map(|(id, language)| language.into_language(id))
            .collect(),
    ))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use serde_json::json;
    use substrate_lsp_domain::messages;
    use substrate_message::Message;

    use super::*;

    const LIST: &str = r#"
        [language.rust]
        extensions = ["rs"]
        server = { command = "/opt/bin/rust-analyzer" }

        [language.toml]
        extensions = [".toml"]
        file_names = ["Cargo.lock"]

        [language.python]
        extensions = ["py"]
        server = { command = "pyright-langserver", args = ["--stdio"], initialization_options = { a = 1 } }
    "#;

    #[test]
    fn a_list_becomes_languages_and_servers() {
        let languages = parse_languages(LIST).unwrap();
        let id = |p: &str| languages.for_path(Path::new(p)).map(|l| l.id.as_str());
        assert_eq!(id("src/MAIN.RS"), Some("rust"));
        assert_eq!(id("Cargo.toml"), Some("toml"));
        assert_eq!(id("Cargo.lock"), Some("toml"));
        assert_eq!(id("notes.txt"), None);

        let rust = languages.find("rust").unwrap().server.as_ref().unwrap();
        assert_eq!(rust.name(), "rust-analyzer");
        let python = languages.find("python").unwrap().server.as_ref().unwrap();
        assert_eq!(python.args, ["--stdio"]);
        assert_eq!(python.initialization_options, Some(json!({ "a": 1 })));
        assert!(languages.find("toml").unwrap().server.is_none());
    }

    #[test]
    fn an_empty_list_is_no_languages() {
        assert_eq!(parse_languages("").unwrap().all(), []);
    }

    #[test]
    fn a_bad_list_says_why() {
        let error = parse_languages("[language.rust]\nextension = [\"rs\"]").unwrap_err();
        assert_eq!(Message::from(error).text, messages::INVALID_LIST);
    }
}
