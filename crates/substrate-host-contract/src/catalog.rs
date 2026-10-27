//! Every message template the core declares, for the generated copy.

/// Every template Substrate declares, for the export.
pub fn catalog() -> Vec<(&'static str, &'static str, &'static str)> {
    let groups: &[&[(&str, &str, &str)]] = &[
        #[cfg(feature = "dap")]
        substrate_dap_domain::messages::ALL,
        #[cfg(feature = "fs")]
        substrate_workspace_fs::messages::ALL,
        #[cfg(feature = "git")]
        substrate_git_domain::messages::ALL,
        #[cfg(feature = "lsp")]
        substrate_lsp_domain::messages::ALL,
        #[cfg(feature = "project")]
        substrate_platform::project::messages::ALL,
        substrate_process::ALL,
        #[cfg(feature = "search")]
        substrate_search_domain::messages::ALL,
        #[cfg(feature = "state")]
        substrate_platform::state::messages::ALL,
        #[cfg(feature = "terminal")]
        substrate_terminal_domain::messages::ALL,
        substrate_workspace_domain::messages::ALL,
    ];
    groups.concat()
}
