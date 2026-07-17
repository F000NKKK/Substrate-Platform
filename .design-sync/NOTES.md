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

## 2026-07-16 re-sync — major restructure

- **Component `group` in the design pane now tracks the real `ui/src` folder** (`widgets`, `windows`, `pages`, `shell`, `dock`, `infra`, `icons`, `glyphs`, `theme`) automatically — no config needed. The old groups (`general`, `menu`, `panels`, `settings`, plus the old flat `shell/*`) were deleted this sync; if `ui/src` gets reorganized again, the group names just follow, and stale old-group paths show up in the next `.sync-diff.json` `upload.deletePaths`.
- **Fixed a fatal bug**: `WindowControls.tsx` called `getCurrentWindow()` from `@tauri-apps/api/window` at **module scope** (`const appWindow = getCurrentWindow();` outside the component). That reaches for `window.__TAURI_INTERNALS__.metadata`, which only exists inside a real Tauri webview — so importing the package AT ALL outside Tauri (this sync's preview capture, a bare `vite dev`, Storybook) threw immediately and broke every single component, not just `WindowControls`. Fixed by resolving the window handle lazily (`useMemo`) and only when `"__TAURI_INTERNALS__" in window`, no-oping the handlers otherwise. **Watch for this class of bug again**: any future Tauri API call must be inside a component/hook, never at module top level, or it re-breaks the whole bundle.
- New components synced: `DataGrid` (multi-sort, drag-to-group, filter row, resize/reorder, hide, inline edit), `Tree` + `FileTree` (generic nested list + file-explorer config layer), `ContextMenu`/`useContextMenu` (shared dropdown backing Tree/MenuBarItem/DataGrid), `Icon`/`IconProvider`/`registerIcon` (by-name icon resolution with override), `WindowControls`, `Outline` (accent-outline renderer), plus new glyphs (`IconChevronRight`, `IconFile`, `IconFolderOpen`, `IconMaximize`, `IconMinimize`, `IconPlus`, `IconRestore`).
- `conventions.md` extended (not rewritten) with sections for all of the above — the original content still verified against the fresh build, so nothing was replaced, only appended.
- `FileTree`'s `WithContextMenu` preview shows the closed state only (dropdown opens on right-click, can't render open statically) — same limitation as `MenuBarItem`, not a bug.

## Re-sync risks
- **React + @types/react are `--no-save` installs into `ui/node_modules`** (not in package.json). A fresh clone / clean `npm ci` will drop them → build fails with "react not found" / `[DTS_REACT]`. Re-run: `cd ui && npm i --no-save react@^19 react-dom@^19 @types/react@^19 @types/react-dom@^19`.
- **No dist / no build script**: the converter bundles TS source directly (`--entry ./ui/src/index.tsx`). If the package later gains a real build, switch `--entry` to the built output.
- **Previews reference two non-existent tokens** (`--sp-text-success`, `--sp-text-accent`) in a couple of shell/panel code snippets (leftover from Haiku authoring). They fall back harmlessly (render in default text color, not green). Cosmetic only; fix if a green "success" line is wanted.
- **TerminalPanel is excluded** (`componentSrcMap: {TerminalPanel: null}`) — it needs Tauri (`invoke`) and can't render statically. Still importable from the bundle; just has no card. Revisit if a mock is wanted.
- **Overlay cards graded from the `.html` card, not the per-story capture** (see Known render warns). If a future capture frame changes, re-check the shipped card.
- Grades live in `.design-sync/.cache/review/` (gitignored) — cross-machine carry-forward comes from the uploaded `_ds_sync.json`. A fresh clone re-verifies from the project anchor, which is expected.
