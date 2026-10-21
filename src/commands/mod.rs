//! Host commands by domain: a product's backend groups its commands into
//! domains, each a list of names under a plugin (`plugin:<plugin>|<command>`).
//! The lists are the one source the permissions, the handlers' registration
//! and the UI's copy (`typescript`) are made from. Substrate's own domains —
//! its host plugin's commands — are here too ([`SUBSTRATE`]).

pub mod fs;
pub mod git;
pub mod open;
pub mod search;
pub mod state;
pub mod terminal;

/// The host plugin's name: Substrate's commands are `plugin:substrate|<command>`.
pub const SUBSTRATE_PLUGIN: &str = "substrate";

/// A domain of commands: its key in the UI's copy, its plugin, its commands.
pub struct Domain {
    pub key: &'static str,
    pub plugin: &'static str,
    pub commands: &'static [&'static str],
}

/// Substrate's own domains: every command of its host plugin.
pub const SUBSTRATE: &[&Domain] = &[
    &open::DOMAIN,
    &fs::DOMAIN,
    &git::DOMAIN,
    &search::DOMAIN,
    &state::DOMAIN,
    &terminal::DOMAIN,
];

/// Every command of `domains`, in order — a plugin's permissions.
pub fn names(domains: &[&Domain]) -> Vec<&'static str> {
    domains
        .iter()
        .flat_map(|d| d.commands.iter().copied())
        .collect()
}

/// `domains` as a TypeScript constant `name`: `name.<domain>.<command>` is
/// `{ plugin, command }` (what the UI's `invokeCommand` takes); `doc` is its comment.
pub fn typescript(name: &str, doc: &str, domains: &[&Domain]) -> String {
    let mut out = format!("/** {doc} */\nexport const {name} = {{\n");
    for domain in domains {
        out.push_str(&format!("  {}: {{\n", domain.key));
        for command in domain.commands {
            out.push_str(&format!(
                "    {}: {{ plugin: {:?}, command: {:?} }},\n",
                crate::message::camel(command),
                domain.plugin,
                command
            ));
        }
        out.push_str("  },\n");
    }
    out.push_str("} as const;\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO: Domain = Domain {
        key: "demo",
        plugin: "app",
        commands: &["path_info"],
    };

    #[test]
    fn a_domain_turns_into_typescript() {
        let ts = typescript("app", "The app's.", &[&DEMO]);
        assert!(ts.contains("export const app = {"));
        assert!(ts.contains("demo: {"));
        assert!(ts.contains(r#"pathInfo: { plugin: "app", command: "path_info" },"#));
    }

    #[test]
    fn every_command_of_substrate_is_listed_once() {
        let all = names(SUBSTRATE);
        let mut unique = all.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(all.len(), unique.len());
    }
}
