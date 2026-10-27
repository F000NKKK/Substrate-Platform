//! How a debug adapter is started.

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::{Map, Value};

/// How a debug adapter is started, and what a launch starts from.
#[derive(Debug, Clone, PartialEq)]
pub struct AdapterDefinition {
    /// The program that speaks the protocol on its standard streams.
    pub command: String,
    /// Its arguments.
    pub args: Vec<String>,
    /// Its environment, on top of the IDE's.
    pub env: BTreeMap<String, String>,
    /// What every `launch` starts from: the arguments the product's launch
    /// configuration then overrides.
    pub launch_defaults: Option<toml::Table>,
}

impl AdapterDefinition {
    /// The adapter's name for the user: its program.
    #[must_use]
    pub fn name(&self) -> &str {
        Path::new(&self.command)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(&self.command)
    }

    /// The arguments of a `launch`: the defaults, each key replaced by
    /// `overrides`' when it has one (`overrides` that isn't an object adds
    /// nothing).
    #[must_use]
    pub fn launch_arguments(&self, overrides: &Value) -> Value {
        let mut arguments = match self
            .launch_defaults
            .as_ref()
            .and_then(|defaults| serde_json::to_value(defaults).ok())
        {
            Some(Value::Object(map)) => map,
            _ => Map::new(),
        };
        if let Value::Object(overrides) = overrides {
            arguments.extend(overrides.clone());
        }
        Value::Object(arguments)
    }
}
