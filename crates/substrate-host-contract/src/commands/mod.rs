//! Host commands by domain: a product's backend groups its commands into
//! domains, each a list of names under a plugin (`plugin:<plugin>|<command>`).
//! The lists are the one source the permissions, the handlers' registration
//! and the UI's copy (`typescript`) are made from. Substrate's own domains —
//! its host plugin's commands — are here too ([`SUBSTRATE`]).

pub mod dap;
mod domain;
pub mod fs;
pub mod git;
pub mod lsp;
pub mod open;
pub mod search;
pub mod state;
mod substrate;
pub mod terminal;
mod typescript;

pub use domain::Domain;
pub use substrate::{SUBSTRATE, SUBSTRATE_PLUGIN};
pub use typescript::{names, typescript};

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
