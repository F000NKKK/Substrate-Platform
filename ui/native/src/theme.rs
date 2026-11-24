//! Native adapter for the canonical Substrate design tokens.

use egui::{Color32, Context, FontFamily, FontId, TextStyle, Theme, Visuals, vec2};

const CSS: &str = include_str!("../../src/infra/tokens/tokens.css");

fn token(name: &str) -> Option<&'static str> {
    CSS.lines().find_map(|line| {
        let (key, value) = line.trim().split_once(':')?;
        (key == name).then(|| value.trim().trim_end_matches(';'))
    })
}

fn color(name: &str) -> Option<Color32> {
    Color32::from_hex(token(name)?).ok()
}

fn number(name: &str, unit: &str) -> Option<f32> {
    token(name)?.strip_suffix(unit)?.parse().ok()
}

fn accent() -> Option<Color32> {
    let h = number("--sp-accent-h", "")? / 360.0;
    let s = number("--sp-accent-s", "%")? / 100.0;
    let l = number("--sp-accent-l", "%")? / 100.0;
    let v = l + s * l.min(1.0 - l);
    let saturation = if v == 0.0 { 0.0 } else { 2.0 * (1.0 - l / v) };
    Some(
        egui::ecolor::HsvaGamma {
            h,
            s: saturation,
            v,
            a: 1.0,
        }
        .into(),
    )
}

/// Parsed design tokens. The CSS file is the single source of truth for both renderers.
#[derive(Debug, Clone)]
pub struct PlatformTheme {
    /// Window background.
    pub surface: Color32,
    /// Panel background.
    pub panel: Color32,
    /// Control background.
    pub control: Color32,
    /// Hover background.
    pub hover: Color32,
    /// Primary text.
    pub text: Color32,
    /// Secondary text.
    pub muted: Color32,
    /// Accent color.
    pub accent: Color32,
    /// Text on the accent.
    pub accent_text: Color32,
    /// Warning severity.
    pub warning: Color32,
    /// Error severity.
    pub danger: Color32,
    /// Standard font size.
    pub font_size: f32,
    /// Secondary font size.
    pub small_font_size: f32,
    /// Standard control height.
    pub control_height: f32,
    /// Standard horizontal control padding.
    pub control_padding: f32,
    /// Standard layout gap.
    pub gap: f32,
}

impl PlatformTheme {
    /// Read the canonical default dark theme. Returns none if a required token is missing.
    #[must_use]
    pub fn load() -> Option<Self> {
        Some(Self {
            surface: color("--sp-surface-0")?,
            panel: color("--sp-surface-1")?,
            control: color("--sp-surface-2")?,
            hover: color("--sp-surface-3")?,
            text: color("--sp-text")?,
            muted: color("--sp-text-muted")?,
            accent: accent()?,
            accent_text: color("--sp-accent-contrast")?,
            warning: color("--sp-warning")?,
            danger: color("--sp-danger")?,
            font_size: number("--sp-font-size", "px")?,
            small_font_size: number("--sp-font-size-sm", "px")?,
            control_height: number("--sp-control-height", "px")?,
            control_padding: number("--sp-control-padding-x", "px")?,
            gap: number("--sp-space-sm", "px")?,
        })
    }

    /// Configure the native renderer with platform colors and typography.
    pub fn install(&self, ctx: &Context) {
        let mut visuals = Visuals::dark();
        visuals.panel_fill = self.panel;
        visuals.window_fill = self.surface;
        visuals.extreme_bg_color = self.surface;
        visuals.override_text_color = Some(self.text);
        visuals.weak_text_color = Some(self.muted);
        visuals.warn_fg_color = self.warning;
        visuals.error_fg_color = self.danger;
        visuals.hyperlink_color = self.accent;
        visuals.selection.bg_fill = self.accent;
        visuals.selection.stroke.color = self.accent_text;
        visuals.widgets.inactive.bg_fill = self.control;
        visuals.widgets.inactive.weak_bg_fill = self.control;
        visuals.widgets.hovered.bg_fill = self.hover;
        visuals.widgets.hovered.weak_bg_fill = self.hover;
        visuals.widgets.active.bg_fill = self.hover;
        visuals.widgets.active.weak_bg_fill = self.hover;
        ctx.set_theme(Theme::Dark);
        ctx.set_visuals_of(Theme::Dark, visuals);
        ctx.style_mut_of(Theme::Dark, |style| {
            style.spacing.item_spacing = vec2(self.gap, self.gap);
            style.spacing.interact_size.y = self.control_height;
            style.spacing.button_padding.x = self.control_padding;
            for kind in [TextStyle::Body, TextStyle::Button] {
                if let Some(font) = style.text_styles.get_mut(&kind) {
                    *font = FontId::new(self.font_size, FontFamily::Proportional);
                }
            }
            if let Some(font) = style.text_styles.get_mut(&TextStyle::Small) {
                *font = FontId::new(self.small_font_size, FontFamily::Proportional);
            }
        });
        install_system_fonts(ctx);
    }
}

fn install_system_fonts(ctx: &Context) {
    // A portable app can use installed Windows fonts without shipping them.
    #[cfg(windows)]
    if let Some(root) = std::env::var_os("SystemRoot") {
        let path = std::path::PathBuf::from(root).join("Fonts/segoeui.ttf");
        if let Ok(font) = std::fs::read(path) {
            let mut definitions = egui::FontDefinitions::default();
            definitions
                .font_data
                .insert("Segoe UI".into(), egui::FontData::from_owned(font).into());
            if let Some(proportional) = definitions.families.get_mut(&FontFamily::Proportional) {
                proportional.insert(0, "Segoe UI".into());
            }
            ctx.set_fonts(definitions);
        }
    }
    #[cfg(not(windows))]
    let _ = ctx;
}

#[cfg(test)]
mod tests {
    use super::PlatformTheme;

    #[test]
    fn canonical_tokens_are_complete() {
        let theme = PlatformTheme::load();
        assert!(theme.is_some());
        if let Some(theme) = theme {
            assert!(theme.control_height > theme.font_size);
            assert_ne!(theme.accent, theme.surface);
        }
    }
}
