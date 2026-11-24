//! Native Substrate UI: shared design tokens and reusable native controls.
//!
//! React widgets remain the web implementation; this adapter keeps the same
//! design language in applications that cannot depend on a WebView.

mod theme;
mod widgets;

pub use theme::PlatformTheme;
pub use widgets::{ButtonVariant, button, caption, confirmation, output_panel, progress};
