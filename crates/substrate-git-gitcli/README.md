# substrate-git-gitcli

Source control through the `git` executable: the adapter of
`substrate-git-app`'s ports, and the anticorruption layer for git's porcelain
output.

**Layer and context:** infra · source-control

- For: `GitCli`, which implements every git port by running `git` with the
  user's own configuration, and the parsers that turn `status --porcelain=v2`,
  `log`, `for-each-ref` and `stash list` into domain types.
- Not for: deciding what may run at the same time; the app crate's
  `WriteLock` does.
- Not for: a git library; the executable is the point (hooks, credential
  helpers and `pull.rebase` apply as the user set them).

## Usage

```rust
use std::path::Path;

use substrate_git_app::ports::WorkingTree;
use substrate_git_gitcli::GitCli;
use substrate_workspace_domain::TextViews;

let git = GitCli::new(TextViews::default());
// `None` when the folder isn't in a repository (or git isn't installed).
let _status = git.status(Path::new("."));
```

## Depends on / used by

Depends on [`substrate-git-app`](../substrate-git-app/README.md),
[`substrate-git-domain`](../substrate-git-domain/README.md),
[`substrate-process`](../substrate-process/README.md) (the clean-environment
`command`) and [`substrate-workspace-domain`](../substrate-workspace-domain/README.md)
(the text views). Used by [`tauri-plugin-substrate`](../../tauri-plugin/README.md).

## License

AGPL-3.0-only; see [LICENSE](../../LICENSE).
