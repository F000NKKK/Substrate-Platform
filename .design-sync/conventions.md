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

For a full application frame, use `PlatformShell` (props `main`, `toolWindows: { left | right | bottom: PanelDef[] }`, `menu`) — a docking IDE shell with tabbed center dock, pinnable/floatable tool windows, and an optional `MenuBar`. Each `PanelDef` is `{ id, title, component }` where `component` is a zero-prop React component rendering the panel body. `MenuBar` also takes a `windowControls` slot (put `<WindowControls />` there for a `decorations: false` Tauri window) and doubles as the window's drag region.

## Data & hierarchy: DataGrid, Tree, FileTree

`DataGrid` (`columns: DataGridColumn<T>[]`, `rows: T[]`, `getRowId`) is a full desktop-grade grid: multi-column sort (`sortable`, ctrl/cmd-click a header for a secondary key), drag-a-header-into-the-group-panel grouping with per-group aggregates (`summary: "sum"|"count"|"avg"` + `summaryValue`), a per-column filter row (default on — `filterRow={false}` to hide), column resize/reorder, column hide (`hiddenColumns`/`onHiddenColumnsChange`), and inline cell editing (`editable`, `onCellEdit`/`onCellEditCommit`). Rows above ~50 should always go through this, not a mapped `<div>` list — it's virtualized.

`Tree` (`nodes: TreeNode<T>[]` where a node is `{id, label, data, children?}` — `children` present, even `[]`, marks a branch) is the generic nested-list primitive: animated expand/collapse, roving-tabindex keyboard nav, optional `renderIcon`, and a `getMenuItems` right-click dropdown. `FileTree` is `Tree` pre-configured for files (`nodes: FileTreeNode[]` with `kind: "folder"|"file"`) — folder/open-folder/file icons for free.

## ContextMenu / useContextMenu

The one dropdown implementation, backing `Tree`'s node menu, `MenuBarItem`'s dropdown, and `DataGrid`'s header/cell menus. Building a new right-click or click-to-open menu: pair `useContextMenu<T>()` (gives `target`, `openAtPoint(node, e)` for right-click, `openAtAnchor(node)` for click-to-open, `close`) with `<ContextMenu target={...} items={...} onClose={...} />`. `ContextMenuItem` is `{label, onSelect?, destructive?, disabled?, checked?, submenu?}`.

## Icon-by-name and IconProvider

Beyond importing `Icon*` components directly, `<Icon name="folder" size={16} />` resolves a glyph from a registry by string name (built-ins: `folder`, `folderOpen`, `file`, `chevronRight`, `pin`, `pinOff`, `close`, `settings`, `minimize`, `maximize`, `restore`, `plus`, and more — see `getDefaultIconNames()`). Override globally with `registerIcon(name, Component)`/`registerIcons({...})`, or scope an override to a subtree with `<IconProvider icons={{ folder: MyIcon }}>` — this is how a product swaps or adds icons without forking the package.
