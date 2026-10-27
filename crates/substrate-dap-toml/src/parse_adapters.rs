//! Parsing the list of adapters.

use substrate_dap_domain::{Adapters, DapError};

use crate::adapter_file_dto::AdapterFileDto;

/// Parses the list of adapters from TOML, in the order of their ids.
///
/// # Errors
///
/// The text is not TOML of that shape.
pub fn parse_adapters(text: &str) -> Result<Adapters, DapError> {
    let file: AdapterFileDto = toml::from_str(text).map_err(|error| DapError::InvalidList {
        reason: error.message().to_string(),
    })?;
    Ok(Adapters::new(
        file.adapter
            .into_iter()
            .map(|(id, adapter)| adapter.into_adapter(id))
            .collect(),
    ))
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};
    use substrate_dap_domain::{AdapterId, messages};
    use substrate_message::Message;

    use super::*;

    const LIST: &str = r#"
        [adapter.lldb]
        languages = ["rust", "c", "cpp"]
        command = "/usr/bin/lldb-dap"

        [adapter.python]
        languages = ["python"]
        command = "python3"
        args = ["-m", "debugpy.adapter"]
        env = { PYTHONUNBUFFERED = "1" }
        launch_defaults = { console = "internalConsole", justMyCode = true }
    "#;

    #[test]
    fn adapters_are_found_by_id_and_language() {
        let adapters = parse_adapters(LIST).unwrap();
        assert_eq!(adapters.all().len(), 2);
        let lldb = adapters.find(&AdapterId::new("lldb")).unwrap();
        assert_eq!(lldb.definition.name(), "lldb-dap");
        assert_eq!(lldb.languages, ["rust", "c", "cpp"]);
        assert_eq!(adapters.for_language("rust").unwrap().id.as_str(), "lldb");
        let python = adapters.for_language("python").unwrap();
        assert_eq!(python.definition.args, ["-m", "debugpy.adapter"]);
        assert_eq!(python.definition.env["PYTHONUNBUFFERED"], "1");
        assert!(adapters.for_language("go").is_none());
    }

    #[test]
    fn a_launch_starts_from_the_listed_defaults() {
        let adapters = parse_adapters(LIST).unwrap();
        let python = &adapters.find(&AdapterId::new("python")).unwrap().definition;
        assert_eq!(
            python.launch_arguments(&json!({ "justMyCode": false, "program": "a.py" })),
            json!({ "console": "internalConsole", "justMyCode": false, "program": "a.py" })
        );
        assert_eq!(python.launch_arguments(&Value::Null)["justMyCode"], true);
    }

    #[test]
    fn a_bad_list_says_why() {
        let error =
            parse_adapters("[adapter.lldb]\ncommand = \"x\"\nlanguage = [\"c\"]").unwrap_err();
        assert_eq!(Message::from(error).text, messages::INVALID_LIST);
        let no_command = parse_adapters("[adapter.lldb]\nlanguages = [\"c\"]").unwrap_err();
        assert!(matches!(no_command, DapError::InvalidList { .. }));
        assert_eq!(parse_adapters("").unwrap().all(), []);
    }
}
