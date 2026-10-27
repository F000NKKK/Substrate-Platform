//! Where an expression to evaluate comes from.

/// Where an expression to `evaluate` comes from — the adapter formats the
/// result for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvaluateContext {
    /// A watch expression.
    Watch,
    /// The debug console.
    Repl,
    /// A hover over the source.
    Hover,
    /// Text to copy.
    Clipboard,
}

impl EvaluateContext {
    /// The protocol's name for it.
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Watch => "watch",
            Self::Repl => "repl",
            Self::Hover => "hover",
            Self::Clipboard => "clipboard",
        }
    }
}
