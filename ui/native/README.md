# Substrate native UI

An egui rendering adapter of Substrate's design system. The canonical design
tokens are read directly from `ui/src/infra/tokens/tokens.css`; applications do
not copy the palette or control metrics. The native counterparts of Button,
Label, ProgressBar and OutputPanel are exposed by this crate. React widgets
remain the web implementation. Native and web renderers share design tokens,
not a JavaScript runtime.

Use `PlatformTheme::load()`, install it into the egui context, and compose
the platform widgets. Keep product-specific workflow and copy in the consumer.
Add reusable native components here, and extend the canonical tokens rather
than introducing application-specific palettes. This first adapter supports the
default dark appearance; custom accent/theme preferences and visual parity
tests are future platform work.
