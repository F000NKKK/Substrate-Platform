//! A long run's lines, to the UI.

use substrate_platform::events;
use substrate_platform::message::Message;
use tauri::{AppHandle, Emitter, Runtime};

/// A long run's lines, to the UI.
pub(super) fn output<R: Runtime>(app: &AppHandle<R>) -> impl Fn(Message) + Sync + '_ {
    move |line| {
        // The UI window is gone: nobody to tell.
        let _ = app.emit(events::GIT_OUTPUT, line);
    }
}
