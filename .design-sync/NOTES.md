# design-sync notes — substrate-platform-ui

Repo-specific gotchas for future syncs. One bullet per gotcha.

## Build / environment
- **No build step, source-only package.** `main`/`types` point at `ui/src/index.tsx`. The converter bundles directly from TS source: `--entry ./ui/src/index.tsx`, `--node-modules ui/node_modules`.
- **React is a peerDependency and is NOT installed by `npm ci`.** The converter needs a real React to vendor for the preview runtime (`_vendor/`), so it must be present under `--node-modules`. Fix per re-sync: `cd ui && npm i --no-save react@^19 react-dom@^19` before building. `--no-save` keeps the lockfile/package.json clean.
- Prod deps `@tauri-apps/api`, `@xterm/xterm`, `@xterm/addon-fit` are in `ui/node_modules` (installed by `npm ci`).
- **`@types/react` must also be under `ui/node_modules`** or prop extraction resolves React utility types (`ComponentPropsWithoutRef`, HTML attribute bags) to `any` and emits empty `.d.ts` bodies (`[DTS_REACT]`). Fix: `cd ui && npm i --no-save -D @types/react@^19 @types/react-dom@^19`.

## Theming / provider
- All theme-dependent components (`AccentColorField`, `AppearanceSettings`, anything using `useTheme`) require `ThemeProvider`. Config sets `provider: {component: "ThemeProvider"}`, which wraps every preview — harmless for the theme-agnostic widgets.
- Only a dark theme is defined in `tokens/tokens.css`. The single user-configurable dimension is the accent color (`--sp-accent-*`), owned by `ThemeProvider`.

## Components
- `TerminalPanel` is PTY-backed via Tauri (`invoke`/`listen`) — cannot render live in a static preview. Expect it to need a `skip` or a mocked/placeholder preview.
- Overlay/window components render absolutely-positioned open state: `ColorPickerWindow`, `SettingsWindow`, `FloatingPanel` — likely need `cfg.overrides.<Name>: {cardMode: "single", viewport: "WxH"}`.
- Shell internals (`ToolWindowDock`, `CenterDock`, `DockStrip`, `PanelSurface`, `FloatingPanel`) are driven by `PlatformShell`/`useShellLayout`; preview them via a composed `PlatformShell` where practical.

## Fonts
- `--sp-font-ui: "Segoe UI", Inter, Avenir, Helvetica, Arial, sans-serif` and `--sp-font-mono: "Cascadia Code", Consolas, monospace` are deliberate system-font stacks with full fallbacks — NOT custom brand webfonts. `[FONT_MISSING]` for Inter/Avenir/Cascadia Code is a false positive; suppressed via `cfg.runtimeFontPrefixes`. Do not try to ship these.

## Preview authoring conventions (this DS)
- **Dark theme only.** Preview cards render on a white product background, but every component is designed for a dark IDE surface (`--sp-surface-*`, light text `--sp-text`). Authored previews MUST wrap content in a dark surface panel using DS tokens, or light/muted components render invisible.
- Icons render at 16px in the card corner by default → author a labeled size row (16/24/32) with `color: var(--sp-text)` on a dark surface.

## Known render warns (triaged legitimate)
- (fill in as authored: e.g. single-look components, genuinely-small ones)

## Re-sync risks
- (to be filled in before finishing)
