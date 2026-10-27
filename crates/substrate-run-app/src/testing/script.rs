//! What a scripted program does.

/// What a scripted program prints and how it ends.
#[derive(Clone, Default)]
pub(crate) struct Script {
    pub(crate) stdout: &'static str,
    pub(crate) stderr: &'static str,
    /// Its exit code; `None` runs until killed.
    pub(crate) exit: Option<i32>,
}
