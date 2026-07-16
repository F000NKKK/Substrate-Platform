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

## Component API facts (from preview authoring)
- **Icons** (`createIcon` factory): props `IconProps` = `size?: number` (default 16) + `SVGProps<SVGSVGElement>`. Color via `currentColor` (set CSS `color`), no dedicated color prop. Default 16px is too small for a specimen — previews render 16/24/32 rows + an accent-tinted specimen.
- **HslColor scale mismatch**: `ColorPickerWindow.value` uses `s`/`l` as **0–100**; `ColorWheel` props expect `saturation`/`lightness` as **0–1**. ColorPickerWindow converts internally. Keep preview values on the right scale for each.
- **Shell**: `PanelDef = {id, title, component: ComponentType /* no props */}`. `PlatformShellProps = {main, toolWindows: Partial<Record<"left"|"right"|"bottom", PanelDef[]>>, defaultPinned?, menu?}`. `CenterDock`/`ToolWindowDock` take a `layout: ShellLayout` from the `useShellLayout(main, toolWindows, defaultPinned?)` hook — previews call the hook inside the story. `DockStrip` takes explicit `{anchor, panelIds, panelsById, activeId, onSelect, onDropPanel}`.
- **SettingsSection** = `{id, label, content: ReactNode}`. **MenuBarDropdownEntry** = `{label, onClick?}`.

## Overlay / sizing overrides (in config)
- `ColorPickerWindow`, `SettingsWindow`, `FloatingPanel`, `PlatformShell` are absolutely-positioned or full-shell → each has `cfg.overrides.<Name>: {cardMode:"single", viewport:"WxH"}` so the open/large state renders inside the card instead of clipping.

## Known render warns (triaged legitimate)
- **MenuBarItem** — `variants render identically`: the dropdown opens on click only, so `Simple` and `WithDropdown` both render the closed item. Cannot show the open dropdown statically. Benign.
- **DockStrip** / **ToolWindowDock RightDock** — genuinely narrow / collapsed-strip states render sparse. Correct, not broken.
- **ColorPickerWindow** / **SettingsWindow** — the per-story `?story=` capture frame clips the absolutely-positioned window, but the shipped `.html` card (cardMode:single + viewport) renders the full window. Grade from the card, not the tight per-story frame.
- **IconButton Default** — a single small ghost icon button reads sparse in a large card; the `Sizes`/`Toolbar` cells carry the component.

## Re-sync risks
- (to be filled in before finishing)
