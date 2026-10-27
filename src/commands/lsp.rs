//! Language servers — feature `lsp`.

use super::{Domain, SUBSTRATE_PLUGIN};

pub const DOMAIN: Domain = Domain {
    key: "lsp",
    plugin: SUBSTRATE_PLUGIN,
    commands: &[
        "lsp_languages",
        "lsp_start",
        "lsp_request",
        "lsp_notify",
        "lsp_did_open",
        "lsp_did_change",
        "lsp_did_save",
        "lsp_did_close",
        "lsp_stop",
    ],
};
