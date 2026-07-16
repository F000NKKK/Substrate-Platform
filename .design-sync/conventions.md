# Substrate Platform UI — how to build with this design system

A **brand-neutral, dark-themed IDE shell** toolkit: theme tokens, a docking `PlatformShell`, generic panels, and form widgets. All components import from the package root `substrate-platform-ui`. React 19.

## Setup — wrap the app in `ThemeProvider`

Wrap your tree once in `ThemeProvider`. It owns the accent color and writes `--sp-accent-h/s/l` to the document root; the theme components (`AccentColorField`, `AppearanceSettings`, anything calling `useTheme`) **throw** without it, and other components render in their static defaults.

```tsx
import { ThemeProvider } from "substrate-platform-ui";
// <ThemeProvider>…your app…</ThemeProvider>   // optional: initialAccent={{ h, s, l }}
```

**This is a dark-only theme.** Every component expects a dark surface underneath it (light/muted text is invisible on white). Give any container you build a `--sp-surface-*` background — see the snippet below.

## Styling idiom — CSS custom properties (`var(--sp-*)`)

There is **no utility-class API** and no per-component `className` styling contract. You compose the real components and, for your own layout glue, use the design tokens. Never invent color/spacing values — always reach for a token:

| Family | Tokens |
|---|---|
| Accent (theme-driven) | `--sp-accent`, `--sp-accent-soft`, `--sp-accent-strong`, `--sp-accent-contrast` (channels `--sp-accent-h/s/l` are set by ThemeProvider) |
| Surfaces | `--sp-surface-0` `--sp-surface-1` `--sp-surface-2` `--sp-surface-3`, `--sp-surface-glass` |
| Text | `--sp-text`, `--sp-text-muted`, `--sp-text-faint` |
| Borders | `--sp-border`, `--sp-border-strong` |
| Typography | `--sp-font-ui`, `--sp-font-mono`, `--sp-font-size` |
| Radius | `--sp-radius-xs` `--sp-radius-sm` `--sp-radius-md` `--sp-radius-lg` |
| Spacing | `--sp-space-xs` `--sp-space-sm` `--sp-space-md` `--sp-space-lg` |
| Elevation / motion | `--sp-shadow-sm` `--sp-shadow-md` `--sp-shadow-glow`, `--sp-blur`, `--sp-duration-fast` `--sp-duration-base`, `--sp-easing` |
| Shell metrics | `--sp-toolwindow-size`, `--sp-toolwindow-bottom-size`, `--sp-toolwindow-strip`, `--sp-menubar-height` |

Component appearance is driven by **props**, not classes: e.g. `Button` `variant="primary" | "subtle" | "ghost"`, `Tab` `orientation="horizontal" | "horizontal-dock" | "vertical-left" | "vertical-right" | "list"`, all `Icon*` `size={n}` (color via CSS `color`/`currentColor`).

## Where the truth lives

- **Tokens + all component styles**: `styles.css` (which `@import`s `_ds_bundle.css`). Read it to see every `--sp-*` value and component rule.
- **Per-component API + usage**: each component's `<Name>.d.ts` (props) and `<Name>.prompt.md` (guidance + examples). Read these before composing a component.

## Idiomatic snippet

A settings card — library components for the controls, tokens for the layout glue, on a dark surface:

```tsx
import { ThemeProvider, Label, TextField, Button } from "substrate-platform-ui";

export function ProjectSettings() {
  return (
    <ThemeProvider>
      <div style={{
        background: "var(--sp-surface-1)", color: "var(--sp-text)",
        fontFamily: "var(--sp-font-ui)", fontSize: "var(--sp-font-size)",
        padding: "var(--sp-space-lg)", borderRadius: "var(--sp-radius-md)",
        display: "flex", flexDirection: "column", gap: "var(--sp-space-md)",
      }}>
        <Label>Project name</Label>
        <TextField defaultValue="substrate-platform" />
        <div style={{ display: "flex", gap: "var(--sp-space-sm)" }}>
          <Button variant="primary">Save</Button>
          <Button variant="subtle">Cancel</Button>
        </div>
      </div>
    </ThemeProvider>
  );
}
```

For a full application frame, use `PlatformShell` (props `main`, `toolWindows: { left | right | bottom: PanelDef[] }`, `menu`) — a docking IDE shell with tabbed center dock, pinnable/floatable tool windows, and an optional `MenuBar`. Each `PanelDef` is `{ id, title, component }` where `component` is a zero-prop React component rendering the panel body.
