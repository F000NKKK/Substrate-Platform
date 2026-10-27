# substrate-git-domain

Source control's vocabulary: repository status, changes, branches, commits,
stashes, diffs, and why a git operation fails.

**Layer and context:** domain · source-control

- For: the types an IDE's Git windows show and the typed `GitError`, with the
  words the user reads for each failure (`messages`).
- Not for: running git. The ports are in `substrate-git-app`, the `git`
  executable behind them in `substrate-git-gitcli`.
- Not for: the wire shapes the UI reads; the Tauri plugin maps these types to
  its own DTOs.

## Usage

```rust
use substrate_git_domain::remotes::clone_directory_name;

assert_eq!(
    clone_directory_name("https://github.com/owner/Some-App.git"),
    "Some-App"
);
```

## Depends on / used by

Depends on [`substrate-message`](../substrate-message/README.md). Used by
[`substrate-git-app`](../substrate-git-app/README.md),
[`substrate-git-gitcli`](../substrate-git-gitcli/README.md) and
[`tauri-plugin-substrate`](../../tauri-plugin/README.md).

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
