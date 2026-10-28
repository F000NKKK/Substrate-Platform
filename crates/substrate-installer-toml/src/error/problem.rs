//! One field the catalog file got wrong.

use std::fmt;

use super::{Location, Reason, Table};

/// One field of the catalog file that is wrong: where it is, which entry and
/// field it is, and why it is refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    /// Where the field's value is.
    pub location: Location,
    /// The table the field is in.
    pub table: Table,
    /// The `id` of the entry, as the file spells it; none for a table without
    /// ids or an entry whose own id is the problem.
    pub entry: Option<String>,
    /// The field, as the file spells it (`source.repo`).
    pub field: &'static str,
    /// Why it is refused.
    pub reason: Reason,
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: in {}", self.location, self.table)?;
        if let Some(entry) = &self.entry {
            write!(f, " `{entry}`")?;
        }
        write!(f, ", `{}`: {}", self.field, self.reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_names_the_place_the_entry_and_the_field() {
        let problem = Problem {
            location: Location { line: 3, column: 8 },
            table: Table::Component,
            entry: Some("a".to_owned()),
            field: "source.kind",
            reason: Reason::UnsupportedKind {
                kind: "zig".to_owned(),
            },
        };
        assert_eq!(
            problem.to_string(),
            "line 3, column 8: in [[component]] `a`, `source.kind`: the source kind `zig` is not supported"
        );
    }
}
