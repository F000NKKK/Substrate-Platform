//! The command lists, as the permissions and the UI's copy need them.

use super::Domain;

/// Every command of `domains`, in order — a plugin's permissions.
#[must_use]
pub fn names(domains: &[&Domain]) -> Vec<&'static str> {
    domains
        .iter()
        .flat_map(|d| d.commands.iter().copied())
        .collect()
}

/// `domains` as a TypeScript constant `name`: `name.<domain>.<command>` is
/// `{ plugin, command }` (what the UI's `invokeCommand` takes); `doc` is its comment.
#[must_use]
pub fn typescript(name: &str, doc: &str, domains: &[&Domain]) -> String {
    let mut out = ["/** ", doc, " */\nexport const ", name, " = {\n"].concat();
    for domain in domains {
        out.push_str(&["  ", domain.key, ": {\n"].concat());
        for command in domain.commands {
            out.push_str(&entry(domain.plugin, command));
        }
        out.push_str("  },\n");
    }
    out.push_str("} as const;\n");
    out
}

/// One command: `camelName: { plugin, command },`.
fn entry(plugin: &str, command: &str) -> String {
    let (plugin, command_name) = (format!("{plugin:?}"), format!("{command:?}"));
    [
        "    ",
        &substrate_message::camel(command),
        ": { plugin: ",
        &plugin,
        ", command: ",
        &command_name,
        " },\n",
    ]
    .concat()
}
