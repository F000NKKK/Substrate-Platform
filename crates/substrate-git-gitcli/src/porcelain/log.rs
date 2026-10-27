//! `git log` as [`Commit`]s.

use substrate_git_domain::commits::Commit;

/// The `--format` that [`parse_log`] reads.
pub const LOG_FORMAT: &str = "--format=%H%x1f%h%x1f%P%x1f%an%x1f%ae%x1f%at%x1f%D%x1f%s%x1e";

/// Parses `git log` in [`LOG_FORMAT`].
#[must_use]
pub fn parse_log(raw: &str) -> Vec<Commit> {
    raw.split('\x1e')
        .filter_map(|record| {
            let fields: Vec<&str> = record.trim_start_matches('\n').split('\x1f').collect();
            let [hash, short, parents, author, email, date, refs, subject] = fields[..] else {
                return None;
            };
            Some(Commit {
                hash: hash.to_string(),
                short_hash: short.to_string(),
                parents: parents.split_whitespace().map(str::to_string).collect(),
                author: author.to_string(),
                email: email.to_string(),
                date: date.parse().unwrap_or(0),
                refs: refs
                    .split(", ")
                    .filter(|r| !r.is_empty())
                    .map(str::to_string)
                    .collect(),
                subject: subject.to_string(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_log_records() {
        let raw = "abc123\x1fabc\x1fp1 p2\x1fAda\x1fada@example.com\x1f1700000000\x1fHEAD -> main, origin/main\x1fMerge it\x1e\n\
                   def456\x1fdef\x1f\x1fBob\x1fbob@example.com\x1f1600000000\x1f\x1fInitial commit\x1e\n";
        let log = parse_log(raw);
        assert_eq!(log.len(), 2);
        assert_eq!(log[0].parents, ["p1", "p2"]);
        assert_eq!(log[0].refs, ["HEAD -> main", "origin/main"]);
        assert_eq!(log[0].subject, "Merge it");
        assert!(log[1].parents.is_empty() && log[1].refs.is_empty());
        assert_eq!(log[1].date, 1_600_000_000);
    }
}
