//! A place in a file.

use std::fmt;

/// A place in a text file: line and column, both counted from 1 (the column
/// in bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    /// The line.
    pub line: usize,
    /// The column.
    pub column: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_as_line_and_column() {
        let at = Location { line: 4, column: 7 };
        assert_eq!(at.to_string(), "line 4, column 7");
    }
}
