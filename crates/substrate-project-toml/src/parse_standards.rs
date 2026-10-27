//! Parsing the list of project standards.

use substrate_project_domain::{ProjectError, ProjectStandard};

use crate::standards_file_dto::StandardsFileDto;

/// Parses the list of project standards from TOML, in the file's order.
///
/// # Errors
///
/// The text is not TOML of that shape.
pub fn parse_standards(text: &str) -> Result<Vec<ProjectStandard>, ProjectError> {
    let file: StandardsFileDto =
        toml::from_str(text).map_err(|error| ProjectError::InvalidStandards {
            reason: error.message().to_string(),
        })?;
    Ok(file
        .standard
        .into_iter()
        .map(|standard| standard.into_standard())
        .collect())
}

#[cfg(test)]
mod tests {
    use substrate_message::Message;
    use substrate_project_domain::messages;

    use super::*;

    #[test]
    fn standards_keep_the_files_order() {
        let standards = parse_standards(
            r#"
            [[standard]]
            id = "rust"
            detect_files = ["Cargo.toml"]

            [[standard]]
            id = "csharp"
            detect_files = ["*.csproj", "src/*.sln"]
            "#,
        )
        .unwrap();
        let files = vec!["App.csproj".to_string(), "src/App.sln".to_string()];
        assert_eq!(standards[0].id, "rust");
        assert_eq!(
            ProjectStandard::detect(&files, &standards).as_deref(),
            Some("csharp")
        );
    }

    #[test]
    fn an_empty_list_is_no_standards() {
        assert!(parse_standards("").unwrap().is_empty());
    }

    #[test]
    fn a_bad_list_says_why() {
        let error = parse_standards("[[standard]]\nid = \"rust\"").unwrap_err();
        assert_eq!(Message::from(error).text, messages::INVALID_STANDARDS);
    }
}
