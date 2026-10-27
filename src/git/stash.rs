//! Stashes: listing, stashing, applying, popping, dropping.

use super::run::{git, require_repo, run, write_lock};
use crate::message::Message;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StashEntry {
    pub index: u32,
    pub message: String,
    pub date: i64,
}

fn parse_stashes(raw: &str) -> Vec<StashEntry> {
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

pub fn list(root: &Path) -> Result<Vec<StashEntry>, Message> {
    let top = require_repo(root)?;
    Ok(parse_stashes(&run(git(&top).args([
        "stash",
        "list",
        "--format=%gd%x1f%gs%x1f%at",
    ]))?))
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum StashMode {
    /// Stash All — untracked files included.
    All,
    /// Stash All and Keep Staged.
    KeepIndex,
    /// Stash Staged.
    Staged,
}

pub fn push(root: &Path, message: &str, mode: StashMode) -> Result<(), Message> {
    let top = require_repo(root)?;
    let _lock = write_lock();
    let mut command = git(&top);
    command.args(["stash", "push", "--quiet"]);
    match mode {
        StashMode::All => command.arg("--include-untracked"),
        StashMode::KeepIndex => command.args(["--include-untracked", "--keep-index"]),
        StashMode::Staged => command.arg("--staged"),
    };
    if !message.trim().is_empty() {
        command.args(["--message", message.trim()]);
    }
    run(&mut command).map(drop)
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum StashOperation {
    Apply,
    Pop,
    Drop,
}

/// Applies, pops or drops stash `index`.
pub fn perform(root: &Path, index: u32, operation: StashOperation) -> Result<(), Message> {
    let top = require_repo(root)?;
    let _lock = write_lock();
    let verb = match operation {
        StashOperation::Apply => "apply",
        StashOperation::Pop => "pop",
        StashOperation::Drop => "drop",
    };
    run(git(&top).args(["stash", verb, "--quiet", &format!("stash@{{{index}}}")])).map(drop)
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
