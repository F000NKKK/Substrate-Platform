//! `git stash list` as [`StashEntry`]s.

use substrate_git_domain::stashes::StashEntry;

/// The `--format` that [`parse_stashes`] reads.
pub const STASH_FORMAT: &str = "--format=%gd%x1f%gs%x1f%at";

/// Parses `git stash list` in [`STASH_FORMAT`].
#[must_use]
pub fn parse_stashes(raw: &str) -> Vec<StashEntry> {
    raw.lines()
        .filter_map(|line| {
            let (selector, rest) = line.split_once('\x1f')?;
            let (message, date) = rest.split_once('\x1f')?;
            let index = selector
                .strip_prefix("stash@{")?
                .strip_suffix('}')?
                .parse()
                .ok()?;
            Some(StashEntry {
                index,
                message: message.to_string(),
                date: date.parse().unwrap_or(0),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_stash_list() {
        let raw = "stash@{0}\x1fOn main: wip\x1f1700000000\nstash@{1}\x1fWIP on dev: abc fix\x1f1600000000\n";
        let stashes = parse_stashes(raw);
        assert_eq!((stashes.len(), stashes[1].index), (2, 1));
        assert_eq!(stashes[0].message, "On main: wip");
    }
}
