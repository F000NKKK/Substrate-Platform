//! The plugin itself.

use tauri::plugin::{Builder, TauriPlugin};
use tauri::{AppHandle, Manager, Runtime};

#[cfg(feature = "dap")]
use super::dap;
#[cfg(feature = "fs")]
use super::fs;
#[cfg(feature = "lsp")]
use super::lsp;
#[cfg(feature = "search")]
use super::search;
#[cfg(feature = "terminal")]
use super::terminal;
use super::{NAME, Options, Workspace, handlers};

/// The plugin with no product options.
#[must_use]
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    init_with(Options::default())
}

/// The plugin, with the commands of each feature turned on (Substrate's
/// domains, `substrate_host_contract::commands`, list the same ones).
#[must_use]
pub fn init_with<R: Runtime>(options: Options) -> TauriPlugin<R> {
    Builder::new(NAME)
        .setup(move |app, _api| {
            manage(app, options);
            Ok(())
        })
        .invoke_handler(handlers::commands())
        .on_drop(|app| stop_servers(&app))
        .build()
}

/// Puts the state each feature's commands share under the app's management.
fn manage<R: Runtime>(app: &AppHandle<R>, options: Options) {
    app.manage(Workspace::default());
    app.manage(options.text_views);
    #[cfg(feature = "git")]
    app.manage(substrate_platform::git::WriteLock::new());
    #[cfg(feature = "terminal")]
    app.manage(terminal::Terminals::default());
    #[cfg(feature = "fs")]
    app.manage(fs::Watcher::default());
    #[cfg(feature = "search")]
    {
        app.manage(search::Searches::default());
        app.manage(options.search);
    }
    #[cfg(feature = "state")]
    if let Some(state) = options.state {
        app.manage(state);
    }
    #[cfg(feature = "lsp")]
    {
        app.manage(lsp::Servers::default());
        app.manage(options.languages);
    }
    #[cfg(feature = "dap")]
    {
        app.manage(dap::Sessions::default());
        app.manage(options.adapters);
    }
}

/// Language servers and debug sessions don't outlive the app.
fn stop_servers<R: Runtime>(app: &AppHandle<R>) {
    #[cfg(feature = "dap")]
    if let Some(sessions) = app.try_state::<dap::Sessions>() {
        sessions.stop_all();
    }
    #[cfg(feature = "lsp")]
    if let Some(servers) = app.try_state::<lsp::Servers>() {
        servers.stop_all();
    }
    #[cfg(not(any(feature = "dap", feature = "lsp")))]
    let _ = app;
}
