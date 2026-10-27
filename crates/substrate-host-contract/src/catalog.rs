//! Every message template the core declares, for the generated copy.

/// Every template Substrate declares, for the export.
pub fn catalog() -> Vec<(&'static str, &'static str, &'static str)> {
    let groups: &[&[(&str, &str, &str)]] = &[
        #[cfg(feature = "dap")]
        substrate_platform::dap::messages::ALL,
        #[cfg(feature = "fs")]
        substrate_platform::fs::messages::ALL,
        #[cfg(feature = "git")]
        substrate_platform::git::messages::ALL,
        #[cfg(feature = "lsp")]
        substrate_platform::lsp::messages::ALL,
        #[cfg(feature = "project")]
        substrate_platform::project::messages::ALL,
        substrate_platform::process::ALL,
        #[cfg(feature = "search")]
        substrate_platform::search::messages::ALL,
        #[cfg(feature = "state")]
        substrate_platform::state::messages::ALL,
        #[cfg(feature = "terminal")]
        substrate_platform::terminal::messages::ALL,
        substrate_platform::workspace::ALL,
    ];
    groups.concat()
}
