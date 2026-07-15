# Substrate Platform

A brand-neutral IDE shell core — the shared foundation for building
desktop, IDE-style tools (dockable panels, a product plugs in its own
panels via a small trait). It has no knowledge of any specific product;
it's the same relationship as JetBrains' IntelliJ Platform to IntelliJ
IDEA/PyCharm/WebStorm/Rider — one core, many products built on top.

## What's here

- [`Shell`] — an `eframe::App` wrapping an `egui_dock` layout.
- [`Panel`] — the trait a product implements per dockable panel kind
  (tree view, property inspector, canvas, ...).
- [`run`] — launches a native window running a `Shell`.

## Status

Early scaffold. See `examples/hello_shell.rs` for the smallest possible
usage — an empty dockable window with one panel.

The first product built on this platform is
[Yog-IDLE](https://github.com/F000NKKK/Yog-IDLE), a visual UI editor for
Yog Minecraft mods with a live preview into a running game client.

## License

AGPL-3.0-only — see [LICENSE](LICENSE).
