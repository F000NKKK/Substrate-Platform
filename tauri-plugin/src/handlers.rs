//! The plugin's command list.

#[cfg(feature = "dap")]
use super::dap;
#[cfg(feature = "fs")]
use super::fs;
#[cfg(feature = "git")]
use super::git;
#[cfg(feature = "lsp")]
use super::lsp;
use super::open;
#[cfg(feature = "search")]
use super::search;
#[cfg(feature = "state")]
use super::state;
#[cfg(feature = "terminal")]
use super::terminal;

/// Every command of the features turned on (Substrate's domains,
/// `substrate_host_contract::commands`, list the same ones).
#[expect(
    clippy::too_many_lines,
    reason = "one command list: tauri's `generate_handler!` can't be split or composed"
)]
pub(crate) fn commands<R: tauri::Runtime>()
-> impl Fn(tauri::ipc::Invoke<R>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        open::open_url,
        open::open_path,
        open::reveal_path,
        #[cfg(feature = "state")]
        state::workspace_load,
        #[cfg(feature = "state")]
        state::workspace_save,
        #[cfg(feature = "state")]
        state::app_state_load,
        #[cfg(feature = "state")]
        state::app_state_save,
        #[cfg(feature = "search")]
        search::commands::find_in_files,
        #[cfg(feature = "search")]
        search::commands::find_cancel,
        #[cfg(feature = "search")]
        search::commands::replace_in_files,
        #[cfg(feature = "search")]
        search::commands::project_items,
        #[cfg(feature = "search")]
        search::commands::project_files_containing,
        #[cfg(feature = "fs")]
        fs::commands::dir_list,
        #[cfg(feature = "fs")]
        fs::commands::dir_create_file,
        #[cfg(feature = "fs")]
        fs::commands::dir_create_dir,
        #[cfg(feature = "fs")]
        fs::commands::dir_rename,
        #[cfg(feature = "fs")]
        fs::commands::dir_remove,
        #[cfg(feature = "fs")]
        fs::commands::dir_copy,
        #[cfg(feature = "fs")]
        fs::commands::file_read,
        #[cfg(feature = "fs")]
        fs::commands::file_write,
        #[cfg(feature = "fs")]
        fs::commands::allow_path,
        #[cfg(feature = "fs")]
        fs::commands::fs_watch,
        #[cfg(feature = "terminal")]
        terminal::commands::terminal_shells,
        #[cfg(feature = "terminal")]
        terminal::commands::terminal_spawn,
        #[cfg(feature = "terminal")]
        terminal::commands::terminal_write,
        #[cfg(feature = "terminal")]
        terminal::commands::terminal_resize,
        #[cfg(feature = "terminal")]
        terminal::commands::terminal_kill,
        #[cfg(feature = "lsp")]
        lsp::commands::lsp_languages,
        #[cfg(feature = "lsp")]
        lsp::commands::lsp_start,
        #[cfg(feature = "lsp")]
        lsp::commands::lsp_request,
        #[cfg(feature = "lsp")]
        lsp::commands::lsp_notify,
        #[cfg(feature = "lsp")]
        lsp::commands::lsp_did_open,
        #[cfg(feature = "lsp")]
        lsp::commands::lsp_did_change,
        #[cfg(feature = "lsp")]
        lsp::commands::lsp_did_save,
        #[cfg(feature = "lsp")]
        lsp::commands::lsp_did_close,
        #[cfg(feature = "lsp")]
        lsp::commands::lsp_stop,
        #[cfg(feature = "dap")]
        dap::commands::dap_adapters,
        #[cfg(feature = "dap")]
        dap::commands::dap_start,
        #[cfg(feature = "dap")]
        dap::commands::dap_request,
        #[cfg(feature = "dap")]
        dap::commands::dap_stop,
        #[cfg(feature = "git")]
        git::changes::git_status,
        #[cfg(feature = "git")]
        git::changes::git_stage,
        #[cfg(feature = "git")]
        git::changes::git_unstage,
        #[cfg(feature = "git")]
        git::changes::git_discard,
        #[cfg(feature = "git")]
        git::changes::git_commit,
        #[cfg(feature = "git")]
        git::changes::git_last_message,
        #[cfg(feature = "git")]
        git::changes::git_diff_texts,
        #[cfg(feature = "git")]
        git::changes::git_head_text,
        #[cfg(feature = "git")]
        git::history::git_log,
        #[cfg(feature = "git")]
        git::history::git_commit_files,
        #[cfg(feature = "git")]
        git::history::git_branches,
        #[cfg(feature = "git")]
        git::history::git_switch,
        #[cfg(feature = "git")]
        git::history::git_branch_create,
        #[cfg(feature = "git")]
        git::history::git_branch_delete,
        #[cfg(feature = "git")]
        git::remote::git_remote,
        #[cfg(feature = "git")]
        git::remote::git_integrate,
        #[cfg(feature = "git")]
        git::remote::git_operation,
        #[cfg(feature = "git")]
        git::remote::git_clone,
        #[cfg(feature = "git")]
        git::remote::git_init,
        #[cfg(feature = "git")]
        git::stash::git_stash_list,
        #[cfg(feature = "git")]
        git::stash::git_stash_push,
        #[cfg(feature = "git")]
        git::stash::git_stash_op,
        #[cfg(feature = "git")]
        git::stash::git_config_get,
        #[cfg(feature = "git")]
        git::stash::git_config_set,
    ]
}
