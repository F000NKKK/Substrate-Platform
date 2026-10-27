//! `git for-each-ref` as [`Branch`]es.

use substrate_git_domain::branches::Branch;

/// The `--format` that [`parse_branches`] reads.
pub const BRANCH_FORMAT: &str = "--format=%(refname)%1f%(refname:short)%1f%(objectname:short)%1f%(upstream:short)%1f%(upstream:track,nobracket)%1f%(HEAD)%1f%(committerdate:unix)%1f%(contents:subject)";
const REMOTES_PREFIX: &str = "refs/remotes/";

/// `ahead 1, behind 2` / `gone` / empty → (ahead, behind, gone).
fn parse_track(track: &str) -> (u32, u32, bool) {
    let mut result = (0, 0, track.trim() == "gone");
    for part in track.split(", ") {
        if let Some(n) = part.strip_prefix("ahead ") {
            result.0 = n.trim().parse().unwrap_or(0);
        } else if let Some(n) = part.strip_prefix("behind ") {
            result.1 = n.trim().parse().unwrap_or(0);
        }
    }
    result
}

/// Parses `git for-each-ref` in [`BRANCH_FORMAT`].
#[must_use]
pub fn parse_branches(raw: &str) -> Vec<Branch> {
    raw.lines()
        .filter_map(|line| {
            let fields: Vec<&str> = line.split('\x1f').collect();
            let [refname, name, hash, upstream, track, head, date, subject] = fields[..] else {
                return None;
            };
            // `origin/HEAD` is just a pointer to the remote's default branch.
            if refname.ends_with("/HEAD") {
                return None;
            }
            let (ahead, behind, gone) = parse_track(track);
            Some(Branch {
                name: name.to_string(),
                remote: refname.starts_with(REMOTES_PREFIX),
                current: head == "*",
                upstream: (!upstream.is_empty()).then(|| upstream.to_string()),
                ahead,
                behind,
                gone,
                hash: hash.to_string(),
                date: date.parse().unwrap_or(0),
                subject: subject.to_string(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_branches_and_tracking() {
        let raw = "refs/heads/main\x1fmain\x1fabc\x1forigin/main\x1fahead 1, behind 2\x1f*\x1f1700000000\x1fTip\n\
                   refs/heads/old\x1fold\x1fdef\x1forigin/old\x1fgone\x1f \x1f1600000000\x1fOld\n\
                   refs/remotes/origin/HEAD\x1forigin\x1fabc\x1f\x1f\x1f \x1f1700000000\x1fTip\n\
                   refs/remotes/origin/main\x1forigin/main\x1fabc\x1f\x1f\x1f \x1f1700000000\x1fTip\n";
        let branches = parse_branches(raw);
        assert_eq!(branches.len(), 3);
        assert!(branches[0].current && !branches[0].remote);
        assert_eq!((branches[0].ahead, branches[0].behind), (1, 2));
        assert!(branches[1].gone && !branches[1].current);
        assert!(branches[2].remote && branches[2].upstream.is_none());
        assert_eq!(branches[2].name, "origin/main");
    }
}
