//! Byte offsets of a text as lines and columns.

use crate::error::Location;

/// Where each line of a text starts, to turn a byte offset into a
/// [`Location`].
#[derive(Debug)]
pub(crate) struct Lines {
    starts: Vec<usize>,
}

impl Lines {
    /// Indexes the lines of `text`.
    pub(crate) fn new(text: &str) -> Self {
        let newlines = text.match_indices('\n').map(|(at, _)| at + 1);
        Self {
            starts: std::iter::once(0).chain(newlines).collect(),
        }
    }

    /// The line and column (both from 1, in bytes) of `offset`.
    pub(crate) fn locate(&self, offset: usize) -> Location {
        let line = self.starts.partition_point(|start| *start <= offset);
        let start = self
            .starts
            .get(line.saturating_sub(1))
            .copied()
            .unwrap_or(0);
        Location {
            line,
            column: offset.saturating_sub(start) + 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offsets_become_lines_and_columns() {
        let lines = Lines::new("ab\ncd\n\nef");
        assert_eq!(lines.locate(0), Location { line: 1, column: 1 });
        assert_eq!(lines.locate(1), Location { line: 1, column: 2 });
        assert_eq!(lines.locate(3), Location { line: 2, column: 1 });
        assert_eq!(lines.locate(6), Location { line: 3, column: 1 });
        assert_eq!(lines.locate(8), Location { line: 4, column: 2 });
    }

    #[test]
    fn an_offset_past_the_end_is_on_the_last_line() {
        let lines = Lines::new("a\nb");
        assert_eq!(lines.locate(99).line, 2);
    }
}
