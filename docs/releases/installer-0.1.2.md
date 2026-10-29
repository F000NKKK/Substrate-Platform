# Substrate Installer 0.1.2

This patch synchronizes the seven `substrate-installer-*` crate versions to 0.1.2 and
ships the tested Substrate UI dock fixes in the matching source release.

- A failed dock tab no longer prevents adjacent tabs from opening.
- Clicking the selected dock tab keeps its pane open.
- Tab selection hides other pinned tabs in the same interaction.
- Live resizing coalesces pointer events to animation frames, preserves the final size,
  and removes listeners and pending callbacks when a handle unmounts.

The UI package remains a source dependency (`private: true`); a crates.io update alone does
not replace a host application's bundled UI. Consumers must also build against this source
release. Rust installer behavior is unchanged from the previously published adapters;
this release gives consumers one consistent installer package version without Git patches.

Validation: the UI typecheck and full unit suite, installer crate tests and clippy, package
verification during crates.io publishing, and Meridian Installer's browser workflow suite.
