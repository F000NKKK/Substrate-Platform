//! `${key}` substitution.

use std::collections::HashMap;
use std::hash::BuildHasher;

/// Replaces every `${key}` in `input` with `vars[key]` — deliberately a plain
/// string substitution, not a templating engine; a step's `run`/`args`/`cwd`
/// only ever need one variable's value dropped in, not conditionals or loops.
#[must_use]
pub fn substitute<S: BuildHasher>(input: &str, vars: &HashMap<String, String, S>) -> String {
    let mut out = input.to_string();
    for (key, value) in vars {
        out = out.replace(&format!("${{{key}}}"), value);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_known_variable_is_replaced() {
        let vars = HashMap::from([
            ("name".to_string(), "core".to_string()),
            ("mode".to_string(), "release".to_string()),
        ]);
        assert_eq!(
            substitute("${name}/${mode}/${name}", &vars),
            "core/release/core"
        );
        assert_eq!(substitute("${unknown} $name", &vars), "${unknown} $name");
    }
}
