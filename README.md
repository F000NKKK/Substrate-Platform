# Substrate Platform

A brand-neutral IDE backend core — the shared Rust foundation for building
desktop, IDE-style tools. It has no knowledge of any specific product and
no UI framework opinion (products bring their own UI — Yog-IDLE's is a
Tauri + React frontend); the same relationship as JetBrains' IntelliJ
Platform to IntelliJ IDEA/PyCharm/WebStorm/Rider — one core, many products
built on top.

## What's here

- [`task`] — spawn an external process (a build tool, a compiler, ...)
  non-blocking, with stdout/stderr streamed line-by-line to a [`log::LogSink`].
- [`log`] — a generic append-only log sink; a product drains it however it
  ships lines to its own UI.
- [`pty`] — a real, cross-platform PTY session (Windows/Linux/macOS) via
  `portable-pty`, for an integrated terminal panel.

## Status

Early scaffold: the three backend pieces above exist and are wired up end-
to-end in [Yog-IDLE](https://github.com/F000NKKK/Yog-IDLE) (its first
product) via Tauri commands — a real terminal backed by [`pty::PtySession`]
streams to an `xterm.js` frontend.

## License

Dual-licensed: **AGPL-3.0-only** (see [LICENSE](LICENSE)) — free for any
use, provided you comply with the AGPL — or a **Commercial License** for
closed-source use, free below $1,000/mo net profit and a sliding royalty
above that. See [COMMERCIAL-LICENSE.md](COMMERCIAL-LICENSE.md) for details.
