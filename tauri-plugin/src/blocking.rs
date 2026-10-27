//! Running blocking work off the UI thread.

#[cfg(any(feature = "git", feature = "lsp", feature = "search"))]
use substrate_platform::message::Message;

/// Runs `work` off the UI thread.
///
/// # Errors
///
/// What `work` returns, or the failure of its thread.
#[cfg(any(feature = "git", feature = "lsp", feature = "search"))]
pub(crate) async fn blocking<T: Send + 'static>(
    work: impl FnOnce() -> Result<T, Message> + Send + 'static,
) -> Result<T, Message> {
    tauri::async_runtime::spawn_blocking(work)
        .await
        .map_err(Message::verbatim)?
}
