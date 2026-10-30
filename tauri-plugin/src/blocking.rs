//! Running blocking work off the UI thread.

#[cfg(any(
    feature = "dap",
    feature = "git",
    feature = "lsp",
    feature = "search",
    feature = "terminal"
))]
use substrate_message::Message;

/// Runs `work` off the UI thread.
///
/// # Errors
///
/// What `work` returns, or the failure of its thread.
#[cfg(any(
    feature = "dap",
    feature = "git",
    feature = "lsp",
    feature = "search",
    feature = "terminal"
))]
pub(crate) async fn blocking<T: Send + 'static, E: Into<Message> + Send + 'static>(
    work: impl FnOnce() -> Result<T, E> + Send + 'static,
) -> Result<T, Message> {
    tauri::async_runtime::spawn_blocking(work)
        .await
        .map_err(Message::verbatim)?
        .map_err(Into::into)
}
