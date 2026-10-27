//! How a language server is started.

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::Value;

/// How a language server is started.
#[derive(Debug, Clone, PartialEq)]
pub struct ServerDefinition {
    /// The program to start.
    pub command: String,
    /// Its arguments.
    pub args: Vec<String>,
    /// Environment variables to set for it.
    pub env: BTreeMap<String, String>,
    /// What `initialize` passes on to it as `initializationOptions`.
    pub initialization_options: Option<Value>,
}

impl ServerDefinition {
    /// A server started by `command` with no arguments, environment or options.
    #[must_use]
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            args: Vec::new(),
            env: BTreeMap::new(),
            initialization_options: None,
        }
    }

    /// The server's name for the user: its program.
    #[must_use]
    pub fn name(&self) -> &str {
        Path::new(&self.command)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(&self.command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_server_is_named_by_its_program() {
        assert_eq!(
            ServerDefinition::new("/opt/bin/rust-analyzer").name(),
            "rust-analyzer"
        );
        assert_eq!(
            ServerDefinition::new("pyright-langserver").name(),
            "pyright-langserver"
        );
        assert_eq!(
            ServerDefinition::new("C:\\ls\\clangd.exe").command,
            "C:\\ls\\clangd.exe"
        );
    }
}
