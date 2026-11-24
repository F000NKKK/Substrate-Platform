//! Brand-neutral native components mirroring the platform widget contracts.

use egui::{Button, ProgressBar, RichText, ScrollArea, Ui, vec2};

use crate::PlatformTheme;

/// Semantic button variants shared with the web UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    /// Main action.
    Primary,
    /// Secondary action.
    Subtle,
    /// Quiet action.
    Ghost,
}

/// Construct a themed native button using the platform control metrics.
#[must_use]
pub fn button<'a>(theme: &PlatformTheme, text: &'a str, variant: ButtonVariant) -> Button<'a> {
    let (fill, foreground) = match variant {
        ButtonVariant::Primary => (theme.accent, theme.accent_text),
        ButtonVariant::Subtle => (theme.control, theme.text),
        ButtonVariant::Ghost => (egui::Color32::TRANSPARENT, theme.muted),
    };
    Button::new(RichText::new(text).color(foreground))
        .fill(fill)
        .min_size(vec2(0.0, theme.control_height))
}

/// Secondary text using the platform muted text token.
#[must_use]
pub fn caption(theme: &PlatformTheme, text: impl Into<String>) -> RichText {
    RichText::new(text.into())
        .size(theme.small_font_size)
        .color(theme.muted)
}

/// Determinate or indeterminate progress with platform accent.
#[must_use]
pub fn progress(theme: &PlatformTheme, fraction: Option<f32>, active: bool) -> ProgressBar {
    let bar = ProgressBar::new(fraction.unwrap_or(0.0))
        .fill(theme.accent)
        .animate(active && fraction.is_none());
    if fraction.is_some() {
        bar.show_percentage()
    } else {
        bar
    }
}

/// Display an operation log in a scrollable output panel.
pub fn output_panel(ui: &mut Ui, lines: &[String]) {
    ScrollArea::vertical()
        .stick_to_bottom(true)
        .show(ui, |ui| {
            for line in lines {
                ui.label(RichText::new(line).monospace());
            }
        });
}

/// Generic inline confirmation with explicit accept and decline actions.
pub fn confirmation(
    ui: &mut Ui,
    theme: &PlatformTheme,
    message: &str,
    subject: &str,
    accept: &str,
    decline: &str,
) -> Option<bool> {
    ui.separator();
    ui.label(RichText::new(message).color(theme.warning));
    ui.label(RichText::new(subject).strong());
    ui.horizontal(|ui| {
        if ui.add(button(theme, accept, ButtonVariant::Primary)).clicked() {
            Some(true)
        } else if ui.add(button(theme, decline, ButtonVariant::Subtle)).clicked() {
            Some(false)
        } else {
            None
        }
    })
    .inner
}
