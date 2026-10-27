//! Source Control — feature `git`.

use super::{Domain, SUBSTRATE_PLUGIN};

pub const DOMAIN: Domain = Domain {
    key: "git",
    plugin: SUBSTRATE_PLUGIN,
    commands: &[
        "git_status",
        "git_stage",
        "git_unstage",
        "git_discard",
        "git_commit",
        "git_last_message",
        "git_diff_texts",
        "git_head_text",
        "git_log",
        "git_commit_files",
        "git_branches",
        "git_switch",
        "git_branch_create",
        "git_branch_delete",
        "git_remote",
        "git_integrate",
        "git_operation",
        "git_clone",
        "git_init",
        "git_stash_list",
        "git_stash_push",
        "git_stash_op",
        "git_config_get",
        "git_config_set",
    ],
};
